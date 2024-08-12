use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::expr_visitor::ExprVisitor;
use crate::ast::object::Object;
use crate::ast::token::TokenType;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: Expr) -> Object {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        if object.is::<Option<()>>() {
            return false;
        }

        if let Some(boolean) = object.downcast_ref::<bool>() {
            return *boolean;
        }

        true
    }

    fn is_equal(&self, a: &Object, b: &Object) -> bool {
        if a.is::<f64>() && b.is::<f64>() {
            return a.downcast_ref::<f64>() == b.downcast_ref::<f64>();
        }

        if a.is::<String>() && b.is::<String>() {
            return a.downcast_ref::<String>() == b.downcast_ref::<String>();
        }

        if a.is::<bool>() && b.is::<bool>() {
            return a.downcast_ref::<bool>() == b.downcast_ref::<bool>();
        }

        false
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Object {
        Object::new(expr.clone())
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Object {
        let left = self.evaluate(Expr::from(*expr.left.clone()));
        let right = self.evaluate(Expr::from(*expr.right.clone()));

        match expr.operator.token_type {
            TokenType::EqualEqual => {
                let value = self.is_equal(&left, &right);
                Object::new(value)
            }
            TokenType::BangEqual => {
                let value = self.is_equal(&left, &right);
                Object::new(!value)
            }
            TokenType::Greater => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num > right_num)
                } else {
                    panic!("Operands must be numbers for the GREATER operation");
                }
            }
            TokenType::GreaterEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num >= right_num)
                } else {
                    panic!("Operands must be numbers for the GREATER EQUAL operation");
                }
            }
            TokenType::Less => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num < right_num)
                } else {
                    panic!("Operands must be numbers for the LESS operation");
                }
            }
            TokenType::LessEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num <= right_num)
                } else {
                    panic!("Operands must be numbers for the LESS EQUAL operation");
                }
            }
            TokenType::Plus => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num + right_num)
                } else if let (Some(left_str), Some(right_str)) = (
                    left.downcast_ref::<String>(),
                    right.downcast_ref::<String>(),
                ) {
                    Object::new(format!("{}{}", left_str, right_str))
                } else {
                    panic!("Operands must be matching types for the PLUS operation");
                }
            }
            TokenType::Minus => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num - right_num)
                } else {
                    panic!("Operands must be numbers for the MINUS operation");
                }
            }
            TokenType::Slash => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num / right_num)
                } else {
                    panic!("Operands must be numbers for the SLASH operation");
                }
            }
            TokenType::Star => {
                if let (Some(left_num), Some(right_num)) =
                    (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                {
                    Object::new(left_num * right_num)
                } else {
                    panic!("Operands must be numbers for the STAR operation");
                }
            }
            _ => {
                panic!("Unknown operator"); // Handle other operators or errors
            }
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Object {
        let value = self.evaluate(Expr::from(*expr.expr.clone()));
        Object::new(value)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Object {
        let value = self.evaluate(Expr::from(*expr.right.clone()));

        match expr.operator.token_type {
            TokenType::Minus => {
                if let Some(num) = value.downcast_ref::<f64>() {
                    Object::new(-num)
                } else {
                    panic!("Operand must be a number");
                }
            }
            TokenType::Bang => Object::new(!self.is_truthy(&value)),
            _ => {
                panic!("Unknown token type");
            }
        }
    }
}
