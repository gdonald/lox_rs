extern crate lox;

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

    let mut printer = AstPrinter;
    let result = printer.print(&expression);
    println!("{}", result);
}