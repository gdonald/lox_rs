use std::error::Error;
use lox_rs::ast::expr::{BinaryExpr, Expr, LiteralExpr};
use lox_rs::ast::parser::{ParseError, Parser};
use lox_rs::ast::token::{Token, TokenType};

#[test]
fn test_parse_error_display() {
    let error = ParseError;
    assert_eq!(error.to_string(), "ParseError");
}

#[test]
fn test_parse_error_display_using_format() {
    let error = ParseError;
    let formatted = format!("{}", error);
    assert_eq!(formatted, "ParseError");
}

#[test]
fn test_parse_error_debug_format() {
    let error = ParseError;
    let debug_formatted = format!("{:?}", error);
    assert_eq!(debug_formatted, "ParseError");
}

#[test]
fn test_parse_error_implements_error() {
    let error = ParseError;
    let _ = &error as &dyn Error;
}

#[test]
fn test_parse_error_source_is_none() {
    let error = ParseError;
    assert!(error.source().is_none());
}

#[test]
fn test_parse_error_debug() {
    let error = ParseError;
    assert_eq!(format!("{:?}", error), "ParseError");
}

#[test]
fn test_parser_initialization() {
    let tokens = vec![
        Token::new(TokenType::Number, "1".to_string(), Some(LiteralExpr::Num(2f64)), 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Number, "2".to_string(), Some(LiteralExpr::Num(3f64)), 1),
    ];
    let parser = Parser::new(tokens.clone());

    assert_eq!(parser.tokens, tokens);
    assert_eq!(parser.current, 0);
    assert_eq!(parser.error.get(), false);
}

#[test]
fn test_parser_error_handling() {
    let tokens = vec![
        Token::new(TokenType::Number, "1".to_string(), Some(LiteralExpr::Num(2f64)), 1),
        Token::new(TokenType::Eof, "".to_string(), None,1),
    ];
    let parser = Parser::new(tokens);
    let token = Token::new(TokenType::Eof, "\0".to_string(), Some(LiteralExpr::Str( "\0".to_string())), 1 );

    assert_eq!(parser.error.get(), false);
    parser.error(&token, &"Test error".to_string());
    assert_eq!(parser.error.get(), true);
    parser.error.set(false);
    assert_eq!(parser.error.get(), false);
}

#[test]
fn test_parser_advance() {
    let tokens = vec![
        Token::new(TokenType::Number, "1".to_string(), Some(LiteralExpr::Num(2f64)), 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Number, "2".to_string(), Some(LiteralExpr::Num(3f64)), 1),
    ];
    let mut parser = Parser::new(tokens);

    parser.current += 1;
    assert_eq!(parser.current, 1);
    parser.current += 1;
    assert_eq!(parser.current, 2);
}

#[test]
fn test_parse_success() {
    let tokens = vec![
        Token::new(TokenType::Number, "1".to_string(), Some(LiteralExpr::Num(2f64)), 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Number, "2".to_string(), Some(LiteralExpr::Num(3f64)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_some());

    if let Some(expr) = result {
        if let Expr::Binary(binary_expr) = expr {
            assert_eq!(binary_expr.operator.token_type, TokenType::Plus);

            if let Expr::Literal(num) = *binary_expr.left.clone() {
                if let LiteralExpr::Num(num) = *num {
                    assert_eq!(num, 2f64);
                } else {
                    panic!("Expected LiteralExpr::Num, got {:?}", num);
                }
            } else {
                panic!("Expected Expr::Literal, got {:?}", binary_expr.left);
            }

            if let Expr::Literal(num) = *binary_expr.right.clone() {
                if let LiteralExpr::Num(num) = *num {
                    assert_eq!(num, 3f64);
                } else {
                    panic!("Expected LiteralExpr::Num, got {:?}", num);
                }
            } else {
                panic!("Expected Expr::Literal, got {:?}", binary_expr.right);
            }
        } else {
            panic!("Expected BinaryExpr, got {:?}", expr);
        }
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_parse_error_final_eof_token_missing() {
    let mut parser = Parser::new(vec![]);
    parser.parse();
}

#[test]
fn test_parse_returns_none_on_error() {
    let tokens = vec![
        Token::new(TokenType::Number, "1".to_string(), Some(LiteralExpr::Num(2f64)), 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Number, "2".to_string(), Some(LiteralExpr::Num(3f64)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];
    let mut parser = Parser::new(tokens);
    parser.error.set(true);

    let result = parser.parse();
    assert!(result.is_none(), "Expected parse to return None due to error being set to true.");
}

#[test]
fn test_synchronize_stops_at_semicolon() {
    let tokens = vec![
        Token::new(TokenType::Identifier, "var".to_string(), None, 1),
        Token::new(TokenType::Number, "42".to_string(), None, 1),
        Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
        Token::new(TokenType::Identifier, "x".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);

    // Move current to the first error token (simulated by Identifier)
    parser.advance();
    parser.synchronize();

    assert_eq!(parser.current, 3); // Should stop at the semicolon
}

#[test]
fn test_synchronize_stops_at_class_keyword() {
    let tokens = vec![
        Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
        Token::new(TokenType::Number, "42".to_string(), None, 1),
        Token::new(TokenType::Class, "class".to_string(), None, 1),
        Token::new(TokenType::Identifier, "x".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);

    // Move current to the first error token (simulated by Identifier)
    parser.advance();
    parser.synchronize();

    assert_eq!(parser.current, 2); // Should stop at the Class keyword
}

#[test]
fn test_synchronize_reaches_end_without_stopping() {
    let tokens = vec![
        Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
        Token::new(TokenType::Number, "42".to_string(), None, 1),
        Token::new(TokenType::Identifier, "x".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);

    // Move current to the first error token (simulated by Identifier)
    parser.advance();
    parser.synchronize();

    assert_eq!(parser.current, 3); // Should reach the end without finding a semicolon or keyword
}

#[test]
fn test_expression_calls_equality() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Token::new(TokenType::Number, "43".to_string(), Some(LiteralExpr::Num(43.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.expression();

    assert_eq!(
        result,
        Expr::Binary(Box::new(BinaryExpr {
            left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(42.0)))),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(43.0)))),
        }))
    );
}

#[test]
fn test_equality_single_equality() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Token::new(TokenType::Number, "43".to_string(), Some(LiteralExpr::Num(43.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Expr::Binary(Box::new(BinaryExpr {
            left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(42.0)))),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(43.0)))),
        }))
    );
}

#[test]
fn test_equality_multiple_comparisons() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Token::new(TokenType::Number, "43".to_string(), Some(LiteralExpr::Num(43.0)), 1),
        Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
        Token::new(TokenType::Number, "44".to_string(), Some(LiteralExpr::Num(44.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Expr::Binary(Box::new(BinaryExpr {
            left: Box::new(Expr::Binary(Box::new(BinaryExpr {
                left: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(42.0)))),
                operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
                right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(43.0)))),
            }))),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            right: Box::new(Expr::Literal(Box::new(LiteralExpr::Num(44.0)))),
        }))
    );
}

#[test]
fn test_equality_no_comparison() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Expr::Literal(Box::new(LiteralExpr::Num(42.0)))
    );
}

#[test]
fn test_is_at_end_true() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);

    // Initially not at the end
    assert_eq!(parser.is_at_end(), false);

    // Move to Eof token
    parser.current += 1;
    assert_eq!(parser.is_at_end(), true);  // Now should be at the end
}

#[test]
fn test_is_at_end_false() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Number, "43".to_string(), Some(LiteralExpr::Num(43.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let parser = Parser::new(tokens);

    // Initially, the parser should not be at the end
    assert_eq!(parser.is_at_end(), false);
}

#[test]
fn test_is_at_end_on_empty_input() {
    let tokens = vec![
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let parser = Parser::new(tokens);

    // Should immediately be at the end since the only token is Eof
    assert_eq!(parser.is_at_end(), true);
}

#[test]
fn test_check_returns_true_for_matching_token() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let parser = Parser::new(tokens);

    // The first token is a Number, so check should return true
    assert!(parser.check(TokenType::Number));
}

#[test]
fn test_check_returns_false_for_non_matching_token() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let parser = Parser::new(tokens);

    // The first token is not a Plus, so check should return false
    assert!(!parser.check(TokenType::Plus));
}

#[test]
fn test_check_returns_false_at_end_of_input() {
    let tokens = vec![
        Token::new(TokenType::Number, "42".to_string(), Some(LiteralExpr::Num(42.0)), 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let mut parser = Parser::new(tokens);

    // Move to the end of the tokens
    parser.current = 1;

    // Since we're at the end, check should return false regardless of the token type
    assert!(!parser.check(TokenType::Number));
}