use crate::domain::WatchUpdate;
use std::path::PathBuf;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(super) enum Cmd {
    WatchForChanges {
        root: PathBuf,
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
