use anyhow::Context;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};

const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";
const BINARY_EXTENSIONS: [&str; 27] = [
    "exe", "dll", "so", "dylib", "bin", "obj", "o", "a", "lib", "png", "jpg", "jpeg", "gif", "bmp",
    "ico", "svg", "mp3", "mp4", "avi", "mov", "wav", "pdf", "zip", "tar", "gz", "bz2", "xz",
];

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

pub(super) fn is_binary_file(path: &Path) -> anyhow::Result<bool> {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        Ok(BINARY_EXTENSIONS.contains(&ext.as_str()))
    } else {
        Ok(false)
    }
}

pub(super) fn is_file_too_large(path: &Path, max_size: u64) -> anyhow::Result<bool> {
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(metadata.len() > max_size),
        Err(_) => Ok(true),
    }
}
