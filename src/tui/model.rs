use super::TuiBehaviours;
use super::common::*;
use crate::domain::{Change, ChangeKind, FileCache, Modification, WatchUpdate};
use crate::notifs::AudioPlayer;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{ListItem, ListState},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{RwLock, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::warn;

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
    pub fn append(&mut self, change: Change, select_newly_added: bool) {
        let item = ChangeItem { change };
        self.items.push(item);

        let selected = match self.state.selected() {
            Some(i) => {
                if select_newly_added {
                    Some(self.items.len() - 1)
                } else {
                    Some(i)
                }
            }
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
            ChangeKind::Created(Ok(_)) => (CREATED_LABEL, ADDITION_COLOR),

            ChangeKind::Created(Err(_)) => (ERROR_LABEL, FILE_ERROR_COLOR),
            ChangeKind::Modified(Ok(_)) => (MODIFIED_LABEL, MODIFICATION_COLOR),
            ChangeKind::Modified(Err(_)) => (ERROR_LABEL, FILE_ERROR_COLOR),
            ChangeKind::Removed => (REMOVED_LABEL, SUBTRACTION_COLOR),
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
    pub behaviours: TuiBehaviours,
    pub root: PathBuf,
    cache: Arc<RwLock<FileCache>>,
    pub active_pane: Pane,
    pub changes: Changes,
    pub last_active_pane: Option<Pane>,
    pub running_state: RunningState,
    pub user_msg: Option<UserMsg>,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub render_counter: u64,
    pub event_counter: u64,
    pub watch_counter: u64,
    pub watch_updates_tx: Sender<WatchUpdate>,
    pub watch_updates_rx: Receiver<WatchUpdate>,
    cancellation_token: CancellationToken,
    pub debug: bool,
    pub help_scroll: usize,
    pub help_line_count: usize,
    pub max_help_scroll_available: usize,
    pub diff_scroll: usize,
    pub max_diff_scroll_available: usize,
    audio_player: Result<Option<AudioPlayer>, ()>,
}

impl Model {
    pub fn new(
        behaviours: TuiBehaviours,
        root: PathBuf,
        terminal_dimensions: TerminalDimensions,
        debug: bool,
    ) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;

        let (changes_tx, changes_rx) = mpsc::channel::<WatchUpdate>(100);

        let audio_player = if behaviours.play_sound {
            match AudioPlayer::new() {
                Ok(ap) => Ok(Some(ap)),
                Err(e) => {
                    warn!("couldn't set up audio player: {e}");
                    Err(())
                }
            }
        } else {
            Ok(None)
        };

        let mut model = Model {
            behaviours,
            root,
            cache: Arc::new(RwLock::new(FileCache::new())),
            active_pane: Pane::Diff,
            changes: Changes::new(),
            last_active_pane: None,
            running_state: RunningState::Running,
            user_msg: None,
            terminal_dimensions,
            terminal_too_small,
            render_counter: 0,
            event_counter: 0,
            watch_counter: 0,
            watch_updates_tx: changes_tx,
            watch_updates_rx: changes_rx,
            cancellation_token: CancellationToken::new(),
            debug,
            help_scroll: 0,
            help_line_count: HELP_CONTENT.lines().count(),
            max_help_scroll_available: 0,
            diff_scroll: 0,
            max_diff_scroll_available: 0,
            audio_player,
        };

        model.compute_max_help_scroll_available();
        if model.is_sound_unavailable() {
            model.user_msg = Some(UserMsg::error("couldn't set up sound notifications"));
        }

        model
    }

    pub(super) fn go_back_or_quit(&mut self) {
        let active_pane = Some(self.active_pane);
        match self.active_pane {
            Pane::Changes => self.active_pane = Pane::Diff,
            Pane::Diff => self.running_state = RunningState::Done,
            Pane::Help => match self.last_active_pane {
                Some(p) => self.active_pane = p,
                None => self.active_pane = Pane::Changes,
            },
        }

        self.last_active_pane = active_pane;
    }

    pub(super) fn select_next(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                if self.changes.items.is_empty() {
                    return;
                }

                if let Some(i) = self.changes.state.selected()
                    && i == self.changes.items.len() - 1
                {
                    return;
                }

                self.changes.state.select_next();
                self.compute_max_diff_scroll_available();
                self.reset_diff_scroll();
            }
            Pane::Help => {}
        }
    }

    pub(super) fn select_previous(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                if self.changes.items.is_empty() {
                    return;
                }

                if let Some(i) = self.changes.state.selected()
                    && i == 0
                {
                    return;
                }

                self.changes.state.select_previous();
                self.compute_max_diff_scroll_available();
                self.reset_diff_scroll();
            }
            Pane::Help => {}
        }
    }

    pub(super) fn select_first(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                if self.changes.items.is_empty() {
                    return;
                }

                if let Some(i) = self.changes.state.selected()
                    && i == 0
                {
                    return;
                }

                self.changes.state.select_first();
                self.compute_max_diff_scroll_available();
                self.reset_diff_scroll();
            }
            _ => {}
        }
    }
    pub(super) fn select_last(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                if self.changes.items.is_empty() {
                    return;
                }

                let last_index = self.changes.items.len() - 1;
                if let Some(i) = self.changes.state.selected()
                    && i == last_index
                {
                    return;
                }

                self.changes.state.select(Some(last_index));
                self.compute_max_diff_scroll_available();
                self.reset_diff_scroll();
            }
            _ => {}
        }
    }

    pub(super) fn scroll_down(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                self.scroll_diff_down();
            }
            Pane::Help => {
                self.scroll_help_down();
            }
        }
    }

    pub(super) fn scroll_up(&mut self) {
        match self.active_pane {
            Pane::Changes | Pane::Diff => {
                self.scroll_diff_up();
            }
            Pane::Help => {
                self.scroll_help_up();
            }
        }
    }

    pub(super) fn add_change(&mut self, change: Change) {
        if let Ok(Some(ap)) = &self.audio_player
            && self.behaviours.play_sound
        {
            ap.play_change_sound(&change.kind);
        }

        self.changes.append(change, self.behaviours.follow_changes);

        if self.behaviours.follow_changes || self.changes.items.len() == 1 {
            self.reset_diff_scroll();
            self.compute_max_diff_scroll_available();
        }
    }

    pub(super) fn play_error_sound(&self) {
        if let Ok(Some(ap)) = &self.audio_player
            && self.behaviours.play_sound
        {
            ap.play_error_sound();
        }
    }

    pub(super) fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    pub(super) fn reset_list(&mut self) {
        self.changes = Changes::new();
        self.reset_diff_scroll();
        self.compute_max_diff_scroll_available();
    }

    pub(super) fn pause_watching(&mut self) {
        self.cancellation_token.cancel();
        self.behaviours.watch = false;
    }

    pub(super) fn regenerate_cancellation_token(&mut self) {
        self.cancellation_token = CancellationToken::new();
        self.behaviours.watch = true;
    }

    pub(super) fn scroll_help_down(&mut self) {
        if self.help_scroll < self.max_help_scroll_available {
            self.help_scroll += 1;
        }
    }

    pub(super) fn scroll_help_up(&mut self) {
        self.help_scroll = self.help_scroll.saturating_sub(1);
    }

    pub(super) fn scroll_diff_down(&mut self) {
        if self.changes.state.selected().is_none() {
            return;
        }

        if self.diff_scroll < self.max_diff_scroll_available {
            self.diff_scroll += 1;
        }
    }

    pub(super) fn scroll_diff_up(&mut self) {
        if self.changes.state.selected().is_none() {
            return;
        }

        self.diff_scroll = self.diff_scroll.saturating_sub(1);
    }

    pub(super) fn reset_help_scroll(&mut self) {
        self.help_scroll = 0;
    }

    pub(super) fn reset_diff_scroll(&mut self) {
        self.diff_scroll = 0;
    }

    // kinda weird that this model method relies on knowledge of the view, but oh well
    pub(super) fn compute_max_help_scroll_available(&mut self) {
        // -help------------------------------
        // |                                 | <- padding top
        // | 1    keymaps                    |
        // | 2    aaaaa                      |
        // | 3    aaaaa                      | <- available height = 10 - 4 = 6
        // | 4    aaaaa                      |
        // | 5    aaaaa                      |
        // | 6    aaaaa                      |
        // |---------------------------------| <- help border
        // | dfft                            | __ lower edge of screen
        // | 7    aaaaa | <- not visible
        // | 8    aaaaa |
        // | 9    aaaaa | <- max scroll available = 3
        // |            |
        // |            |
        // |            |
        // --------------

        // fully scrolled view

        // -help------------------------------
        // |                                 |
        // | 4    aaaaa                      |
        // | 5    aaaaa                      |
        // | 6    aaaaa                      |
        // | 7    aaaaa                      |
        // | 8    aaaaa                      |
        // | 9    aaaaa                      |
        // |---------------------------------|
        // | dfft                            |

        self.max_help_scroll_available = if self.terminal_too_small {
            0
        } else {
            //      top border + padding top + lower border + status line
            // 4 => 1          + 1           + 1            + 1
            let available_height = self.terminal_dimensions.height as usize - 4;
            self.help_line_count.saturating_sub(available_height)
        };
    }

    // kinda weird that this model method relies on knowledge of the view, but oh well
    pub(super) fn compute_max_diff_scroll_available(&mut self) {
        let selected_index = self.changes.state.selected();
        let change_item = selected_index.and_then(|i| self.changes.items.get(i));

        //       top border + padding top + lower border + changes pane (fixed) + status bar height
        // 16 => 1          + 1           + 1            + 12                   + 1
        let available_height = self.terminal_dimensions.height as usize - 16;

        self.max_diff_scroll_available = match change_item {
            Some(item) => match &item.change.kind {
                ChangeKind::Modified(Ok(Modification::Diff(diff))) if !self.terminal_too_small => {
                    diff.num_lines().saturating_sub(available_height)
                }
                ChangeKind::Created(Ok(contents)) if !self.terminal_too_small => {
                    contents.lines().count().saturating_sub(available_height)
                }
                _ => 0,
            },
            None => {
                if selected_index.is_some() {
                    self.user_msg = Some(UserMsg::error(UNEXPECTED_ERROR_MSG));
                }
                0
            }
        };
    }

    pub(super) fn cache(&self) -> Arc<RwLock<FileCache>> {
        self.cache.clone()
    }

    pub(super) fn snapshots_in_memory(&self) -> Option<usize> {
        self.cache.try_read().ok().map(|c| c.len())
    }

    pub(super) fn toggle_sound(&mut self) {
        match &self.audio_player {
            Ok(Some(_)) => {
                self.behaviours.play_sound = !self.behaviours.play_sound;
            }
            Ok(None) => {
                self.audio_player = match AudioPlayer::new() {
                    Ok(ap) => Ok(Some(ap)),
                    Err(_) => {
                        self.user_msg = Some(UserMsg::error("couldn't set up sound notifications"));
                        Err(())
                    }
                };

                if let Ok(Some(_)) = &self.audio_player {
                    self.behaviours.play_sound = true;
                }
            }
            Err(_) => {
                self.user_msg = Some(UserMsg::error("sound notifications are unavailable"));
            }
        }
    }

    pub(super) fn is_sound_unavailable(&self) -> bool {
        self.audio_player.is_err()
    }
}
