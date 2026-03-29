# How a Compiler Thinks (My Understanding)

## Introduction

This README is basically my understanding after reading the introduction of *Crafting Interpreters* and thinking through it myself.

Earlier, I used to feel like compilers/interpreters are some kind of magic thing that only hardcore systems people understand.

But now I think:

> There is no magic. It’s just a bunch of small steps done properly.

---

## Why I Think Learning Compilers Matters

At first I thought:
- “Why would I ever build a compiler?”
- “This is probably useless unless I make a programming language”

But now I feel like that thinking was wrong.

Because small “languages” are literally everywhere:
- Config files
- Query systems
- Feature definitions (this is important for my project)
- Even APIs sometimes behave like DSLs

So yeah, compilers are not rare — they’re just hidden.

---

## My Mindset Shift

Before ❌:
- Compiler = very hard / magical
- Only geniuses build it

Now ✅:
- Compiler = pipeline of transformations
- If I break it, I can build it

---

## How I Currently Think a Compiler Works

Right now, my mental model is something like:

```
Code → Tokens → Structure → Meaning → Execution
```

Not 100% perfect, but this is how I visualize it.

---

### 1. Tokenization

I think of this as:
- Breaking code into small pieces

Example:
```
int x = 20;
```

becomes:
```
int | x | = | 20 | ;
```

Simple but important.

---

### 2. Parsing

Here the compiler gives “shape” to the code.

- Tokens → Tree (AST)
- Now code is structured

---

### 3. Understanding (Semantics)

This part I feel is where things get interesting:

- Does the code actually make sense?
- Are types correct?
- Are variables defined?

---

### 4. Execution / Translation

Finally:
- Either run directly (interpreter)
- Or convert to machine code (compiler)

---

## Interpreter vs Compiler (My Understanding)

| Interpreter | Compiler |
|------------|--------|
| Runs step by step | Translates everything first |
| Easier to build (I think) | More optimized |
| Example: Python | Example: C++ |

---

## One Important Thing I Realized

> You don’t learn compilers by reading… you learn by building.

Theory alone = confusing  
Implementation = clarity

---

## Skills I Think This Builds

While learning this, I feel like I'm also improving in:

- Recursion
- Trees
- Problem decomposition
- System thinking

So it’s not just “compiler”, it’s like full brain training.

---

## Two Ways to Learn (From What I Read)

### 1. High-Level (Java style)
- Easier to understand
- Less control

### 2. Low-Level (C style)
- Harder
- But full control (memory, bytecode, etc.)

---

## My Biggest Takeaway So Far

I think compilers feel hard because:

> They are many simple things combined together.

Individually:
- Tokenization = easy  
- Parsing = manageable  
- Semantic checks = logical  

Together:
- Feels complex

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

Still refining this as I learn.

---

## My Plan Going Forward

Instead of just reading, I want to build:

- Lexer
- Parser
- AST
- Execution engine

Step-by-step, not rushing.

---

## Why Compilers Exist (My Understanding)

I think a compiler is not just a translator.

It’s more like:

> A system that catches problems before execution.

It reduces uncertainty.

---

## Syntax vs Semantic Errors (My View)

### 🔴 Syntax Errors

- Grammar problems
- Code structure is wrong

Example:
```
feature user_spend from transactions
  window 30d
  aggregation: sum
```

I think this fails because `:` is missing.

---

### 🔵 Semantic Errors

- Structure is valid
- But meaning is unclear

Example:
```
feature user_spend from transactions
  window: 30d
  aggregation: sum
```

This looks correct, but I feel like:
→ “sum of what?”

---

## Why I’m Building This Project

This is not just about a DSL.

What I actually want is:

> A system that guarantees ML feature correctness before execution

Because current systems:
- Fail at runtime
- Have silent bugs
- Cause training-serving mismatch

---

## My Mental Model of Errors (3 Layers)

### 🧩 Syntax Layer
- Grammar
- Structure

### 🧠 Semantic Layer
- Logic
- Types
- References

### 🌍 Domain Layer
- ML correctness
- Time consistency
- Data validity

---

## 🚨 Errors I Think Should Be Caught

(Not final, but this is what I think should be included)

### 🔴 Syntax

- Misspelled keywords (like `windw`)
- Missing `:`
- Wrong duration format
- Bad structure / indentation
- Unknown fields

---

### 🔵 Semantic

- Undefined data source
- Aggregation without column (this one feels important)
- Invalid aggregation type
- Ambiguous column usage
- Missing feature type
- Window without time reference

---

### 🌍 Domain

- Time not defined properly (event vs ingestion etc.)
- Missing entity (user/account)
- Possible training-serving skew
- No timestamp in data
- Late data handling unclear
- Null handling missing
- Timezone confusion

---

## Big Mental Shift

Earlier I used to think:

> “Will this run?”

Now I’m trying to think:

> “Can this be guaranteed correct before running?”

---

## Next Step (What I Want to Improve)

Take this:

```
feature user_spend from transactions
  window: 30d
  aggregation: sum
```

And make it:
- Explicit
- Unambiguous
- Fully defined

---

## Final Thought

I think the biggest realization for me is:

> A compiler is not just translating code  
> It is enforcing correctness before execution