use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
}

#[derive(Clone, Debug)]
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
    Float(f64),
    Error(String),
    EOF,
}

#[derive(Clone, Debug)]
pub enum Keyword {
    Structural,
    Property,
    Aggregation,
    Connector,
}

#[derive(Clone, Debug)]
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

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    keyword_map: HashMap<&'static str, Keyword>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        Lexer {
            input: chars,
            position: 0,
            line: 1,
            keyword_map: Self::build_keyword_map(),
        }
    }

    pub fn build_keyword_map() -> HashMap<&'static str, Keyword> {
        let mut m = HashMap::new();

        m.insert("FEATURE", Keyword::Structural);
        m.insert("FROM", Keyword::Structural);
        m.insert("JOIN", Keyword::Structural);
        m.insert("ON", Keyword::Structural);

        m.insert("WINDOW", Keyword::Property);
        m.insert("GROUP_BY", Keyword::Property);
        m.insert("AGGREGATION", Keyword::Property);
        m.insert("FILTER", Keyword::Property);

        m.insert("SUM", Keyword::Aggregation);
        m.insert("COUNT", Keyword::Aggregation);
        m.insert("AVG", Keyword::Aggregation);
        m.insert("MIN", Keyword::Aggregation);
        m.insert("MAX", Keyword::Aggregation);

        m.insert("AND", Keyword::Connector);
        m.insert("OR", Keyword::Connector);
        m.insert("NOT", Keyword::Connector);

        m
    }

    pub fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            self.position += 1;
        }
        self.input[start..self.position].iter().collect()
    }

    pub fn read_number(&mut self) -> Result<i64, String> {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let s: String = self.input[start..self.position].iter().collect();
        s.parse::<i64>().map_err(|_| format!("Invalid integer: {}", s))
    }

    pub fn read_float(&mut self, int_part: i64) -> Result<f64, String> {
        self.position += 1;
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let frac: String = self.input[start..self.position].iter().collect();
        let combined = format!("{}.{}", int_part, frac);
        combined.parse::<f64>().map_err(|_| format!("Invalid float: {}", combined))
    }

    pub fn read_duration_or_number(&mut self) -> TokenType {
        let number = match self.read_number() {
            Ok(n) => n,
            Err(e) => return TokenType::Error(e),
        };

        if self.position < self.input.len() {
            if self.input[self.position] == '.' {
                return match self.read_float(number) {
                    Ok(f) => TokenType::Float(f),
                    Err(e) => TokenType::Error(e),
                };
            }

            let start = self.position;
            while self.position < self.input.len() && self.input[self.position].is_alphabetic() {
                self.position += 1;
            }

            if start != self.position {
                let unit_str: String = self.input[start..self.position].iter().collect();
                let unit = match unit_str.as_str() {
                    "ms" => Some(TimeUnit::Milliseconds),
                    "s" => Some(TimeUnit::Seconds),
                    "m" => Some(TimeUnit::Minutes),
                    "h" => Some(TimeUnit::Hours),
                    "d" => Some(TimeUnit::Days),
                    "w" => Some(TimeUnit::Weeks),
                    "mo" => Some(TimeUnit::Months),
                    "y" => Some(TimeUnit::Years),
                    _ => None,
                };

                if let Some(u) = unit {
                    return TokenType::Duration(number, u);
                } else {
                    return TokenType::Error(format!("Invalid duration unit: {}", unit_str));
                }
            }
        }

        TokenType::Integer(number)
    }

    pub fn read_comparison_operator(&mut self) -> String {
        let first = self.input[self.position];
        let op = first.to_string();

        if self.position + 1 < self.input.len() {
            let second = self.input[self.position + 1];
            let two = format!("{}{}", first, second);

            let valid = ["==", "!=", ">=", "<="];

            if valid.contains(&two.as_str()) {
                self.position += 2;
                return two;
            }
        }

        self.position += 1;
        op
    }

    pub fn lookup_keyword(&self, identifier: &str) -> TokenType {
        let upper = identifier.to_uppercase();
        if let Some(k) = self.keyword_map.get(upper.as_str()) {
            TokenType::Keyword(k.clone())
        } else {
            TokenType::Identifier(identifier.to_string())
        }
    }

    pub fn check_separator(&self, c: char) 

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let c = self.input[self.position];

            if c == '\n' {
                self.line += 1;
                self.position += 1;
            } else if c.is_whitespace() {
                self.position += 1;
            } else if c.is_alphabetic() {
                let id = self.read_identifier();
                let t = self.lookup_keyword(&id);
                tokens.push(Token {
                    token_type: t,
                    value: id,
                    line: self.line,
                });
            } else if c.is_ascii_digit() {
                let t = self.read_duration_or_number();
                let value = match &t {
                    TokenType::Duration(n, u) => format!("{}{}", n, unit_to_str(u)),
                    TokenType::Integer(n) => n.to_string(),
                    TokenType::Float(f) => f.to_string(),
                    TokenType::Error(e) => e.clone(),
                    _ => "".to_string(),
                };
                tokens.push(Token {
                    token_type: t,
                    value,
                    line: self.line,
                });
            } else if matches!(c, '>' | '<' | '=' | '!') {
                let op = self.read_comparison_operator();
                tokens.push(Token {
                    token_type: TokenType::ComparisonOperator(op.clone()),
                    value: op,
                    line: self.line,
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::Error(format!("Unexpected character: {}", c)),
                    value: c.to_string(),
                    line: self.line,
                });
                self.position += 1;
            }
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            value: "".to_string(),
            line: self.line,
        });

        tokens
    }
}