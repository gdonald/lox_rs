use lox_rs::ast::expr::LiteralExpr;
use lox_rs::ast::scanner::KEYWORDS;
use lox_rs::ast::scanner::{ScanError, Scanner};
use lox_rs::ast::token::TokenType;

#[test]
fn test_initial_state_is_false() {
    let error = ScanError::new();
    assert!(!error.detected(), "Initial state should be false");
}

#[test]
fn test_set_detected_to_true() {
    let error = ScanError::new();
    error.error(1, "Test error");
    assert!(
        error.detected(),
        "State should be true after setting to true"
    );
}

#[test]
fn test_keywords_present() {
    assert_eq!(KEYWORDS.get("and"), Some(&TokenType::And));
    assert_eq!(KEYWORDS.get("class"), Some(&TokenType::Class));
    assert_eq!(KEYWORDS.get("else"), Some(&TokenType::Else));
    assert_eq!(KEYWORDS.get("false"), Some(&TokenType::False));
    assert_eq!(KEYWORDS.get("for"), Some(&TokenType::For));
    assert_eq!(KEYWORDS.get("fun"), Some(&TokenType::Fun));
    assert_eq!(KEYWORDS.get("if"), Some(&TokenType::If));
    assert_eq!(KEYWORDS.get("nil"), Some(&TokenType::Nil));
    assert_eq!(KEYWORDS.get("or"), Some(&TokenType::Or));
    assert_eq!(KEYWORDS.get("print"), Some(&TokenType::Print));
    assert_eq!(KEYWORDS.get("return"), Some(&TokenType::Return));
    assert_eq!(KEYWORDS.get("super"), Some(&TokenType::Super));
    assert_eq!(KEYWORDS.get("this"), Some(&TokenType::This));
    assert_eq!(KEYWORDS.get("true"), Some(&TokenType::True));
    assert_eq!(KEYWORDS.get("var"), Some(&TokenType::Var));
    assert_eq!(KEYWORDS.get("while"), Some(&TokenType::While));
}

#[test]
fn test_keywords_not_present() {
    assert_eq!(KEYWORDS.get("foobar"), None);
    assert_eq!(KEYWORDS.get("hello"), None);
    assert_eq!(KEYWORDS.get("world"), None);
}

#[test]
fn test_keyword_count() {
    assert_eq!(KEYWORDS.len(), 16);
}

#[test]
fn test_scanner_initialization() {
    let source = "class".to_string();
    let scanner = Scanner::new(source.clone(), ScanError::new());
    assert_eq!(scanner.source, source);
    assert_eq!(scanner.tokens.len(), 0);
    assert_eq!(scanner.start, 0);
    assert_eq!(scanner.current, 0);
    assert_eq!(scanner.line, 1);
    assert!(
        !scanner.error.detected(),
        "Error state should be false initially"
    );
}

#[test]
fn test_scan_tokens_empty_input() {
    let mut scanner = Scanner::new("".to_string(), ScanError::new());
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Eof);
}

#[test]
fn test_scan_tokens_single_character() {
    let mut scanner = Scanner::new("(".to_string(), ScanError::new());
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_scan_tokens_multiple_characters() {
    let mut scanner = Scanner::new("(+)".to_string(), ScanError::new());
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::RightParen);
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_scan_tokens_with_number() {
    let mut scanner = Scanner::new("42".to_string(), ScanError::new());
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
    if let Some(LiteralExpr::Num(value)) = &tokens[0].literal {
        assert_eq!(*value, 42.0);
    } else {
        panic!("Expected number literal");
    }
}

#[test]
fn test_is_at_end_at_start() {
    let scanner = Scanner::new("test".to_string(), ScanError::new());
    assert!(
        !scanner.is_at_end(),
        "Scanner should not be at the end initially"
    );
}

#[test]
fn test_is_at_end_at_middle() {
    let mut scanner = Scanner::new("test".to_string(), ScanError::new());
    scanner.current = 2;
    assert!(
        !scanner.is_at_end(),
        "Scanner should not be at the end when in the middle of the input"
    );
}

#[test]
fn test_is_at_end_at_end() {
    let mut scanner = Scanner::new("test".to_string(), ScanError::new());
    scanner.current = 4;
    assert!(
        scanner.is_at_end(),
        "Scanner should be at the end when current equals the length of the input"
    );
}

#[test]
fn test_is_at_end_beyond_end() {
    let mut scanner = Scanner::new("test".to_string(), ScanError::new());
    scanner.current = 5;
    assert!(
        scanner.is_at_end(),
        "Scanner should be at the end when current is beyond the length of the input"
    );
}

#[test]
fn test_is_at_end_with_empty_string() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(
        scanner.is_at_end(),
        "Scanner should be at the end for an empty string"
    );
}

#[test]
fn test_scan_single_character_tokens() {
    let source = "_(){},.-+;*=><!\"".to_string();
    let len = source.len();
    let mut scanner = Scanner::new(source, ScanError::new());
    for _ in 0..len {
        scanner.scan_token();
    }

    let expected_tokens = vec![
        TokenType::Identifier,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Minus,
        TokenType::Plus,
        TokenType::Semicolon,
        TokenType::Star,
        TokenType::Equal,
        TokenType::Greater,
        TokenType::Less,
        TokenType::Bang,
        TokenType::String,
    ];

    for (i, token) in scanner.tokens.iter().enumerate() {
        assert_eq!(token.token_type, expected_tokens[i]);
    }
}

#[test]
fn test_scan_two_character_tokens() {
    let source = "!= == <= >=".to_string();
    let len = source.len();
    let mut scanner = Scanner::new(source, ScanError::new());
    while scanner.current < len {
        scanner.scan_token();
    }

    let expected_tokens = vec![
        TokenType::BangEqual,
        TokenType::EqualEqual,
        TokenType::LessEqual,
        TokenType::GreaterEqual,
    ];

    for (i, token) in scanner.tokens.iter().enumerate() {
        assert_eq!(token.token_type, expected_tokens[i]);
    }
}

#[test]
fn test_scan_slash_token() {
    let source = "/ // comment\n".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.scan_token();
    scanner.scan_token();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Slash);
}

#[test]
fn test_scan_comment_token() {
    let source = "// comment\n".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.scan_token();
    scanner.scan_token();

    assert_eq!(scanner.tokens.len(), 0);
}

#[test]
fn test_scan_whitespace_and_newline() {
    let source = " \n\t".to_string();
    let len = source.len();
    let mut scanner = Scanner::new(source, ScanError::new());
    for _ in 0..len {
        scanner.scan_token();
    }

    assert_eq!(scanner.tokens.len(), 0);
}

#[test]
fn test_scan_unexpected_character() {
    let source = "@".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.scan_token();

    assert_eq!(scanner.tokens.len(), 0);
    assert!(scanner.error.detected());
}

#[test]
fn test_is_alpha_with_lowercase_letters() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha('a'));
    assert!(scanner.is_alpha('z'));
}

#[test]
fn test_is_alpha_with_uppercase_letters() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha('A'));
    assert!(scanner.is_alpha('Z'));
}

#[test]
fn test_is_alpha_with_underscore() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha('_'));
}

#[test]
fn test_is_alpha_with_non_alpha_characters() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(!scanner.is_alpha('1'));
    assert!(!scanner.is_alpha(' '));
    assert!(!scanner.is_alpha('!'));
    assert!(!scanner.is_alpha('-'));
}

#[test]
fn test_is_alpha_with_boundary_cases() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(!scanner.is_alpha('`')); // ASCII before 'a'
    assert!(!scanner.is_alpha('{')); // ASCII after 'z'
    assert!(!scanner.is_alpha('@')); // ASCII before 'A'
    assert!(!scanner.is_alpha('[')); // ASCII after 'Z'
}

#[test]
fn test_is_alpha_numeric_with_letters() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha_numeric('a'));
    assert!(scanner.is_alpha_numeric('Z'));
}

#[test]
fn test_is_alpha_numeric_with_underscore() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha_numeric('_'));
}

#[test]
fn test_is_alpha_numeric_with_digits() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha_numeric('0'));
    assert!(scanner.is_alpha_numeric('9'));
}

#[test]
fn test_is_alpha_numeric_with_non_alphanumeric_characters() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(!scanner.is_alpha_numeric(' '));
    assert!(!scanner.is_alpha_numeric('-'));
    assert!(!scanner.is_alpha_numeric('!'));
    assert!(!scanner.is_alpha_numeric('@'));
}

#[test]
fn test_is_alpha_numeric_with_boundary_cases() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_alpha_numeric('a'));
    assert!(scanner.is_alpha_numeric('Z'));
    assert!(scanner.is_alpha_numeric('0'));
    assert!(scanner.is_alpha_numeric('9'));
    assert!(!scanner.is_alpha_numeric('/')); // ASCII before '0'
    assert!(!scanner.is_alpha_numeric(':')); // ASCII after '9'
    assert!(!scanner.is_alpha_numeric('`')); // ASCII before 'a'
    assert!(!scanner.is_alpha_numeric('[')); // ASCII after 'Z'
}

#[test]
fn test_identifier_as_keyword() {
    let source = "var".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.identifier();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Var);
    assert_eq!(scanner.tokens[0].lexeme, "var");
}

#[test]
fn test_identifier_as_regular_identifier() {
    let source = "my_var".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.identifier();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[0].lexeme, "my_var");
}

#[test]
fn test_identifier_as_identifier_with_embedded_number() {
    let source = "my_var_87".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.identifier();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[0].lexeme, "my_var_87");
}

#[test]
fn test_scan_integer_number() {
    let source = "12345".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.number();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, "12345");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Num(12345.0))
    );
}

#[test]
fn test_scan_float_number() {
    let source = "123.456".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.number();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, "123.456");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Num(123.456))
    );
}

#[test]
fn test_scan_number_with_trailing_dot() {
    let source = "123.".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.number();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, "123");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Num(123.0))
    );
}

#[test]
fn test_scan_number_with_leading_dot() {
    let source = ".456".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.number();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, ".456");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Num(0.456))
    );
}

#[test]
fn test_is_digit_with_digits() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_digit('0'));
    assert!(scanner.is_digit('5'));
    assert!(scanner.is_digit('9'));
}

#[test]
fn test_is_digit_with_non_digits() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(!scanner.is_digit('a'));
    assert!(!scanner.is_digit('Z'));
    assert!(!scanner.is_digit(' '));
    assert!(!scanner.is_digit('-'));
}

#[test]
fn test_is_digit_with_boundary_cases() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert!(scanner.is_digit('0'));  // Lower boundary
    assert!(scanner.is_digit('9'));  // Upper boundary
    assert!(!scanner.is_digit('/')); // Just before '0'
    assert!(!scanner.is_digit(':')); // Just after '9'
}

#[test]
fn test_string_literal() {
    let source = "\"hello world\"".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.string();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::String);
    assert_eq!(scanner.tokens[0].lexeme, "\"hello world\"");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Str("hello world".to_string()))
    );
}

#[test]
fn test_multi_line_string_literal() {
    let source = "\"hello\nworld\"".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.string();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::String);
    assert_eq!(scanner.tokens[0].lexeme, "\"hello\nworld\"");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Str("hello\nworld".to_string()))
    );
    assert_eq!(scanner.line, 2);
}

#[test]
fn test_unterminated_string_literal() {
    let source = "\"hello world".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.string();

    assert!(scanner.error.detected());
    assert_eq!(scanner.tokens.len(), 0);
}

#[test]
fn test_peek_at_start() {
    let scanner = Scanner::new("hello".to_string(), ScanError::new());
    assert_eq!(scanner.peek(), 'h');
}

#[test]
fn test_peek_at_middle() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 2;
    assert_eq!(scanner.peek(), 'l');
}

#[test]
fn test_peek_at_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 5; // Position at the end of the string
    assert_eq!(scanner.peek(), '\0');
}

#[test]
fn test_peek_beyond_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 10; // Position beyond the end of the string
    assert_eq!(scanner.peek(), '\0');
}

#[test]
fn test_peek_with_empty_string() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert_eq!(scanner.peek(), '\0');
}

#[test]
fn test_peek_next_at_start() {
    let scanner = Scanner::new("hello".to_string(), ScanError::new());
    assert_eq!(scanner.peek_next(), 'e');
}

#[test]
fn test_peek_next_in_middle() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 2;
    assert_eq!(scanner.peek_next(), 'l');
}

#[test]
fn test_peek_next_at_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 4; // Position at the last character
    assert_eq!(scanner.peek_next(), '\0');
}

#[test]
fn test_peek_next_beyond_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 5; // Position beyond the end of the string
    assert_eq!(scanner.peek_next(), '\0');
}

#[test]
fn test_peek_next_with_empty_string() {
    let scanner = Scanner::new("".to_string(), ScanError::new());
    assert_eq!(scanner.peek_next(), '\0');
}

#[test]
fn test_match_char_success() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    assert!(scanner.match_char('h'));
    assert_eq!(scanner.current, 1);
}

#[test]
fn test_match_char_failure() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    assert!(!scanner.match_char('x'));
    assert_eq!(scanner.current, 0); // Should not advance
}

#[test]
fn test_match_char_at_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 5; // Position at the end of the string
    assert!(!scanner.match_char('o'));
    assert_eq!(scanner.current, 5); // Should not advance
}

#[test]
fn test_match_char_advance_on_success() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    assert!(scanner.match_char('h'));
    assert_eq!(scanner.current, 1); // Should advance
    assert!(scanner.match_char('e'));
    assert_eq!(scanner.current, 2); // Should advance
}

#[test]
fn test_match_char_with_multibyte_characters() {
    let mut scanner = Scanner::new("héllo".to_string(), ScanError::new());
    assert!(scanner.match_char('h'));
    assert_eq!(scanner.current, 1); // Should advance by 1
    assert!(scanner.match_char('é'));
    assert_eq!(scanner.current, 3); // Should advance by 2 (UTF-8 encoding)
}

#[test]
fn test_advance_at_start() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    let ch = scanner.advance();
    assert_eq!(ch, 'h');
    assert_eq!(scanner.current, 1);
}

#[test]
fn test_advance_in_middle() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 2;
    let ch = scanner.advance();
    assert_eq!(ch, 'l');
    assert_eq!(scanner.current, 3);
}

#[test]
fn test_advance_at_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 4; // Position at 'o'
    let ch = scanner.advance();
    assert_eq!(ch, 'o');
    assert_eq!(scanner.current, 5);
}

#[test]
fn test_advance_with_multibyte_characters() {
    let mut scanner = Scanner::new("héllo".to_string(), ScanError::new());
    let ch = scanner.advance();
    assert_eq!(ch, 'h');
    assert_eq!(scanner.current, 1);
    let ch = scanner.advance();
    assert_eq!(ch, 'é');
    assert_eq!(scanner.current, 3); // UTF-8 character 'é' takes 2 bytes
}

#[test]
#[should_panic(expected = "unwrap")]
fn test_advance_past_end() {
    let mut scanner = Scanner::new("hello".to_string(), ScanError::new());
    scanner.current = 5; // Position beyond the end
    scanner.advance(); // This should panic
}

#[test]
fn test_add_token_with_identifier() {
    let mut scanner = Scanner::new("identifier".to_string(), ScanError::new());
    scanner.current = 10; // Simulating the end of the identifier
    scanner.add_token(TokenType::Identifier);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[0].lexeme, "identifier");
    assert_eq!(scanner.tokens[0].literal, None);
}

#[test]
fn test_add_token_with_number() {
    let mut scanner = Scanner::new("123".to_string(), ScanError::new());
    scanner.current = 3; // Simulating the end of the number
    scanner.add_token(TokenType::Number);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, "123");
    assert_eq!(scanner.tokens[0].literal, None);
}

#[test]
fn test_add_token_with_plus() {
    let mut scanner = Scanner::new("+".to_string(), ScanError::new());
    scanner.current = 1; // Simulating the end of the plus sign
    scanner.add_token(TokenType::Plus);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Plus);
    assert_eq!(scanner.tokens[0].lexeme, "+");
    assert_eq!(scanner.tokens[0].literal, None);
}

#[test]
fn test_add_token_with_minus() {
    let mut scanner = Scanner::new("-".to_string(), ScanError::new());
    scanner.current = 1; // Simulating the end of the minus sign
    scanner.add_token(TokenType::Minus);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Minus);
    assert_eq!(scanner.tokens[0].lexeme, "-");
    assert_eq!(scanner.tokens[0].literal, None);
}

#[test]
fn test_add_token_with_literal_string() {
    let mut scanner = Scanner::new("\"hello\"".to_string(), ScanError::new());
    scanner.current = 7; // Simulating the end of the string literal
    scanner.add_token_with_literal(TokenType::String, Some(LiteralExpr::Str("hello".to_string())));

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::String);
    assert_eq!(scanner.tokens[0].lexeme, "\"hello\"");
    assert_eq!(
        scanner.tokens[0].literal,
        Some(LiteralExpr::Str("hello".to_string()))
    );
}

#[test]
fn test_add_token_with_literal_number() {
    let mut scanner = Scanner::new("123".to_string(), ScanError::new());
    scanner.current = 3; // Simulating the end of the number
    scanner.add_token_with_literal(TokenType::Number, Some(LiteralExpr::Num(123.0)));

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[0].lexeme, "123");
    assert_eq!(scanner.tokens[0].literal, Some(LiteralExpr::Num(123.0)));
}

#[test]
fn test_add_token_with_literal_none() {
    let mut scanner = Scanner::new("+".to_string(), ScanError::new());
    scanner.current = 1; // Simulating the end of the plus sign
    scanner.add_token_with_literal(TokenType::Plus, None);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Plus);
    assert_eq!(scanner.tokens[0].lexeme, "+");
    assert_eq!(scanner.tokens[0].literal, None);
}

#[test]
fn test_add_token_with_literal_identifier() {
    let mut scanner = Scanner::new("variable".to_string(), ScanError::new());
    scanner.current = 8; // Simulating the end of the identifier
    scanner.add_token_with_literal(TokenType::Identifier, None);

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[0].lexeme, "variable");
    assert_eq!(scanner.tokens[0].literal, None);
}
