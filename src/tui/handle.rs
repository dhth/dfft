use super::cmd::Cmd;
use super::msg::Msg;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Cmd, _: Sender<Msg>) {
    match command {
        Cmd::Dummy => {}
    }
}
