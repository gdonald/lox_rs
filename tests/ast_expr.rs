use lox_rs::ast::expr::{Expr, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr};
use lox_rs::ast::expr::Visitor;
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

impl Visitor<String> for MockVisitor {
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

#[test]
#[should_panic(expected = "Unhandled expression type")]
fn test_trigger_panic() {
    let expr = Expr::Unhandled;
    let mut visitor = MockVisitor;
    expr.accept(&mut visitor);
}

#[test]
fn test_create_binary_expr() {
    let left_expr = Expr::Literal(Box::new(LiteralExpr::Num(3.0)));
    let right_expr = Expr::Literal(Box::new(LiteralExpr::Num(4.0)));
    let operator = Token {
        token_type: TokenType::Plus,
        lexeme: "+".to_string(),
        literal: None,
        line: 1,
    };

    let binary_expr = BinaryExpr::new(left_expr.clone(), operator.clone(), right_expr.clone());

    if let Expr::Literal(lit) = *binary_expr.left {
        if let LiteralExpr::Num(value) = *lit {
            assert_eq!(value, 3.0);
        } else {
            panic!("Expected LiteralExpr::Num on the left side");
        }
    } else {
        panic!("Expected Expr::Literal on the left side");
    }

    if let Expr::Literal(lit) = *binary_expr.right {
        if let LiteralExpr::Num(value) = *lit {
            assert_eq!(value, 4.0);
        } else {
            panic!("Expected LiteralExpr::Num on the right side");
        }
    } else {
        panic!("Expected Expr::Literal on the right side");
    }

    assert_eq!(binary_expr.operator.token_type, TokenType::Plus);
    assert_eq!(binary_expr.operator.lexeme, "+");
}

#[test]
fn test_create_grouping_expr_with_literal() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Num(42.0)));
    let grouping_expr = GroupingExpr::new(literal_expr.clone());

    if let Expr::Literal(lit) = *grouping_expr.expr {
        if let LiteralExpr::Num(value) = *lit {
            assert_eq!(value, 42.0);
        } else {
            panic!("Expected LiteralExpr::Num inside GroupingExpr");
        }
    } else {
        panic!("Expected Expr::Literal inside GroupingExpr");
    }
}

#[test]
fn test_create_grouping_expr_with_unary() {
    let unary_expr = Expr::Unary(Box::new(UnaryExpr {
        operator: Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(10.0)))),
    }));
    let grouping_expr = GroupingExpr::new(unary_expr.clone());

    if let Expr::Unary(unary) = *grouping_expr.expr {
        if let Expr::Literal(lit) = *unary.right {
            if let LiteralExpr::Num(value) = *lit {
                assert_eq!(value, 10.0);
            } else {
                panic!("Expected LiteralExpr::Num inside UnaryExpr within GroupingExpr");
            }
        } else {
            panic!("Expected Expr::Literal inside UnaryExpr within GroupingExpr");
        }
    } else {
        panic!("Expected Expr::Unary inside GroupingExpr");
    }
}

#[test]
fn test_create_grouping_expr_with_binary() {
    let binary_expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(1.0)))),
        operator: Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(2.0)))),
    }));
    let grouping_expr = GroupingExpr::new(binary_expr.clone());

    if let Expr::Binary(binary) = *grouping_expr.expr {
        if let Expr::Literal(left) = *binary.left {
            if let LiteralExpr::Num(left_value) = *left {
                assert_eq!(left_value, 1.0);
            } else {
                panic!("Expected LiteralExpr::Num on the left side of BinaryExpr within GroupingExpr");
            }
        } else {
            panic!("Expected Expr::Literal on the left side of BinaryExpr within GroupingExpr");
        }

        if let Expr::Literal(right) = *binary.right {
            if let LiteralExpr::Num(right_value) = *right {
                assert_eq!(right_value, 2.0);
            } else {
                panic!("Expected LiteralExpr::Num on the right side of BinaryExpr within GroupingExpr");
            }
        } else {
            panic!("Expected Expr::Literal on the right side of BinaryExpr within GroupingExpr");
        }

        assert_eq!(binary.operator.token_type, TokenType::Plus);
        assert_eq!(binary.operator.lexeme, "+");
    } else {
        panic!("Expected Expr::Binary inside GroupingExpr");
    }
}

#[test]
fn test_literal_expr_new_from_f64() {
    let literal_expr = LiteralExpr::new(42.0f64);
    if let LiteralExpr::Num(value) = literal_expr {
        assert_eq!(value, 42.0);
    } else {
        panic!("Expected LiteralExpr::Num");
    }
}

#[test]
fn test_literal_expr_new_from_string() {
    let literal_expr = LiteralExpr::new("Hello".to_string());
    if let LiteralExpr::Str(value) = literal_expr {
        assert_eq!(value, "Hello");
    } else {
        panic!("Expected LiteralExpr::Str");
    }
}

#[test]
fn test_literal_expr_new_from_bool() {
    let literal_expr = LiteralExpr::new(true);
    if let LiteralExpr::Bool(value) = literal_expr {
        assert!(value);
    } else {
        panic!("Expected LiteralExpr::Bool");
    }
}

#[test]
fn test_extract_num_success() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Num(42.0)));
    let value = LiteralExpr::extract_num(&expr);
    assert_eq!(value, 42.0);
}

#[test]
#[should_panic(expected = "Failed to extract number from expression")]
fn test_extract_num_panic() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Str("Not a number".to_string())));
    LiteralExpr::extract_num(&expr);
}

#[test]
fn test_create_unary_expr_with_minus_operator() {
    let operator = Token {
        token_type: TokenType::Minus,
        lexeme: "-".to_string(),
        literal: None,
        line: 1,
    };
    let right_expr = Expr::Literal(Box::new(LiteralExpr::Num(42.0)));
    let unary_expr = UnaryExpr::new(operator.clone(), right_expr.clone());

    assert_eq!(unary_expr.operator.token_type, TokenType::Minus);
    assert_eq!(unary_expr.operator.lexeme, "-");
    
    if let Expr::Literal(lit) = *unary_expr.right {
        if let LiteralExpr::Num(value) = *lit {
            assert_eq!(value, 42.0);
        } else {
            panic!("Expected LiteralExpr::Num inside UnaryExpr");
        }
    } else {
        panic!("Expected Expr::Literal inside UnaryExpr");
    }
}

#[test]
fn test_create_unary_expr_with_not_operator() {
    let operator = Token {
        token_type: TokenType::Bang,
        lexeme: "!".to_string(),
        literal: None,
        line: 1,
    };
    let right_expr = Expr::Literal(Box::new(LiteralExpr::Bool(true)));
    let unary_expr = UnaryExpr::new(operator.clone(), right_expr.clone());
    
    assert_eq!(unary_expr.operator.token_type, TokenType::Bang);
    assert_eq!(unary_expr.operator.lexeme, "!");
    
    if let Expr::Literal(lit) = *unary_expr.right {
        if let LiteralExpr::Bool(value) = *lit {
            assert!(value);
        } else {
            panic!("Expected LiteralExpr::Bool inside UnaryExpr");
        }
    } else {
        panic!("Expected Expr::Literal inside UnaryExpr");
    }
}

#[test]
fn test_create_unary_expr_with_string_literal() {
    let operator = Token {
        token_type: TokenType::Minus, // Assume a hypothetical use of '-' with strings
        lexeme: "-".to_string(),
        literal: None,
        line: 1,
    };
    let right_expr = Expr::Literal(Box::new(LiteralExpr::Str("Hello".to_string())));
    let unary_expr = UnaryExpr::new(operator.clone(), right_expr.clone());

    assert_eq!(unary_expr.operator.token_type, TokenType::Minus);
    assert_eq!(unary_expr.operator.lexeme, "-");

    if let Expr::Literal(lit) = *unary_expr.right {
        if let LiteralExpr::Str(value) = *lit {
            assert_eq!(value, "Hello");
        } else {
            panic!("Expected LiteralExpr::Str inside UnaryExpr");
        }
    } else {
        panic!("Expected Expr::Literal inside UnaryExpr");
    }
}
