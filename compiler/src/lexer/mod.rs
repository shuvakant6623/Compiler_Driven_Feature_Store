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
    Duration(String),
    Identifier(String),
    Integer(i64),
    ComparisionOperator(String),
    MathOperator(char),
    Separator(char),
    LParen(char),
    RParen(char),
    Float(f64),
    Error(String),
    EOF,
}

struct Lexer {
    input: String,
    position: usize,
    line: usize,
    tokens: Vec<Token>,
    peek_position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            tokens: Vec::new(),
            if input.len() > 1 {
                peek_position: 1
            } else { 
                peek_position: 0,
            }
        }
    }

    fn tokenize(&mut self) {
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_cahr == '\n' {
                self.line += 1;
                self.position += 1;;
            } else if current_char.is_whitespace() {
                self.position += 1;
            } else if current_char.isalphabetic() {
                let identifier = self.read_identifier();
                let token_type = self.lookup_keyword(&identifier);
                self.tokens.push(Token {
                    token_type,
                    value: identifier,
                    line: self.line,
                });
            } else if current_char.is_digit(10) {
                let number = self.read_number();
                self.tokens.push(Token {
                    token_type: TokenType::Integer(number),
                    value: number.to_string(),
                    line: self.line,
                });
            } else {
                self.tokens.push(Token {
                    token_type: TokenType::Error(format!("Unexpected character: {}", current_char)),
                    value: current_char.to_string(),
                    line: self.line,
                });
                self.position += 1;
            }
        }
    }
}