use super::helpers::{get_ignore, is_file_too_large, is_path_to_be_ignored};
use crate::domain::{Change, ChangeKind, Diff, FileCache, Modification, WatchUpdate};
use anyhow::Context;
use ignore::{Walk, gitignore::Gitignore};
use notify::EventKind;
use notify::RecursiveMode;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify_debouncer_full::new_debouncer;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{Sender, channel};
use tokio_util::sync::CancellationToken;
use tracing::debug;

const EVENT_CHANNEL_BUFFER: usize = 100;
const PREPOPULATION_MAX_THRESHOLD: usize = 10000;
const FS_EVENTS_DEBOUNCE_MILLIS: u64 = 500;

// How this function behaves
// - touch file.txt                                                            = CREATED
// - echo "content" > new.txt                                                  = CREATED
// - echo "content" > existing.txt                                             = MODIFIED
// - touch existing.txt                                                        = SKIPPED
// - rm existing.txt                                                           = REMOVED
// - creation via a temp file (new.txt.tmp -> rename to new.txt)               = CREATED
// - modification via a temp file (existing.txt.tmp -> rename to existing.txt) = MODIFIED
//
// the last two are how files are sometimes created/modified by agents/editors
pub async fn watch_for_changes(
    root: PathBuf,
    cache: Arc<RwLock<FileCache>>,
    updates_tx: Sender<WatchUpdate>,
    cancellation_token: CancellationToken,
    prepopulate_cache: bool,
) -> anyhow::Result<()> {
    let gitignore = get_ignore(&root)?;

    if prepopulate_cache {
        match populate_cache(&cache, &gitignore, &root, PREPOPULATION_MAX_THRESHOLD).await {
            Ok(count) => {
                debug!("prepopulated cache with {} files", count);
                let _ = updates_tx.try_send(WatchUpdate::PrepopulationFinished);
            }
            Err(e) => {
                debug!("prepopulation failed: {}, continuing without cache", e);
                let _ = updates_tx.try_send(WatchUpdate::PrepopulationFailed(e.to_string()));
            }
        }
    }

    let (tx, mut rx) = channel(EVENT_CHANNEL_BUFFER);

    let runtime_handle = tokio::runtime::Handle::current();
    let mut debouncer = new_debouncer(
        Duration::from_millis(FS_EVENTS_DEBOUNCE_MILLIS),
        None,
        move |res| {
            let tx = tx.clone();
            let runtime_handle = runtime_handle.clone();
            runtime_handle.spawn(async move {
                let _ = tx.send(res).await;
            });
        },
    )
    .context("couldn't create notifications debouncer")?;

    debouncer
        .watch(&root, RecursiveMode::Recursive)
        .context("couldn't start watching for changes")?;
    debug!("watching for changes...");

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                break;
            }
            Some(result) = rx.recv() => {
                #[allow(clippy::single_match)]
                match result {
                    Ok(events) => {
                        for event in events {
                            match event.kind {
                                EventKind::Create(CreateKind::File) => {
                                    for event_path in &event.paths {
                                        debug!("got create event, path: {}", &event_path.to_string_lossy());
                                        if is_path_to_be_ignored(event_path, &gitignore).await {
                                            continue;
                                        }

                                        if is_file_too_large(event_path).await {
                                            continue;
                                        }

                                        let path = event_path
                                            .strip_prefix(&root)
                                            .unwrap_or(event_path)
                                            .to_string_lossy()
                                            .to_string();

                                        let change = match tokio::fs::read_to_string(event_path).await {
                                            Ok(contents) => {
                                                let was_held = {
                                                    let mut cache_guard = cache.write().await;
                                                    cache_guard.insert(&path, &contents)
                                                };
                                                match was_held {
                                                    Some(old) => {
                                                        debug!("got create event, but was already in cache, path: {}", &event_path.to_string_lossy());
                                                        Diff::new(&old, &contents).map(|diff| Change {
                                                                path,
                                                                kind: ChangeKind::Modified(Ok(
                                                                    Modification::Diff(diff),
                                                                )),
                                                            })
                                                    }
                                                    None => Some(Change {
                                                        path,
                                                        kind: ChangeKind::Created(Ok(contents)),
                                                    }),
                                                }
                                            }
                                            Err(e) => Some(Change {
                                                path,
                                                kind: ChangeKind::Created(Err(e.to_string())),
                                            }),
                                        };

                                        if let Some(c) = change {
                                            let _ = updates_tx.send(WatchUpdate::ChangeReceived(c)).await;
                                        }
                                    }
                                }
                                EventKind::Modify(modify_kind) => {
                                    for event_path in &event.paths {
                                        debug!("got modify event, path: {}", &event_path.to_string_lossy());
                                        if is_path_to_be_ignored(event_path, &gitignore).await {
                                            continue;
                                        }

                                        let path = event_path
                                            .strip_prefix(&root)
                                            .unwrap_or(event_path)
                                            .to_string_lossy()
                                            .to_string();

                                        // Renames are tricky to handle
                                        // Two events might show up for a rename, with the modify kind
                                        // ModifyKind::Any, in which case it's tricky to determine
                                        // which path no longer exists
                                        // Sometimes a file removal also shows up as a modification
                                        // So, we check if the path no longer exists, and if so,
                                        // send out a removed event.
                                        // Also, if the modification event is for a directory, we
                                        // ignore it
                                        match tokio::fs::metadata(event_path).await {
                                            Ok(metadata) => {
                                                if metadata.is_dir() {
                                                    continue;
                                                }
                                            }
                                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                                    let was_held = {
                                                        let mut cache_guard = cache.write().await;
                                                        cache_guard.remove(&path).is_some()
                                                    };

                                                    if was_held {
                                                        let change = Change {
                                                            path,
                                                            kind: ChangeKind::RemovedFile,
                                                        };

                                                        let _ = updates_tx.send(WatchUpdate::ChangeReceived(change)).await;
                                                    }

                                                continue;
                                            }
                                            Err(e) => {
                                                debug!("couldn't get metadata for path {}: {e}", &event_path.to_string_lossy());
                                                continue;
                                            }
                                        }

                                        if is_file_too_large(event_path).await {
                                            continue;
                                        }

                                        let change = match tokio::fs::read_to_string(event_path).await {
                                            Ok(contents) => {
                                                let was_held = {
                                                    let mut cache_guard = cache.write().await;
                                                    cache_guard.insert(&path, &contents)
                                                };
                                                match was_held {
                                                    Some(old) => {
                                                        Diff::new(&old, &contents).map(|diff| Change {
                                                                path,
                                                                kind: ChangeKind::Modified(Ok(
                                                                    Modification::Diff(diff),
                                                                )),
                                                            })
                                                    }
                                                    None => {
                                                        match modify_kind {
                                                            ModifyKind::Name(_) => {
                                                                // Some agents will create a temporary
                                                                // file and then rename it to the
                                                                // target file, registering a MODIFY
                                                                // event instead of a CREATE, but for
                                                                // our purposes, the file was CREATED
                                                                Some(Change {
                                                                    path,
                                                                    kind: ChangeKind::Created(Ok(contents)),
                                                                })
                                                            }
                                                            _ => Some(Change {
                                                                path,
                                                                kind: ChangeKind::Modified(Ok(Modification::InitialSnapshot)),
                                                            })
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => Some(Change {
                                                path,
                                                kind: ChangeKind::Modified(Err(e.to_string())),
                                            }),
                                        };

                                        if let Some(c) = change {
                                            let _ = updates_tx.send(WatchUpdate::ChangeReceived(c)).await;
                                        }
                                    }
                                }
                                EventKind::Remove(RemoveKind::File) => {
                                    for event_path in &event.paths {
                                        debug!("got delete event, path: {}", &event_path.to_string_lossy());
                                        if is_path_to_be_ignored(event_path, &gitignore).await {
                                            continue;
                                        }

                                        // "git checkout" emits a DELETE followed by a CREATE
                                        // if file still exists when we receive this event, we can
                                        // skip it. The arm handling CREATE events already checks
                                        // if the path in question is in the cache, ultimately
                                        // making git checkouts appear as MODIFICATIONS
                                        if tokio::fs::try_exists(event_path).await.unwrap_or(false) {
                                            continue;
                                        };

                                        let path = event_path
                                            .strip_prefix(&root)
                                            .unwrap_or(event_path)
                                            .to_string_lossy()
                                            .to_string();
                                        {
                                            let mut cache_guard = cache.write().await;
                                            cache_guard.remove(&path);
                                        }
                                        let change = Change {
                                            path,
                                            kind: ChangeKind::RemovedFile,
                                        };

                                        let _ = updates_tx.send(WatchUpdate::ChangeReceived(change)).await;
                                    }
                                }
                                EventKind::Remove(RemoveKind::Folder) => {
                                    for event_path in &event.paths {
                                        debug!("got delete event for a folder, path: {}", &event_path.to_string_lossy());
                                        if is_path_to_be_ignored(event_path, &gitignore).await {
                                            continue;
                                        }

                                        let path = event_path
                                            .strip_prefix(&root)
                                            .unwrap_or(event_path)
                                            .to_string_lossy()
                                            .to_string();

                                        let were_files_removed = {
                                            let mut cache_guard = cache.write().await;
                                            cache_guard.remove_directory(&path)
                                        };

                                        debug!("removed files from cache for deleted directory: {}", &path);

                                        if were_files_removed {
                                            let change = Change {
                                                path,
                                                kind: ChangeKind::RemovedDir,
                                            };

                                            let _ = updates_tx.send(WatchUpdate::ChangeReceived(change)).await;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    // TODO: make these errors visible
                    Err(_) => {}
                }
            }
        }
    }

    debug!("exiting change watcher");
    Ok(())
}

async fn populate_cache<P>(
    cache: &Arc<RwLock<FileCache>>,
    gitignore: &Option<Gitignore>,
    root: P,
    max_files: usize,
) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let mut file_count = 0;

    // TODO: build this Walk with the same ignore paths as super::helpers::get_ignore
    for result in Walk::new(&root) {
        if file_count >= max_files {
            debug!("prepopulate threshold exceeded");
            break;
        }

        let entry = match result {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if is_path_to_be_ignored(path, gitignore).await {
            continue;
        }

        if is_file_too_large(path).await {
            continue;
        }

        match tokio::fs::read_to_string(path).await {
            Ok(contents) => {
                let file_path = path
                    .strip_prefix(&root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                {
                    let mut cache_guard = cache.write().await;
                    cache_guard.insert(&file_path, &contents);
                }
                file_count += 1;
                debug!("added to cache: {:?}", &file_path);
            }
            Err(_) => continue,
        }
    }

    Ok(file_count)
}
