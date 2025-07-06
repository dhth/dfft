use crate::changes::watch_for_changes;

use super::cmd::Cmd;
use super::msg::Msg;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Cmd, event_tx: Sender<Msg>) {
    match command {
        Cmd::WatchForChanges((changes_tx, cancellation_token)) => {
            tokio::spawn(async move {
                if let Err(e) = watch_for_changes(changes_tx.clone(), cancellation_token).await {
                    let _ = event_tx.try_send(Msg::WatchingFailed(e.to_string()));
                }
            });
        }
    }
}
