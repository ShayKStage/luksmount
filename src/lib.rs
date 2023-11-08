#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::{ffi::OsStr, process::Command};

pub fn run_command<I, S>(program: &str, arguments: I, error: &str, quit_on: QuitOn) -> (bool, i32)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    let mut success = true;
    let mut retval = 0;

    for argument in arguments {
        command.arg(argument);
    }

    match command.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("{error}");
                retval = status.code().unwrap_or(-1);
                success = false;
                if quit_on == QuitOn::Error {
                    std::process::exit(retval)
                }
            }
        }
        Err(error) => {
            eprintln!("Command \"{program}\" failed with error: {error}");
            retval = -1;
            success = false;
            if quit_on == QuitOn::Error || quit_on == QuitOn::Fail {
                std::process::exit(retval)
            }
        }
    }
    (success, retval)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum QuitOn {
    Fail,
    Error,
}
