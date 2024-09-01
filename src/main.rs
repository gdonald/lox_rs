extern crate lox_rs;

use std::env;
use std::io::{self};

use lox_rs::run;

mod ast;
mod interpreter;

fn main() -> io::Result<()> {
    run(env::args().collect(), io::stdin().lock(), io::stdout())
}
