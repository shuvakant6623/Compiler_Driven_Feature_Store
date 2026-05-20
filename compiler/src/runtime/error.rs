use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    UndefinedVariable(String),
    DivisionByZero,
    TypeMismatch(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable '{}'", name)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            RuntimeError::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            }
        }
    }
}