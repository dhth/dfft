mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
#[cfg(feature = "sound")]
fn debug_flag_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--debug", "run"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     false
    no prepopulation:   false
    no watch:           false
    no sound:           false

    ----- stderr -----
    ");
}

#[test]
#[cfg(feature = "sound")]
fn turning_off_following_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--follow-changes", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     true
    no prepopulation:   false
    no watch:           false
    no sound:           false

    ----- stderr -----
    ");
}

#[test]
#[cfg(feature = "sound")]
fn turning_off_prepopulation_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--no-prepop", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     false
    no prepopulation:   true
    no watch:           false
    no sound:           false

    ----- stderr -----
    ");
}

#[test]
#[cfg(feature = "sound")]
fn turning_off_watching_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--no-watch", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     false
    no prepopulation:   false
    no watch:           true
    no sound:           false

    ----- stderr -----
    ");
}

#[test]
#[cfg(feature = "sound")]
fn turning_off_sound_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--no-sound", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     false
    no prepopulation:   false
    no watch:           false
    no sound:           true

    ----- stderr -----
    ");
}

#[test]
#[cfg(not(feature = "sound"))]
fn sound_flag_is_not_shown_if_feature_is_off() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:            run TUI
    follow changes:     false
    no prepopulation:   false
    no watch:           false

    ----- stderr -----
    ");
}
