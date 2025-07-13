use insta_cmd::get_cargo_bin;
use std::{ffi::OsStr, path::PathBuf, process::Command};

pub struct Fixture {
    _bin_path: PathBuf,
}

#[cfg(test)]
#[allow(unused)]
impl Fixture {
    pub fn new() -> Self {
        let bin_path = get_cargo_bin("dfft");

        Self {
            _bin_path: bin_path,
        }
    }

    pub fn base_cmd(&self) -> Command {
        Command::new(&self._bin_path)
    }

    pub fn cmd<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(&self._bin_path);
        command.args(args);
        command
    }
}
