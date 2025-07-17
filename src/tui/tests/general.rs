use super::super::TuiBehaviours;
use super::super::model::{Model, UserMsg};
use super::super::{msg::Msg, update::update, view::view};
use super::helpers::{get_test_terminal, get_test_terminal_with_dims};
use crate::domain::{Change, ChangeKind, Modification};
use insta::assert_snapshot;
use std::path::PathBuf;

#[test]
fn terminal_too_small_view_is_shown_when_width_too_small() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(60, 24);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
    "│                      will show you changes in this directory as they happen                      │"
    "│                      ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾                      │"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "new_file.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  new_file.txt ──────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn main_view_renders_removed_file_change() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "deleted_file.txt".to_string(),
        kind: ChangeKind::RemovedFile,
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  deleted_file.txt ──────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "error_file.txt".to_string(),
        kind: ChangeKind::Created(Err("Permission denied".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  error_file.txt ────────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "error_modified.txt".to_string(),
        kind: ChangeKind::Modified(Err("File not found".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  error_modified.txt ────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "snapshot_file.txt".to_string(),
        kind: ChangeKind::Modified(Ok(Modification::InitialSnapshot)),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  snapshot_file.txt ─────────────────────────────────────────────────────┐"
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
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
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
fn changes_list_shows_item_count_in_title() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    for i in 0..3 {
        let change = Change {
            path: format!("file{i}.txt"),
            kind: ChangeKind::Created(Ok("test file contents".to_string())),
        };
        update(&mut model, Msg::ChangeReceived(change));
    }

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  file0.txt ─────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/3) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "a/very/long/path/to/a/file/that/goes/beyond/the/terminal/width/dimension.txt"
            .to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  a/very/long/path/to/a/file/that/goes/beyond/the/terminal/width/dimensio┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   a/very/long/path/to/a/file/that/goes/beyond/the/terminal/width/di│"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test().with_watch_off(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

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
    "│            will show you changes in this directory as they happen            │"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│            will show you changes in this directory as they happen            │"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│            will show you changes in this directory as they happen            │"
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
    " dfft  [watching] Test info message                                             "
    "#);
}

#[test]
fn status_line_shows_error_message() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│            will show you changes in this directory as they happen            │"
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
    " dfft  [watching] Test error message                                            "
    "#);
}

#[test]
fn info_message_disappears_after_its_frame_budget_expires() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│            will show you changes in this directory as they happen            │"
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
    " dfft  [watching] This will disappear after 2 renders                           "
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
    "│            will show you changes in this directory as they happen            │"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::ToggleFollowChanges);

    let change = Change {
        path: "first.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    let change = Change {
        path: "this-should-be-selected.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  this-should-be-selected.txt ───────────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (2/2) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    // WHEN
    // THEN
    for i in 1..=3 {
        let change = Change {
            path: format!("file-{i}.txt"),
            kind: ChangeKind::Created(Ok("test file contents".to_string())),
        };
        update(&mut model, Msg::ChangeReceived(change));
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  file-1.txt ────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/3) ──────────────────────────────────────────────────────────────┐"
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
        path: "this-should-be-selected.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  this-should-be-selected.txt ───────────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (4/4) ──────────────────────────────────────────────────────────────┐"
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
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    let change = Change {
        path: "this-will-still-be-selected.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    let change = Change {
        path: "second.txt".to_string(),
        kind: ChangeKind::Created(Ok("test file contents".to_string())),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  this-will-still-be-selected.txt ───────────────────────────────────────┐"
    "│                                                                              │"
    "│ test file contents                                                           │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "│                                                                              │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/2) ──────────────────────────────────────────────────────────────┐"
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

#[test]
fn scrolling_for_created_file_contents_doesnt_go_beyond_limits() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let contents = (1..=10)
        .map(|n| format!("line {n}"))
        .collect::<Vec<_>>()
        .join("\n");

    let change = Change {
        path: "created.txt".to_string(),
        kind: ChangeKind::Created(Ok(contents)),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    // THEN
    for _ in 1..=3 {
        update(&mut model, Msg::ScrollUp);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  created.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ line 1                                                                       │"
    "│ line 2                                                                       │"
    "│ line 3                                                                       │"
    "│ line 4                                                                       │"
    "│ line 5                                                                       │"
    "│ line 6                                                                       │"
    "│ line 7                                                                       │"
    "│ line 8                                                                       │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   created.txt                                                      │"
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
    "┌ diff  created.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ line 3                                                                       │"
    "│ line 4                                                                       │"
    "│ line 5                                                                       │"
    "│ line 6                                                                       │"
    "│ line 7                                                                       │"
    "│ line 8                                                                       │"
    "│ line 9                                                                       │"
    "│ line 10                                                                      │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   created.txt                                                      │"
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
fn max_scroll_for_created_file_contents_is_recomputed_when_terminal_height_changes() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 30);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let contents = (1..=20)
        .map(|n| format!("line {n}"))
        .collect::<Vec<_>>()
        .join("\n");

    let change = Change {
        path: "created.txt".to_string(),
        kind: ChangeKind::Created(Ok(contents)),
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
    "┌ diff  created.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ line 6                                                                       │"
    "│ line 7                                                                       │"
    "│ line 8                                                                       │"
    "│ line 9                                                                       │"
    "│ line 10                                                                      │"
    "│ line 11                                                                      │"
    "│ line 12                                                                      │"
    "│ line 13                                                                      │"
    "│ line 14                                                                      │"
    "│ line 15                                                                      │"
    "│ line 16                                                                      │"
    "│ line 17                                                                      │"
    "│ line 18                                                                      │"
    "│ line 19                                                                      │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   created.txt                                                      │"
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
    "┌ diff  created.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ line 6                                                                       │"
    "│ line 7                                                                       │"
    "│ line 8                                                                       │"
    "│ line 9                                                                       │"
    "│ line 10                                                                      │"
    "│ line 11                                                                      │"
    "│ line 12                                                                      │"
    "│ line 13                                                                      │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   created.txt                                                      │"
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
    "┌ diff  created.txt ───────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│ line 13                                                                      │"
    "│ line 14                                                                      │"
    "│ line 15                                                                      │"
    "│ line 16                                                                      │"
    "│ line 17                                                                      │"
    "│ line 18                                                                      │"
    "│ line 19                                                                      │"
    "│ line 20                                                                      │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ──────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│>  created   created.txt                                                      │"
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

#[cfg(feature = "sound")]
#[test]
fn sound_unavailable_indicator_is_shown_when_applicable() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(90, 24);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    model.behaviours.play_sound = true;
    model.make_sound_unavailable();

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff ──────────────────────────────────────────────────────────────────────────────────┐"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                          dfft                                          │"
    "│                                          ‾‾‾‾                                          │"
    "│                                                                                        │"
    "│                 will show you changes in this directory as they happen                 │"
    "│                                                                                        │"
    "│                                                                                        │"
    "└────────────────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes ───────────────────────────────────────────────────────────────────────────────┐"
    "│                                                                                        │"
    "│ changes will appear here                                                               │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "└────────────────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching] [sound unavailable]                                                     "
    "#);
}

#[test]
fn showing_debug_info_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(90, 24);
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        true,
    );
    let contents = (1..=5)
        .map(|n| format!("line {n}"))
        .collect::<Vec<_>>()
        .join("\n");

    let change = Change {
        path: "created.txt".to_string(),
        kind: ChangeKind::Created(Ok(contents)),
    };
    update(&mut model, Msg::ChangeReceived(change));

    // WHEN
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

    // THEN
    assert_snapshot!(terminal.backend(), @r#"
    "┌ diff  created.txt ─────────────────────────────────────────────────────────────────────┐"
    "│                                                                                        │"
    "│ line 1                                                                                 │"
    "│ line 2                                                                                 │"
    "│ line 3                                                                                 │"
    "│ line 4                                                                                 │"
    "│ line 5                                                                                 │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "└────────────────────────────────────────────────────────────────────────────────────────┘"
    "┌ changes  (1/1) ────────────────────────────────────────────────────────────────────────┐"
    "│                                                                                        │"
    "│>  created   created.txt                                                                │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "│                                                                                        │"
    "└────────────────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching] [render: 0] [event: 0] [watch: 0] [dimensions: 90x24]                   "
    "#);
}
