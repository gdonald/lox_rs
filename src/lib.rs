use crate::ast::parser::Parser;
use crate::ast::scanner::{ScanError, Scanner};
use crate::interpreter::Interpreter;
use std::{fs, io};

pub mod ast;
pub mod interpreter;

pub fn run(
    args: Vec<String>,
    stdin: impl io::BufRead,
    mut stdout: impl io::Write,
    exit: impl Fn(i32),
) -> io::Result<()> {
    if args.len() > 2 {
        writeln!(stdout, "Usage: lox [script]")?;
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt(stdin, stdout)?;
    }
    Ok(())
}

pub fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run_source(contents);
    Ok(())
}

pub fn run_prompt<R: io::BufRead, W: io::Write>(mut input: R, mut output: W) -> io::Result<()> {
    loop {
        write!(output, "lox> ")?;
        output.flush()?;
        let mut line = String::new();
        input.read_line(&mut line)?;
        if line.trim().is_empty() {
            break; // tarpaulin: ignore
        }
        run_source(line);
    }
    Ok(())
}

pub fn run_source(source: String) {
    if source.is_empty() {
        panic!("Source is empty");
    }

    let mut scanner = Scanner::new(source, ScanError::new());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut interpreter = Interpreter;
    interpreter.interpret(statements);
}

pub fn run_main(
    args: Vec<String>,
    stdin: impl io::BufRead,
    stdout: impl io::Write,
    exit: impl Fn(i32),
) -> io::Result<()> {
    let exit_code = run(args, stdin, stdout, exit);
    exit_code
}
