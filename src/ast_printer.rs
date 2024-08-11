use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::expr_visitor::ExprVisitor;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            let result = expr.accept(self);
            builder.push_str(result.as_str());
        }
        builder.push(')');

        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", &[&expr.expr])
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        match expr {
            LiteralExpr::Str(s) => s.clone(),
            LiteralExpr::Num(n) => n.to_string(),
            LiteralExpr::Bool(b) => b.to_string(),
            LiteralExpr::Nil => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}