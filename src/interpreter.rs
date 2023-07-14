use std::error::Error;
use std::{fs, process};
use std::{io, io::Write};

use crate::error::RloxErrorDetail;
use crate::scanner::Scanner;

pub fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(error) => {
                panic!("Error reading input: {error}");
            }
        }

        let trimmed_line = buffer.trim();
        if trimmed_line == "exit" {
            break;
        }
        match run(&buffer) {
            Ok(_) => {}
            Err(err) => {
                err.report();
            }
        };
        buffer.clear();
    }
    process::exit(0);
}

pub fn run_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    match run(&contents) {
        Ok(_) => Ok(()),
        Err(err) => {
            err.report();
            process::exit(65);
        }
    }
}

pub fn run(source: &str) -> Result<(), RloxErrorDetail> {
    let mut scanner = Scanner::new(source.to_string());
    let res = scanner.scan_tokens()?;

    for token in res {
        println!("{}", token.to_string());
    }

    Ok(())
}
