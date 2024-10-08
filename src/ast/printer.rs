use crate::ast::expr::Visitor;
use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};

use super::expr::VariableExpr;
use super::stmt::{ExpressionStmt, PrintStmt, VarStmt};

pub struct Printer;

impl Printer {
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

impl Visitor<String> for Printer {
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

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> String {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> String {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> String {
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> String {
        todo!()
    }
}
