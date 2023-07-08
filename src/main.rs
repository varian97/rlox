use std::{env, process};

use rlox::interpreter::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        run_file(&args[1]).unwrap_or_else(|error| {
            eprintln!("Error running file {}, error: {}", &args[1], error);
        });
    } else {
        eprintln!("usage: rlox [script]");
        process::exit(64);
    }
}
