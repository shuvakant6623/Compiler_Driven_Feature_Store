use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    UndefinedVariable(String),
    #[allow(dead_code)]
    InvalidAssignment(String),
    TypeMismatch(String),
    DivisionByZero,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable '{}'", name)
            }
            RuntimeError::InvalidAssignment(name) => {
                write!(f, "Cannot assign to undeclared variable '{}'", name)
            }
            RuntimeError::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
        }
    }
}