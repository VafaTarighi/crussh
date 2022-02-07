use std::{env, path::Path};

use home::home_dir;

pub fn change_dir(args: &Vec<String>) -> Result<(), String> {

    match args.len() {
        0 => {
            let home = home_dir().unwrap();
            env::set_current_dir(home).map_err(|e| e.to_string())
        },
        1 => {
            let path = Path::new(args.get(0).unwrap());
            env::set_current_dir(path).map_err(|e| e.to_string())
        },
        _ => {
            Err("crussh: cd: too many arguments".to_string())
        }
    }

}