use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::token::{Token, TokenType};
use std::cell::Cell;

use super::stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt};
#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError")
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub error: Cell<bool>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            error: Cell::new(false),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();

            statements.push(stmt);
        }

        statements
    }

    pub fn declaration(&mut self) -> Stmt {
        if self.match_tokens(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    pub fn var_declaration(&mut self) -> Stmt {
        let result = self.consume(TokenType::Identifier, "Expect variable name.");
        let name = result.unwrap().clone();

        let initializer = if self.match_tokens(&[TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };

        let _ = self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        Stmt::Var(VarStmt {
            name: name,
            initializer: initializer.unwrap(),
        })
    }

    pub fn statement(&mut self) -> Stmt {
        if self.match_tokens(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    pub fn print_statement(&mut self) -> Stmt {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");

        Stmt::Print(PrintStmt {
            expression: expression,
        })
    }

    pub fn expression_statement(&mut self) -> Stmt {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");

        Stmt::Expression(ExpressionStmt {
            expression: expression,
        })
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    pub fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();

            expr = Expr::Binary(Box::new(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        expr
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn error(&self, token: &Token, message: &str) -> String {
        self.error.set(true);
        format!("Error at {}: {}", token.lexeme, message)
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    pub fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::Bool(false))));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::Bool(true))));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::Nil)));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let literal = self.previous().literal.clone().unwrap();
            return match literal {
                LiteralExpr::Str(s) => Ok(Expr::Literal(Box::new(LiteralExpr::Str(s)))),
                LiteralExpr::Num(n) => Ok(Expr::Literal(Box::new(LiteralExpr::Num(n)))),
                _ => Err("Unexpected literal type".to_string()),
            };
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(GroupingExpr {
                expr: Box::new(expr),
            })));
        }

        Err("Expect expression.".to_string())
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Box::new(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        if let Ok(expr) = self.primary() {
            return expr;
        } else {
            panic!("Unary error");
        }
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        expr
    }

    pub fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        expr
    }

    pub fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }));
        }

        expr
    }
}
