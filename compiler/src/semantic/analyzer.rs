use crate::semantic::symbol_table::{SymbolTable, Symbol, SymbolKind, Type};
use crate::semantic::errors::SemanticError;
use crate::parser::ast::Identifier;

pub struct Analyzer {
    pub symbol_table: SymbolTable,
    pub errors: Vec<SemanticError>,
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
                ty: var_type,
            };
            self.symbol_table.insert(symbol);
        }
    }

    pub fn get_variable_type(&mut self, ident: &Identifier) -> Type {
        let name = &ident.0;

        match self.symbol_table.lookup(name) {
            Some(sym) => sym.ty.clone(),
            None => {
                self.errors.push(
                    SemanticError::new(&format!("Variable '{}' not declared", name))
                );
                Type::Unknown
            }
        }
    }

    pub fn check_binary(&mut self, left: &Identifier, right: &Identifier) -> Type {
        let left_type = self.get_variable_type(&left.0);
        let right_type = self.get_variable_type(&right.0);

        if left_type == right_type {
            left_type
        } else {
            self.errors.push(SemanticError::new(&format!(
                "Type mismatch: '{}' is {:?} but '{}' is {:?}",
                left.0, left_type, right.0, right_type
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