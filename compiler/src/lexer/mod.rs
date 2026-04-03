struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
}

enum TokenType {
    StructuralKeyword(String),
    PropertyKeyword(String),
    AggregationKeyword(String),
    ConnectorKeyword(String),
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

enum TimeUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

struct Lexer {
    input: String,
    position: usize,
    line: usize,
    tokens: Vec<Token>,
    peek_position: usize,
}

pub fn read_identifier(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

pub fn read_number(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            tokens: Vec::new(),
            peek_position: if input.len() > 1 { 1 } else { 0 },
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char == '\n' {
                self.line += 1;
                self.position += 1;
            } else if current_char.is_whitespace() {
                self.position += 1;
            } else if current_char.is_alphabetic() {
                let identifier = self.read_identifier();
                let token_type = self.lookup_keyword(&identifier);
                tokens.push(Token {
                    token_type,
                    value: identifier,
                    line: self.line,
                });
            } else if current_char.is_ascii_digit() {
                let number = self.read_number();
                tokens.push(Token {
                    token_type: TokenType::Integer(number),
                    value: number.to_string(),
                    line: self.line,
                });
            } else if ":".contains(current_char) {
                tokens.push(Token {
                    token_type : TokenType::Separator(current_char),
                    value: current_char.to_string(),
                    line: self.line,
                });
            } else if ">,<,=,<=, >=, <<, >>, <<=, >>=, !=".contains(current_char) {
                let operator = self.read_comparision_operator();
                tokens.push(Token {
                    token_type : TokenType::ComparisonOperator(operator.clone()),
                    value: operator,
                    line: self.line,
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::Error(format!("Unexpected character: {}", current_char)),
                    value: current_char.to_string(),
                    line: self.line,
                });
                self.position += 1;
            }
        }
        tokens
    }
}