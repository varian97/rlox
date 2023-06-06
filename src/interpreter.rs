use std::error::Error;
use std::{fs, process};
use std::{io, io::Write};

pub struct Interpreter {
    pub had_error: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { had_error: false }
    }

    pub fn run_prompt(self: &mut Self) {
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
            self.run(&buffer);

            self.had_error = false;
            buffer.clear();
        }
        process::exit(0);
    }

    pub fn run_file(self: &mut Self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        self.run(&contents);

        if self.had_error {
            process::exit(65);
        }

        Ok(())
    }

    pub fn run(self: &mut Self, source: &str) {
        println!("The source code: {source}");
    }

    pub fn error(self: &mut Self, line_number: i32, message: String) {
        self._report(line_number, String::from(""), message)
    }

    fn _report(self: &mut Self, line_number: i32, code_that_error: String, message: String) {
        eprintln!("[Line {line_number}] Error {code_that_error}: {message}");
        self.had_error = true;
    }
}
