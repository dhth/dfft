use super::diff::get_unified_diff;
use super::get_ignore;
use crate::domain::{Change, ChangeKind, ModifiedResult};
use anyhow::Context;
use notify::EventKind;
use notify::RecursiveMode;
use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind};
use notify_debouncer_full::new_debouncer;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::{Sender, channel};
use tokio_util::sync::CancellationToken;
use tracing::debug;

const EVENT_CHANNEL_BUFFER: usize = 100;

pub async fn listen_for_changes(
    event_tx: Sender<Change>,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    let root = tokio::fs::canonicalize(".").await?;
    let gitignore = get_ignore(&root)?;

    let mut cache: HashMap<String, String> = HashMap::new();

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
                debug!("stopping change listener");
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

                                        let change = match tokio::fs::read_to_string(f).await {
                                            Ok(contents) => {
                                                cache.insert(f.to_string_lossy().to_string(), contents);
                                                Change {
                                                    file_path: f.clone(),
                                                    kind: ChangeKind::Created(Ok(())),
                                                }
                                            }
                                            Err(e) => Change {
                                                file_path: f.clone(),
                                                kind: ChangeKind::Created(Err(e.to_string())),
                                            },
                                        };

                                        debug!("got a change: {:?}", &change);
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

                                        let change = match tokio::fs::read_to_string(f).await {
                                            Ok(contents) => {
                                                let was_held = cache.insert(
                                                    f.to_string_lossy().to_string(),
                                                    contents.clone(),
                                                );
                                                match was_held {
                                                    Some(old) => {
                                                        if let Some(diff) = get_unified_diff(&old, &contents) {
                                                            Change {
                                                                file_path: f.clone(),
                                                                kind: ChangeKind::Modified(Ok(
                                                                    ModifiedResult::Diff(Some(diff)),
                                                                )),
                                                            }
                                                        } else {
                                                            Change {
                                                                file_path: f.clone(),
                                                                kind: ChangeKind::Modified(Ok(
                                                                    ModifiedResult::Diff(None),
                                                                )),
                                                            }
                                                        }
                                                    }
                                                    None => Change {
                                                        file_path: f.clone(),
                                                        kind: ChangeKind::Modified(Ok(
                                                            ModifiedResult::InitialSnapshot,
                                                        )),
                                                    },
                                                }
                                            }
                                            Err(e) => Change {
                                                file_path: f.clone(),
                                                kind: ChangeKind::Modified(Err(e.to_string())),
                                            },
                                        };
                                        debug!("got a change: {:?}", &change);
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
                                        let change = Change {
                                            file_path: f.clone(),
                                            kind: ChangeKind::Removed,
                                        };

                                        debug!("got a change: {:?}", &change);
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

    Ok(())
}
