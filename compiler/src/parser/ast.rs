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
