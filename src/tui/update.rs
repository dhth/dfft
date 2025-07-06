use super::cmd::Cmd;
use super::common::*;
use super::model::*;
use super::msg::Msg;
use std::time::Instant;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    let mut cmds = Vec::new();

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
        Msg::ClearUserMessage => {
            let now = Instant::now();
            let reset_message = match &model.user_message {
                Some(message) => match message {
                    UserMessage::Info(_, instant) => {
                        now.saturating_duration_since(instant.to_owned()).as_secs()
                            > CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS
                    }
                    UserMessage::Error(_, instant) => {
                        now.saturating_duration_since(instant.to_owned()).as_secs()
                            > CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS
                    }
                },
                None => false,
            };

            if reset_message {
                model.user_message = None;
            }
        }
        Msg::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Msg::GoBackOrQuit => model.go_back_or_quit(),
        Msg::QuitImmediately => model.running_state = RunningState::Done,
    }

    cmds
}
