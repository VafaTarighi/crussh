use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let mut last_command = if crussh_cli::load_history(&mut rl).is_err() {
        println!("No previous history.");
        String::from("")
    } else {
        rl.history().get(0).unwrap().clone()
    };

    
    loop {
        let readline = crussh_cli::prompt(&mut rl);

            
        match readline {
            Ok(line) => {
                if line.trim() == "" {
                    continue;
                }

                let line = if line.eq("!!") {
                    if last_command.is_empty() {
                        continue;
                    }
                    println!("{}", last_command);
                    last_command.clone()
                } else {
                    rl.add_history_entry(line.as_str());
                    last_command = line.clone();
                    line
                };

                if let Err(e) = run(line.as_str(), &mut rl) {
                    println!("{}", e);
                }
            },
            Err(ReadlineError::Interrupted) => {
                continue
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }


    rl.save_history("history.txt").unwrap();
    
}


fn run(input: &str, rl: &mut Editor<()>) -> Result<(), String> {
    let parse = crussh::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let res = crussh_cli::check_built_ins(&parse, rl);
    
    if res == Ok(()) || res != Err("not a built-in".to_string()) {
        return res;
    }

    parse.execute()
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    Ok(())
}