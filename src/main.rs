extern crate lox_rs;

use std::io::{self};
use std::{env, process};

use lox_rs::run;

mod ast;
mod interpreter;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let exit_code = run(args, io::stdin().lock(), io::stdout(), |code| {
        process::exit(code);
    });
    exit_code
}
