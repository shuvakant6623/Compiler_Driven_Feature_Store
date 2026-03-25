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

## 📌 Final Thought

> “There is no wizardry in compilers.  
> Just structured thinking and careful implementation.”

---