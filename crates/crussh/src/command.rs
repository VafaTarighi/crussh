mod filename;
mod args;
mod pipe;

use filename::FileName;
use args::Args;
use nix::errno::Errno;
use nix::libc::{c_int, waitpid};
use nix::sys::wait::{WaitPidFlag, wait};
use nix::unistd::{ForkResult, Pid, execvp, fork};
use pipe::Pipe;

use crate::utils;


#[derive(Debug, PartialEq)]
pub(crate) struct Command {
    filename: FileName,
    args: Args,
    pipe: Option<Box<Pipe>>,
    background: bool
}

impl Command {
    
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, filename) = FileName::new(s)?;

        let (s, _) =  utils::extract_whitespace(s);

        let (s, mut args) = Args::new(s)?;

        let (s, _) = utils::extract_whitespace(s);


        let (s, pipe) = if !s.is_empty() {
            let (s, pipe) = Pipe::new(s)?;
            (s, Some(Box::new(pipe)))
        } else {
            (s, None)
        };

        let mut background = false;
        if let Some(arg) = args.peek_last() {
            if arg.eq("&") {
                background = true;
                if let None = args.red_out {
                    args.red_out = Some(FileName::new(format!("{}-log", filename.as_str()).as_str()).unwrap().1);
                }
                args.pop_last();
            }
        }

        Ok((s, Self { filename, args, pipe, background}))
    }

    pub(crate) fn exec(&self, fd_read: i32) -> Result<(), Errno> {
        let filename = self.filename.as_cstring();
        let args = self.args.as_cstring_vec(filename.clone());

        let has_pipe = self.pipe != None;
        let mut fd = [0, 1];
        if has_pipe {
            Pipe::create(&mut fd);
        }

        let mut cpid: Pid = Pid::this();
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                Pipe::pipe_child(&self.pipe, fd_read, fd)?;
                Args::redirect(&self.args)?;
                execvp(filename.as_c_str(), &args)?;
            },
            Ok(ForkResult::Parent { child}) => {

                Pipe::pipe_parent(&self.pipe, fd)?;
                cpid = child;
            },
            Err(errno) => {
                return Err(errno)
            }
        }

        if self.background && cpid != Pid::this() {
            let mut status: c_int = 0;
            let mut wpf = WaitPidFlag::empty();
            wpf.set(WaitPidFlag::WNOHANG, true);
            unsafe {waitpid(cpid.as_raw(), &mut status, wpf.bits());}
            dbg!("waited without hang!");
        }

        wait()?;
        Ok(())
    }

    pub(crate) fn get_filename(&self) -> &str {
        self.filename.as_str()
    }

    pub(crate) fn get_args(&self) -> &Vec<String> {
        self.args.as_vec()
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::command::filename::FileName;


//     #[test]
//     fn parse_simple_command() {
//         assert_eq!(
//             Command::new("ls"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec![],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_args() {
//         assert_eq!(
//             Command::new("ls -la"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-la".to_string()],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_multiple_args() {
//         assert_eq!(
//             Command::new("ls -l -a"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-l".to_string(), "-a".to_string()],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_dir_in() {
//         assert_eq!(
//             Command::new("wc < hello.txt"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("wc".to_string()),
//                     args: Args {
//                         arg_vec: vec![],
//                         red_in: Some(FileName("hello.txt".to_string())),
//                         red_out: None
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_dir_out() {
//         assert_eq!(
//             Command::new("ls >hello.txt"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec![],
//                         red_in: None,
//                         red_out: Some(FileName("hello.txt".to_string()))
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_pipe() {
//         assert_eq!(
//             Command::new("ls | grep \"hello world.c\""),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec![],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: Some(Box::new(Pipe(Command {
//                         filename: FileName("grep".to_string()),
//                         args: Args {
//                             arg_vec: vec!["hello world.c".to_string()],
//                             red_in: None,
//                             red_out: None
//                         },
//                         pipe: None,
//                         background: false
//                     }))),
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_dirs() {
//         assert_eq!(
//             Command::new("wc < hello.c > \"good bye\""),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("wc".to_string()),
//                     args: Args {
//                         arg_vec: vec![],
//                         red_in: Some(FileName("hello.c".to_string())),
//                         red_out: Some(FileName("good bye".to_string()))
//                     },
//                     pipe: None,
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_dirs_pipe() {
//         assert_eq!(
//             Command::new("wc < hello.c -a | grep -b > fdaf32r_cdg.txt -s"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("wc".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-a".to_string()],
//                         red_in: Some(FileName("hello.c".to_string())),
//                         red_out: None
//                     },
//                     pipe: Some(Box::new(Pipe(Command {
//                         filename: FileName("grep".to_string()),
//                         args: Args {
//                             arg_vec: vec!["-b".to_string(), "-s".to_string()],
//                             red_in: None,
//                             red_out: Some(FileName("fdaf32r_cdg.txt".to_string()))
//                         },
//                         pipe: None,
//                         background: false
//                     }))),
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_with_multiple_pipes() {
//         assert_eq!(
//             Command::new("ls -la | grep foo | wc"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-la".to_string()],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: Some(Box::new(Pipe(Command {
//                         filename: FileName("grep".to_string()),
//                         args: Args {
//                             arg_vec: vec!["foo".to_string()],
//                             red_in: None,
//                             red_out: None
//                         },
//                         pipe: Some(Box::new(Pipe(Command {
//                             filename: FileName("wc".to_string()),
//                             args: Args {
//                                 arg_vec: vec![],
//                                 red_in: None,
//                                 red_out: None
//                             },
//                             pipe: None,
//                             background: false
//                         }))),
//                         background: false
//                     }))),
//                     background: false
//                 }
//             ))
//         )
//     }


//     #[test]
//     fn parse_command_complex() {
//         assert_eq!(
//             Command::new("ls -la | grep foo | wc > hello.rs"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-la".to_string()],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: Some(Box::new(Pipe(Command {
//                         filename: FileName("grep".to_string()),
//                         args: Args {
//                             arg_vec: vec!["foo".to_string()],
//                             red_in: None,
//                             red_out: None
//                         },
//                         pipe: Some(Box::new(Pipe(Command {
//                             filename: FileName("wc".to_string()),
//                             args: Args {
//                                 arg_vec: vec![],
//                                 red_in: None,
//                                 red_out: Some(FileName("hello.rs".to_string()))
//                             },
//                             pipe: None,
//                             background: false
//                         }))),
//                         background: false
//                     }))),
//                     background: false
//                 }
//             ))
//         )
//     }

//     #[test]
//     fn parse_command_complex_background() {
//         assert_eq!(
//             Command::new("ls -la | grep foo | wc > hello.rs &"),
//             Ok((
//                 "",
//                 Command {
//                     filename: FileName("ls".to_string()),
//                     args: Args {
//                         arg_vec: vec!["-la".to_string()],
//                         red_in: None,
//                         red_out: None
//                     },
//                     pipe: Some(Box::new(Pipe(Command {
//                         filename: FileName("grep".to_string()),
//                         args: Args {
//                             arg_vec: vec!["foo".to_string()],
//                             red_in: None,
//                             red_out: None
//                         },
//                         pipe: Some(Box::new(Pipe(Command {
//                             filename: FileName("wc".to_string()),
//                             args: Args {
//                                 arg_vec: vec![],
//                                 red_in: None,
//                                 red_out: Some(FileName("hello.rs".to_string()))
//                             },
//                             pipe: None,
//                             background: true
//                         }))),
//                         background: false
//                     }))),
//                     background: false
//                 }
//             ))
//         )
//     }
// }