mod lexer;
mod parser;
mod semantic;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::ast::Value;
use crate::semantic::analyzer::Analyzer;
use crate::semantic::symbol_table::Type;
use crate::semantic::errors::SemanticError;

fn main() {
    let tests = vec![
        // ----------------------
        // VALID CASES
        // ----------------------
        "feature clicks from events",

        "feature user_activity from logs\n window: 7d",

        "feature total_sales from orders\n aggregation: sum",

        "feature high_value from transactions\n filter: amount > 1000",

        "feature user_spend from transactions\n window: 30d\n aggregation: sum\n filter: amount > 100",

        "feature avg_rating from reviews\n aggregation: avg",

        "feature low_balance from accounts\n filter: balance < 500",

        "feature premium_users from subscriptions\n filter: price > 99.99",

        "feature f1 from t1\n window: 7d\n\
         feature f2 from t2\n aggregation: count\n\
         feature f3 from t3\n filter: value > 10",

        // ----------------------
        // PARSE ERROR
        // ----------------------
        "feature broken_feature transactions",

        // ----------------------
        // SEMANTIC ERRORS
        // ----------------------
        "feature bad_feature from transactions\n filter: unknown_field > 10",

        "feature weird from table\n filter: amount > \"hello\"",
    ];

    for (i, input) in tests.iter().enumerate() {
        println!("\n==============================");
        println!("Test Case {}:", i + 1);
        println!("Input:\n{}\n", input);

        // ----------------------
        // LEXER
        // ----------------------
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();

        // ----------------------
        // PARSER
        // ----------------------
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                println!("✅ Parsed successfully!");
                println!("Feature count: {}", program.feature.len());

                // ----------------------
                // SEMANTIC ANALYSIS
                // ----------------------
                let mut analyzer = Analyzer::new();

                // Simulated schema (columns)
                analyzer.declare_variable("amount", Type::Int);
                analyzer.declare_variable("price", Type::Float);
                analyzer.declare_variable("balance", Type::Int);
                analyzer.declare_variable("value", Type::Int);

                for feature in &program.feature {
                    println!("→ Analyzing feature: {}", feature.name);

                    if let Some(filter) = &feature.filter {
                        let left = &filter.expression.left;   // String
                        let right = &filter.expression.right;
                        let left_ty = analyzer.get_variable_type(&left.0);

                        let right_ty = match right {
                            Value::Integer(_) => Type::Int,
                            Value::Float(_) => Type::Float,
                            Value::Text(_) => Type::Unknown, 
                            Value::Identifier(name) => analyzer.get_variable_type(name),
                        };

                        if left_ty != right_ty {
                            analyzer.errors.push(SemanticError::new(
                                &format!(
                                    "Type mismatch: {:?} vs {:?} in '{} op {:?}'",
                                    left_ty, right_ty, left.0, right
                                ),
                            ));
                        }
                    }
                }

                // ----------------------
                // PRINT ERRORS
                // ----------------------
                analyzer.print_errors();
            }

            Err(e) => {
                println!("❌ Parse error: {}", e);
            }
        }
    }

    println!("\n🚀 All test cases executed!");
}