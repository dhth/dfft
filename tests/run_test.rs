mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
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
    no audio:           false

    ----- stderr -----
    ");
}

#[test]
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
    no audio:           false

    ----- stderr -----
    ");
}

#[test]
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
    no audio:           false

    ----- stderr -----
    ");
}

#[test]
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
    no audio:           false

    ----- stderr -----
    ");
}

#[test]
fn turning_off_audio_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--no-audio", "--debug"]);

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
    no audio:           true

    ----- stderr -----
    ");
}
