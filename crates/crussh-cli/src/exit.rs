use std::process::exit;

pub fn terminate(args: &Vec<String>) -> Result<(), String> {
    match args.len() {
        0 => {
            exit(0);
        },
        1 => {
            let code = args.get(0).unwrap();
            let code: i32 = code.parse().map_err(|_| format!("crussh: exit: {}: numeric argument required", code))?;
            exit(code)
        },
        _ => {
            Err("crussh: exit: too many arguments".to_string())
        }
    }
}