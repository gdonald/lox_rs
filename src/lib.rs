use crate::ast::parser::Parser;
use crate::ast::scanner::{ScanError, Scanner};
use crate::interpreter::Interpreter;
use std::io::Write;
use std::{fs, io};

pub mod ast;
pub mod interpreter;

pub fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(contents);
    Ok(())
}

pub fn run_prompt<R: io::BufRead, W: io::Write>(mut input: R, mut output: W) -> io::Result<()> {
    loop {
        write!(output, "lox> ")?;
        output.flush()?;
        let mut line = String::new();
        input.read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        run(line);
    }
    Ok(())
}

pub fn run(source: String) {
    if source.is_empty() {
        panic!("Source is empty");
    }

    let mut scanner = Scanner::new(source, ScanError::new());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);

    let expression = match parser.parse() {
        Some(expr) => expr,
        None => {
            return;
        }
    };

    let mut interpreter = Interpreter;
    let obj = interpreter.interpret(&expression);
    println!("{}", obj.to_string());
}
