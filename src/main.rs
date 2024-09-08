extern crate lox_rs;

use std::io::{self};
use std::{env, process};

use lox_rs::run_main;

mod ast;
mod interpreter;

fn main() -> io::Result<()> {
    run_main(
        env::args().collect(),
        io::stdin().lock(),
        io::stdout(),
        |code| {
            process::exit(code);
        },
    )
}
