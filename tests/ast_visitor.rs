use lox_rs::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor};
use lox_rs::ast::stmt::{ExpressionStmt, PrintStmt};
use lox_rs::ast::token::{Token, TokenType};

struct MockVisitor {
    pub log: Vec<String>,
}

impl Visitor<String> for MockVisitor {
    fn visit_binary_expr(&mut self, _expr: &BinaryExpr) -> String {
        self.log.push("Visited BinaryExpr".to_string());
        "BinaryExpr".to_string()
    }

    fn visit_grouping_expr(&mut self, _expr: &GroupingExpr) -> String {
        self.log.push("Visited GroupingExpr".to_string());
        "GroupingExpr".to_string()
    }

    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) -> String {
        self.log.push("Visited LiteralExpr".to_string());
        "LiteralExpr".to_string()
    }

    fn visit_unary_expr(&mut self, _expr: &UnaryExpr) -> String {
        self.log.push("Visited UnaryExpr".to_string());
        "UnaryExpr".to_string()
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> String {
        self.log.push("Visited ExpressionStmt".to_string());
        "ExpressionStmt".to_string()
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> String {
        self.log.push("Visited PrintStmt".to_string());
        "PrintStmt".to_string()
    }
}

#[test]
fn test_visit_binary_expr() {
    let expr = BinaryExpr {
        left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(1.0)))),
        operator: Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(2.0)))),
    };
    let mut visitor = MockVisitor { log: Vec::new() };
    let result = visitor.visit_binary_expr(&expr);
    assert_eq!(result, "BinaryExpr");
    assert_eq!(visitor.log, vec!["Visited BinaryExpr"]);
}

#[test]
fn test_visit_grouping_expr() {
    let expr = GroupingExpr {
        expr: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(42.0)))),
    };
    let mut visitor = MockVisitor { log: Vec::new() };
    let result = visitor.visit_grouping_expr(&expr);
    assert_eq!(result, "GroupingExpr");
    assert_eq!(visitor.log, vec!["Visited GroupingExpr"]);
}

#[test]
fn test_visit_literal_expr() {
    let expr = LiteralExpr::Num(42.0);
    let mut visitor = MockVisitor { log: Vec::new() };
    let result = visitor.visit_literal_expr(&expr);
    assert_eq!(result, "LiteralExpr");
    assert_eq!(visitor.log, vec!["Visited LiteralExpr"]);
}

#[test]
fn test_visit_unary_expr() {
    let expr = UnaryExpr {
        operator: Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(42.0)))),
    };
    let mut visitor = MockVisitor { log: Vec::new() };
    let result = visitor.visit_unary_expr(&expr);
    assert_eq!(result, "UnaryExpr");
    assert_eq!(visitor.log, vec!["Visited UnaryExpr"]);
}
