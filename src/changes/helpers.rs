use anyhow::Context;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};

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
