use crate::lexer::token::TimeUnit;

pub struct FeatureProgram {
    pub feature: Vec<Feature>,
}

pub struct Feature {
    pub name: String,
    pub source: String,
    pub window: Option<Window>,
    pub aggregation: Option<Aggregation>,
    pub filter: Option<Filter>,
}

pub struct Window {
    pub duration: Option<(i64, TimeUnit)>,
}

pub struct Aggregation {
    pub function: AggregationFunc,
}

pub struct Filter {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Identifier(String),
}

pub struct Expression {
    pub left: Identifier,
    pub operator: ComparisonOperator,
    pub right: Value,
}

pub struct Identifier(pub String);
pub struct ComparisonOperator(pub String);

#[derive(Debug)]
pub enum AggregationFunc {
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),

    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),

    Assign(String, Expr),

    ExprStmt(Expr),
}