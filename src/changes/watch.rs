use super::helpers::*;
use crate::domain::{Change, ChangeKind, Diff, Modification};
use anyhow::Context;
use ignore::{Walk, gitignore::Gitignore};
use notify::EventKind;
use notify::RecursiveMode;
use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind};
use notify_debouncer_full::new_debouncer;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc::{Sender, channel};
use tokio_util::sync::CancellationToken;
use tracing::debug;

const EVENT_CHANNEL_BUFFER: usize = 100;
const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB

pub async fn watch_for_changes(
    root: PathBuf,
    event_tx: Sender<Change>,
    cancellation_token: CancellationToken,
    prepopulate_cache: bool,
) -> anyhow::Result<()> {
    let gitignore = get_ignore(&root)?;

    // TODO: maybe this should live in the TUI's model
    let mut cache: HashMap<String, String> = HashMap::new();

    if prepopulate_cache {
        match populate_cache(&mut cache, &gitignore, &root, 2000).await {
            Ok(count) => debug!("prepopulated cache with {} files", count),
            Err(e) => debug!("prepopulation failed: {}, continuing without cache", e),
        }
    }

    let (tx, mut rx) = channel(EVENT_CHANNEL_BUFFER);

    let runtime_handle = tokio::runtime::Handle::current();
    let mut debouncer = new_debouncer(Duration::from_millis(1000), None, move |res| {
        let tx = tx.clone();
        let runtime_handle = runtime_handle.clone();
        runtime_handle.spawn(async move {
            let _ = tx.send(res).await;
        });
    })
    .context("couldn't create notifications debouncer")?;

    debouncer
        .watch(".", RecursiveMode::Recursive)
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
                                    for f in &event.paths {
                                        if gitignore.as_ref().is_some_and(|g| {
                                            g.matched_path_or_any_parents(f, false).is_ignore()
                                        }) {
                                            continue;
                                        }
                                        let file_path = f.strip_prefix(&root).unwrap_or(f).to_string_lossy().to_string();

                                        let change = match tokio::fs::read_to_string(f).await {
                                            Ok(contents) => {
                                                cache.insert(f.to_string_lossy().to_string(), contents);
                                                Change {
                                                    file_path,
                                                    kind: ChangeKind::Created(Ok(())),
                                                }
                                            }
                                            Err(e) => Change {
                                                file_path,
                                                kind: ChangeKind::Created(Err(e.to_string())),
                                            },
                                        };

                                        let _ = event_tx.send(change).await;
                                    }
                                }
                                EventKind::Modify(ModifyKind::Data(DataChange::Content)) => {
                                    for f in &event.paths {
                                        if gitignore.as_ref().is_some_and(|g| {
                                            g.matched_path_or_any_parents(f, false).is_ignore()
                                        }) {
                                            continue;
                                        }

                                        let file_path = f.strip_prefix(&root).unwrap_or(f).to_string_lossy().to_string();
                                        let change = match tokio::fs::read_to_string(f).await {
                                            Ok(contents) => {
                                                let was_held = cache.insert(
                                                    f.to_string_lossy().to_string(),
                                                    contents.clone(),
                                                );
                                                match was_held {
                                                    Some(old) => {
                                                        if let Some(diff) = Diff::new(&old, &contents) {
                                                            Change {
                                                                file_path,
                                                                kind: ChangeKind::Modified(Ok(
                                                                    Modification::Diff(Some(diff)),
                                                                )),
                                                            }
                                                        } else {
                                                            Change {
                                                                file_path,
                                                                kind: ChangeKind::Modified(Ok(
                                                                    Modification::Diff(None),
                                                                )),
                                                            }
                                                        }
                                                    }
                                                    None => Change {
                                                        file_path,
                                                        kind: ChangeKind::Modified(Ok(
                                                            Modification::InitialSnapshot,
                                                        )),
                                                    },
                                                }
                                            }
                                            Err(e) => Change {
                                                file_path,
                                                kind: ChangeKind::Modified(Err(e.to_string())),
                                            },
                                        };
                                        let _ = event_tx.send(change).await;
                                    }
                                }
                                EventKind::Remove(RemoveKind::File) => {
                                    for f in &event.paths {
                                        if gitignore.as_ref().is_some_and(|g| {
                                            g.matched_path_or_any_parents(f, false).is_ignore()
                                        }) {
                                            continue;
                                        }

                                        cache.remove(&f.to_string_lossy().to_string());
                                        let file_path = f.strip_prefix(&root).unwrap_or(f).to_string_lossy().to_string();
                                        let change = Change {
                                            file_path,
                                            kind: ChangeKind::Removed,
                                        };

                                        let _ = event_tx.send(change).await;
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
    cache: &mut HashMap<String, String>,
    gitignore: &Option<Gitignore>,
    root: P,
    max_files: usize,
) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let mut file_count = 0;

    // TODO: build this Walk with the same ignore paths as super::helpers::get_ignore
    for result in Walk::new(root) {
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

        if let Some(ignore) = gitignore {
            if ignore.matched_path_or_any_parents(path, false).is_ignore() {
                debug!("ignored: {:?}", path);
                continue;
            }
        }

        if is_binary_file(path)? || is_file_too_large(path, MAX_FILE_SIZE)? {
            debug!("ignored binary or large file: {:?}", path);
            continue;
        }

        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                // TODO: putting the full path here is kind of a waste
                let cache_key = path.to_string_lossy().to_string();
                cache.insert(cache_key, content);
                file_count += 1;
                debug!("added to cache: {:?}", path);
            }
            Err(_) => continue,
        }
    }

    Ok(file_count)
}
