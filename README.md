# 🚀 Compiler Driven Feature Store

<p align="center">
  <b>A compiler-inspired system for defining, validating, and executing feature engineering logic using a custom DSL.</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2024-orange" />
  <img src="https://img.shields.io/badge/Architecture-Compiler--Driven-blue" />
  <img src="https://img.shields.io/badge/Status-Experimental-yellow" />
</p>

---

## 📚 Table of Contents

* [Problem Statement](#problem-statement)
* [Solution](#solution)
* [Core Ideas](#core-ideas)
* [Key Features](#key-features)
* [Architecture](#architecture)
* [Execution Pipeline](#execution-pipeline)
* [DSL Examples](#dsl-examples)
* [Technology Stack](#technology-stack)
* [Project Structure](#project-structure)
* [Component Breakdown](#component-breakdown)
* [Design Decisions](#design-decisions)
* [Runtime Behavior](#runtime-behavior)
* [Limitations](#limitations)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Author](#author)

---

## 🧩 Problem Statement

In modern data and machine learning systems, feature engineering logic is often:

* duplicated across training and inference pipelines
* tightly coupled with application code
* difficult to validate before execution
* inconsistent across environments

This results in:

* fragile pipelines
* silent logical errors
* high maintenance overhead
* lack of reusability

Most systems treat feature logic as *code*, rather than as a **first-class, structured representation**.

---

## 💡 Solution

This project introduces a **compiler-driven approach to feature engineering**.

Instead of writing feature logic imperatively in code, features are defined declaratively using a **Domain-Specific Language (DSL)** and processed through a compiler-style pipeline:

* **Lexing** → Token generation
* **Parsing** → AST construction
* **Semantic Analysis** → Validation
* **Runtime Execution** → Evaluation

This allows feature logic to be:

* validated before execution
* structured and reusable
* decoupled from application code
* consistently interpreted

---

## 🧠 Core Ideas

* Feature definitions should be **declarative, not imperative**
* DSL → AST enables **structured reasoning**
* Semantic analysis should catch **errors early**
* Execution should be **deterministic and consistent**
* Compiler principles can improve **ML infrastructure design**

---

## ⚙️ Key Features

| Feature               | Description                                  |
| --------------------- | -------------------------------------------- |
| **Custom DSL**        | Defines features declaratively               |
| **Lexer**             | Tokenizes input using maximal munch strategy |
| **Parser**            | Builds structured AST representation         |
| **Semantic Analyzer** | Validates types, references, and logic       |
| **Runtime Engine**    | Executes validated expressions               |
| **Mock Environment**  | Enables isolated testing of expressions      |

---

## 🏗️ Architecture

```text
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

## 🔄 Execution Pipeline

### 1. Lexing

* Converts raw DSL input into tokens
* Supports identifiers, keywords, numbers, strings, operators

### 2. Parsing

* Converts tokens into an **Abstract Syntax Tree (AST)**
* Represents feature definitions structurally

### 3. Semantic Analysis

* Validates:

  * variable existence
  * type compatibility
  * expression correctness
  * logical consistency

### 4. Runtime Execution

* Evaluates expressions only if semantic checks pass
* Uses a mock environment for testing

---

## 🧪 DSL Examples

### ✅ Valid

```dsl
feature user_spend from transactions
window: 30d
aggregation: sum
filter: amount > 100
```

### ❌ Invalid

```dsl
feature bad_feature from transactions
filter: unknown_field > 10
```

---

## 🛠️ Technology Stack

| Layer        | Technology          |
| ------------ | ------------------- |
| Language     | Rust (2024 Edition) |
| Architecture | Compiler Design     |
| Runtime      | Custom Evaluator    |
| Dependency   | ordered-float       |

---

## 📂 Project Structure

```text
Compiler_Driven_Feature_Store/
├── compiler/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lexer/
│   │   ├── parser/
│   │   ├── semantic/
│   │   └── runtime/
└── notes/
```

---

## 🔍 Component Breakdown

### Lexer (`lexer/`)

* Responsible for token generation
* Implements longest-match (maximal munch) strategy

### Parser (`parser/`)

* Builds AST from token stream
* Defines grammar for feature DSL

### Semantic Analyzer (`semantic/`)

* Validates logical correctness
* Detects undefined variables and type mismatches

### Runtime (`runtime/`)

* Evaluates expressions
* Executes only validated AST

---

## ⚙️ Runtime Behavior

The runtime operates on a **mock environment**, which includes predefined variables such as:

```text
amount
price
balance
value
```

Example evaluated expressions:

```text
amount > 1000
balance < 500
price > 99.99
```

---

## 🧠 Design Decisions

**Compiler-first architecture**
Treating feature logic as a language enables structured validation and extensibility.

**AST as central representation**
All transformations and validations operate on a unified structure.

**Separation of concerns**
Each stage (lexer, parser, semantic, runtime) is modular and independent.

**Early validation over runtime errors**
Errors are caught before execution to improve reliability.

**Mock runtime environment**
Allows isolated testing without dependency on real data systems.

---

## ⚠️ Limitations

* DSL is currently minimal
* No persistent feature storage
* Runtime execution is basic
* No optimization layer
* No dependency graph handling yet

---

## 🔮 Roadmap

* Feature dependency graph
* Cycle detection in feature definitions
* Type inference system
* Query planning and execution engine
* Persistent feature storage
* Compiler optimizations (caching, reuse)

---

## 🤝 Contributing

Contributions are welcome in:

* Lexer improvements
* Parser extensions
* Semantic validation rules
* Runtime enhancements
* Documentation

---

## 📜 License

This project is licensed under the MIT License.
See the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

**Shuvakant Patra**

---

<p align="center">
  ⭐ If you found this interesting, consider starring the repository!
</p>
