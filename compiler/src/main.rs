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
            peek_position: 1,
        }
    }

    fn tokenize(&mut self) {
        
    }
}