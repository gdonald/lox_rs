use lox_rs::ast::expr::{Expr, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr};

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

