use super::cmd::Cmd;
use super::common::*;
use super::model::*;
use super::msg::Msg;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    let mut cmds = vec![];
    match msg {
        // user actions
        Msg::GoBackOrQuit => model.go_back_or_quit(),
        Msg::GoDown => model.go_down(),
        Msg::GoToBottom => model.go_to_bottom(),
        Msg::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Msg::GoToTop => model.go_to_top(),
        Msg::GoUp => model.go_up(),
        Msg::QuitImmediately => model.running_state = RunningState::Done,
        Msg::ResetList => model.reset_list(),
        Msg::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);
        }
        Msg::ToggleFollowChanges => {
            model.follow_changes = !model.follow_changes;
        }
        Msg::ToggleWatching => {
            if model.watching {
                model.pause_watching();
            } else {
                model.regenerate_cancellation_token();
                cmds.push(Cmd::WatchForChanges((
                    model.changes_tx.clone(),
                    model.get_cancellation_token(),
                )));
            }
        }
        // internal
        Msg::ChangeReceived(change) => model.add_change(change),
        Msg::WatchingFailed(e) => {
            model.watching = false;
            model.user_msg = Some(UserMsg::error(format!("watching for changes failed: {e}")));
        }
    }

    if let Some(message) = &mut model.user_msg {
        let clear = if message.frames_left == 0 {
            true
        } else {
            message.frames_left -= 1;
            false
        };

        if clear {
            model.user_msg = None;
        }
    }

    cmds
}
