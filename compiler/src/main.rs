mod lexer;
mod parser;

use crate::lexer::lexer::Lexer;
use parser::Parser;

fn main() {
    let tests = vec![
        // 1. Minimal
        "feature clicks from events",

        // 2. Window only
        "feature user_activity from logs\n window: 7d",

        // 3. Aggregation only
        "feature total_sales from orders\n aggregation: sum",

        // 4. Filter only
        "feature high_value from transactions\n filter: amount > 1000",

        // 5. Full pipeline
        "feature user_spend from transactions\n window: 30d\n aggregation: sum\n filter: amount > 100",

        // 6. Different aggregation
        "feature avg_rating from reviews\n aggregation: avg",

        // 7. Different operator
        "feature low_balance from accounts\n filter: balance < 500",

        // 8. Float filter
        "feature premium_users from subscriptions\n filter: price > 99.99",

        // 9. Multiple features
        "feature f1 from t1\n window: 7d\n\
         feature f2 from t2\n aggregation: count\n\
         feature f3 from t3\n filter: value > 10",

        // 10. Invalid (should fail)
        "feature broken_feature transactions",
    ];

    for (i, input) in tests.iter().enumerate() {
        println!("\n==============================");
        println!("Test Case {}:", i + 1);
        println!("Input:\n{}\n", input);

        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                println!("✅ Parsed successfully!");
                println!("Feature count: {}", program.feature.len());
            }
            Err(e) => {
                println!("❌ Parse error: {}", e);
            }
        }
    }

    println!("\n🚀 All test cases executed!");
}