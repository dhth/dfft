use super::common::*;
use crate::domain::{Change, ChangeKind, ModifiedResult};
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};
use std::time::Instant;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum UserMessage {
    Info(String, Instant),
    Error(String, Instant),
}

impl UserMessage {
    pub(super) fn info(message: &str) -> Self {
        UserMessage::Info(message.to_string(), Instant::now())
    }
    pub(super) fn error(message: &str) -> Self {
        UserMessage::Error(message.to_string(), Instant::now())
    }
}

#[derive(Debug)]
pub struct ChangeItem {
    pub change: Change,
}

#[derive(Debug)]
pub struct Changes {
    pub items: Vec<ChangeItem>,
    pub state: ListState,
}

impl Changes {
    fn new() -> Self {
        let mut state = ListState::default();
        let items = vec![];

        Self { items, state }
    }
}

impl Changes {
    pub fn append(&mut self, change: Change) {
        let item = ChangeItem { change };
        self.items.push(item);

        let selected = match self.state.selected() {
            Some(i) => Some(i),
            None => {
                if self.items.is_empty() {
                    None
                } else {
                    Some(0)
                }
            }
        };

        self.state = self.state.clone().with_selected(selected);
    }
}

impl From<&ChangeItem> for ListItem<'_> {
    fn from(value: &ChangeItem) -> Self {
        let line = Line::from(value.change.file_path.to_string_lossy().to_string());
        ListItem::new(line)
    }
}

pub struct Model {
    pub active_pane: Pane,
    pub changes: Changes,
    pub last_active_pane: Option<Pane>,
    pub running_state: RunningState,
    pub user_message: Option<UserMessage>,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub render_counter: u64,
    pub event_counter: u64,
    pub debug: bool,
}

impl Model {
    pub fn new(terminal_dimensions: TerminalDimensions, debug: bool) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;

        Self {
            active_pane: Pane::ChangesList,
            changes: Changes::new(),
            last_active_pane: None,
            running_state: RunningState::Running,
            user_message: None,
            terminal_dimensions,
            terminal_too_small,
            render_counter: 0,
            event_counter: 0,
            debug,
        }
    }

    pub(super) fn go_back_or_quit(&mut self) {
        let active_pane = Some(self.active_pane);
        match self.active_pane {
            Pane::ChangesList => self.running_state = RunningState::Done,
            Pane::Help => {}
        }

        self.last_active_pane = active_pane;
    }

    pub(super) fn select_next_list_item(&mut self) {
        match self.active_pane {
            Pane::ChangesList => self.changes.state.select_next(),
            Pane::Help => {}
        }
    }

    pub(super) fn select_previous_list_item(&mut self) {
        match self.active_pane {
            Pane::ChangesList => self.changes.state.select_previous(),
            Pane::Help => {}
        }
    }

    pub(super) fn select_first_list_item(&mut self) {
        match self.active_pane {
            Pane::ChangesList => self.changes.state.select_first(),
            Pane::Help => {}
        }
    }
    pub(super) fn select_last_list_item(&mut self) {
        if self.active_pane == Pane::ChangesList {
            self.changes.state.select_last()
        }
    }
    pub(super) fn add_change(&mut self, change: Change) {
        self.changes.append(change);
    }
}
