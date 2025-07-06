use super::cmd::Cmd;
use super::msg::Msg;
// use crate::changes::listen_for_changes;
// use crate::domain::Change;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Cmd, event_tx: Sender<Msg>) {
    match command {
        Cmd::Dummy => {}
    }
}

// pub(super) async fn start_listening_for_changes(event_tx: Sender<Change>) {
//     tokio::spawn(async move {
//         listen_for_changes(event_tx).await;
//     });
// }
