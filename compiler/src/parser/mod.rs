mod lexer;
use lexer::TimeUnit;
use lexer::Keyword;
use lexer::TokenType;

struct FeatureProgram {
    feature: Vec<Feature>,
}

struct Feature {
    name: String,
    source: String,
    window: Option<Window>,
    aggregation: Option<Aggregation>,
    filter: Option<Filter>,
}

struct Window {
    duration: Option<(i64, TimeUnit)>,
}

struct Aggregation {
    function: Keyword,
}

struct Filter {
    expression: Expression,
}

enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Identifier(String),
}

struct Expression {
    left: Identifier,
    operator: ComparisonOperator,
    right: Value,
}

struct Identifier(String);
struct ComparisonOperator(String);

