use super::common::*;
use crate::domain::{Change, ChangeKind};
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{ListItem, ListState},
};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;

const USER_MESSAGE_DEFAULT_FRAMES: u16 = 4;
const CREATED_LABEL: &str = " created  ";
const MODIFIED_LABEL: &str = " modified ";
const REMOVED_LABEL: &str = " removed  ";
const ERROR_LABEL: &str = "  error   ";

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum MessageKind {
    Info,
    Error,
}

pub struct UserMsg {
    pub frames_left: u16,
    pub value: String,
    pub kind: MessageKind,
}

#[allow(unused)]
impl UserMsg {
    pub(super) fn info<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        UserMsg {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.into(),
            kind: MessageKind::Info,
        }
    }
    pub(super) fn error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        UserMsg {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.into(),
            kind: MessageKind::Error,
        }
    }

    #[allow(unused)]
    pub(super) fn with_frames_left(mut self, frames_left: u16) -> Self {
        self.frames_left = frames_left;
        self
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
        let state = ListState::default();
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
        let (label, color) = match value.change.kind {
            ChangeKind::Created(Ok(_)) => (CREATED_LABEL, ADDED_COLOR),

            ChangeKind::Created(Err(_)) => (ERROR_LABEL, ERROR_COLOR),
            ChangeKind::Modified(Ok(_)) => (MODIFIED_LABEL, MODIFIED_COLOR),
            ChangeKind::Modified(Err(_)) => (ERROR_LABEL, ERROR_COLOR),
            ChangeKind::Removed => (REMOVED_LABEL, DIFF_REMOVED_COLOR),
        };

        let line = Line::from(vec![
            Span::styled(label, Style::default().bg(color).black().bold()),
            " ".into(),
            Span::from(value.change.file_path.clone()),
        ]);

        ListItem::new(line)
    }
}

pub struct Model {
    pub active_pane: Pane,
    pub watching: bool,
    pub changes: Changes,
    pub last_active_pane: Option<Pane>,
    pub running_state: RunningState,
    pub user_msg: Option<UserMsg>,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub render_counter: u64,
    pub event_counter: u64,
    pub changes_tx: Sender<Change>,
    pub changes_rx: Receiver<Change>,
    cancellation_token: CancellationToken,
    pub debug: bool,
}

impl Model {
    pub fn new(terminal_dimensions: TerminalDimensions, watching: bool, debug: bool) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;

        let (changes_tx, changes_rx) = mpsc::channel::<Change>(100);

        Self {
            active_pane: Pane::ChangesList,
            watching,
            changes: Changes::new(),
            last_active_pane: None,
            running_state: RunningState::Running,
            user_msg: None,
            terminal_dimensions,
            terminal_too_small,
            render_counter: 0,
            event_counter: 0,
            changes_tx,
            changes_rx,
            cancellation_token: CancellationToken::new(),
            debug,
        }
    }

    pub(super) fn go_back_or_quit(&mut self) {
        let active_pane = Some(self.active_pane);
        match self.active_pane {
            Pane::ChangesList => self.running_state = RunningState::Done,
            Pane::Diff => {}
            Pane::Help => {}
        }

        self.last_active_pane = active_pane;
    }

    pub(super) fn select_next_list_item(&mut self) {
        if self.active_pane == Pane::ChangesList {
            if self.changes.state.selected().is_none() {
                return;
            }

            if let Some(i) = self.changes.state.selected()
                && i == self.changes.items.len() - 1
            {
                return;
            }

            self.changes.state.select_next();
        }
    }

    pub(super) fn select_previous_list_item(&mut self) {
        if self.active_pane == Pane::ChangesList {
            if self.changes.state.selected().is_none() {
                return;
            }

            if let Some(i) = self.changes.state.selected()
                && i == 0
            {
                return;
            }

            self.changes.state.select_previous();
        }
    }

    pub(super) fn select_first_list_item(&mut self) {
        if self.active_pane == Pane::ChangesList {
            if self.changes.state.selected().is_none() {
                return;
            }

            if let Some(i) = self.changes.state.selected()
                && i == 0
            {
                return;
            }

            self.changes.state.select_first();
        };
    }
    pub(super) fn select_last_list_item(&mut self) {
        if self.active_pane == Pane::ChangesList {
            if self.changes.state.selected().is_none() {
                return;
            }

            if let Some(i) = self.changes.state.selected()
                && i == self.changes.items.len() - 1
            {
                return;
            }

            self.changes.state.select_last()
        }
    }

    pub(super) fn add_change(&mut self, change: Change) {
        self.changes.append(change);
    }

    pub(super) fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    pub(super) fn reset_list(&mut self) {
        self.changes = Changes::new();
    }

    pub(super) fn pause_watching(&mut self) {
        self.cancellation_token.cancel();
        self.watching = false;
    }

    pub(super) fn regenerate_cancellation_token(&mut self) {
        self.cancellation_token = CancellationToken::new();
        self.watching = true;
    }
}
