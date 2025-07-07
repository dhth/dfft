use crate::domain::Change;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(super) enum Cmd {
    WatchForChanges((Sender<Change>, CancellationToken)),
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::WatchForChanges(_) => write!(f, "watch for changes"),
        }
    }
}
