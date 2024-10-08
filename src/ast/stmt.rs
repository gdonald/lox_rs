use super::{
    expr::{Expr, Visitor},
    token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            // Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
            _ => {
                panic!("Unhandled statement type")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: expression,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintStmt {
    pub expression: Expr,
}

impl PrintStmt {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: expression,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Expr,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Expr) -> Self {
        Self {
            name: name,
            initializer: initializer,
        }
    }
}
