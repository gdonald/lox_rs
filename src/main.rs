extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<Literal>),
    Unary(Box<UnaryExpr>),
}

#[derive(Debug, Clone)]
struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr)
        }
    }
}

#[derive(Debug, Clone)]
enum Literal {
    Str(String),
    Num(f64),
}

impl Literal {
    fn new(value: impl Into<Literal>) -> Self {
        value.into()
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::Str(value)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Num(value)
    }
}

#[derive(Debug, Clone)]
struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct ScannerError {
    had_error: Cell<bool>,
}

impl ScannerError {
    fn new() -> Self {
        Self {
            had_error: Cell::new(false),
        }
    }

    fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error.set(true);
    }

    fn had_error(&self) -> bool {
        self.had_error.get()
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_string(), TokenType::And);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("false".to_string(), TokenType::False);
        m.insert("for".to_string(), TokenType::For);
        m.insert("fun".to_string(), TokenType::Fun);
        m.insert("if".to_string(), TokenType::If);
        m.insert("nil".to_string(), TokenType::Nil);
        m.insert("or".to_string(), TokenType::Or);
        m.insert("print".to_string(), TokenType::Print);
        m.insert("return".to_string(), TokenType::Return);
        m.insert("super".to_string(), TokenType::Super);
        m.insert("this".to_string(), TokenType::This);
        m.insert("true".to_string(), TokenType::True);
        m.insert("var".to_string(), TokenType::Var);
        m.insert("while".to_string(), TokenType::While);
        m
    };
}

struct Scanner {
    keywords: &'static HashMap<String, TokenType>,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error: ScannerError,
}

impl Scanner {
    fn new(source: String, error: ScannerError) -> Self {
        Self {
            keywords: &KEYWORDS,
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            
            ')' => self.add_token(TokenType::RightParen),

            '{' => self.add_token(TokenType::LeftBrace),

            '}' => self.add_token(TokenType::RightBrace),

            ',' => self.add_token(TokenType::Comma),

            '.' => self.add_token(TokenType::Dot),

            '-' => self.add_token(TokenType::Minus),

            '+' => self.add_token(TokenType::Plus),

            ';' => self.add_token(TokenType::Semicolon),

            '*' => self.add_token(TokenType::Star),

            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }

            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }

            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }

            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }

            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' => {}

            '\r' => {}

            '\t' => {}

            '\n' => self.line += 1,

            '"' => self.string(),

            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error.error(self.line, "Unexpected character.");
                }
            }
        }
    }
    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_alphanumeric()
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text).cloned().unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_with_literal(TokenType::Number, Some(Literal::Num(value)));
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error.error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Some(Literal::Str(value)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().next().unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1..].chars().next().unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current_char = self.source[self.current..].chars().next().unwrap();
        if current_char != expected {
            return false;
        }

        self.current += current_char.len_utf8();
        true
    }

    fn advance(&mut self) -> char {
        let current_char = self.source[self.current..].chars().next().unwrap();
        self.current += current_char.len_utf8();

        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(contents);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
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
