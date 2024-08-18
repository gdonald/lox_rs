use crate::ast::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
    Unhandled,
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            _ => { panic!("Unhandled expression type") }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

impl LiteralExpr {
    pub fn new(value: impl Into<LiteralExpr>) -> Self {
        value.into()
    }

    pub fn extract_num(expr: &Expr) -> f64 {
        if let Expr::Literal(literal) = expr {
            if let LiteralExpr::Num(value) = **literal {
                return value;
            }
        }

        panic!("Failed to extract number from expression {:?}.", expr);
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

impl From<bool> for LiteralExpr {
    fn from(value: bool) -> Self {
        LiteralExpr::Bool(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> R;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> R;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> R;
}
