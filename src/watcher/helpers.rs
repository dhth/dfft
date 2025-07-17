use super::consts::EXTENSIONS_TO_IGNORE;
use anyhow::Context;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use std::path::Path;

const GITIGNORE_PATH: &str = ".gitignore";
const DFFTIGNORE_PATH: &str = ".dfftignore";
const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB
const VCS_DIRS: [&str; 4] = [".git", ".jj", ".hg", ".svn"];

pub(super) fn get_ignore<P>(root: P) -> anyhow::Result<Option<Gitignore>>
where
    P: AsRef<Path>,
{
    let root_path = root.as_ref();
    let ignore_paths = vec![
        root_path.join(".git").join("info").join("exclude"),
        root_path.join(GITIGNORE_PATH),
        root_path.join(DFFTIGNORE_PATH),
    ];

    let mut builder = GitignoreBuilder::new(&root);
    for vcs_dir in VCS_DIRS {
        let _ = builder.add_line(None, vcs_dir);
    }

    let mut skip = true;
    for path in &ignore_paths {
        if path.exists() && path.is_file() {
            skip = false;
            if let Some(e) = builder.add(path) {
                return Err(anyhow::anyhow!(
                    r#"couldn't parse file "{}": {e}"#,
                    path.to_string_lossy()
                ));
            }
        }
    }

    if skip {
        return Ok(None);
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

pub async fn is_file_too_large<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    tokio::fs::metadata(path)
        .await
        .map(|m| m.len() > MAX_FILE_SIZE)
        .unwrap_or(true)
}
