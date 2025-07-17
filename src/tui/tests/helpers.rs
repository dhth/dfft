use super::super::common::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, TerminalDimensions};
use ratatui::{Terminal, backend::TestBackend};

pub(super) fn get_test_terminal() -> (Terminal<TestBackend>, TerminalDimensions) {
    let terminal = Terminal::new(TestBackend::new(MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT))
        .expect("terminal should've been created");
    let terminal_dimensions = TerminalDimensions::from((MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT));

    (terminal, terminal_dimensions)
}

pub(super) fn get_test_terminal_with_dims(
    width: u16,
    height: u16,
) -> (Terminal<TestBackend>, TerminalDimensions) {
    let terminal =
        Terminal::new(TestBackend::new(width, height)).expect("terminal should've been created");
    let terminal_dimensions = TerminalDimensions::from((width, height));

    (terminal, terminal_dimensions)
}
