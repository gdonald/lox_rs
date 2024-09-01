use lox_rs::{
    ast::{
        expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
        object::Object,
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
fn test_interpreter_binary_addition_numbers() {
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

#[test]
fn test_interpreter_binary_addition_strings() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Str(" world".to_string())));
    let operator = Token::new(TokenType::Plus, "+".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<String>());
    assert_eq!(
        *result.value.downcast_ref::<String>().unwrap(),
        "hello world".to_string()
    );
}

#[test]
#[should_panic(
    expected = "Operands Some(\"hello\"), None must be matching types for the Plus operation"
)]
fn test_interpreter_binary_addition_mismatch() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Plus, "+".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
fn test_interpreter_binary_subtraction_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let operator = Token::new(TokenType::Minus, "-".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.value.downcast_ref::<f64>().unwrap(), 1.0);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the Minus operation")]
fn test_interpreter_binary_subtraction_mismatch() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let operator = Token::new(TokenType::Minus, "-".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
fn test_interpreter_binary_division_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(4.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Slash, "/".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.value.downcast_ref::<f64>().unwrap(), 2.0);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the Slash operation")]
fn test_interpreter_binary_division_mismatch() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let operator = Token::new(TokenType::Slash, "/".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
fn test_interpreter_binary_multiplication_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(3.0)));
    let operator = Token::new(TokenType::Star, "*".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.value.downcast_ref::<f64>().unwrap(), 6.0);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the Star operation")]
fn test_interpreter_binary_multiplication_mismatch() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let operator = Token::new(TokenType::Star, "*".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
#[should_panic(expected = "Unknown binary expression operator LeftParen")]
fn test_interpreter_binary_unknown_token_type() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let operator = Token::new(TokenType::LeftParen, "(".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
fn test_interpreter_binary_equal_equal_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_interpreter_binary_equal_equal_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_bang_equal_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_bang_equal_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_interpreter_binary_greater_than_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(3.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Greater, ">".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_greater_than_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Greater, ">".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the Greater operation")]
fn test_interpreter_binary_greater_than_panic_not_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Greater, ">".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_interpreter_binary_greater_than_equal_true_different() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(3.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_greater_than_equal_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_greater_than_equal_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the GreaterEqual operation")]
fn test_interpreter_binary_greater_than_equal_panic_not_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_interpreter_binary_less_than_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_less_than_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the Less operation")]
fn test_interpreter_binary_less_than_panic_not_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_interpreter_binary_less_than_equal_true_different() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(1.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_less_than_equal_true() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), true);
}

#[test]
fn test_interpreter_binary_less_than_equal_false() {
    let left = Expr::Literal(Box::new(LiteralExpr::Num(3.0)));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Operands must be numbers for the LessEqual operation")]
fn test_interpreter_binary_less_than_equal_panic_not_numbers() {
    let left = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let right = Expr::Literal(Box::new(LiteralExpr::Num(2.0)));
    let operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

    let expr = Expr::Binary(Box::new(BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.value.downcast_ref::<bool>().unwrap(), false);
}

#[test]
fn test_is_truthy_none() {
    let obj = Object::new(None::<()>);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), false);
}

#[test]
fn test_is_truthy_true_bool() {
    let obj = Object::new(true);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), true);
}

#[test]
fn test_is_truthy_false_bool() {
    let obj = Object::new(false);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), false);
}

#[test]
fn test_is_truthy_string() {
    let obj = Object::new("hello".to_string());
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), true);
}

#[test]
fn test_is_truthy_empty_string() {
    let obj = Object::new("".to_string());
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), false);
}

#[test]
fn test_is_truthy_number() {
    let obj = Object::new(42.0);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), true);
}

#[test]
fn test_is_truthy_number_zero() {
    let obj = Object::new(0.0);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), false);
}

#[test]
fn test_is_truthy_catchall() {
    let obj = Object::new(LiteralExpr::Nil);
    let interpreter = Interpreter;
    assert_eq!(interpreter.is_truthy(&obj), true);
}

#[test]
fn test_is_equal_f64_equal() {
    let obj1 = Object::new(42.0);
    let obj2 = Object::new(42.0);
    let interpreter = Interpreter;
    assert!(interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_f64_not_equal() {
    let obj1 = Object::new(42.0);
    let obj2 = Object::new(43.0);
    let interpreter = Interpreter;
    assert!(!interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_string_equal() {
    let obj1 = Object::new("hello".to_string());
    let obj2 = Object::new("hello".to_string());
    let interpreter = Interpreter;
    assert!(interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_string_not_equal() {
    let obj1 = Object::new("hello".to_string());
    let obj2 = Object::new("world".to_string());
    let interpreter = Interpreter;
    assert!(!interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_bool_equal() {
    let obj1 = Object::new(true);
    let obj2 = Object::new(true);
    let interpreter = Interpreter;
    assert!(interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_bool_not_equal() {
    let obj1 = Object::new(true);
    let obj2 = Object::new(false);
    let interpreter = Interpreter;
    assert!(!interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_is_equal_different_types() {
    let obj1 = Object::new(42.0);
    let obj2 = Object::new("42".to_string());
    let interpreter = Interpreter;
    assert!(!interpreter.is_equal(&obj1, &obj2));
}

#[test]
fn test_interpret_string_literal() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Str("hello".to_string())));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&literal_expr);
    assert_eq!(result.type_name, std::any::type_name::<String>());
    assert_eq!(result.get_value::<String>().unwrap(), "hello");
}

#[test]
fn test_interpret_number_literal() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Num(42.0)));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&literal_expr);
    assert_eq!(result.type_name, std::any::type_name::<f64>());
    assert_eq!(*result.get_value::<f64>().unwrap(), 42.0);
}

#[test]
fn test_interpret_boolean_true_literal() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Bool(true)));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&literal_expr);
    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.get_value::<bool>().unwrap(), true);
}

#[test]
fn test_interpret_boolean_false_literal() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Bool(false)));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&literal_expr);
    assert_eq!(result.type_name, std::any::type_name::<bool>());
    assert_eq!(*result.get_value::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Unhandled literal expression type")]
fn test_interpret_unhandled_literal_type() {
    let literal_expr = Expr::Literal(Box::new(LiteralExpr::Nil));
    let mut interpreter = Interpreter;
    interpreter.interpret(&literal_expr);
}

#[test]
fn test_visit_grouping_expr_literal() {
    let grouping_expr = GroupingExpr {
        expr: Box::new(Expr::Literal(Box::new(LiteralExpr::new(42.0)))),
    };
    let expr = Expr::Grouping(Box::new(grouping_expr));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(*result.get_value::<f64>().unwrap(), 42.0);
}

#[test]
fn test_visit_grouping_expr_nested() {
    let grouping_expr = GroupingExpr {
        expr: Box::new(Expr::Literal(Box::new(LiteralExpr::new(42.0)))),
    };
    let outer_grouping_expr = GroupingExpr {
        expr: Box::new(Expr::Grouping(Box::new(grouping_expr))),
    };
    let expr = Expr::Grouping(Box::new(outer_grouping_expr));
    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);

    assert_eq!(*result.get_value::<f64>().unwrap(), 42.0);
}

#[test]
fn test_interpret_unary_minus_operator_with_number() {
    let operator = Token::new(TokenType::Minus, "-".to_string(), None, 1);
    let expr = Expr::Unary(Box::new(UnaryExpr {
        operator: operator.clone(),
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::new(42.0)))),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);
    assert_eq!(*result.get_value::<f64>().unwrap(), -42.0);
}

#[test]
#[should_panic(
    expected = "Unary operand Object { type_name: \"alloc::string::String\", value: Any { .. } } must be a number"
)]
fn test_interpret_unary_minus_operator_without_a_number() {
    let operator = Token::new(TokenType::Minus, "-".to_string(), None, 1);
    let expr = Expr::Unary(Box::new(UnaryExpr {
        operator: operator.clone(),
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::new(
            "not a number".to_string(),
        )))),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}

#[test]
fn test_interpret_unary_bang_operator_with_a_bool() {
    let operator = Token::new(TokenType::Bang, "!".to_string(), None, 1);
    let expr = Expr::Unary(Box::new(UnaryExpr {
        operator: operator.clone(),
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::new(true)))),
    }));

    let mut interpreter = Interpreter;
    let result = interpreter.interpret(&expr);
    assert_eq!(*result.get_value::<bool>().unwrap(), false);
}

#[test]
#[should_panic(expected = "Unknown unary token type LeftParen")]
fn test_interpret_unary_unknown_operator() {
    let operator = Token::new(TokenType::LeftParen, "(".to_string(), None, 1);
    let expr = Expr::Unary(Box::new(UnaryExpr {
        operator: operator.clone(),
        right: Box::new(Expr::Literal(Box::new(LiteralExpr::new(42.0)))),
    }));

    let mut interpreter = Interpreter;
    interpreter.interpret(&expr);
}
