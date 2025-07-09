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
const BANNER: &str = r#"


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
        None => Paragraph::new(BANNER)
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
