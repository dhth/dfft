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

    Command:                                                  Run TUI
    Start with change following disabled:                     false
    Skip prepopulating cache with existing file snapshots:    false
    Start with file watching disabled:                        false

    ----- stderr -----
    ");
}

#[test]
fn turning_off_following_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--debug", "run", "--no-follow"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    Command:                                                  Run TUI
    Start with change following disabled:                     true
    Skip prepopulating cache with existing file snapshots:    false
    Start with file watching disabled:                        false

    ----- stderr -----
    ");
}

#[test]
fn turning_off_prepopulation_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--debug", "run", "--no-prepop"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    Command:                                                  Run TUI
    Start with change following disabled:                     false
    Skip prepopulating cache with existing file snapshots:    true
    Start with file watching disabled:                        false

    ----- stderr -----
    ");
}

#[test]
fn turning_off_watching_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--debug", "run", "--no-watch"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    Command:                                                  Run TUI
    Start with change following disabled:                     false
    Skip prepopulating cache with existing file snapshots:    false
    Start with file watching disabled:                        true

    ----- stderr -----
    ");
}

#[test]
fn short_flags_work() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--debug", "run", "-F", "-P", "-W"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    Command:                                                  Run TUI
    Start with change following disabled:                     true
    Skip prepopulating cache with existing file snapshots:    true
    Start with file watching disabled:                        true

    ----- stderr -----
    ");
}
