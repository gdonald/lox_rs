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
    let source = "(){},.-+;*".to_string();
    let len = source.len();
    let mut scanner = Scanner::new(source, ScanError::new());
    for _ in 0..len {
        scanner.scan_token();
    }

    let expected_tokens = vec![
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
    let source = "/ // comment".to_string();
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.scan_token();
    scanner.scan_token();

    assert_eq!(scanner.tokens.len(), 1);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Slash);
}

#[test]
fn test_scan_whitespace_and_newline() {
    let source = " \n\t".to_string();
    let len = source.len();
    let mut scanner = Scanner::new(source, ScanError::new());
    for _ in 0..len {
        scanner.scan_token();
    }

    // No tokens should be added for whitespace and newlines
    assert_eq!(scanner.tokens.len(), 0);
}

#[test]
fn test_scan_unexpected_character() {
    let source = "@".to_string(); // '@' is an unexpected character
    let mut scanner = Scanner::new(source, ScanError::new());
    scanner.scan_token();

    assert!(scanner.error.detected());
}
