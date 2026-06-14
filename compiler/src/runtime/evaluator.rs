use crate::runtime::environment::Environment;
use crate::runtime::value::Value;
use crate::runtime::error::RuntimeError;

use crate::parser::ast::{Expr, Stmt};

pub struct Evaluator {
    pub env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn eval_value(&mut self, value: &crate::parser::ast::Value) -> Result<Value, RuntimeError> {
        match value {
            crate::parser::ast::Value::Integer(i) => Ok(Value::Int(*i)),
            crate::parser::ast::Value::Float(f) => Ok(Value::Float(*f)),
            crate::parser::ast::Value::Text(s) => Ok(Value::String(s.clone())),
            crate::parser::ast::Value::Identifier(name) => {
                match self.env.get(name) {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError::UndefinedVariable(name.clone())),
                }
            }
        }
    }

    pub fn eval_binary(&self, left: Value, op: &str, right: Value) -> Result<Value, RuntimeError> {
        match op {
            "+" => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
                _ => Err(RuntimeError::TypeMismatch("Invalid types for +".to_string())),
            },

            "-" => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
                _ => Err(RuntimeError::TypeMismatch("Invalid types for -".to_string())),
            },

            "*" => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
                _ => Err(RuntimeError::TypeMismatch("Invalid types for *".to_string())),
            },

            "/" => match (left, right) {
                (Value::Int(_), Value::Int(0)) => Err(RuntimeError::DivisionByZero),
                (Value::Float(_), Value::Float(b)) if b == 0.0 => Err(RuntimeError::DivisionByZero),

                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),

                _ => Err(RuntimeError::TypeMismatch("Invalid types for /".to_string())),
            },

            "==" => Ok(Value::Bool(left == right)),
            "!=" => Ok(Value::Bool(left != right)),

            ">" => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)),
                _ => Err(RuntimeError::TypeMismatch("Invalid types for >".to_string())),
            },

            "<" => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)),
                _ => Err(RuntimeError::TypeMismatch("Invalid types for <".to_string())),
            },

            _ => Err(RuntimeError::TypeMismatch(format!("Unknown operator {}", op))),
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Value(v) => self.eval_value(v),

            Expr::Binary { left, operator, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary(left_val, operator, right_val)
            }
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Let(name, expr) => {

                let value = self.eval_expr(expr)?;
                self.env.set(name.clone(), value);
                Ok(())
            }

            Stmt::Assign(name, expr) => {
                let value = self.eval_expr(expr)?;
                self.env.assign(name, value)?;
                Ok(())
            }

            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    pub fn eval_program(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }
}