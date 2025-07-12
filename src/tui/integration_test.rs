use super::common::TerminalDimensions;
use super::{msg::Msg, update::update, view::view};
use crate::domain::{Change, ChangeKind, Diff, Modification};
use crate::tui::common::Pane;
use crate::tui::model::{Model, UserMsg};
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
    let terminal =
        Terminal::new(TestBackend::new(width, height)).expect("terminal should've been created");
    let terminal_dimensions = TerminalDimensions::from((width, height));

    (terminal, terminal_dimensions)
}

#[test]
fn rendering_help_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 40);

    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Help));

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
        "│ Diff Pane                                                                    │"
        "│     Tab/<S-Tab>/J/K      switch to changes pane                              │"
        "│     j / ↓                scroll down                                         │"
        "│     k / ↑                scroll up                                           │"
        "│     l/→                  select next change                                  │"
        "│     h/←                  select previous change                              │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│     f                    toggle following changes                            │"
        "│                                                                              │"
        "│ Changes Pane                                                                 │"
        "│     j / ↓                select next change                                  │"
        "│     k / ↑                select previous change                              │"
        "│     g                    select first change                                 │"
        "│     G                    select last change                                  │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab/<S-Tab>/J/K      switch to diff pane                                 │"
        "│                                                                              │"
        "│ Help Pane                                                                    │"
        "│     j / ↓                scroll down                                         │"
        "│     k / ↑                scroll up                                           │"
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
fn scrolling_help_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();

    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Help));

    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     ?                    show/hide help view                                 │"
        "│     Esc / q              go back/exit                                        │"
        "│     <ctrl+c>             exit immediately                                    │"
        "│                                                                              │"
        "│ Diff Pane                                                                    │"
        "│     Tab/<S-Tab>/J/K      switch to changes pane                              │"
        "│     j / ↓                scroll down                                         │"
        "│     k / ↑                scroll up                                           │"
        "│     l/→                  select next change                                  │"
        "│     h/←                  select previous change                              │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│     f                    toggle following changes                            │"
        "│                                                                              │"
        "│ Changes Pane                                                                 │"
        "│     j / ↓                select next change                                  │"
        "│     k / ↑                select previous change                              │"
        "│     g                    select first change                                 │"
        "│     G                    select last change                                  │"
        "│     f                    toggle following changes                            │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
}

#[test]
fn help_pane_doesnt_scroll_beyond_lower_limit() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();

    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Help));

    for _ in 1..=20 {
        update(&mut model, Msg::ScrollDown);
    }

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ help ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     k / ↑                scroll up                                           │"
        "│     l/→                  select next change                                  │"
        "│     h/←                  select previous change                              │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│     f                    toggle following changes                            │"
        "│                                                                              │"
        "│ Changes Pane                                                                 │"
        "│     j / ↓                select next change                                  │"
        "│     k / ↑                select previous change                              │"
        "│     g                    select first change                                 │"
        "│     G                    select last change                                  │"
        "│     f                    toggle following changes                            │"
        "│     <c-r>                reset list                                          │"
        "│     <space>              toggle watching                                     │"
        "│     Tab/<S-Tab>/J/K      switch to diff pane                                 │"
        "│                                                                              │"
        "│ Help Pane                                                                    │"
        "│     j / ↓                scroll down                                         │"
        "│     k / ↑                scroll up                                           │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        " dfft  [watching]                                                               "
        "#);
}

#[test]
fn help_pane_doesnt_scroll_above_upper_limit() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();

    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Help));

    for _ in 1..=3 {
        update(&mut model, Msg::ScrollUp);
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
        "│ Diff Pane                                                                    │"
        "│     Tab/<S-Tab>/J/K      switch to changes pane                              │"
        "│     j / ↓                scroll down                                         │"
        "│     k / ↑                scroll up                                           │"
        "│     l/→                  select next change                                  │"
        "│     h/←                  select previous change                              │"
        "│     <space>              toggle watching                                     │"
        "│     <c-r>                reset list                                          │"
        "│     f                    toggle following changes                            │"
        "│                                                                              │"
        "│ Changes Pane                                                                 │"
        "│     j / ↓                select next change                                  │"
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(100, 42);
    let mut model = Model::new(terminal_dimensions, true, false);

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
        "│                                         In the diff pane:                                        │"
        "│                                   - use j/k/↓/↑ to scroll diff                                   │"
        "│                               - use h/l/←/→ to move between changes                              │"
        "│                                                                                                  │"
        "│                                       In the changes pane:                                       │"
        "│                               - use j/k/↓/↑ to move between changes                              │"
        "│                                                                                                  │"
        "│                               Tab/<S-Tab>/J/K to move between panes                              │"
        "│                                                                                                  │"
        "│                                         Press ? for help                                         │"
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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

    let diff = Diff::new(
        "
line 1
line 2
line 3
",
        "
line 1 (changed)
new line
line 2
(prefix) line 3 (changed)
",
    )
    .expect("diff should've been generated");
    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 1   1   |                                                                    │"
        "│ 2       |-line 1                                                             │"
        "│     2   |+line 1 (changed)                                                   │"
        "│     3   |+new line                                                           │"
        "│ 3   4   | line 2                                                             │"
        "│ 4       |-line 3                                                             │"
        "│     5   |+(prefix) line 3 (changed)                                          │"
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
fn diff_pane_renders_diff_with_several_hunks_correctly() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 34);
    let mut model = Model::new(terminal_dimensions, true, false);

    let mut lines = (1..=10001).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[8] = "line 9 (modified)".to_string();
    lines[9] = "line 10 (modified)".to_string();

    lines[9998] = "line 9999 (modified)".to_string();
    lines[9999] = "line 10000 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 6      6      | line 6                                                       │"
        "│ 7      7      | line 7                                                       │"
        "│ 8      8      | line 8                                                       │"
        "│ 9             |-line 9                                                       │"
        "│ 10            |-line 10                                                      │"
        "│        9      |+line 9 (modified)                                            │"
        "│        10     |+line 10 (modified)                                           │"
        "│ 11     11     | line 11                                                      │"
        "│ 12     12     | line 12                                                      │"
        "│ 13     13     | line 13                                                      │"
        "│ -----------------------------------------------------------------------------│"
        "│ 9996   9996   | line 9996                                                    │"
        "│ 9997   9997   | line 9997                                                    │"
        "│ 9998   9998   | line 9998                                                    │"
        "│ 9999          |-line 9999                                                    │"
        "│ 10000         |-line 10000                                                   │"
        "│        9999   |+line 9999 (modified)                                         │"
        "│        10000  |+line 10000 (modified)                                        │"
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
fn scrolling_diff_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Diff));

    let mut lines = (1..=30).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[3] = "line 4 (modified)".to_string();
    lines[4] = "line 5 (modified)".to_string();
    lines[23] = "line 24 (modified)".to_string();
    lines[24] = "line 25 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 1   1   | line 1                                                             │"
        "│ 2   2   | line 2                                                             │"
        "│ 3   3   | line 3                                                             │"
        "│ 4       |-line 4                                                             │"
        "│ 5       |-line 5                                                             │"
        "│     4   |+line 4 (modified)                                                  │"
        "│     5   |+line 5 (modified)                                                  │"
        "│ 6   6   | line 6                                                             │"
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

    // WHEN
    for _ in 1..=3 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 4       |-line 4                                                             │"
        "│ 5       |-line 5                                                             │"
        "│     4   |+line 4 (modified)                                                  │"
        "│     5   |+line 5 (modified)                                                  │"
        "│ 6   6   | line 6                                                             │"
        "│ 7   7   | line 7                                                             │"
        "│ 8   8   | line 8                                                             │"
        "│ -----------------------------------------------------------------------------│"
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
fn diff_scrolling_is_reset_when_follow_mode_is_on() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Diff));
    update(&mut model, Msg::ToggleFollowChanges);

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     10  |+line 10 (modified)                                                 │"
        "│ 11  11  | line 11                                                            │"
        "│ 12  12  | line 12                                                            │"
        "│ 13  13  | line 13                                                            │"
        "│ -----------------------------------------------------------------------------│"
        "│ 27  27  | line 27                                                            │"
        "│ 28  28  | line 28                                                            │"
        "│ 29  29  | line 29                                                            │"
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
        " dfft  [watching] [following changes]                                           "
        "#);

    // WHEN
    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[0] = "line 1 (modified)".to_string();

    let new = lines.join("\n");
    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "another_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 1       |-line 1                                                             │"
        "│     1   |+line 1 (modified)                                                  │"
        "│ 2   2   | line 2                                                             │"
        "│ 3   3   | line 3                                                             │"
        "│ 4   4   | line 4                                                             │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (2) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│   modified  modified_file.txt                                                │"
        "│>  modified  another_modified_file.txt                                        │"
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
fn diff_scrolling_is_reset_when_another_change_is_selected() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Diff));

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[0] = "line 1 (modified)".to_string();

    let new = lines.join("\n");
    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "another_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     10  |+line 10 (modified)                                                 │"
        "│ 11  11  | line 11                                                            │"
        "│ 12  12  | line 12                                                            │"
        "│ 13  13  | line 13                                                            │"
        "│ -----------------------------------------------------------------------------│"
        "│ 27  27  | line 27                                                            │"
        "│ 28  28  | line 28                                                            │"
        "│ 29  29  | line 29                                                            │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (2) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  modified_file.txt                                                │"
        "│   modified  another_modified_file.txt                                        │"
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

    // WHEN
    update(&mut model, Msg::SelectNext);
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 1       |-line 1                                                             │"
        "│     1   |+line 1 (modified)                                                  │"
        "│ 2   2   | line 2                                                             │"
        "│ 3   3   | line 3                                                             │"
        "│ 4   4   | line 4                                                             │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (2) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│   modified  modified_file.txt                                                │"
        "│>  modified  another_modified_file.txt                                        │"
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
fn max_diff_scroll_is_reset_when_change_list_is_reset() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::GoToPane(Pane::Diff));

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        file_path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│     10  |+line 10 (modified)                                                 │"
        "│ 11  11  | line 11                                                            │"
        "│ 12  12  | line 12                                                            │"
        "│ 13  13  | line 13                                                            │"
        "│ -----------------------------------------------------------------------------│"
        "│ 27  27  | line 27                                                            │"
        "│ 28  28  | line 28                                                            │"
        "│ 29  29  | line 29                                                            │"
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

    // WHEN
    update(&mut model, Msg::ResetList);
    let diff = Diff::new("old", "new").expect("diff should've been created");

    let change = Change {
        file_path: "newly_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(Some(diff)))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│ 1       |-old                                                                │"
        "│     1   |+new                                                                │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "└──────────────────────────────────────────────────────────────────────────────┘"
        "┌ changes (1) ─────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│>  modified  newly_modified_file.txt                                          │"
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
        kind: ChangeKind::Modified(Ok(Modification::InitialSnapshot)),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
        kind: ChangeKind::Modified(Ok(Modification::Diff(None))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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
    update(&mut model, Msg::ToggleFollowChanges);

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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
    model.user_msg = Some(UserMsg::info("This will disappear after 2 renders").with_frames_left(1));

    // WHEN
    // THEN
    update(&mut model, Msg::SelectNext);
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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

    update(&mut model, Msg::SelectNext);
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
        "┌ diff ────────────────────────────────────────────────────────────────────────┐"
        "│                                                                              │"
        "│                                                                              │"
        "│                                                                              │"
        "│                                     dfft                                     │"
        "│                                     ‾‾‾‾                                     │"
        "│                                                                              │"
        "│              see changes to files in a directory as they happen              │"
        "│                                                                              │"
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
fn cursor_moves_automatically_when_following_enabled() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(terminal_dimensions, true, false);
    update(&mut model, Msg::ToggleFollowChanges);

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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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

    // WHEN
    // THEN
    for i in 1..=3 {
        let change = Change {
            file_path: format!("file-{i}.txt"),
            kind: ChangeKind::Created(Ok(())),
        };
        update(&mut model, Msg::ChangeReceived(change));
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
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

    update(&mut model, Msg::ToggleFollowChanges);
    let change = Change {
        file_path: "this-should-be-selected.txt".to_string(),
        kind: ChangeKind::Created(Ok(())),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
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
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
