use lox_rs::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor};
use lox_rs::ast::printer::Printer;
use lox_rs::ast::token::{Token, TokenType};

#[test]
fn test_ast_printer() {
    let expression = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(Expr::Unary(Box::new(UnaryExpr {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(123.0)))),
        }))),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(Box::new(GroupingExpr {
            expr: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(45.67)))),
        }))),
    }));

    let mut printer = Printer;
    let result = printer.print(&expression);

    assert_eq!(result, "(* (- 123) (group 45.67))");
}

#[test]
fn test_visit_literal_str() {
    let expr = LiteralExpr::Str("Hello".to_string());
    let mut printer = Printer;
    let result = printer.visit_literal_expr(&expr);
    assert_eq!(result, "Hello");
}

#[test]
fn test_visit_literal_num() {
    let expr = LiteralExpr::Num(42.0);
    let mut printer = Printer;
    let result = printer.visit_literal_expr(&expr);
    assert_eq!(result, "42");
}

#[test]
fn test_visit_literal_bool_true() {
    let expr = LiteralExpr::Bool(true);
    let mut printer = Printer;
    let result = printer.visit_literal_expr(&expr);
    assert_eq!(result, "true");
}

#[test]
fn test_visit_literal_bool_false() {
    let expr = LiteralExpr::Bool(false);
    let mut printer = Printer;
    let result = printer.visit_literal_expr(&expr);
    assert_eq!(result, "false");
}

#[test]
fn test_visit_literal_nil() {
    let expr = LiteralExpr::Nil;
    let mut printer = Printer;
    let result = printer.visit_literal_expr(&expr);
    assert_eq!(result, "nil");
}
