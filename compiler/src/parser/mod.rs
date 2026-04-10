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
    duration: Option<i64, TimeUnit>,
}

struct Aggregation {
    function: keyword::Aggregation,
}

struct Filter {
    expression: {
        left: Identifier(String),
        operator: ComparisonOperator(String),
        right: Value(String),
    },
}