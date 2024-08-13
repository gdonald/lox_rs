use crate::ast::parser::Parser;
use crate::ast::scanner::{ScanError, Scanner};
use crate::ast_printer::AstPrinter;
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
    let error = ScanError::new();
    let mut scanner = Scanner::new(source, error);
    let tokens = scanner.scan_tokens();

    // // For now, just print the tokens.
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    //
    // if scanner.error.detected() {
    //     println!("There were errors during scanning.");
    // }

    let mut parser = Parser::new(tokens);

    let expression = match parser.parse() {
        Some(expr) => expr,
        None => {
            return;
        }
    };

    // let mut printer = AstPrinter;
    // println!("{}", printer.print(&expression));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expression);
}
