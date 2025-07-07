use super::cmd::Cmd;
use super::common::*;
use super::model::*;
use super::msg::Msg;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    let mut cmds = vec![];
    match msg {
        Msg::ChangeReceived(change) => model.add_change(change),
        Msg::GoToNextListItem => model.select_next_list_item(),
        Msg::GoToPreviousListItem => model.select_previous_list_item(),
        Msg::GoToFirstListItem => model.select_first_list_item(),
        Msg::GoToLastListItem => model.select_last_list_item(),
        Msg::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);
        }
        Msg::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Msg::GoBackOrQuit => model.go_back_or_quit(),
        Msg::QuitImmediately => model.running_state = RunningState::Done,
        Msg::WatchingFailed(e) => {
            model.user_msg = Some(UserMsg::error(format!("watching for changes failed: {e}")));
        }
        Msg::UserRequestedPausingWatching => {
            model.user_msg = Some(UserMsg::info("watching paused"));
            model.pause_watching();
        }
        Msg::UserRequestedResumingWatching => {
            model.user_msg = Some(UserMsg::info("watching for changes"));
            model.regenerate_cancellation_token();
            cmds.push(Cmd::WatchForChanges((
                model.changes_tx.clone(),
                model.get_cancellation_token(),
            )));
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
