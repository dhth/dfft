use super::super::TuiBehaviours;
use super::super::common::Pane;
use super::super::model::Model;
use super::super::{msg::Msg, update::update, view::view};
use super::helpers::get_test_terminal;
use crate::domain::{Change, ChangeKind};
use insta::assert_snapshot;
use std::path::PathBuf;

#[test]
fn selecting_first_change_from_an_empty_changes_list_doesnt_crash() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    // WHEN
    update(&mut model, Msg::SelectFirst);
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
    " dfft  [watching]                                                               "
    "#);
}

#[test]
fn selecting_last_change_from_an_empty_changes_list_doesnt_crash() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );

    // WHEN
    update(&mut model, Msg::SelectLast);
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
    " dfft  [watching]                                                               "
    "#);
}

#[test]
fn selecting_first_change_from_changes_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::GoToPane(Pane::Changes));
    assert_eq!(model.active_pane, Pane::Changes);

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
fn selecting_last_change_from_changes_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::GoToPane(Pane::Changes));
    assert_eq!(model.active_pane, Pane::Changes);

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
