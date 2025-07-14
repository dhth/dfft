use super::common::Pane;
use super::model::Model;
use crate::domain::Change;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Msg {
    // user actions
    GoBackOrQuit,
    GoToPane(Pane),
    QuitImmediately,
    ResetList,
    ScrollDown,
    ScrollUp,
    SelectFirst,
    SelectLast,
    SelectNext,
    SelectPrevious,
    TerminalResize(u16, u16),
    ToggleFollowChanges,
    ToggleSound,
    ToggleWatching,
    // internal
    ChangeReceived(Change),
    PrepopulationFailed(String),
    PrepopulationFinished,
    WatchingFailed(String),
}

pub fn get_event_handling_msg(model: &Model, event: Event) -> Option<Msg> {
    match event {
        Event::Key(key_event) => match model.terminal_too_small {
            true => match key_event.kind {
                KeyEventKind::Press => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                    _ => None,
                },
                _ => None,
            },
            false => match key_event.kind {
                KeyEventKind::Press => match model.active_pane {
                    Pane::Changes => match key_event.code {
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::SelectNext),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::SelectPrevious),
                        KeyCode::Char('J') => Some(Msg::ScrollDown),
                        KeyCode::Char('K') => Some(Msg::ScrollUp),
                        KeyCode::Char('g') => Some(Msg::SelectFirst),
                        KeyCode::Char('G') => Some(Msg::SelectLast),
                        KeyCode::Char('f') => Some(Msg::ToggleFollowChanges),
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
                        KeyCode::Char('s') => Some(Msg::ToggleSound),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                        KeyCode::Tab | KeyCode::BackTab => Some(Msg::GoToPane(Pane::Diff)),
                        KeyCode::Char(' ') => Some(Msg::ToggleWatching),
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::QuitImmediately)
                        }
                        KeyCode::Char('?') => Some(Msg::GoToPane(Pane::Help)),
                        _ => None,
                    },
                    Pane::Diff => match key_event.code {
                        KeyCode::Char('j') => Some(Msg::SelectNext),
                        KeyCode::Char('k') => Some(Msg::SelectPrevious),
                        KeyCode::Char('g') => Some(Msg::SelectFirst),
                        KeyCode::Char('G') => Some(Msg::SelectLast),
                        KeyCode::Char('J') | KeyCode::Down => Some(Msg::ScrollDown),
                        KeyCode::Char('K') | KeyCode::Up => Some(Msg::ScrollUp),
                        KeyCode::Tab | KeyCode::BackTab => Some(Msg::GoToPane(Pane::Changes)),
                        KeyCode::Char('f') => Some(Msg::ToggleFollowChanges),
                        KeyCode::Char(' ') => Some(Msg::ToggleWatching),
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
                        KeyCode::Char('s') => Some(Msg::ToggleSound),
                        KeyCode::Char('?') => Some(Msg::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::QuitImmediately)
                        }
                        _ => None,
                    },
                    Pane::Help => match key_event.code {
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::ScrollDown),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::ScrollUp),
                        KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
                            Some(Msg::GoBackOrQuit)
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Msg::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                },
                _ => None,
            },
        },
        Event::Resize(w, h) => Some(Msg::TerminalResize(w, h)),
        _ => None,
    }
}
