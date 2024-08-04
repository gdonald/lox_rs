use std::{fs, io};
use std::io::Write;
use crate::ast::scanner::{Scanner, ScannerError};

pub mod ast;
pub mod printer;

pub fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(contents);
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("lox> ");
        stdout.flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;

        if line.trim().is_empty() {
            break;
        }

        run(line);
    }

    Ok(())
}

fn run(source: String) {
    let error = ScannerError::new();
    let mut scanner = Scanner::new(source, error);
    let tokens = scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("{:?}", token);
    }

    if scanner.error.had_error() {
        println!("There were errors during scanning.");
    }
}