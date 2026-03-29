# Day 02 — Lexer Design Notes

## Complete Token Type List

So today I tried to think what all tokens my lexer actually needs and why.

### 1. Identifiers
Examples: user, age, totalCount  
Used for naming variables and stuff.

---

### 2. Keywords
Examples: if, else, while, return  
These are reserved words. I cannot treat them like normal names.

---

### 3. Numbers
Examples: 10, 3.14, 42  
Needed for values and calculations.

---

### 4. Strings
Examples: "hello", "world"  
Used for text.

---

### 5. Operators
Examples:
- Arithmetic → + - * /
- Comparison → > < >= <= == !=
- Assignment → =

These basically define operations.

---

### 6. Delimiters / Symbols
Examples: ( ) { } , ;  
These help in structuring the code.

---

### 7. Whitespace
Spaces, tabs, newlines  
I don’t store them, but they are important to separate tokens.

---

### 8. EOF (End Of File)
Marks the end so lexer knows when to stop.

---

## Maximal Munch — My Understanding

What I understood is:

Always try to take the **longest valid token** instead of stopping early.

Example:

Input:
>=

If I think badly:
- take >
- then =

But correct way:
- check next char
- >= is a valid token → take it fully

So basically:
Don’t break tokens early if a bigger valid one exists.

---

## Keyword HashMap Design

I decided to use a hashmap to check keywords.

Idea:
Whenever I read a word, I first assume it is an identifier.  
Then I check if it exists in the keyword map.

Structure:

{
  "if"     → TOKEN_IF,
  "else"   → TOKEN_ELSE,
  "while"  → TOKEN_WHILE,
  "return" → TOKEN_RETURN,
  "func"   → TOKEN_FUNC,
  "var"    → TOKEN_VAR
}

Flow:
- Read word
- Check map
- If found → keyword
- Else → identifier

---

## Operator Decision Tree

For operators like > < = ! I need to peek next character.

### >
if current == '>'
    if next == '='
        token = '>='
    else
        token = '>'

---

### <
if current == '<'
    if next == '='
        token = '<='
    else
        token = '<'

---

### =
if current == '='
    if next == '='
        token = '=='
    else
        token = '='

---

### !
if current == '!'
    if next == '='
        token = '!='
    else
        token = '!' (or error)

Main idea:
Always check next character before finalizing.

---

## Hard Cases I Found

### 1. 30d case

Input:
30d

Confusion:
Is it number or identifier?

My decision:
- 30 → number
- d → identifier

Because numbers should not contain letters.

---

### 2. >= vs >

If I don’t peek, I will split it wrong.

So I must always check next char.

---

### 3. Keywords vs Identifiers

Input:
if
iff

- if → keyword
- iff → identifier

So exact match matters.

---

## Final Thought

Today I understood that lexer is not just splitting text.

It’s about making correct decisions.

If I mess up here, everything later (parser etc.) will break.

So better to be slow and clear right now.