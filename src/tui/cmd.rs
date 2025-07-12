use crate::domain::{FileCache, WatchUpdate};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(super) enum Cmd {
    WatchForChanges {
        root: PathBuf,
        cache: Arc<RwLock<FileCache>>,
        sender: Sender<WatchUpdate>,
        cancellation_token: CancellationToken,
        prepopulate_cache: bool,
    },
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::WatchForChanges { .. } => write!(f, "watch for changes"),
        }
    }
}
