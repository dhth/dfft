#![allow(dead_code)]
mod diff;
mod tui;

use anyhow::Context;
use console::Style;
use diff::get_diff;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use notify::EventKind;
use notify::RecursiveMode;
use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind};
use notify_debouncer_full::new_debouncer;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const READ_LABEL: &str = "   READ   ";
const CREATED_LABEL: &str = " CREATED  ";
const MODIFIED_LABEL: &str = " MODIFIED ";
const REMOVED_LABEL: &str = " REMOVED  ";
const ERROR_LABEL: &str = "  ERROR   ";
const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tui::run().await?;

    Ok(())
}

fn main2() -> anyhow::Result<()> {
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
    println!("watching for changes...\n");

    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| match event.kind {
                EventKind::Access(_) => {
                    for f in &event.paths {
                        if gitignore.as_ref().is_some_and(|g| g.matched_path_or_any_parents(f, false).is_ignore()) {
                            continue;
                        }

                        let path_str = f.strip_prefix(&root).unwrap_or(f).to_string_lossy();
                        println!("{} {path_str}", &read);
                    }
                }
                EventKind::Create(CreateKind::File) => {
                    for f in &event.paths {
                        if gitignore.as_ref().is_some_and(|g| g.matched_path_or_any_parents(f, false).is_ignore()) {
                            continue;
                        }

                        let path_str = f.strip_prefix(&root).unwrap_or(f).to_string_lossy();

                        match fs::read_to_string(f) {
                            Ok(contents) => {
                                cache.insert(f.to_string_lossy().to_string(), contents);
                                println!("{} {path_str}", &created);
                            }
                            Err(e) => {
                                println!("{} {path_str}: {e}", &error);
                            }
                        }
                    }
                }
                EventKind::Modify(ModifyKind::Data(DataChange::Content)) => {
                    for f in &event.paths {
                        if gitignore.as_ref().is_some_and(|g| g.matched_path_or_any_parents(f, false).is_ignore()) {
                            continue;
                        }

                        let path_str = f.strip_prefix(&root).unwrap_or(f).to_string_lossy();

                        if let Ok(contents) = fs::read_to_string(f).context("couldn't read file contents") {
                        let was_held = cache.insert(f.to_string_lossy().to_string(), contents.clone());
                        println!("{} {path_str}", &modified);
                        match was_held {
                            Some(old) => {
                                if let Some(diff) = get_diff(&old, &contents) {
                                    println!("\n{diff}\n");
                                }
                            }
                            None => println!("\nfirst snapshot captured, diffs will be available from now onwards\n"),
                        }
                        }
                    }
                }
                EventKind::Remove(RemoveKind::File) => {
                    for f in &event.paths {
                        if gitignore.as_ref().is_some_and(|g| g.matched_path_or_any_parents(f, false).is_ignore()) {
                            continue;
                        }

                        let path_str = f.strip_prefix(&root).unwrap_or(f).to_string_lossy();

                        cache.remove(&f.to_string_lossy().to_string());
                            println!("{} {path_str}", &removed);
                    }
                }
                _ => {}
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
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
