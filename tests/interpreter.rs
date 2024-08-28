use lox_rs::{
    ast::{
        expr::{BinaryExpr, Expr, LiteralExpr},
        token::{Token, TokenType},
    },
    interpreter::{Interpreter, RuntimeError},
};

#[test]
fn test_runtime_error_creation() {
    let token = Token::new(TokenType::Identifier, "test".to_string(), None, 1);

    let message = "An error occurred.".to_string();
    let error = RuntimeError::new(token.clone(), message.clone());

    assert_eq!(error.token, token);
    assert_eq!(error.message, message);
}

#[test]
fn test_runtime_error_message_content() {
    let token = Token::new(
        TokenType::Number,
        "42".to_string(),
        Some(LiteralExpr::Num(42.0)),
        2,
    );

    let message = "Invalid number format.".to_string();
    let error = RuntimeError::new(token.clone(), message.clone());

    assert_eq!(error.message, message);
    assert!(error.message.contains("Invalid"));
}

#[test]
fn test_interpreter_literal_number() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Num(42.0)));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);
    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.value.downcast_ref::<f64>().unwrap(), 42.0);
}

#[test]
fn test_interpreter_literal_string() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);
    assert_eq!(result.type_name, std::any::type_name::<String>());
    assert_eq!(result.value.downcast_ref::<String>().unwrap(), "hello");
}

#[test]
fn test_interpreter_literal_bool() {
    let expr = Expr::Literal(Box::new(LiteralExpr::Bool(true)));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);
    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_addition() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Plus, "+".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.value.downcast_ref::<f64>().unwrap(), 3.0);
}
