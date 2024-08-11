use super::expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};

pub(crate) trait ExprVisitor<R> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> R;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> R;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> R;
}
