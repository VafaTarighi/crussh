use nix::{errno::Errno, libc::{STDIN_FILENO, STDOUT_FILENO}, unistd::{close, dup2}};

use crate::{command::Command, utils};

#[derive(Debug, PartialEq)]
pub(crate) struct Pipe(Command);

impl Pipe {
    
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("|", s)?;
        let (s, _) = utils::extract_whitespace(s);
        let (s, command) = Command::new(s)?;

        Ok((s, Self(command)))
    }

    pub(crate) fn do_pipe(&self, fd_read: i32) -> Result<(), Errno> {
        self.0.exec(fd_read)
    }

    pub(crate) fn create(fd: &mut [i32; 2]) {
        let (fd0, fd1) = nix::unistd::pipe().unwrap();
        fd[0] = fd0;
        fd[1] = fd1;
    }

    pub(crate) fn pipe_child(pipe: &Option<Box<Pipe>>,  fd_read: i32, pipe_fd: [i32; 2]) -> Result<(), Errno> {
        dup2(fd_read, STDIN_FILENO)?;
        if *pipe == None {
            return Ok(())
        }

        dup2(pipe_fd[1], STDOUT_FILENO)?;
        close(pipe_fd[0])?;
        close(pipe_fd[1])?;

        Ok(())
    }

    pub(crate) fn pipe_parent(pipe: &Option<Box<Pipe>>, fd: [i32; 2]) -> Result<(), Errno> {
        if let Some(pipe) = pipe {
            close(fd[1])?;
            pipe.do_pipe(fd[0])?;
        }

        Ok(())
    }
}