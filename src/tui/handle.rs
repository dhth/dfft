use super::cmd::Cmd;
use super::msg::Msg;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Cmd, event_tx: Sender<Msg>) {
    match command {
        Cmd::Dummy => {}
    }
}
