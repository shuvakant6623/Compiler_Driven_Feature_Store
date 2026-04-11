mod lexer;
mod parser;
use lexer::{TokenType, Token};
use parser::{Parser, FeatureProgram};

fn main() {
    let input = String::from(
        "feature user_spend from transactions\n  window: 30d\n  aggregation: sum\n  filter: amount > 100"
    );
    
    let mut lex = lexer::Lexer::new(input);
    let tokens = lex.tokenize();
    
    let mut parser = parser::Parser::new(tokens);
    match parser.parse() {
        Ok(program) => println!("Parsed {} features successfully", program.feature.len()),
        Err(e) => println!("Parse error: {}", e),
    }
}