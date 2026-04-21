#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable,
}

#[derive(Debug, Clone)] 
pub struct Symbol {
    pub name: String, 
    pub kind: SymbolKind,
    pub ty: Type,
}