use crate::runtime::value::Value;
use crate::runtime::environment::Environment;
use crate::runtime::error::RuntimeError;
use crate::parser::ast::Value as AstValue;

pub struct Evaluator {
    pub env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn eval_value(&self, value: &AstValue) -> Result<Value, RuntimeError> {
        match value {
            AstValue::Integer(i) => Ok(Value::Int(*i)),
            AstValue::Float(f) => Ok(Value::Int(*f as i64)), // simple for now
            AstValue::Text(s) => Ok(Value::String(s.clone())),
            AstValue::Identifier(name) => {
                match self.env.get(name) {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError::UndefinedVariable(name.clone())),
                }
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
            (Value::Int(a), Value::Int(b)) => {
                match op {
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
                        "Unsupported operator '{}'", op
                    ))),
                }
            }

            (Value::String(a), Value::String(b)) => {
                match op {
                    "==" => Ok(Value::Bool(a == b)),
                    "!=" => Ok(Value::Bool(a != b)),
                    _ => Err(RuntimeError::TypeMismatch(format!(
                        "Invalid string operation '{}'", op
                    ))),
                }
            }

            _ => Err(RuntimeError::TypeMismatch(
                "Mismatched types in binary operation".to_string(),
            )),
        }
    }
}