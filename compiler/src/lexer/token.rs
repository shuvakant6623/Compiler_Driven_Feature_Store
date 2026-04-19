use ordered_float::OrderedFloat;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
pub token_type: TokenType,
pub value: String,
pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum TokenType {
Keyword(Keyword),
Duration(i64, TimeUnit),
Identifier(String),
Integer(i64),
ComparisonOperator(String),
MathOperator(char),
Separator(char),
LParen(char),
RParen(char),
Float(OrderedFloat<f64>),
Error(String),
EOF,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
Structural,
Property,
Aggregation,
Connector,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimeUnit {
Milliseconds,
Seconds,
Minutes,
Hours,
Days,
Weeks,
Months,
Years,
}

pub fn unit_to_str(u: &TimeUnit) -> &'static str {
match u {
TimeUnit::Milliseconds => "ms",
TimeUnit::Seconds => "s",
TimeUnit::Minutes => "m",
TimeUnit::Hours => "h",
TimeUnit::Days => "d",
TimeUnit::Weeks => "w",
TimeUnit::Months => "mo",
TimeUnit::Years => "y",
}
}
