pub const MIN_TERMINAL_WIDTH: u16 = 80;
pub const MIN_TERMINAL_HEIGHT: u16 = 30;

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Pane {
    ChangesList,
    Diff,
    Help,
}

impl std::fmt::Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pane::ChangesList => write!(f, "cl"),
            Pane::Diff => write!(f, "diff"),
            Pane::Help => write!(f, "help"),
        }
    }
}

pub(super) struct TerminalDimensions {
    pub(super) width: u16,
    pub(super) height: u16,
}
