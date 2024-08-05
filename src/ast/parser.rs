use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
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

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
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

    fn error(&self, token: &Token, message: &str) -> String {
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
                LiteralExpr::Str(s) => {
                    s.parse::<f64>()
                        .map(|num| Expr::Literal(Box::new(LiteralExpr::Num(num))))
                        .map_err(|_| "Failed to parse string to f64".to_string())
                }
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

        self.primary().unwrap()
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
