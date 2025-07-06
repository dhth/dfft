use super::diff::get_unified_diff;
use crate::domain::{Change, ChangeKind, ModifiedResult};
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
use tokio::sync::mpsc::{Sender, channel};
use tokio_util::sync::CancellationToken;
use tracing::debug;

const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";

pub(super) fn get_ignore<P>(root: P) -> anyhow::Result<Option<Gitignore>>
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
