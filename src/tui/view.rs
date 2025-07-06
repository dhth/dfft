use super::common::*;
use super::model::{Model, UserMessage};
use crate::domain::{Change, ChangeKind, ModifiedResult};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph, Wrap},
};

pub fn view(model: &mut Model, frame: &mut Frame) {
    if model.terminal_too_small {
        render_terminal_too_small_view(&model.terminal_dimensions, frame);
        return;
    }

    render_changes_view(model, frame)
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

fn render_change(model: &Model, frame: &mut Frame, rect: Rect) {
    let (border_color, title_color, highlight_color) = if model.active_pane == Pane::ChangesList {
        (PRIMARY_COLOR, PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_TITLE_BG_COLOR,
            INACTIVE_PANE_SELECTED_COLOR,
        )
    };

    let title = " change ";

    let maybe_selected_index = model.changes.state.selected();

    let lines = if let Some(selected_index) = maybe_selected_index {
        let maybe_change = model.changes.items.get(selected_index);
        match maybe_change {
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
        }
    } else {
        vec![Line::raw("change details will appear here")]
    };

    let details = Paragraph::new(lines)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(title_color)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(title)
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .style(Style::new().white().on_black())
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(&details, rect);
}

fn get_colored_diff<'a>(diff: &'a str) -> Vec<Line<'a>> {
    let mut lines = vec![];
    for line in diff.lines() {
        if line.starts_with("@@") {
            lines.push(Line::raw(line).blue());
        } else if line.starts_with("-") {
            lines.push(Line::raw(line).red());
        } else if line.starts_with("+") {
            lines.push(Line::raw(line).green());
        } else {
            lines.push(Line::raw(line).gray());
        }
    }

    lines
}

fn render_changes_list(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let items: Vec<ListItem> = model.changes.items.iter().map(ListItem::from).collect();

    let title = " changes ";

    let (border_color, title_color, highlight_color) = if model.active_pane == Pane::ChangesList {
        (PRIMARY_COLOR, PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_TITLE_BG_COLOR,
            INACTIVE_PANE_SELECTED_COLOR,
        )
    };

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .padding(Padding::new(0, 0, 1, 0))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(title_color)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(title),
        )
        .style(Style::new().white())
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true)
        .highlight_style(Style::new().fg(highlight_color))
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, rect, &mut model.changes.state);
}

fn render_status_line(model: &Model, frame: &mut Frame, rect: Rect) {
    let mut status_bar_lines = vec![Span::styled(
        TITLE,
        Style::new()
            .bold()
            .bg(PRIMARY_COLOR)
            .fg(SECTION_TITLE_FG_COLOR),
    )];

    if let Some(msg) = &model.user_message {
        let span = match msg {
            UserMessage::Info(m, _) => {
                Span::styled(format!(" {m}"), Style::new().fg(INFO_MESSAGE_COLOR))
            }
            UserMessage::Error(m, _) => {
                Span::styled(format!(" {m}"), Style::new().fg(ERROR_MESSAGE_COLOR))
            }
        };

        status_bar_lines.push(span);
    }

    if model.debug {
        status_bar_lines.push(Span::from(format!(
            " [index: {:?}]",
            model.changes.state.selected()
        )));
    }

    let status_bar_text = Line::from(status_bar_lines);

    let status_bar = Paragraph::new(status_bar_text).block(Block::default());

    frame.render_widget(&status_bar, rect);
}

fn render_changes_view(model: &mut Model, frame: &mut Frame) {
    let main_rect = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Min(10),
            Constraint::Max(12),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_change(model, frame, main_rect[0]);
    render_changes_list(model, frame, main_rect[1]);
    render_status_line(model, frame, main_rect[2]);
}
