use nix::{errno::Errno, libc::STDIN_FILENO};

use crate::command::Command;

#[derive(Debug, PartialEq)]
pub(crate) enum Cmd {
    Command(Command)
}

impl Cmd {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, command) = Command::new(s)?;
        // dbg!(&command);
        Ok((s, Self::Command(command)))
    }

    pub(crate) fn execute(&self) -> Result<(), Errno> {
        let Self::Command(command) = self;

        command.exec(STDIN_FILENO)
    }

    pub(crate) fn get_filename(&self) -> &str {
        let Self::Command(command) = self;
        command.get_filename()
    }

    pub(crate) fn get_args(&self) -> &Vec<String> {
        let Self::Command(command) = self;
        command.get_args()
    }
}