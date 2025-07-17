mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn showing_help_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    monitor changes as AI agents modify your codebase

    Usage: dfft [OPTIONS] <COMMAND>

    Commands:
      run   Run dfft's TUI
      help  Print this message or the help of the given subcommand(s)

    Options:
          --debug  Output debug information without doing anything
      -h, --help   Print help

    ----- stderr -----
    ");
}

#[test]
#[cfg(feature = "sound")]
fn showing_help_for_run_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["run", "--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Run dfft's TUI

    Usage: dfft run [OPTIONS]

    Options:
      -p, --path <PATH>     Path of the directory to watch (defaults to current directory)
          --debug           Output debug information without doing anything
      -f, --follow-changes  Start with the setting "follow changes" enabled
          --no-prepop       Skip prepopulating cache with file snapshots
          --no-watch        Start with file watching disabled
          --no-sound        Start with sound notifications disabled
      -h, --help            Print help

    ----- stderr -----
    "#);
}
