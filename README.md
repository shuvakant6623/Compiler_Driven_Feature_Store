# Compiler Driven Feature Store

A Rust-based compiler-style prototype for parsing, validating, and evaluating a feature-definition DSL.

---

## Overview

**Compiler Driven Feature Store** is an experimental project that treats feature engineering like a compiler problem.

Instead of hardcoding feature logic in application code, features are described in a small domain-specific language (DSL) and then processed through a compiler pipeline:

* **Lexing**: break the input into tokens
* **Parsing**: build an AST
* **Semantic analysis**: validate meaning, types, and references
* **Runtime evaluation**: execute valid expressions

The goal is to make feature definitions more structured, reusable, and easier to validate before execution.

---

## Why this project exists

In many data and ML systems, feature logic becomes:

* duplicated across services
* hard to validate
* difficult to maintain
* inconsistent between definition and execution

This project explores a compiler-inspired approach to reduce those problems by turning feature logic into a well-defined language pipeline.

---

## Core ideas

The project is built around these principles:

* feature definitions should be declarative
* parsing should produce a structured tree
* semantic errors should be caught early
* feature expressions should be evaluated consistently
* compiler concepts can improve ML feature workflows

---

## Repository layout

```text
Compiler_Driven_Feature_Store/
├── compiler/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── lexer/
│       ├── parser/
│       ├── semantic/
│       └── runtime/
└── notes/
    ├── How_Compilers_Think.md
    ├── Lexer_design.md
    ├── Parser_and_AST.md
    └── semantic_analyzer.md
```

---

## Architecture

The system follows a classic compiler-style flow:

```
Feature DSL Input
        ↓
      Lexer
        ↓
      Parser
        ↓
       AST
        ↓
Semantic Analyzer
        ↓
      Runtime
```

---

## 1. Lexer

The lexer converts raw DSL text into tokens.

It supports tokenization for:

* identifiers
* keywords
* numbers
* strings
* operators
* delimiters
* end-of-file markers

It follows the **maximal munch rule**, meaning it matches the longest valid token first.

---

## 2. Parser

The parser converts tokens into an **AST (Abstract Syntax Tree)**.

The AST represents feature definitions in a structured form so later stages can reason about them reliably.

---

## 3. Semantic Analyzer

The semantic analyzer ensures the program is meaningful.

Typical checks include:

* whether referenced variables exist
* whether types are compatible
* whether expressions are valid
* whether feature dependencies make sense
* whether source and target fields are valid

---

## 4. Runtime

The runtime evaluates expressions after semantic validation succeeds.

Currently, it uses a mock environment to test and validate filter expressions.

---

## DSL examples

### Valid examples

```
feature clicks from events
feature user_activity from logs
window: 7d

feature total_sales from orders
aggregation: sum

feature high_value from transactions
filter: amount > 1000

feature user_spend from transactions
window: 30d
aggregation: sum
filter: amount > 100

feature avg_rating from reviews
aggregation: avg

feature low_balance from accounts
filter: balance < 500
```

---

### Invalid / edge-case examples

```
feature broken_feature transactions
feature bad_feature from transactions
filter: unknown_field > 10

feature weird from table
filter: amount > "hello"
```

---

## What the current Rust program does

The `compiler/src/main.rs` file demonstrates the full pipeline:

* builds a list of DSL test cases
* tokenizes input using the lexer
* parses tokens into a program
* runs semantic checks
* prints semantic errors
* evaluates valid expressions in a runtime environment

It also seeds mock runtime values like:

* `amount`
* `price`
* `balance`
* `value`

This allows testing expressions such as:

```
amount > 1000
balance < 500
price > 99.99
```

---

## Project goals

Future directions include:

* richer DSL support
* stronger semantic validation
* feature dependency handling
* derived features
* improved AST design
* execution planning
* feature storage integration
* compiler-style optimization

---

## Design notes

See the `notes/` directory:

* `How_Compilers_Think.md`
* `Lexer_design.md`
* `Parser_and_AST.md`
* `semantic_analyzer.md`

---

## Tech stack

* **Language**: Rust
* **Edition**: 2024
* **Dependency**: ordered-float

---

## Getting started

### Prerequisites

* Rust installed
* Cargo available

Install Rust:

```
https://www.rust-lang.org/tools/install
```

---

### Build the project

```bash
cd compiler
cargo build
```

---

### Run the project

```bash
cd compiler
cargo run
```

---

### Check compilation only

```bash
cd compiler
cargo check
```

---

## Expected output

When run, the program prints:

* tokenization results
* parse success or errors
* semantic validation errors
* runtime evaluation results

---

## Limitations

This is still experimental:

* DSL is limited
* schema handling is mocked
* runtime is basic
* no production feature storage
* optimizations are conceptual

---

## Future improvements

* extend DSL syntax
* richer AST nodes
* dependency resolution
* cycle detection
* better type inference
* real schema integration
* persistent feature storage
* improved runtime error handling
* comprehensive tests

---

## Contribution

Good starting points:

* lexer improvements
* parser extensions
* semantic rules
* runtime enhancements
* documentation
* testing

---

## License

No license file is currently included.
Treat as **all rights reserved** until a license is added.

---

## Author

**Shuvakant Patra**
