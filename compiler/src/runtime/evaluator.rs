use crate::runtime::value::Value;
use crate::runtime::environment::Environment;
use crate::runtime::error::RuntimeError;
use crate::parser::ast::{Value as AstValue, Stmt};

pub struct Evaluator {
    pub env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn eval_value(&mut self, value: &AstValue) -> Result<Value, RuntimeError> {
        match value {
            AstValue::Integer(i) => Ok(Value::Int(*i)),

            AstValue::Float(_) => Err(RuntimeError::TypeMismatch(
                "Float not supported yet".to_string(),
            )),

            AstValue::Text(s) => Ok(Value::String(s.clone())),

            AstValue::Identifier(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or(RuntimeError::UndefinedVariable(name.clone()))
            }
        }
    }

    pub fn eval_binary(
        &mut self,
        left: Value,
        op: &str,
        right: Value,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => match op {
                "+" => Ok(Value::Int(a + b)),
                "-" => Ok(Value::Int(a - b)),
                "*" => Ok(Value::Int(a * b)),
                "/" => {
                    if b == 0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Int(a / b))
                    }
                }
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),
                ">" => Ok(Value::Bool(a > b)),
                "<" => Ok(Value::Bool(a < b)),

                _ => Err(RuntimeError::TypeMismatch(format!(
                    "Unsupported operator '{}'",
                    op
                ))),
            },

            (Value::String(a), Value::String(b)) => match op {
                "==" => Ok(Value::Bool(a == b)),
                "!=" => Ok(Value::Bool(a != b)),

                _ => Err(RuntimeError::TypeMismatch(format!(
                    "Invalid string operation '{}'",
                    op
                ))),
            },

            _ => Err(RuntimeError::TypeMismatch(
                "Mismatched types in binary operation".to_string(),
            )),
        }
    }

    pub fn eval_stmt(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            // let x = value;
            Stmt::Let(name, expr) => {
                let value = self.eval_value(&expr)?;
                self.env.set(name, value);   
                Ok(())
            }

            // x = value;
            Stmt::Assign(name, expr) => {
                let value = self.eval_value(&expr)?;
                self.env.assign(name, value)?; 
                Ok(())
            }

            // expression statement (like: x + 5)
            Stmt::Expr(expr) => {
                self.eval_value(&expr)?;  // just evaluate
                Ok(())
            }
        }
    }
}