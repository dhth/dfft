use super::common::*;
use super::model::{MessageKind, Model};
use crate::domain::{ChangeKind, Diff, DiffOperation, Modification};
use ratatui::style::{Color, Modifier};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph, Wrap},
};

const PANE_TITLE_FG_COLOR: Color = Color::from_u32(0x151515);
const PRIMARY_COLOR: Color = Color::from_u32(0xa6d189);
const INACTIVE_PANE_TITLE_BG_COLOR: Color = Color::from_u32(0x838ba7);
const INACTIVE_PANE_BORDER_COLOR: Color = Color::from_u32(0x737994);
const INFO_MESSAGE_COLOR: Color = Color::from_u32(0x83a598);
const ERROR_MESSAGE_COLOR: Color = Color::from_u32(0xfb4934);
const WATCHING_COLOR: Color = Color::from_u32(0xbabbf1);
const PAUSED_COLOR: Color = Color::from_u32(0xe5c890);
const WATCHING_LABEL: &str = " [watching]";
const PAUSED_LABEL: &str = " [ paused ]";
const SNAPSHOTS_COLOR: Color = Color::from_u32(0xca9ee6);
const FOLLOWING_CHANGES_COLOR: Color = Color::from_u32(0x99d1db);
const HELP_COLOR: Color = Color::from_u32(0xbabbf1);
const DIM_COLOR: Color = Color::Gray;
#[cfg(feature = "sound")]
const SOUND_UNAVAILABLE_COLOR: Color = Color::from_u32(0xe78284);
#[cfg(feature = "sound")]
const SOUND_ON_COLOR: Color = Color::from_u32(0xf5a97f);

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


will show you changes in this directory as they happen
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

In the diff pane:
- use j/k/↓/↑ to scroll diff
- use h/l/←/→ to move between changes

In the changes pane:
- use j/k/↓/↑ to move between changes

Tab/<S-Tab>/J/K to move between panes

Press ? for help
"#;

const BANNER_SMALL: &str = r#"
                 
dfft
‾‾‾‾

will show you changes in this directory as they happen
"#;

pub fn view(model: &mut Model, frame: &mut Frame) {
    if model.terminal_too_small {
        render_terminal_too_small_view(&model.terminal_dimensions, frame);
        return;
    }

    match model.active_pane {
        Pane::Changes | Pane::Diff => render_main_view(model, frame),
        Pane::Help => render_help_pane(model, frame),
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

    render_diff_pane(model, frame, main_rect[0]);
    render_changes_pane(model, frame, main_rect[1]);
    render_status_line(model, frame, main_rect[2]);
}

fn render_diff_pane(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let (color, border_color, title_color) = if model.active_pane == Pane::Diff {
        (PRIMARY_COLOR, PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_BORDER_COLOR,
            INACTIVE_PANE_TITLE_BG_COLOR,
        )
    };

    let pane_name = " diff ";

    let maybe_selected_index = model.changes.state.selected();
    let details = match maybe_selected_index {
        Some(selected_index) => {
            let maybe_change = model.changes.items.get(selected_index);
            let lines = match maybe_change {
                Some(change) => match &change.change.kind {
                    ChangeKind::Created(Ok(contents)) => contents
                        .lines()
                        .skip(model.diff_scroll)
                        .map(Line::raw)
                        .collect(),
                    ChangeKind::Created(Err(e)) => {
                        vec![Line::raw(format!("error reading file contents: {e}"))]
                    }
                    ChangeKind::Modified(Ok(result)) => match result {
                        Modification::InitialSnapshot => {
                            vec![Line::raw(
                                "initial snapshot captured; diffs will be available from now onwards",
                            )]
                        }
                        Modification::Diff(diff) => get_diff_lines(diff)
                            .into_iter()
                            .skip(model.diff_scroll)
                            .collect(),
                    },
                    ChangeKind::Modified(Err(e)) => {
                        vec![Line::raw(format!("error reading file contents: {e}"))]
                    }
                    ChangeKind::RemovedFile => vec![Line::raw("file removed")],
                    ChangeKind::RemovedDir => vec![Line::raw("directory removed")],
                },
                None => vec![Line::raw("something went wrong")],
            };

            let section_title_span = Span::from(pane_name)
                .bold()
                .bg(title_color)
                .fg(PANE_TITLE_FG_COLOR);

            let title_spans = if let Some(fp) = model.current_file_path() {
                vec![
                    section_title_span,
                    Span::from(" "),
                    Span::from(fp).fg(title_color),
                    Span::from(" "),
                ]
            } else {
                vec![section_title_span.clone()]
            };

            Paragraph::new(lines)
                .block(
                    Block::bordered()
                        .border_style(Style::default().fg(border_color))
                        .title(Line::from(title_spans))
                        .padding(Padding::new(1, 0, 1, 0)),
                )
                .style(Style::new().white().on_black())
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
                .title(pane_name)
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .style(Style::new().fg(color))
        .alignment(Alignment::Center),
    };

    frame.render_widget(&details, rect);
}

fn render_changes_pane(model: &mut Model, frame: &mut Frame, rect: Rect) {
    let items: Vec<ListItem> = model.changes.items.iter().map(ListItem::from).collect();

    let pane_name = " changes ";

    let (border_color, title_color) = if model.active_pane == Pane::Changes {
        (PRIMARY_COLOR, PRIMARY_COLOR)
    } else {
        (INACTIVE_PANE_BORDER_COLOR, INACTIVE_PANE_TITLE_BG_COLOR)
    };

    let section_title_span = Span::from(pane_name)
        .bold()
        .bg(title_color)
        .fg(PANE_TITLE_FG_COLOR);

    let title_spans = if let Some(i) = model.changes.state.selected() {
        vec![
            section_title_span,
            Span::from(format!(" ({}/{}) ", i + 1, items.len())).fg(title_color),
        ]
    } else {
        vec![section_title_span]
    };

    if items.is_empty() {
        let p = Paragraph::new("changes will appear here")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .title_style(Style::new().bold().bg(title_color).fg(PANE_TITLE_FG_COLOR))
                    .title(pane_name)
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
                .title(Line::from(title_spans)),
        )
        .highlight_symbol("> ")
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, rect, &mut model.changes.state);
}

fn render_help_pane(model: &Model, frame: &mut Frame) {
    let rect = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    let help_content = get_help_content();
    let lines: Vec<Line> = help_content
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

    if let Some(n) = model.snapshots_in_memory()
        && n > 0
    {
        status_bar_lines.push(Span::styled(
            if n == 1 {
                " [1 snapshot in memory]".to_string()
            } else {
                format!(" [{n} snapshots in memory]")
            },
            Style::default().fg(SNAPSHOTS_COLOR).bold(),
        ));
    }

    let (watching_label, watching_color) = if model.behaviours.watch {
        (WATCHING_LABEL, WATCHING_COLOR)
    } else {
        (PAUSED_LABEL, PAUSED_COLOR)
    };

    status_bar_lines.push(Span::styled(
        watching_label,
        Style::default().fg(watching_color).bold(),
    ));

    #[cfg(feature = "sound")]
    if model.is_sound_unavailable() {
        status_bar_lines.push(Span::styled(
            " [sound unavailable]",
            Style::default().fg(SOUND_UNAVAILABLE_COLOR).bold(),
        ));
    } else if model.behaviours.play_sound {
        status_bar_lines.push(Span::styled(
            " [sound on]",
            Style::default().fg(SOUND_ON_COLOR).bold(),
        ));
    }

    if model.behaviours.follow_changes {
        status_bar_lines.push(Span::styled(
            " [following changes]",
            Style::default().fg(FOLLOWING_CHANGES_COLOR).bold(),
        ));
    }

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
        status_bar_lines.push(Span::from(format!(" [render: {}]", model.render_counter)));
        status_bar_lines.push(Span::from(format!(" [event: {}]", model.event_counter)));
        status_bar_lines.push(Span::from(format!(" [watch: {}]", model.watch_counter)));
        status_bar_lines.push(Span::from(format!(
            " [dimensions: {}x{}] ",
            model.terminal_dimensions.width, model.terminal_dimensions.height
        )));
    }

    let status_bar_text = Line::from(status_bar_lines);

    let status_bar = Paragraph::new(status_bar_text).block(Block::default());

    frame.render_widget(&status_bar, rect);
}

// inspired by https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
fn get_diff_lines(diff: &Diff) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    let line_number_padding = diff.line_num_padding();

    for (idx, hunk) in diff.hunks.iter().enumerate() {
        if idx > 0 {
            lines.push(Line::from(vec![Span::styled(
                format!("{:-^80}", "-"),
                Style::new().fg(DIM_COLOR),
            )]));
        }

        for diff_line in &hunk.lines {
            let sign = diff_line.kind.sign();
            let style = match diff_line.kind {
                DiffOperation::Delete => Style::new().fg(SUBTRACTION_COLOR),
                DiffOperation::Insert => Style::new().fg(ADDITION_COLOR),
                DiffOperation::Equal => Style::new().fg(DIM_COLOR),
            };

            let old_line = diff_line
                .old_line_num
                .map(|n| format!("{:<padding$}", n + 1, padding = line_number_padding))
                .unwrap_or_else(|| " ".repeat(line_number_padding));

            let new_line = diff_line
                .new_line_num
                .map(|n| format!("{:<padding$}", n + 1, padding = line_number_padding))
                .unwrap_or_else(|| " ".repeat(line_number_padding));

            let mut line_spans = vec![
                Span::styled(old_line, Style::new().fg(DIM_COLOR)),
                Span::styled(new_line, Style::new().fg(DIM_COLOR)),
                Span::styled(format!("|{sign}"), style.add_modifier(Modifier::BOLD)),
            ];

            for inline_change in &diff_line.inline_changes {
                let final_style = if inline_change.emphasized {
                    style.add_modifier(Modifier::UNDERLINED)
                } else {
                    style
                };
                line_spans.push(Span::styled(inline_change.value.clone(), final_style));
            }

            lines.push(Line::from(line_spans));
        }
    }

    lines
}
