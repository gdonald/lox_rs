use std::{
    fs::File,
    io::{self, Write},
};

use lox_rs::{run, run_file, run_prompt, run_source};
use std::io::Cursor;
use tempfile::tempdir;

#[test]
fn test_run_file_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.lox");

    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "1 + 2").expect("Failed to write to file");

    file.flush().expect("Failed to flush the file");

    let result = run_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
}

#[test]
fn test_run_file_nonexistent() {
    let result = run_file("nonexistent_file.lox");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
}

#[test]
#[should_panic(expected = "Source is empty")]
fn test_run_file_empty() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("empty.lox");

    File::create(&file_path).unwrap();

    let result = run_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
}

#[test]
fn test_run_prompt_single_line() {
    let input = b"1 + 2;\n";
    let mut output = Vec::new();

    let result = run_prompt(Cursor::new(input), &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("lox> "));
}

#[test]
fn test_run_prompt_empty_line() {
    let input = b"\n";
    let mut output = Vec::new();

    let result = run_prompt(Cursor::new(input), &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("lox> "));
}

#[test]
fn test_run_prompt_multiple_lines() {
    let input = b"1 + 2;\n2 + 3;\n\n";
    let mut output = Vec::new();

    let result = run_prompt(Cursor::new(input), &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(output_str.matches("lox> ").count(), 3);
}

#[test]
fn test_run_prompt_triggers_break_on_empty_input() {
    let input = b"1 + 2;\n\n";
    let mut output = Vec::new();

    let result = run_prompt(Cursor::new(input), &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(output_str.matches("lox> ").count(), 2);
}

#[test]
#[should_panic(expected = "Unary error")]
fn test_run_source_invalid_script() {
    let source = "invalid syntax".to_string();
    run_source(source);
}

#[test]
#[should_panic(expected = "Unary error")]
fn test_run_source_with_script() {
    let source = "print 42;".to_string();
    run_source(source);
}

#[test]
fn test_run_source_with_simple_expr() {
    let source = "1 + 2".to_string();
    run_source(source);
}

#[test]
fn test_no_args_runs_prompt() {
    let args = vec!["lox".to_string()];
    let input = Cursor::new("");
    let mut output = Vec::new();

    let result = run(args, input, &mut output);

    assert!(result.is_ok());
    println!("{}", String::from_utf8(output.clone()).unwrap());
    assert_eq!(output.clone(), "lox> ".as_bytes());
}
