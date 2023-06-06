use std::{env, process};

use rlox::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rlox_interpreter = Interpreter::new();

    if args.len() == 1 {
        rlox_interpreter.run_prompt();
    } else if args.len() == 2 {
        rlox_interpreter.run_file(&args[1]).unwrap_or_else(|error| {
            eprintln!("Error running file {}, error: {}", &args[1], error);
        });
    } else {
        eprintln!("usage: rlox [script]");
        process::exit(64);
    }
}
