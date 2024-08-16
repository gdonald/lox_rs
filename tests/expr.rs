use lox_rs::ast::expr::{Expr, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr};
use lox_rs::ast::expr_visitor::ExprVisitor;
use lox_rs::ast::token::{Token, TokenType};

#[test]
fn test_create_literal_expr_with_num() {
    let literal = Expr::Literal(Box::new(LiteralExpr::Num(17.0)));
    if let Expr::Literal(lit) = literal {
        if let LiteralExpr::Num(value) = *lit {
            assert_eq!(value, 17.0);
        } else {
            panic!("Expected LiteralExpr::Num");
        }
    } else {
        panic!("Expected Expr::Literal");
    }
}

#[test]
fn test_create_literal_expr_with_str() {
    let literal = Expr::Literal(Box::new(LiteralExpr::Str(String::from("Hello"))));
    if let Expr::Literal(lit) = literal {
        if let LiteralExpr::Str(value) = *lit {
            assert_eq!(value, "Hello");
        } else {
            panic!("Expected LiteralExpr::Str");
        }
    } else {
        panic!("Expected Expr::Literal");
    }
}

#[test]
fn test_create_literal_expr_with_bool() {
    let literal = Expr::Literal(Box::new(LiteralExpr::Bool(true)));
    if let Expr::Literal(lit) = literal {
        if let LiteralExpr::Bool(value) = *lit {
            assert_eq!(value, true);
        } else {
            panic!("Expected LiteralExpr::Bool");
        }
    } else {
        panic!("Expected Expr::Literal");
    }
}


struct MockVisitor;

impl ExprVisitor<String> for MockVisitor {
    fn visit_binary_expr(&mut self, _expr: &BinaryExpr) -> String {
        "Visited BinaryExpr".to_string()
    }

    fn visit_grouping_expr(&mut self, _expr: &GroupingExpr) -> String {
        "Visited GroupingExpr".to_string()
    }

    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) -> String {
        "Visited LiteralExpr".to_string()
    }

    fn visit_unary_expr(&mut self, _expr: &UnaryExpr) -> String {
        "Visited UnaryExpr".to_string()
    }
}

#[test]
fn test_accept_binary_expr() {
    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(1.0)))),
        operator: Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(2.0)))),
    }));
    let mut visitor = MockVisitor;
    let result = expr.accept(&mut visitor);
    assert_eq!(result, "Visited BinaryExpr");
}

#[test]
fn test_accept_grouping_expr() {
    let expr = Expr::Grouping(Box::new(GroupingExpr {
        expr: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(17.0)))),
    }));
    let mut visitor = MockVisitor;
    let result = expr.accept(&mut visitor);
    assert_eq!(result, "Visited GroupingExpr");
}

#[test]
fn test_accept_literal_expr() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Num(17.0)));
    let mut visitor = MockVisitor;
    let result = expr.accept(&mut visitor);
    assert_eq!(result, "Visited LiteralExpr");
}

#[test]
fn test_accept_unary_expr() {
    let expr = Expr::Unary(Box::new(UnaryExpr {
        operator: Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(17.0)))),
    }));
    let mut visitor = MockVisitor;
    let result = expr.accept(&mut visitor);
    assert_eq!(result, "Visited UnaryExpr");
}