use super::cmd::Cmd;
use super::common::*;
use super::model::*;
use super::msg::Msg;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    let mut cmds = vec![];
    match msg {
        // user actions
        Msg::GoBackOrQuit => model.go_back_or_quit(),
        Msg::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
            if pane == Pane::Help {
                model.reset_help_scroll();
            }
        }
        Msg::QuitImmediately => model.running_state = RunningState::Done,
        Msg::ResetList => model.reset_list(),
        Msg::ScrollDown => model.scroll_down(),
        Msg::ScrollUp => model.scroll_up(),
        Msg::SelectFirst => model.select_first(),
        Msg::SelectLast => model.select_last(),
        Msg::SelectNext => model.select_next(),
        Msg::SelectPrevious => model.select_previous(),
        Msg::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);

            model.compute_max_help_scroll_available();
            model.compute_max_diff_scroll_available();
        }
        Msg::ToggleFollowChanges => {
            model.follow_changes = !model.follow_changes;
        }
        Msg::ToggleWatching => {
            if model.watching {
                model.pause_watching();
            } else {
                model.regenerate_cancellation_token();
                cmds.push(Cmd::WatchForChanges {
                    root: model.root.clone(),
                    cache: model.cache(),
                    sender: model.watch_updates_tx.clone(),
                    cancellation_token: model.get_cancellation_token(),
                    prepopulate_cache: false,
                });
            }
        }
        // internal
        Msg::ChangeReceived(change) => model.add_change(change),
        Msg::PrepopulationFailed(e) => {
            model.watching = false;
            model.user_msg = Some(UserMsg::error(format!("prepopulating changes failed: {e}")));
        }
        // this is just to trigger a render of TUI
        Msg::PrepopulationFinished => {}
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
