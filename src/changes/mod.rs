use crate::domain::{Change, ChangeKind, ModifiedResult};

use crate::diff::get_diff2;
use anyhow::Context;
use console::Style;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use notify::EventKind;
use notify::RecursiveMode;
use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind};
use notify_debouncer_full::{DebouncedEvent, new_debouncer};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tracing::info;

const READ_LABEL: &str = "   READ   ";
const CREATED_LABEL: &str = " CREATED  ";
const MODIFIED_LABEL: &str = " MODIFIED ";
const REMOVED_LABEL: &str = " REMOVED  ";
const ERROR_LABEL: &str = "  ERROR   ";
const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";

pub async fn listen_for_changes(event_tx: Sender<Change>) -> anyhow::Result<()> {
    let root = std::fs::canonicalize(".")?;
    let gitignore = get_ignore(&root)?;

    let read = Style::new().black().on_white().bold().apply_to(READ_LABEL);
    let created = Style::new()
        .black()
        .on_green()
        .bold()
        .apply_to(CREATED_LABEL);
    let modified = Style::new()
        .black()
        .on_yellow()
        .bold()
        .apply_to(MODIFIED_LABEL);
    let removed = Style::new().black().on_red().bold().apply_to(REMOVED_LABEL);
    let error = Style::new()
        .black()
        .on_color256(244)
        .bold()
        .apply_to(ERROR_LABEL);
    let mut cache: HashMap<String, String> = HashMap::new();

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(1000), None, tx)
        .context("couldn't create notifications debouncer")?;

    debouncer
        .watch(".", RecursiveMode::Recursive)
        .context("couldn't start watching for changes")?;
    info!("watching for changes...");

    for result in rx {
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

                                let change = match fs::read_to_string(f) {
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

                                info!("got a change: {:?}", &change);
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

                                let change = match fs::read_to_string(f) {
                                    Ok(contents) => {
                                        let was_held = cache.insert(
                                            f.to_string_lossy().to_string(),
                                            contents.clone(),
                                        );
                                        match was_held {
                                            Some(old) => {
                                                if let Some(diff) = get_diff2(&old, &contents) {
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
                                info!("got a change: {:?}", &change);
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

                                info!("got a change: {:?}", &change);
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

    Ok(())
}

fn get_ignore<P>(root: P) -> anyhow::Result<Option<Gitignore>>
where
    P: AsRef<Path>,
{
    let gitignore_path = PathBuf::from(GITIGNORE_PATH);
    let dfftignore_path = PathBuf::from(DFFTIGNORE_PATH);

    if !gitignore_path.exists()
        && !gitignore_path.is_file()
        && !dfftignore_path.exists()
        && !dfftignore_path.is_file()
    {
        return Ok(None);
    }

    let mut builder = GitignoreBuilder::new(&root);

    if gitignore_path.exists() && gitignore_path.is_file() {
        if let Some(e) = builder.add(&gitignore_path) {
            return Err(anyhow::anyhow!("couldn't parse .gitignore file: {e}"));
        }
    }

    if dfftignore_path.exists() && dfftignore_path.is_file() {
        if let Some(e) = builder.add(&dfftignore_path) {
            return Err(anyhow::anyhow!("couldn't parse .dfftignore file: {e}"));
        }
    }

    Ok(Some(
        builder
            .build()
            .context("couldn't set up a matcher for ignoring files")?,
    ))
}
