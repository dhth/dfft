use ratatui::style::Color;

pub const MIN_TERMINAL_WIDTH: u16 = 80;
pub const MIN_TERMINAL_HEIGHT: u16 = 24;
pub const ADDITION_COLOR: Color = Color::from_u32(0x9ece6a);
pub const SUBTRACTION_COLOR: Color = Color::from_u32(0xf7768e);
pub const MODIFICATION_COLOR: Color = Color::from_u32(0xdf8e1d);
pub const FILE_ERROR_COLOR: Color = Color::from_u32(0xfb4934);

const HELP_CONTENT_RAW: &str = include_str!("static/help.txt");

pub fn get_help_content() -> String {
    #[cfg(feature = "sound")]
    {
        HELP_CONTENT_RAW.to_string()
    }
    #[cfg(not(feature = "sound"))]
    {
        HELP_CONTENT_RAW
            .lines()
            .filter(|line| !line.contains("toggle sound notifications"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
pub const UNEXPECTED_ERROR_MSG: &str = "an unexpected error occurred";

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Pane {
    Changes,
    Diff,
    Help,
}

impl std::fmt::Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pane::Changes => write!(f, "changes"),
            Pane::Diff => write!(f, "diff"),
            Pane::Help => write!(f, "help"),
        }
    }
}

pub(super) struct TerminalDimensions {
    pub(super) width: u16,
    pub(super) height: u16,
}

impl TerminalDimensions {
    pub(super) fn update(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
    }

    #[cfg(test)]
    pub(super) fn values(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    #[cfg(test)]
    pub(super) fn min_needed() -> Self {
        Self {
            width: MIN_TERMINAL_WIDTH,
            height: MIN_TERMINAL_HEIGHT,
        }
    }
}

#[cfg(test)]
impl From<(u16, u16)> for TerminalDimensions {
    fn from(value: (u16, u16)) -> Self {
        let (width, height) = value;
        Self { width, height }
    }
}
