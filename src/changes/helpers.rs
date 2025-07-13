use super::consts::EXTENSIONS_TO_IGNORE;
use anyhow::Context;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};

const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";
const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB

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

pub(super) async fn is_file_to_be_ignored<P>(path: P, gitignore: &Option<Gitignore>) -> bool
where
    P: AsRef<Path>,
{
    if gitignore
        .as_ref()
        .is_some_and(|g| g.matched_path_or_any_parents(&path, false).is_ignore())
    {
        return true;
    }

    // other reasons for ignoring files
    if is_file_too_large(&path, MAX_FILE_SIZE)
        .await
        .unwrap_or(true)
    {
        return true;
    }

    if is_extension_to_be_ignored(&path).unwrap_or(true) {
        return true;
    }

    false
}

fn is_extension_to_be_ignored<P>(path: P) -> anyhow::Result<bool>
where
    P: AsRef<Path>,
{
    if let Some(ext) = path.as_ref().extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        Ok(EXTENSIONS_TO_IGNORE.contains(&ext.as_str()))
    } else {
        Ok(false)
    }
}

async fn is_file_too_large<P>(path: P, max_size: u64) -> anyhow::Result<bool>
where
    P: AsRef<Path>,
{
    match tokio::fs::metadata(path).await {
        Ok(metadata) => Ok(metadata.len() > max_size),
        Err(_) => Ok(true),
    }
}
