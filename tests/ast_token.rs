use lox_rs::ast::expr::LiteralExpr;
use lox_rs::ast::token::{Token, TokenType};

macro_rules! test_create_token {
    ($name:ident, $token_type:expr, $lexeme:expr, $literal:expr, $line:expr) => {
        #[test]
        fn $name() {
            let token = Token::new($token_type, $lexeme.to_string(), $literal, $line);

            assert_eq!(token.token_type, $token_type);
            assert_eq!(token.lexeme, $lexeme);
            assert_eq!(token.literal, $literal);
            assert_eq!(token.line, $line);
        }
    };
}

test_create_token!(
    test_create_token_with_left_paren_token_type,
    TokenType::LeftParen,
    "(",
    None,
    1
);
test_create_token!(
    test_create_token_with_right_paren_token_type,
    TokenType::RightParen,
    ")",
    None,
    2
);
test_create_token!(
    test_create_token_with_left_brace_token_type,
    TokenType::LeftBrace,
    "{",
    None,
    3
);
test_create_token!(
    test_create_token_with_right_brace_token_type,
    TokenType::RightBrace,
    "}",
    None,
    4
);
test_create_token!(
    test_create_token_with_comma_token_type,
    TokenType::Comma,
    ",",
    None,
    5
);
test_create_token!(
    test_create_token_with_dot_token_type,
    TokenType::Dot,
    ".",
    None,
    6
);
test_create_token!(
    test_create_token_with_minus_token_type,
    TokenType::Minus,
    "-",
    None,
    7
);
test_create_token!(
    test_create_token_with_plus_token_type,
    TokenType::Plus,
    "+",
    None,
    8
);
test_create_token!(
    test_create_token_with_semicolon_token_type,
    TokenType::Semicolon,
    ";",
    None,
    9
);
test_create_token!(
    test_create_token_with_slash_token_type,
    TokenType::Slash,
    "/",
    None,
    10
);
test_create_token!(
    test_create_token_with_star_token_type,
    TokenType::Star,
    "*",
    None,
    11
);

test_create_token!(
    test_create_token_with_bang_token_type,
    TokenType::Bang,
    "!",
    None,
    12
);
test_create_token!(
    test_create_token_with_bang_equal_token_type,
    TokenType::BangEqual,
    "!=",
    None,
    13
);
test_create_token!(
    test_create_token_with_equal_token_type,
    TokenType::Equal,
    "=",
    None,
    14
);
test_create_token!(
    test_create_token_with_equal_equal_token_type,
    TokenType::EqualEqual,
    "==",
    None,
    15
);
test_create_token!(
    test_create_token_with_greater_token_type,
    TokenType::Greater,
    ">",
    None,
    16
);
test_create_token!(
    test_create_token_with_greater_equal_token_type,
    TokenType::GreaterEqual,
    ">=",
    None,
    17
);
test_create_token!(
    test_create_token_with_less_token_type,
    TokenType::Less,
    "<",
    None,
    18
);
test_create_token!(
    test_create_token_with_less_equal_token_type,
    TokenType::LessEqual,
    "<=",
    None,
    19
);

test_create_token!(
    test_create_token_with_identifier_token_type,
    TokenType::Identifier,
    "identifier",
    Some(LiteralExpr::Str("x".to_string())),
    20
);
test_create_token!(
    test_create_token_with_string_token_type,
    TokenType::String,
    "string",
    Some(LiteralExpr::Str("x".to_string())),
    21
);
test_create_token!(
    test_create_token_with_number_token_type,
    TokenType::Number,
    "number",
    Some(LiteralExpr::Num(17f64)),
    22
);

test_create_token!(
    test_create_token_with_and_token_type,
    TokenType::And,
    "and",
    None,
    23
);
test_create_token!(
    test_create_token_with_class_token_type,
    TokenType::Class,
    "class",
    None,
    24
);
test_create_token!(
    test_create_token_with_else_token_type,
    TokenType::Else,
    "else",
    None,
    25
);
test_create_token!(
    test_create_token_with_false_token_type,
    TokenType::False,
    "false",
    None,
    26
);
test_create_token!(
    test_create_token_with_fun_token_type,
    TokenType::Fun,
    "fun",
    None,
    27
);
test_create_token!(
    test_create_token_with_for_token_type,
    TokenType::For,
    "for",
    None,
    28
);
test_create_token!(
    test_create_token_with_if_token_type,
    TokenType::If,
    "if",
    None,
    29
);
test_create_token!(
    test_create_token_with_nil_token_type,
    TokenType::Nil,
    "nil",
    None,
    30
);
test_create_token!(
    test_create_token_with_or_token_type,
    TokenType::Or,
    "or",
    None,
    31
);
test_create_token!(
    test_create_token_with_print_token_type,
    TokenType::Print,
    "print",
    None,
    32
);
test_create_token!(
    test_create_token_with_return_token_type,
    TokenType::Return,
    "return",
    None,
    33
);
test_create_token!(
    test_create_token_with_super_token_type,
    TokenType::Super,
    "super",
    None,
    34
);
test_create_token!(
    test_create_token_with_this_token_type,
    TokenType::This,
    "this",
    None,
    35
);
test_create_token!(
    test_create_token_with_true_token_type,
    TokenType::True,
    "true",
    None,
    36
);
test_create_token!(
    test_create_token_with_var_token_type,
    TokenType::Var,
    "var",
    None,
    37
);
test_create_token!(
    test_create_token_with_while_token_type,
    TokenType::While,
    "while",
    None,
    38
);

test_create_token!(
    test_create_token_with_eof_token_type,
    TokenType::Eof,
    "",
    None,
    39
);

macro_rules! test_token_display {
    ($name:ident, $token_type:expr, $lexeme:expr, $literal:expr, $line:expr, $expected_output:expr) => {
        #[test]
        fn $name() {
            let token = Token::new($token_type, $lexeme.to_string(), $literal, $line);
            assert_eq!(token.to_string(), $expected_output);
        }
    };
}

test_token_display!(
    test_token_display_with_number_literal,
    TokenType::Number,
    "42",
    Some(LiteralExpr::Num(42.0)),
    1,
    "Number 42 Some(Num(42.0))"
);

test_token_display!(
    test_token_display_with_string_literal,
    TokenType::String,
    "\"hello\"",
    Some(LiteralExpr::Str("hello".to_string())),
    2,
    "String \"hello\" Some(Str(\"hello\"))"
);

test_token_display!(
    test_token_display_with_no_literal,
    TokenType::Plus,
    "+",
    None,
    3,
    "Plus + None"
);

test_token_display!(
    test_token_display_with_boolean_literal,
    TokenType::Minus,
    "-",
    Some(LiteralExpr::Bool(true)),
    4,
    "Minus - Some(Bool(true))"
);

macro_rules! test_display_token_type {
    ($name:ident, $token_type:expr, $expected_output:expr) => {
        #[test]
        fn $name() {
            let token_type = $token_type;
            assert_eq!(token_type.to_string(), $expected_output);
        }
    };
}

test_display_token_type!(test_display_left_paren, TokenType::LeftParen, "LeftParen");
test_display_token_type!(
    test_display_right_paren,
    TokenType::RightParen,
    "RightParen"
);
test_display_token_type!(test_display_left_brace, TokenType::LeftBrace, "LeftBrace");
test_display_token_type!(
    test_display_right_brace,
    TokenType::RightBrace,
    "RightBrace"
);
test_display_token_type!(test_display_comma, TokenType::Comma, "Comma");
test_display_token_type!(test_display_dot, TokenType::Dot, "Dot");
test_display_token_type!(test_display_minus, TokenType::Minus, "Minus");
test_display_token_type!(test_display_plus, TokenType::Plus, "Plus");
test_display_token_type!(test_display_semicolon, TokenType::Semicolon, "Semicolon");
test_display_token_type!(test_display_slash, TokenType::Slash, "Slash");
test_display_token_type!(test_display_star, TokenType::Star, "Star");

test_display_token_type!(test_display_bang, TokenType::Bang, "Bang");
test_display_token_type!(test_display_bang_equal, TokenType::BangEqual, "BangEqual");
test_display_token_type!(test_display_equal, TokenType::Equal, "Equal");
test_display_token_type!(
    test_display_equal_equal,
    TokenType::EqualEqual,
    "EqualEqual"
);
test_display_token_type!(test_display_greater, TokenType::Greater, "Greater");
test_display_token_type!(
    test_display_greater_equal,
    TokenType::GreaterEqual,
    "GreaterEqual"
);
test_display_token_type!(test_display_less, TokenType::Less, "Less");
test_display_token_type!(test_display_less_equal, TokenType::LessEqual, "LessEqual");

test_display_token_type!(test_display_identifier, TokenType::Identifier, "Identifier");
test_display_token_type!(test_display_string, TokenType::String, "String");
test_display_token_type!(test_display_number, TokenType::Number, "Number");

test_display_token_type!(test_display_and, TokenType::And, "And");
test_display_token_type!(test_display_class, TokenType::Class, "Class");
test_display_token_type!(test_display_else, TokenType::Else, "Else");
test_display_token_type!(test_display_false, TokenType::False, "False");
test_display_token_type!(test_display_fun, TokenType::Fun, "Fun");
test_display_token_type!(test_display_for, TokenType::For, "For");
test_display_token_type!(test_display_if, TokenType::If, "If");
test_display_token_type!(test_display_nil, TokenType::Nil, "Nil");
test_display_token_type!(test_display_or, TokenType::Or, "Or");
test_display_token_type!(test_display_print, TokenType::Print, "Print");
test_display_token_type!(test_display_return, TokenType::Return, "Return");
test_display_token_type!(test_display_super, TokenType::Super, "Super");
test_display_token_type!(test_display_this, TokenType::This, "This");
test_display_token_type!(test_display_true, TokenType::True, "True");
test_display_token_type!(test_display_var, TokenType::Var, "Var");
test_display_token_type!(test_display_while, TokenType::While, "While");

test_display_token_type!(test_display_eof, TokenType::Eof, "Eof");
