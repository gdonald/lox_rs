use core::panic;

use crate::ast::expr::Visitor;
use crate::ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::ast::object::Object;
use crate::ast::token::{Token, TokenType};

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, expression: &Expr) -> Object {
        self.evaluate(expression.clone())
    }

    pub fn evaluate(&mut self, expr: Expr) -> Object {
        expr.accept(self)
    }

    pub fn is_truthy(&self, object: &Object) -> bool {
        if object.is::<Option<()>>() {
            return false;
        }

        if let Some(boolean) = object.get_value::<bool>() {
            return *boolean;
        }

        if let Some(string) = object.get_value::<String>() {
            return !string.is_empty();
        }

        if let Some(number) = object.get_value::<f64>() {
            return *number != 0.0;
        }

        true
    }

    pub fn is_equal(&self, a: &Object, b: &Object) -> bool {
        if a.is::<f64>() && b.is::<f64>() {
            return a.get_value::<f64>() == b.get_value::<f64>();
        }

        if a.is::<String>() && b.is::<String>() {
            return a.get_value::<String>() == b.get_value::<String>();
        }

        if a.is::<bool>() && b.is::<bool>() {
            return a.get_value::<bool>() == b.get_value::<bool>();
        }

        false
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Object {
        match expr {
            LiteralExpr::Str(value) => Object::new(value.clone()),
            LiteralExpr::Num(value) => Object::new(*value),
            LiteralExpr::Bool(value) => Object::new(*value),
            _ => {
                panic!("Unhandled literal expression type");
            }
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Object {
        let left = self.evaluate(Expr::from(*(expr.left).clone()));
        let right = self.evaluate(Expr::from(*(expr.right).clone()));

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
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num > right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::GreaterEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num >= right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::Less => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num < right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::LessEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num <= right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::Plus => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num + right_num)
                } else if let (Some(left_str), Some(right_str)) =
                    (left.get_value::<String>(), right.get_value::<String>())
                {
                    Object::new(format!("{}{}", left_str, right_str))
                } else {
                    panic!(
                        "Operands {:?}, {:?} must be matching types for the {:?} operation",
                        left.get_value::<String>(),
                        right.get_value::<String>(),
                        expr.operator.token_type
                    );
                }
            }
            TokenType::Minus => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num - right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::Slash => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num / right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            TokenType::Star => {
                if let (Some(left_num), Some(right_num)) =
                    (left.get_value::<f64>(), right.get_value::<f64>())
                {
                    Object::new(left_num * right_num)
                } else {
                    panic!(
                        "Operands must be numbers for the {:?} operation",
                        expr.operator.token_type
                    );
                }
            }
            _ => {
                panic!("Unknown operator {:?}", expr.operator.token_type);
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
                if let Some(num) = value.get_value::<f64>() {
                    Object::new(-num)
                } else {
                    panic!("Unary operand {:?} must be a number", value);
                }
            }
            TokenType::Bang => Object::new(!self.is_truthy(&value)),
            _ => {
                panic!("Unknown token type {:?}", expr.operator.token_type);
            }
        }
    }
}
