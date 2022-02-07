mod cmd;
mod command;
mod utils;

#[derive(Debug)]
pub struct Parse(cmd::Cmd);

impl Parse {
    pub fn execute(&self) -> Result<(), String> {
        self.0.execute().map_err(|e| e.to_string())
    }

    pub fn get_filename(&self) -> &str {
        self.0.get_filename()
    }

    pub fn get_args(&self) -> &Vec<String> {
        self.0.get_args()
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = cmd::Cmd::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        dbg!(s);
        Err("input was not fully consumed by parser".to_string())
    }
}