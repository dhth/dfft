use super::common::*;
use super::model::{MessageKind, Model};
use crate::domain::{ChangeKind, ModifiedResult};
use ratatui::style::Color;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph, Wrap},
};

const PANE_TITLE_FG_COLOR: Color = Color::from_u32(0x151515);
const PRIMARY_COLOR: Color = Color::from_u32(0xa6d189);
const INACTIVE_PANE_TITLE_BG_COLOR: Color = Color::from_u32(0x737994);
const INACTIVE_PANE_BORDER_COLOR: Color = Color::from_u32(0x737994);
const INACTIVE_PANE_SELECTED_COLOR: Color = Color::from_u32(0xfabd2f);
const INFO_MESSAGE_COLOR: Color = Color::from_u32(0x83a598);
const ERROR_MESSAGE_COLOR: Color = Color::from_u32(0xfb4934);
const WATCHING_COLOR: Color = Color::from_u32(0xbabbf1);
const PAUSED_COLOR: Color = Color::from_u32(0xef9f76);
const WATCHING_LABEL: &str = " [watching]";
const PAUSED_LABEL: &str = " [ paused ]";
const FOLLOWING_CHANGES_COLOR: Color = Color::from_u32(0xca9ee6);
const HELP_COLOR: Color = Color::from_u32(0x8caaee);
const TITLE: &str = " dfft ";
const BANNER_LARGE: &str = r#"


     888  .d888  .d888 888    
     888 d88P"  d88P"  888    
     888 888    888    888    
 .d88888 888888 888888 888888 
d88" 888 888    888    888    
888  888 888    888    888    
Y88b 888 888    888    Y88b.  
 "Y88888 888    888     "Y888


see changes to files in a directory as they happen
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
"#;

const BANNER_SMALL: &str = r#"
                 
dfft
‾‾‾‾

see changes to files in a directory as they happen
"#;

pub fn view(model: &mut Model, frame: &mut Frame) {
    if model.terminal_too_small {
        render_terminal_too_small_view(&model.terminal_dimensions, frame);
        return;
    }

    match model.active_pane {
        Pane::ChangesList | Pane::Diff => render_main_view(model, frame),
        Pane::Help => render_help_view(model, frame),
    }
}

fn render_terminal_too_small_view(dimensions: &TerminalDimensions, frame: &mut Frame) {
    let message = format!(
        r#"
Terminal size too small:
  Width = {} Height = {}

Minimum dimensions needed:
  Width = {} Height = {}

Press (q/<ctrl+c>/<esc> to exit)
"#,
        dimensions.width, dimensions.height, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT
    );

    let p = Paragraph::new(message)
        .block(Block::bordered())
        .style(Style::new().fg(PRIMARY_COLOR))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    frame.render_widget(p, frame.area());
}

fn render_main_view(model: &mut Model, frame: &mut Frame) {
    let main_rect = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Min(10),
            Constraint::Max(12),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_diff(model, frame, main_rect[0]);
    render_changes_list(model, frame, main_rect[1]);
    render_status_line(model, frame, main_rect[2]);
}

fn render_diff(model: &Model, frame: &mut Frame, rect: Rect) {
    let (border_color, title_color, _) = if model.active_pane == Pane::Diff {
        (PRIMARY_COLOR, PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_TITLE_BG_COLOR,
            INACTIVE_PANE_SELECTED_COLOR,
        )
    };

    let title = " diff ";

    let maybe_selected_index = model.changes.state.selected();
    let details = match maybe_selected_index {
        Some(selected_index) => {
            let maybe_change = model.changes.items.get(selected_index);
            let lines = match maybe_change {
                Some(change) => match &change.change.kind {
                    ChangeKind::Created(Ok(_)) => vec![Line::raw("created").gray()],
                    ChangeKind::Created(Err(e)) => {
                        vec![Line::raw(format!("error reading file contents: {e}"))]
                    }
                    ChangeKind::Modified(Ok(result)) => match result {
                        ModifiedResult::InitialSnapshot => {
                            vec![Line::raw(
                                "initial snapshot captured; diffs will be available from now onwards",
                            )]
                        }
                        ModifiedResult::Diff(None) => vec![Line::raw("nothing changed")],
                        ModifiedResult::Diff(Some(d)) => get_colored_diff(d),
                    },
                    ChangeKind::Modified(Err(e)) => {
                        vec![Line::raw(format!("error reading file contents: {e}"))]
                    }
                    ChangeKind::Removed => vec![Line::raw("file removed")],
                },
                None => vec![Line::raw("something went wrong")],
            };

            Paragraph::new(lines)
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(border_color))
                        .title_style(Style::new().bold().bg(title_color).fg(PANE_TITLE_FG_COLOR))
                        .title(title)
                        .padding(Padding::new(1, 0, 1, 0)),
                )
                .style(Style::new().white().on_black())
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left)
        }
        None => Paragraph::new(if model.terminal_dimensions.height >= 30 {
            BANNER_LARGE
        } else {
            BANNER_SMALL
        })
        .block(
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_style(Style::new().bold().bg(title_color).fg(PANE_TITLE_FG_COLOR))
                .title(title)
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .style(Style::new().fg(PRIMARY_COLOR))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center),
    };

    frame.render_widget(&details, rect);
}

fn render_changes_list(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let items: Vec<ListItem> = model.changes.items.iter().map(ListItem::from).collect();

    let title = if items.is_empty() {
        " changes ".to_string()
    } else {
        format!(" changes ({}) ", items.len())
    };

    let (border_color, title_color) = if model.active_pane == Pane::ChangesList {
        (PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (INACTIVE_PANE_BORDER_COLOR, INACTIVE_PANE_TITLE_BG_COLOR)
    };

    if items.is_empty() {
        let p = Paragraph::new("changes will appear here")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .title_style(Style::new().bold().bg(title_color).fg(PANE_TITLE_FG_COLOR))
                    .title(title)
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        return frame.render_widget(&p, rect);
    }

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .padding(Padding::new(0, 0, 1, 0))
                .title_style(Style::new().bold().bg(title_color).fg(PANE_TITLE_FG_COLOR))
                .title(title),
        )
        .highlight_symbol("> ")
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, rect, &mut model.changes.state);
}

fn render_help_view(model: &Model, frame: &mut Frame) {
    let rect = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    let lines: Vec<Line> = HELP_CONTENT
        .lines()
        .skip(model.help_scroll)
        .map(Line::raw)
        .collect();

    let title = " help ";

    let help_widget = Paragraph::new(lines)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(HELP_COLOR))
                .title_style(Style::new().bold().bg(HELP_COLOR).fg(PANE_TITLE_FG_COLOR))
                .title(title)
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .style(Style::new().white().on_black())
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(&help_widget, rect[0]);
    render_status_line(model, frame, rect[1]);
}

fn render_status_line(model: &Model, frame: &mut Frame, rect: Rect) {
    let mut status_bar_lines = vec![Span::styled(
        TITLE,
        Style::new()
            .bold()
            .bg(PRIMARY_COLOR)
            .fg(PANE_TITLE_FG_COLOR),
    )];

    if let Some(msg) = &model.user_msg {
        let span = match msg.kind {
            MessageKind::Info => Span::styled(
                format!(" {}", msg.value),
                Style::new().fg(INFO_MESSAGE_COLOR),
            ),
            MessageKind::Error => Span::styled(
                format!(" {}", msg.value),
                Style::new().fg(ERROR_MESSAGE_COLOR),
            ),
        };

        status_bar_lines.push(span);
    }

    if model.debug {
        status_bar_lines.push(Span::from(format!(
            " [index: {:?}]",
            model.changes.state.selected()
        )));

        status_bar_lines.push(Span::from(format!(" [watching: {:?}]", model.watching,)));
    }

    let (watching_label, watching_color) = if model.watching {
        (WATCHING_LABEL, WATCHING_COLOR)
    } else {
        (PAUSED_LABEL, PAUSED_COLOR)
    };

    status_bar_lines.push(Span::styled(
        watching_label,
        Style::default().fg(watching_color).bold(),
    ));

    if model.follow_changes {
        status_bar_lines.push(Span::styled(
            " [following changes]",
            Style::default().fg(FOLLOWING_CHANGES_COLOR).bold(),
        ));
    }

    let status_bar_text = Line::from(status_bar_lines);

    let status_bar = Paragraph::new(status_bar_text).block(Block::default());

    frame.render_widget(&status_bar, rect);
}

fn get_colored_diff<'a>(diff: &'a str) -> Vec<Line<'a>> {
    let mut lines = vec![];
    for line in diff.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("@@") {
            lines.push(Line::raw(line).blue());
        } else if line.starts_with("-") {
            lines.push(Line::raw(line).fg(DIFF_REMOVED_COLOR));
        } else if line.starts_with("+") {
            lines.push(Line::raw(line).fg(ADDED_COLOR));
        } else {
            lines.push(Line::raw(line).gray());
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use crate::domain::{Change, ChangeKind, ModifiedResult};
    use crate::tui::model::UserMsg;
    use crate::tui::{msg::Msg, update::update};

    use super::*;
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    fn get_test_terminal() -> (Terminal<TestBackend>, TerminalDimensions) {
        let terminal =
            Terminal::new(TestBackend::new(80, 24)).expect("terminal should've been created");
        let terminal_dimensions = TerminalDimensions::from((80, 24));

        (terminal, terminal_dimensions)
    }

    fn get_test_terminal_with_dims(
        width: u16,
        height: u16,
    ) -> (Terminal<TestBackend>, TerminalDimensions) {
        let terminal = Terminal::new(TestBackend::new(width, height))
            .expect("terminal should've been created");
        let terminal_dimensions = TerminalDimensions::from((width, height));

        (terminal, terminal_dimensions)
    }

    #[test]
    fn rendering_help_view_works() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();

        let mut model = Model::new(terminal_dimensions, true, false);
        model.active_pane = Pane::Help;

        // WHEN
        terminal
            .draw(|f| view(&mut model, f))
            .expect("frame should've been drawn");

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ Keymaps                                                                      │"
        "│ ---                                                                          │"
        "│                                                                              │"
        "│ General                                                                      │"
        "│     ?                    show/hide help view                                 │"
        "│     Esc / q              go back/exit                                        │"
        "│     <ctrl+c>             exit immediately                                    │"
        "│                                                                              │"
        "│ Changes List Pane                                                            │"
        "│     j / ↓                go down                                             │"
        "│     k / ↑                go up                                               │"
        "│     g                    go to top                                           │"
        "│     G                    go to the bottom                                    │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab                  switch to diff pane                                 │"
        "│                                                                              │"
        "│ Diff Pane                                                                    │"
        "│     Tab                  switch to changes list pane                         │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn scrolling_help_view_works() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();

        let mut model = Model::new(terminal_dimensions, true, false);
        model.active_pane = Pane::Help;
        for _ in 1..=3 {
            update(&mut model, Msg::GoDown);
        }

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ General                                                                      │"
        "│     ?                    show/hide help view                                 │"
        "│     Esc / q              go back/exit                                        │"
        "│     <ctrl+c>             exit immediately                                    │"
        "│                                                                              │"
        "│ Changes List Pane                                                            │"
        "│     j / ↓                go down                                             │"
        "│     k / ↑                go up                                               │"
        "│     g                    go to top                                           │"
        "│     G                    go to the bottom                                    │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab                  switch to diff pane                                 │"
        "│                                                                              │"
        "│ Diff Pane                                                                    │"
        "│     Tab                  switch to changes list pane                         │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn help_pane_doesnt_scroll_beyond_lower_limit() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();

        let mut model = Model::new(terminal_dimensions, true, false);
        model.active_pane = Pane::Help;
        for _ in 1..=20 {
            update(&mut model, Msg::GoDown);
        }

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     <ctrl+c>             exit immediately                                    │"
        "│                                                                              │"
        "│ Changes List Pane                                                            │"
        "│     j / ↓                go down                                             │"
        "│     k / ↑                go up                                               │"
        "│     g                    go to top                                           │"
        "│     G                    go to the bottom                                    │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab                  switch to diff pane                                 │"
        "│                                                                              │"
        "│ Diff Pane                                                                    │"
        "│     Tab                  switch to changes list pane                         │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│                                                                              │"
        "│ Help Pane                                                                    │"
        "│     j / ↓                go down                                             │"
        "│     k / ↑                go up                                               │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn help_pane_doesnt_scroll_above_upper_limit() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();

        let mut model = Model::new(terminal_dimensions, true, false);
        model.active_pane = Pane::Help;
        for _ in 1..=3 {
            update(&mut model, Msg::GoUp);
        }

        // WHEN
        terminal
            .draw(|f| view(&mut model, f))
            .expect("frame should've been drawn");

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ Keymaps                                                                      │"
        "│ ---                                                                          │"
        "│                                                                              │"
        "│ General                                                                      │"
        "│     ?                    show/hide help view                                 │"
        "│     Esc / q              go back/exit                                        │"
        "│     <ctrl+c>             exit immediately                                    │"
        "│                                                                              │"
        "│ Changes List Pane                                                            │"
        "│     j / ↓                go down                                             │"
        "│     k / ↑                go up                                               │"
        "│     g                    go to top                                           │"
        "│     G                    go to the bottom                                    │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab                  switch to diff pane                                 │"
        "│                                                                              │"
        "│ Diff Pane                                                                    │"
        "│     Tab                  switch to changes list pane                         │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn terminal_too_small_view_is_shown_when_width_too_small() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(60, 24);
        let mut model = Model::new(terminal_dimensions, true, false);

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌──────────────────────────────────────────────────────────┐"
        "│                                                          │"
        "│                 Terminal size too small:                 │"
        "│                   Width = 60 Height = 24                 │"
        "│                                                          │"
        "│                Minimum dimensions needed:                │"
        "│                   Width = 80 Height = 24                 │"
        "│                                                          │"
        "│             Press (q/<ctrl+c>/<esc> to exit)             │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "│                                                          │"
        "└──────────────────────────────────────────────────────────┘"
        "#);
    }

    #[test]
    fn terminal_too_small_view_is_shown_when_height_too_small() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 20);
        let mut model = Model::new(terminal_dimensions, true, false);

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌──────────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                           Terminal size too small:                           │"
        "│                             Width = 80 Height = 20                           │"
        "│                                                                              │"
        "│                          Minimum dimensions needed:                          │"
        "│                             Width = 80 Height = 24                           │"
        "│                                                                              │"
        "│                       Press (q/<ctrl+c>/<esc> to exit)                       │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "#);
    }

    #[test]
    fn terminal_too_small_view_is_shown_when_both_dimensions_small() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(40, 20);
        let mut model = Model::new(terminal_dimensions, true, false);

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌──────────────────────────────────────┐"
        "│                                      │"
        "│       Terminal size too small:       │"
        "│         Width = 40 Height = 20       │"
        "│                                      │"
        "│      Minimum dimensions needed:      │"
        "│         Width = 80 Height = 24       │"
        "│                                      │"
        "│   Press (q/<ctrl+c>/<esc> to exit)   │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "│                                      │"
        "└──────────────────────────────────────┘"
        "#);
    }

    #[test]
    fn main_view_renders_banner_when_no_changes_present() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(100, 32);
        let mut model = Model::new(terminal_dimensions, true, false);

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────────────────────────┐"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                       888  .d888  .d888 888                                      │"
        "│                                       888 d88P"  d88P"  888                                      │"
        "│                                       888 888    888    888                                      │"
        "│                                   .d88888 888888 888888 888888                                   │"
        "│                                  d88" 888 888    888    888                                      │"
        "│                                  888  888 888    888    888                                      │"
        "│                                  Y88b 888 888    888    Y88b.                                    │"
        "│                                    "Y88888 888    888     "Y888                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                        see changes to files in a directory as they happen                        │"
        "│                        ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾                        │"
        "│                                                                                                  │"
        "└──────────────────────────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────────────────────────┐"
        "│                                                                                                  │"
        "│ changes will appear here                                                                         │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "│                                                                                                  │"
        "└──────────────────────────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                                                   "
        "#);
    }

    #[test]
    fn main_view_renders_created_file_change() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "new_file.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  created   new_file.txt                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_modified_file_with_diff() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let diff = "@@ -1,3 +1,4 @@\n line1\n-old line\n+new line\n line3".to_string();
        let change = Change {
            file_path: "modified_file.txt".to_string(),
            kind: ChangeKind::Modified(Ok(ModifiedResult::Diff(Some(diff)))),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ @@ -1,3 +1,4 @@                                                              │"
        "│  line1                                                                       │"
        "│ -old line                                                                    │"
        "│ +new line                                                                    │"
        "│  line3                                                                       │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  modified_file.txt                                                │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_removed_file_change() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "deleted_file.txt".to_string(),
            kind: ChangeKind::Removed,
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ file removed                                                                 │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  removed   deleted_file.txt                                                 │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_created_file_with_error() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "error_file.txt".to_string(),
            kind: ChangeKind::Created(Err("Permission denied".to_string())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ error reading file contents: Permission denied                               │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>   error    error_file.txt                                                   │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_modified_file_with_error() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "error_modified.txt".to_string(),
            kind: ChangeKind::Modified(Err("File not found".to_string())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ error reading file contents: File not found                                  │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>   error    error_modified.txt                                               │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_initial_snapshot_change() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "snapshot_file.txt".to_string(),
            kind: ChangeKind::Modified(Ok(ModifiedResult::InitialSnapshot)),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ initial snapshot captured; diffs will be available from now onwards          │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  snapshot_file.txt                                                │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn main_view_renders_modified_file_with_no_diff() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "no_diff_file.txt".to_string(),
            kind: ChangeKind::Modified(Ok(ModifiedResult::Diff(None))),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ nothing changed                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  no_diff_file.txt                                                 │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn changes_list_shows_item_count_in_title() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        for i in 0..3 {
            let change = Change {
                file_path: format!("file{i}.txt"),
                kind: ChangeKind::Created(Ok(())),
            };
            update(&mut model, Msg::ChangeReceived(change));
        }

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (3) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  created   file0.txt                                                        │"
        "│   created   file1.txt                                                        │"
        "│   created   file2.txt                                                        │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn changes_list_handles_long_file_paths() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);

        let change = Change {
            file_path: "very/long/path/to/a/file/that/exceeds/normal/length/limits/file.txt"
                .to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  created   very/long/path/to/a/file/that/exceeds/normal/length/limits/file.t│"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn status_line_shows_paused_status() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, false, false);

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [ paused ]                                                               "
        "#);
    }

    #[test]
    fn status_line_shows_following_changes_indicator() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.follow_changes = true;

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching] [following changes]                                           "
        "#);
    }

    #[test]
    fn status_line_shows_info_message() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.user_msg = Some(UserMsg::info("Test info message"));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  Test info message [watching]                                             "
        "#);
    }

    #[test]
    fn status_line_shows_error_message() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.user_msg = Some(UserMsg::error("Test error message"));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  Test error message [watching]                                            "
        "#);
    }

    #[test]
    fn info_message_disappears_after_its_frame_budget_expires() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.user_msg =
            Some(UserMsg::info("This will disappear after 2 renders").with_frames_left(1));

        // WHEN
        // THEN
        update(&mut model, Msg::GoDown);
        terminal.draw(|f| view(&mut model, f)).unwrap();
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  This will disappear after 2 renders [watching]                           "
        "#);

        update(&mut model, Msg::GoDown);
        terminal.draw(|f| view(&mut model, f)).unwrap();
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes ─────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ changes will appear here                                                     │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn handles_malformed_diff_format() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.active_pane = Pane::Diff;

        let change = Change {
            file_path: "malformed.txt".to_string(),
            kind: ChangeKind::Modified(Ok(ModifiedResult::Diff(Some(
                "not a real diff".to_string(),
            )))),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ not a real diff                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  malformed.txt                                                    │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }

    #[test]
    fn cursor_moves_automatically_when_following_enabled() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.follow_changes = true;

        let change = Change {
            file_path: "first.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        let change = Change {
            file_path: "this-should-be-selected.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (2) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│   created   first.txt                                                        │"
        "│>  created   this-should-be-selected.txt                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching] [following changes]                                           "
        "#);
    }

    #[test]
    fn cursor_moves_to_the_end_when_following_is_turned_on_after_a_while() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.follow_changes = false;

        // WHEN
        // THEN
        for i in 1..=3 {
            let change = Change {
                file_path: format!("file-{}.txt", i),
                kind: ChangeKind::Created(Ok(())),
            };
            update(&mut model, Msg::ChangeReceived(change));
        }
        terminal.draw(|f| view(&mut model, f)).unwrap();
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (3) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  created   file-1.txt                                                       │"
        "│   created   file-2.txt                                                       │"
        "│   created   file-3.txt                                                       │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);

        model.follow_changes = true;
        let change = Change {
            file_path: "this-should-be-selected.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));
        terminal.draw(|f| view(&mut model, f)).unwrap();
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (4) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│   created   file-1.txt                                                       │"
        "│   created   file-2.txt                                                       │"
        "│   created   file-3.txt                                                       │"
        "│>  created   this-should-be-selected.txt                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching] [following changes]                                           "
        "#);
    }

    #[test]
    fn cursor_doesnt_move_by_itself_when_following_disabled() {
        // GIVEN
        let (mut terminal, terminal_dimensions) = get_test_terminal();
        let mut model = Model::new(terminal_dimensions, true, false);
        model.follow_changes = false;

        let change = Change {
            file_path: "this-will-still-be-selected.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        let change = Change {
            file_path: "second.txt".to_string(),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));

        // WHEN
        terminal.draw(|f| view(&mut model, f)).unwrap();

        // THEN
        assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ created                                                                      │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (2) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  created   this-will-still-be-selected.txt                                  │"
        "│   created   second.txt                                                       │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
    }
}
