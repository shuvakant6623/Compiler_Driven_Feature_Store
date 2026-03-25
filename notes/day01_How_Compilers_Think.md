# How a Compiler Thinks (My Understanding)

## Introduction

This README is my understanding after reading the introduction of *Crafting Interpreters*.

Initially, I used to think compilers and interpreters are something very complex or “magical”.  
But now I realize:

> There is no magic. It’s just code, broken into small understandable steps.

---

## Why Learn Compilers?

From what I understood:

- Most people think building a programming language is rare and only experts do it
- But in reality, **small languages (DSLs)** are everywhere:
  - Config files
  - Template engines
  - Query languages
  - Scripting systems

So learning compilers is not useless — it is actually **very practical**

---

## Key Mindset Shift

Earlier thinking ❌:
- Compiler = Magic
- Only geniuses can build it

Now thinking ✅:
- Compiler = Set of transformations
- Anyone can build it by learning step-by-step

---

## How a Compiler Actually Works (My Mental Model)

A compiler doesn’t directly understand code.

It processes code in stages:

```
Code → Tokens → Structure → Meaning → Execution
```


Breaking it down:

### 1. Tokenization
- Break code into small pieces
- Example:
```
int x = 10;
```
becomes:
```
int | x | = | 10 | ;
```

---

### 2. Parsing
- Convert tokens into a structure (tree)
- This is where code gets “shape”

---

### 3. Understanding (Semantics)
- Check if code makes sense:
- Types
- Variables
- Scope

---

### 4. Execution / Translation
- Either:
- Run directly (Interpreter)
- Convert to machine code (Compiler)

---

## Interpreter vs Compiler (What I Understood)

| Interpreter | Compiler |
|------------|--------|
| Executes code step-by-step | Translates whole code first |
| Easier to build | Faster execution |
| Example: Python | Example: C++ |

---

## Important Insight

The book emphasizes:

> You learn compilers by building them, not by just reading theory.

This is very important.

- Theory alone is confusing
- Implementation builds real understanding

---

## Skills You Build

While building a compiler, you learn:

- Recursion
- Trees & Graphs
- Data structures
- Memory handling
- System-level thinking

So it’s like **full brain training for a programmer**

---

## Two Approaches (From the Book)

The book teaches:

### 1. High-Level Interpreter (Java)
- Focus on understanding
- Simple and clean
- Uses existing runtime

### 2. Low-Level Interpreter + Compiler (C)
- Focus on performance
- Build everything from scratch:
- Memory
- Data structures
- Bytecode

---

## Biggest Takeaway

What I understood most clearly:

> Compilers are not hard because of complexity,  
> they feel hard because they involve many small parts together.

If we break them down:
- Each part is simple
- Together they form a powerful system

---

## My Current Mental Model
```
Input Code
↓
Tokenizer
↓
Parser (Tree)
↓
Semantic Checks
↓
Execution / Machine Code
```

---

## My Learning Approach (Going Forward)

Based on this:

- I will not treat compilers as theory
- I will build:
  - Lexer
  - Parser
  - AST
  - Execution engine

Step-by-step.

---

### 1. Why Compilers Exist

A compiler is not just a translator. It is a **verification system** that ensures correctness *before execution*.

It prevents:

* Invalid structure (syntax errors)
* Invalid meaning (semantic errors)
* Some classes of unsafe behavior

👉 Key realization:

> A compiler reduces uncertainty before runtime.

---

### 2. Syntax vs Semantic Errors

#### 🔴 Syntax Errors

* Violations of grammar rules
* Prevent AST formation

**Example (DSL):**

```
feature user_spend from transactions
  window 30d
  aggregation: sum
```

❌ Missing ':' → cannot parse

---

#### 🔵 Semantic Errors

* Structure is valid, but meaning is invalid
* AST exists but fails during analysis

**Example (DSL):**

```
feature user_spend from transactions
  window: 30d
  aggregation: sum
```

❌ Sum of what column?

---

### 3. Why This Project Exists

This project is not just about building a DSL.
It is about building a **compiler that guarantees correctness of ML features before execution**.

Traditional systems fail because:

* Errors are detected at runtime
* Silent data issues corrupt models
* Training-serving skew goes unnoticed

👉 Goal of this system:

> Catch all critical errors at compile time

---

### 4. The 3 Layers of Errors (Core Mental Model)

#### 🧩 Layer 1 — Syntax

* Grammar correctness
* Token validity
* Structural rules

#### 🧠 Layer 2 — Semantic

* Logical correctness
* Type correctness
* Reference resolution

#### 🌍 Layer 3 — Domain

* Feature store correctness
* ML system guarantees
* Time & data consistency

---

## 🚨 12+ Errors Your Compiler MUST Catch

### 🔴 Syntax Errors

1. Misspelled keywords (`windw` instead of `window`)
2. Missing `:` in key-value pairs
3. Invalid duration format (`30 d`)
4. Wrong indentation / block structure
5. Unknown fields (`windows` instead of `window`)

---

### 🔵 Semantic Errors

6. Undefined data source (`transactions` not registered)
7. Missing aggregation column (`sum` of what?)
8. Invalid aggregation for data type
9. Ambiguous column selection
10. Missing or unresolved feature type
11. Window defined without time column reference

---

### 🌍 Domain Errors

12. Undefined time reference (event vs ingestion vs request time)
13. Missing entity key (user? account?)
14. Training-serving skew risk
15. Missing timestamp in source data
16. Late data handling undefined
17. Null handling not specified
18. Timezone ambiguity

---

## 🎯 Key Mental Shift

From:

> "Will this code run?"

To:

> "Can this system guarantee correctness before execution?"

---

## 🚀 Next Step

Design a **correct and unambiguous DSL version** of:

```
feature user_spend from transactions
  window: 30d
  aggregation: sum
```

👉 Add:

* Explicit column
* Entity key
* Time reference
* Clear semantics

---

## 🧠 Final Insight

> A compiler is not a code translator.
> It is a system that enforces truth before execution.
