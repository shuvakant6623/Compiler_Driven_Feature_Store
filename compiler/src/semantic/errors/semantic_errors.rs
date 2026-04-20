#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
}

impl SemanticError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}