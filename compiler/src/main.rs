mod lexer;
use lexer::{Lexer, Token};

fn main() {
    let input = String::from(
        "feature user_spend from transactions\n
         window: 30d\n
         aggregation: sum\n
         filter: amount > 100"
    );
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    
    for token in &tokens {
        println!("{:?}", token);
    }
}