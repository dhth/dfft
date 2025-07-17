use super::super::TuiBehaviours;
use super::super::common::{Pane, TerminalDimensions};
use super::super::model::Model;
use super::super::{msg::Msg, update::update, view::view};
use super::helpers::{get_test_terminal, get_test_terminal_with_dims};
use crate::domain::{Change, ChangeKind, Diff, Modification};
use insta::assert_snapshot;
use std::path::PathBuf;

#[test]
fn diff_pane_doesnt_scroll_beyond_limits() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let mut lines = (1..=20).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[3] = "line 4 (modified)".to_string();
    lines[4] = "line 5 (modified)".to_string();
    lines[13] = "line 14 (modified)".to_string();
    lines[14] = "line 15 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    // THEN
    for _ in 1..=5 {
        update(&mut model, Msg::ScrollUp);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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

    for _ in 1..=20 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ 13  13  | line 13                                                            │"
    "│ 14      |-line 14                                                            │"
    "│ 15      |-line 15                                                            │"
    "│     14  |+line 14 (modified)                                                 │"
    "│     15  |+line 15 (modified)                                                 │"
    "│ 16  16  | line 16                                                            │"
    "│ 17  17  | line 17                                                            │"
    "│ 18  18  | line 18                                                            │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn main_view_renders_modified_file_with_diff() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn selecting_first_change_from_diff_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    assert_eq!(model.active_pane, Pane::Diff);

    for i in 1..=20 {
        let change = Change {
            path: format!("file-{i}.txt"),
            kind: ChangeKind::Created(Ok(format!("file {i} contents"))),
        };
        update(&mut model, Msg::ChangeReceived(change));
    }
    for _ in 1..=4 {
        update(&mut model, Msg::SelectNext);
    }

    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  file-5.txt ────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ file 5 contents                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (5/20) ─────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│   created   file-1.txt                                                       │"
    "│   created   file-2.txt                                                       │"
    "│   created   file-3.txt                                                       │"
    "│   created   file-4.txt                                                       │"
    "│>  created   file-5.txt                                                       │"
    "│   created   file-6.txt                                                       │"
    "│   created   file-7.txt                                                       │"
    "│   created   file-8.txt                                                       │"
    "│   created   file-9.txt                                                       │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching]                                                               "
    "#);

    // WHEN
    update(&mut model, Msg::SelectFirst);

    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  file-1.txt ────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ file 1 contents                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/20) ─────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   file-1.txt                                                       │"
    "│   created   file-2.txt                                                       │"
    "│   created   file-3.txt                                                       │"
    "│   created   file-4.txt                                                       │"
    "│   created   file-5.txt                                                       │"
    "│   created   file-6.txt                                                       │"
    "│   created   file-7.txt                                                       │"
    "│   created   file-8.txt                                                       │"
    "│   created   file-9.txt                                                       │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching]                                                               "
    "#);
}

#[test]
fn selecting_last_change_from_diff_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    assert_eq!(model.active_pane, Pane::Diff);

    for i in 1..=20 {
        let change = Change {
            path: format!("file-{i}.txt"),
            kind: ChangeKind::Created(Ok(format!("file {i} contents"))),
        };
        update(&mut model, Msg::ChangeReceived(change));
    }

    // WHEN
    update(&mut model, Msg::SelectLast);

    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  file-20.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ file 20 contents                                                             │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (20/20) ────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│   created   file-12.txt                                                      │"
    "│   created   file-13.txt                                                      │"
    "│   created   file-14.txt                                                      │"
    "│   created   file-15.txt                                                      │"
    "│   created   file-16.txt                                                      │"
    "│   created   file-17.txt                                                      │"
    "│   created   file-18.txt                                                      │"
    "│   created   file-19.txt                                                      │"
    "│>  created   file-20.txt                                                      │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching]                                                               "
    "#);
}

#[test]
fn diff_pane_renders_diff_with_several_hunks_correctly() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 34);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let mut lines = (1..=10001).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[8] = "line 9 (modified)".to_string();
    lines[9] = "line 10 (modified)".to_string();

    lines[9998] = "line 9999 (modified)".to_string();
    lines[9999] = "line 10000 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn scrolling_diff_by_a_line_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
    // THEN
    for _ in 1..=3 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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

    for _ in 1..=3 {
        update(&mut model, Msg::ScrollUp);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn scrolling_diff_by_half_a_page_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
    // THEN
    update(&mut model, Msg::ScrollHalfPageDown);
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ 5       |-line 5                                                             │"
    "│     4   |+line 4 (modified)                                                  │"
    "│     5   |+line 5 (modified)                                                  │"
    "│ 6   6   | line 6                                                             │"
    "│ 7   7   | line 7                                                             │"
    "│ 8   8   | line 8                                                             │"
    "│ -----------------------------------------------------------------------------│"
    "│ 21  21  | line 21                                                            │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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

    update(&mut model, Msg::ScrollHalfPageUp);
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn diff_scroll_is_reset_when_follow_mode_is_on() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::GoToPane(Pane::Diff));
    update(&mut model, Msg::ToggleFollowChanges);

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
        path: "another_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  another_modified_file.txt ─────────────────────────────────────────────┐"
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
    "┌ changes  (2/2) ──────────────────────────────────────────────────────────────┐"
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
fn diff_scroll_is_reset_when_another_change_is_selected() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::GoToPane(Pane::Diff));

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
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
        path: "another_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/2) ──────────────────────────────────────────────────────────────┐"
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
    "┌ diff  another_modified_file.txt ─────────────────────────────────────────────┐"
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
    "┌ changes  (2/2) ──────────────────────────────────────────────────────────────┐"
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
fn max_scroll_for_diff_is_reset_when_change_list_is_reset() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let mut lines = (1..=50).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[9] = "line 10 (modified)".to_string();
    lines[29] = "line 30 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    for _ in 1..=4 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
        path: "newly_modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
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
    "┌ diff  newly_modified_file.txt ───────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn max_scroll_for_diff_is_recomputed_when_terminal_height_changes() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 30);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let mut lines = (1..=20).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[0] = "line 1 (modified)".to_string();
    lines[1] = "line 2 (modified)".to_string();
    lines[14] = "line 15 (modified)".to_string();
    lines[15] = "line 16 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    // THEN
    for _ in 1..=5 {
        update(&mut model, Msg::ScrollDown);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ 3   3   | line 3                                                             │"
    "│ 4   4   | line 4                                                             │"
    "│ 5   5   | line 5                                                             │"
    "│ -----------------------------------------------------------------------------│"
    "│ 12  12  | line 12                                                            │"
    "│ 13  13  | line 13                                                            │"
    "│ 14  14  | line 14                                                            │"
    "│ 15      |-line 15                                                            │"
    "│ 16      |-line 16                                                            │"
    "│     15  |+line 15 (modified)                                                 │"
    "│     16  |+line 16 (modified)                                                 │"
    "│ 17  17  | line 17                                                            │"
    "│ 18  18  | line 18                                                            │"
    "│ 19  19  | line 19                                                            │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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

    let (mut new_terminal, new_terminal_dimensions) = get_test_terminal();
    update(
        &mut model,
        Msg::TerminalResize(
            new_terminal_dimensions.width,
            new_terminal_dimensions.height,
        ),
    );
    new_terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(new_terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ 3   3   | line 3                                                             │"
    "│ 4   4   | line 4                                                             │"
    "│ 5   5   | line 5                                                             │"
    "│ -----------------------------------------------------------------------------│"
    "│ 12  12  | line 12                                                            │"
    "│ 13  13  | line 13                                                            │"
    "│ 14  14  | line 14                                                            │"
    "│ 15      |-line 15                                                            │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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

    for _ in 1..=20 {
        update(&mut model, Msg::ScrollDown);
    }

    new_terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(new_terminal.backend(), @r#"
    "┌ diff  modified_file.txt ─────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ 14  14  | line 14                                                            │"
    "│ 15      |-line 15                                                            │"
    "│ 16      |-line 16                                                            │"
    "│     15  |+line 15 (modified)                                                 │"
    "│     16  |+line 16 (modified)                                                 │"
    "│ 17  17  | line 17                                                            │"
    "│ 18  18  | line 18                                                            │"
    "│ 19  19  | line 19                                                            │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn max_scroll_for_diff_doesnt_change_when_only_terminal_width_changes() {
    // GIVEN
    let terminal_dimensions = TerminalDimensions::min_needed();
    let (width, height) = terminal_dimensions.values();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let mut lines = (1..=20).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[0] = "line 1 (modified)".to_string();
    lines[1] = "line 2 (modified)".to_string();
    lines[14] = "line 15 (modified)".to_string();
    lines[15] = "line 16 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    for _ in 1..=5 {
        update(&mut model, Msg::ScrollDown);
    }
    let max_diff_scroll = model.max_diff_scroll_available;

    // WHEN
    // THEN
    update(&mut model, Msg::TerminalResize(width + 20, height));
    assert_eq!(model.max_diff_scroll_available, max_diff_scroll);
}

#[test]
fn max_scroll_for_diff_is_recomputed_when_terminal_size_crosses_minimum_threshold() {
    // GIVEN
    let terminal_dimensions = TerminalDimensions::min_needed();
    let (width, height) = terminal_dimensions.values();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let mut lines = (1..=20).map(|n| format!("line {n}")).collect::<Vec<_>>();
    let old = lines.join("\n");

    lines[0] = "line 1 (modified)".to_string();
    lines[1] = "line 2 (modified)".to_string();
    lines[14] = "line 15 (modified)".to_string();
    lines[15] = "line 16 (modified)".to_string();

    let new = lines.join("\n");

    let diff = Diff::new(&old, &new).expect("diff should've been created");

    let change = Change {
        path: "modified_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::Diff(diff))),
    };
    update(&mut model, Msg::ChangeReceived(change));
    for _ in 1..=5 {
        update(&mut model, Msg::ScrollDown);
    }
    let max_diff_scroll = model.max_diff_scroll_available;

    // WHEN
    // THEN
    update(&mut model, Msg::TerminalResize(width - 20, height));
    assert_eq!(model.max_diff_scroll_available, 0);

    update(&mut model, Msg::TerminalResize(width + 20, height));
    assert_eq!(model.max_diff_scroll_available, max_diff_scroll);
}
