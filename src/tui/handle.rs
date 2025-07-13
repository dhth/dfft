use crate::changes::watch_for_changes;

use super::cmd::Cmd;
use super::msg::Msg;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Cmd, event_tx: Sender<Msg>) {
    match command {
        Cmd::WatchForChanges {
            root,
            cache,
            sender,
            cancellation_token,
            prepopulate_cache,
        } => {
            tokio::spawn(async move {
                if let Err(e) = watch_for_changes(
                    root,
                    cache,
                    sender.clone(),
                    cancellation_token,
                    prepopulate_cache,
                )
                .await
                {
                    let _ = event_tx.try_send(Msg::PrepopulationFailed(e.to_string()));
                }
            });
        }
    }
}
