extern crate lox_rs;

use std::env;
use std::io::{self};

use lox_rs::{run_file, run_prompt};

mod ast;
mod interpreter;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}
