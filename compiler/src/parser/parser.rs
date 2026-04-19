use crate::lexer::token::{Token, TokenType, TimeUnit, Keyword};
use crate::parser::ast::*;

pub struct Parser {
pub tokens: Vec<Token>,
pub position: usize,
}

impl Parser {
pub fn new(tokens: Vec<Token>) -> Parser {
Parser { tokens, position: 0 }
}

pub fn peek(&self) -> &Token {
    &self.tokens[self.position.min(self.tokens.len() - 1)]
}

pub fn advance(&mut self) -> &Token {
    let token = &self.tokens[self.position];
    if self.position < self.tokens.len() - 1 {
        self.position += 1;
    }
    token
}

pub fn expect_identifier(&mut self) -> Result<&Token, String> {
    let token = self.advance();
    match &token.token_type {
        TokenType::Identifier(_) => Ok(token),
        _ => Err(format!(
            "Expected Identifier but found {:?} at line {}",
            token.token_type, token.line
        )),
    }
}

pub fn expect_keyword(&mut self, expected: &str) -> Result<&Token, String> {
    let token = self.advance();
    match &token.token_type {
        TokenType::Keyword(_)
            if token.value.to_uppercase() == expected.to_uppercase() => Ok(token),
        _ => Err(format!(
            "Expected keyword '{}' but found {:?} at line {}",
            expected, token.token_type, token.line
        )),
    }
}

pub fn expect_separator(&mut self) -> Result<&Token, String> {
    let token = self.advance();
    match &token.token_type {
        TokenType::Separator(_) => Ok(token),
        _ => Err(format!(
            "Expected separator but found {:?} at line {}",
            token.token_type, token.line
        )),
    }
}

pub fn parse(&mut self) -> Result<FeatureProgram, String> {
    let mut features = Vec::new();

    while self.position < self.tokens.len()
        && self.peek().token_type != TokenType::EOF
    {
        if self.is_feature_keyword() {
            let feature = self.parse_feature()?;
            features.push(feature);
        } else {
            self.advance();
        }
    }

    Ok(FeatureProgram { feature: features })
}

pub fn is_feature_keyword(&self) -> bool {
    matches!(&self.peek().token_type, TokenType::Keyword(_))
        && self.peek().value.to_uppercase() == "FEATURE"
}

pub fn parse_window(&mut self) -> Result<Window, String> {
    self.expect_separator()?; 

    match &self.peek().token_type {
        TokenType::Duration(n, unit) => {
            let num = *n;
            let unit = unit.clone();
            self.advance();

            Ok(Window {
                duration: Some((num, unit)),
            })
        }
        _ => Err(format!(
            "Expected duration after ':' but found {:?} at line {}",
            self.peek().token_type,
            self.peek().line
        )),
    }
}

pub fn parse_aggregation(&mut self) -> Result<Aggregation, String> {
    self.expect_separator()?; 

    let token = self.advance();

    let function = match token.value.to_uppercase().as_str() {
        "SUM" => AggregationFunc::Sum,
        "AVG" => AggregationFunc::Avg,
        "MIN" => AggregationFunc::Min,
        "MAX" => AggregationFunc::Max,
        "COUNT" => AggregationFunc::Count,
        _ => {
            return Err(format!(
                "Unknown aggregation '{}' at line {}",
                token.value, token.line
            ))
        }
    };

    Ok(Aggregation { function })
}

pub fn parse_filter(&mut self) -> Result<Filter, String> {
    self.expect_separator()?;
    let left_token = self.expect_identifier()?;
    let left = match &left_token.token_type {
        TokenType::Identifier(name) => Identifier(name.clone()),
        _ => unreachable!(),
    };

    let op_token = self.advance();
    let operator = match &op_token.token_type {
        TokenType::ComparisonOperator(op) => ComparisonOperator(op.clone()),
        _ => {
            return Err(format!(
                "Expected comparison operator but found {:?} at line {}",
                op_token.token_type, op_token.line
            ))
        }
    };

    let right_token = self.advance();
    let right = match &right_token.token_type {
        TokenType::Integer(n) => Value::Integer(*n),
        TokenType::Float(f) => Value::Float(**f),
        TokenType::Identifier(id) => Value::Identifier(id.clone()),
        _ => {
            return Err(format!(
                "Expected value but found {:?} at line {}",
                right_token.token_type, right_token.line
            ))
        }
    };

    Ok(Filter {
        expression: Expression { left, operator, right },
    })
}

pub fn parse_feature(&mut self) -> Result<Feature, String> {
    self.expect_keyword("feature")?;

    let name_token = self.expect_identifier()?;
    let name = match &name_token.token_type {
        TokenType::Identifier(val) => val.clone(),
        _ => unreachable!(),
    };

    self.expect_keyword("from")?;

    let source_token = self.expect_identifier()?;
    let source = match &source_token.token_type {
        TokenType::Identifier(val) => val.clone(),
        _ => unreachable!(),
    };

    let mut window = None;
    let mut aggregation = None;
    let mut filter = None;

    loop {
        match &self.peek().token_type {
            TokenType::Keyword(_) if self.peek().value.to_uppercase() == "WINDOW" => {
                self.advance();
                window = Some(self.parse_window()?);
            }

            TokenType::Keyword(_) if self.peek().value.to_uppercase() == "AGGREGATION" => {
                self.advance();
                aggregation = Some(self.parse_aggregation()?);
            }

            TokenType::Keyword(_) if self.peek().value.to_uppercase() == "FILTER" => {
                self.advance();
                filter = Some(self.parse_filter()?);
            }

            _ => break,
        }
    }

    Ok(Feature {
        name,
        source,
        window,
        aggregation,
        filter,
    })
}

}
