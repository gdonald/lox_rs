use crate::ast::token::Token;
use crate::ast::visitor::ExprVisitor;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),

            _ => {
                panic!("Unhandled expression type")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

impl GroupingExpr {
    fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr)
        }
    }
}

#[derive(Debug, Clone)]
pub enum LiteralExpr {
    Str(String),
    Num(f64),
    Nil,
}

impl LiteralExpr {
    fn new(value: impl Into<LiteralExpr>) -> Self {
        value.into()
    }
}

impl From<String> for LiteralExpr {
    fn from(value: String) -> Self {
        LiteralExpr::Str(value)
    }
}

impl From<f64> for LiteralExpr {
    fn from(value: f64) -> Self {
        LiteralExpr::Num(value)
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
