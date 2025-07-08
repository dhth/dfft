use super::cmd::Cmd;
use super::common::*;
use super::model::*;
use super::msg::Msg;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    let mut cmds = vec![];
    match msg {
        // user actions
        Msg::GoBackOrQuit => model.go_back_or_quit(),
        Msg::GoToFirstListItem => model.select_first_list_item(),
        Msg::GoToLastListItem => model.select_last_list_item(),
        Msg::GoToNextListItem => model.select_next_list_item(),
        Msg::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Msg::GoToPreviousListItem => model.select_previous_list_item(),
        Msg::PauseWatching => {
            model.pause_watching();
        }
        Msg::QuitImmediately => model.running_state = RunningState::Done,
        Msg::ResetList => model.reset_list(),
        Msg::ResumeWatching => {
            model.regenerate_cancellation_token();
            cmds.push(Cmd::WatchForChanges((
                model.changes_tx.clone(),
                model.get_cancellation_token(),
            )));
        }
        Msg::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);
        }
        Msg::ToggleFollowChanges => {
            model.follow_changes = !model.follow_changes;
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
