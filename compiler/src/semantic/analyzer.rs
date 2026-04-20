use crate::semantic::symbol_table::{SymbolTable, Symbol, SymbolKind, Type};
use crate::semantic::errors::SemanticError;

pub struct Analyzer {
    symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: vec![],
        }
    }

    pub fn declare_variable(&mut self, name: &str, var_type: Type) {
        if self.symbol_table.lookup(name).is_some() {
            self.errors.push(SemanticError::new(&format!(
                "Variable '{}' is already declared",
                name
            )));
        } else {
            let symbol = Symbol {
                name: name.to_string(),
                kind: SymbolKind::Variable,
                var_type,
            };
            self.symbol_table.insert(symbol);
        }
    }

    pub fn get_variable_type(&mut self, name: &str) -> Type {
        if let Some(symbol) = self.symbol_table.lookup(name) {
            if let SymbolKind::Variable = symbol.kind {
                return symbol.var_type.clone();
            } else {
                self.errors.push(SemanticError::new(&format!(
                    "'{}' is not a variable",
                    name
                )));
            }
        } else {
            self.errors.push(SemanticError::new(&format!(
                "Variable '{}' is not declared",
                name
            )));
        }
        Type::Unknown
    }

    pub fn check_binary(&mut self, left: &str, right: &str) -> Type {
        let left_type = self.get_variable_type(left);
        let right_type = self.get_variable_type(right);

        if left_type == right_type {
            left_type
        } else {
            self.errors.push(SemanticError::new(&format!(
                "Type mismatch: '{}' is {:?} but '{}' is {:?}",
                left, left_type, right, right_type
            )));
            Type::Unknown
        }
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            println!("Semantic Error: {}", error.message);
        }
    }
}