use super::super::TuiBehaviours;
use super::super::common::{Pane, TerminalDimensions};
use super::super::model::Model;
use super::super::{msg::Msg, update::update, view::view};
#[cfg(feature = "sound")]
use super::helpers::get_test_terminal;
use super::helpers::get_test_terminal_with_dims;
use insta::assert_snapshot;
use std::path::PathBuf;

#[test]
#[cfg(feature = "sound")]
fn rendering_help_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 50);

    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     <space>              toggle watching                                     │"
    "│     <c-r>                reset list                                          │"
    "│     f                    toggle following changes                            │"
    "│     s                    toggle sound notifications                          │"
    "│     <tab>/<s-tab>        switch to changes pane                              │"
    "│                                                                              │"
    "│ Changes Pane                                                                 │"
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     f                    toggle following changes                            │"
    "│     s                    toggle sound notifications                          │"
    "│     <c-r>                reset list                                          │"
    "│     <space>              toggle watching                                     │"
    "│     <tab>/<s-tab>        switch to diff pane                                 │"
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
#[cfg(not(feature = "sound"))]
fn help_doesnt_show_keymaps_for_sound_if_feature_is_off() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal_with_dims(80, 48);

    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     <space>              toggle watching                                     │"
    "│     <c-r>                reset list                                          │"
    "│     f                    toggle following changes                            │"
    "│     <tab>/<s-tab>        switch to changes pane                              │"
    "│                                                                              │"
    "│ Changes Pane                                                                 │"
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     f                    toggle following changes                            │"
    "│     <c-r>                reset list                                          │"
    "│     <space>              toggle watching                                     │"
    "│     <tab>/<s-tab>        switch to diff pane                                 │"
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
#[cfg(feature = "sound")]
fn scrolling_help_pane_works() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();

    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
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
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     <space>              toggle watching                                     │"
    "│     <c-r>                reset list                                          │"
    "│     f                    toggle following changes                            │"
    "│     s                    toggle sound notifications                          │"
    "│     <tab>/<s-tab>        switch to changes pane                              │"
    "│                                                                              │"
    "│ Changes Pane                                                                 │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching]                                                               "
    "#);
}

#[test]
#[cfg(feature = "sound")]
fn help_pane_doesnt_scroll_beyond_limits() {
    // GIVEN
    let (mut terminal, terminal_dimensions) = get_test_terminal();

    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    update(&mut model, Msg::GoToPane(Pane::Help));

    // WHEN
    // THEN
    for _ in 1..=3 {
        update(&mut model, Msg::ScrollUp);
    }
    terminal
        .draw(|f| view(&mut model, f))
        .expect("frame should've been drawn");

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
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     <space>              toggle watching                                     │"
    "│     <c-r>                reset list                                          │"
    "│     f                    toggle following changes                            │"
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
    "┌ help ────────────────────────────────────────────────────────────────────────┐"
    "│                                                                              │"
    "│     s                    toggle sound notifications                          │"
    "│     <tab>/<s-tab>        switch to changes pane                              │"
    "│                                                                              │"
    "│ Changes Pane                                                                 │"
    "│     j / ↓                select next change                                  │"
    "│     k / ↑                select previous change                              │"
    "│     g                    select first change                                 │"
    "│     G                    select last change                                  │"
    "│     J                    scroll diff down by a line                          │"
    "│     K                    scroll diff up by a line                            │"
    "│     <c-d>                scroll diff down by half page                       │"
    "│     <c-u>                scroll diff up by half page                         │"
    "│     f                    toggle following changes                            │"
    "│     s                    toggle sound notifications                          │"
    "│     <c-r>                reset list                                          │"
    "│     <space>              toggle watching                                     │"
    "│     <tab>/<s-tab>        switch to diff pane                                 │"
    "│                                                                              │"
    "│ Help Pane                                                                    │"
    "│     j / ↓                scroll down                                         │"
    "└──────────────────────────────────────────────────────────────────────────────┘"
    " dfft  [watching]                                                               "
    "#);
}

#[test]
fn max_scroll_for_help_doesnt_change_when_only_terminal_width_changes() {
    // GIVEN
    let terminal_dimensions = TerminalDimensions::min_needed();
    let (width, height) = terminal_dimensions.values();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let max_help_scroll = model.max_help_scroll_available;

    // WHEN
    update(&mut model, Msg::TerminalResize(width + 20, height));

    // THEN
    assert_eq!(model.max_help_scroll_available, max_help_scroll);
}

#[test]
fn max_scroll_for_help_is_recomputed_when_terminal_size_crosses_minimum_threshold() {
    // GIVEN
    let terminal_dimensions = TerminalDimensions::min_needed();
    let (width, height) = terminal_dimensions.values();
    let mut model = Model::new(
        TuiBehaviours::default_for_test(),
        PathBuf::new(),
        terminal_dimensions,
        false,
    );
    let max_help_scroll = model.max_help_scroll_available;

    // WHEN
    // THEN
    update(&mut model, Msg::TerminalResize(width - 20, height));
    assert_eq!(model.max_help_scroll_available, 0);

    update(&mut model, Msg::TerminalResize(width + 20, height));
    assert_eq!(model.max_help_scroll_available, max_help_scroll);
}
