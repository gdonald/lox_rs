use lazy_static::lazy_static;
use std::cell::Cell;
use std::collections::HashMap;

use crate::ast::expr::LiteralExpr;
use crate::ast::token::{Token, TokenType};

pub struct ScanError {
    detected: Cell<bool>,
}

impl ScanError {
    pub fn new() -> Self {
        Self {
            detected: Cell::new(false),
        }
    }

    pub fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.detected.set(true);
    }

    pub fn detected(&self) -> bool {
        self.detected.get()
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
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

pub struct Scanner {
    pub keywords: &'static HashMap<String, TokenType>,
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub error: ScanError,
}

impl Scanner {
    pub fn new(source: String, error: ScanError) -> Self {
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
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

    pub fn is_alpha(&self, c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    pub fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    pub fn number(&mut self) {
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
        self.add_token_with_literal(TokenType::Number, Some(LiteralExpr::Num(value)));
    }

    pub fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    pub fn string(&mut self) {
        if !self.is_at_end() {
            self.advance();
        }

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

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Some(LiteralExpr::Str(value)));
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().next().unwrap()
        }
    }

    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1..].chars().next().unwrap()
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
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

    pub fn advance(&mut self) -> char {
        let current_char = self.source[self.current..].chars().next().unwrap();
        self.current += current_char.len_utf8();

        current_char
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    pub fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LiteralExpr>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}
