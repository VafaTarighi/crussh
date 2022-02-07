use std::ffi::CString;

use crate::{command::filename, utils};
use filename::FileName;
use nix::errno::Errno;
use nix::fcntl::{OFlag, open};
use nix::libc::{STDIN_FILENO, STDOUT_FILENO};
use nix::sys::stat::Mode;
use nix::unistd::dup2;

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
    arg_vec: Vec<String>,
    pub(crate) red_in: Option<FileName>,
    pub(crate) red_out: Option<FileName>
}

impl Args {
    
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let mut arg_vec: Vec<String> = Vec::new();
        let mut s = s;
        let mut red_in = None;
        let mut red_out = None;

        loop {
            let (mut new_s, _) =  utils::extract_whitespace(s);

            if new_s.is_empty() {
                return Ok((new_s, Self { arg_vec, red_in, red_out}))
            }

            match &new_s[0..1] {
                "|" => {
                    return Ok((new_s, Self { arg_vec, red_in, red_out}))
                },
                "<" => {
                    let s = utils::tag("<", new_s)?;
                    let (s, _) = utils::extract_whitespace(s);
                    let (s, ri) = FileName::new(s)?;
                    red_in = Some(ri);
                    new_s = s;
                },
                ">" => {
                    let s = utils::tag(">", new_s)?;
                    let (s, _) = utils::extract_whitespace(s);
                    let (s, ro) = FileName::new(s)?;
                    red_out = Some(ro);
                    new_s = s;
                },
                _ => {
                    let (s, arg_str) = utils::extract_shell_ident(new_s)?;
                    arg_vec.push(arg_str.to_string());
                    new_s = s;
                }
            }
            s = new_s;
        }

    }

    pub(crate) fn peek_last(&self) -> Option<&String> {
        self.arg_vec.last()
    }

    pub(crate) fn pop_last(&mut self) {
        self.arg_vec.pop();
    }

    pub(crate) fn as_cstring_vec(&self, filename: CString) -> Vec<CString> {
        let mut args_cs_vec: Vec<CString> = self.arg_vec.iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
        args_cs_vec.insert(0, filename);

        return args_cs_vec
    }

    pub(crate) fn as_vec(&self) -> &Vec<String> {
        &self.arg_vec
    }

    pub(crate) fn redirect(&self) -> Result<(), Errno> {
        if let Some(out_fn) = &self.red_out {
            let fd = open(out_fn.as_str(), OFlag::O_WRONLY | OFlag::O_CREAT, Mode::S_IRUSR | Mode::S_IWUSR)?;

            dup2(fd, STDOUT_FILENO)?;
        }

        if let Some(in_fn) = &self.red_in {
            let fd = open(in_fn.as_str(), OFlag::O_RDONLY, Mode::S_IRUSR)?;

            dup2(fd, STDIN_FILENO)?;
        }

        Ok(())
    }
}