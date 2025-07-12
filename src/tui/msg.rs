use super::common::Pane;
use super::model::Model;
use crate::domain::Change;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Msg {
    // user actions
    GoBackOrQuit,
    SelectNext,
    SelectLast,
    GoToPane(Pane),
    SelectFirst,
    SelectPrevious,
    QuitImmediately,
    ResetList,
    ScrollDown,
    ScrollUp,
    TerminalResize(u16, u16),
    ToggleFollowChanges,
    ToggleWatching,
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
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::SelectNext),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::SelectPrevious),
                        KeyCode::Char('g') => Some(Msg::SelectFirst),
                        KeyCode::Char('G') => Some(Msg::SelectLast),
                        KeyCode::Char('f') => Some(Msg::ToggleFollowChanges),
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
                        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                        KeyCode::Tab
                        | KeyCode::BackTab
                        | KeyCode::Char('J')
                        | KeyCode::Char('K') => Some(Msg::GoToPane(Pane::Diff)),
                        KeyCode::Char(' ') => Some(Msg::ToggleWatching),
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::QuitImmediately)
                        }
                        KeyCode::Char('?') => Some(Msg::GoToPane(Pane::Help)),
                        _ => None,
                    },
                    Pane::Diff => match key_event.code {
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::ScrollDown),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::ScrollUp),
                        KeyCode::Char('l') | KeyCode::Right => Some(Msg::SelectNext),
                        KeyCode::Char('h') | KeyCode::Left => Some(Msg::SelectPrevious),
                        KeyCode::Tab
                        | KeyCode::BackTab
                        | KeyCode::Char('J')
                        | KeyCode::Char('K') => Some(Msg::GoToPane(Pane::ChangesList)),
                        KeyCode::Char('f') => Some(Msg::ToggleFollowChanges),
                        KeyCode::Char(' ') => Some(Msg::ToggleWatching),
                        KeyCode::Char('r') if key_event.modifiers == KeyModifiers::CONTROL => {
                            Some(Msg::ResetList)
                        }
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
