use crate::ast::parser::Parser;
use crate::ast::scanner::{ScanError, Scanner};
// use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;
use std::io::Write;
use std::{fs, io};

pub mod ast;
pub mod ast_printer;
pub mod interpreter;

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
    interpreter.interpret(&expression);
}
