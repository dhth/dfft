use super::common::Pane;
use super::model::Model;
use crate::domain::Change;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Msg {
    // user actions
    GoBackOrQuit,
    GoToFirstListItem,
    GoToLastListItem,
    GoToNextListItem,
    GoToPane(Pane),
    GoToPreviousListItem,
    PauseWatching,
    QuitImmediately,
    ResetList,
    ResumeWatching,
    TerminalResize(u16, u16),
    // internal
    ChangeReceived(Change),
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
                    Pane::ChangesList => match key_event.code {
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::GoToNextListItem),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::GoToPreviousListItem),
                        KeyCode::Char('g') => Some(Msg::GoToFirstListItem),
                        KeyCode::Char('G') => Some(Msg::GoToLastListItem),
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
                        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                        KeyCode::Tab | KeyCode::BackTab => Some(Msg::GoToPane(Pane::Diff)),
                        KeyCode::Char(' ') => {
                            if model.watching {
                                Some(Msg::PauseWatching)
                            } else {
                                Some(Msg::ResumeWatching)
                            }
                        }
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::QuitImmediately)
                        }
                        _ => None,
                    },
                    Pane::Diff => match key_event.code {
                        KeyCode::Tab | KeyCode::BackTab => Some(Msg::GoToPane(Pane::ChangesList)),
                        KeyCode::Char(' ') => {
                            if model.watching {
                                Some(Msg::PauseWatching)
                            } else {
                                Some(Msg::ResumeWatching)
                            }
                        }
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::QuitImmediately)
                        }
                        _ => None,
                    },
                    Pane::Help => match key_event.code {
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
