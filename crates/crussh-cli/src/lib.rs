mod cd;
mod exit;

use std::{env, io::Write};

use colored::{ColoredString, Colorize};
use crussh::Parse;
use rustyline::{Editor, error::ReadlineError};

pub fn exit() {
    
}

pub fn check_built_ins(parse: &Parse, rl: &mut Editor<()>) -> Result<(), String> {
    match parse.get_filename() {
        "cd" => {
            cd::change_dir(parse.get_args())
        },
        "exit" => {
            rl.save_history(&history_path()).unwrap();
            exit::terminate(parse.get_args())
        },
        _ => Err("not a built-in".to_string())
    }
}

pub fn prompt(rl: &mut Editor<()>) -> Result<String, ReadlineError> {
    std::io::stdout().flush().unwrap();
    let p_str = format!("\n[{}]\n{}({}){}", get_working_dir(), get_username(), get_hostname(), "-> ".yellow());
    rl.readline(&p_str)
}

pub fn load_history(rl: &mut Editor<()>) -> Result<(), ReadlineError> {
    rl.load_history(&history_path())
}
pub fn save_history(rl: &mut Editor<()>) -> Result<(), ReadlineError> {
    rl.save_history(&history_path())
}

fn get_working_dir() -> ColoredString {
    let curr_dir = env::current_dir().unwrap();
    let mut curr_dir = curr_dir.into_os_string().into_string().unwrap();
    if curr_dir.starts_with(&home()) {
        // str::replacen(&self, pat, to, count)
        curr_dir = curr_dir.replacen(&home(), "~",1).as_str().to_string();
    }
    curr_dir.green()
}

fn get_hostname() -> String {
    whoami::hostname()
}

fn get_username() -> ColoredString {
    whoami::username().bold().blue()
}

fn home() -> String {
    home::home_dir().unwrap().to_str().unwrap().to_string()
}

fn history_path() -> String {
    format!("{}/{}", &home(),".crussh_history")
}