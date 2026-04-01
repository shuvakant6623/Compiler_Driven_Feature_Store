# Semantic Analyzer Notes (my understanding)

## 1) What semantic analyzer does
Parser checks syntax.  
Semantic analyzer checks if the DSL actually makes sense.

It validates things like:
- source exists or not
- target column exists or not
- aggregation is valid for that type or not
- required fields are present
- feature-to-feature references are valid

So simple way:  
**parser = grammar check**  
**semantic analyzer = meaning check**

---

## 2) Visitor pattern (how it walks AST)
Analyzer visits nodes one by one and runs checks at each node.

Typical order I think:
1. Program
2. Each feature
3. Inside feature: source -> window -> aggregation -> target

Traversal idea:

```text
Program
 ├─ Feature(user_spend)
 │   ├─ Source(transactions)
 │   ├─ Window(30d)
 │   ├─ Aggregation(sum)
 │   └─ Target(amount)
 ├─ Feature(merchant_count)
 │   ├─ Source(transactions)
 │   ├─ Window(7d)
 │   ├─ Aggregation(count)
 │   └─ Target(amount)
 └─ Feature(fraud_score)
     ├─ Source(user_spend)
     ├─ Window(7d)
     └─ Aggregation(avg)
```

---

## 3) Symbol table design

### What it stores
- data sources and their columns/types
- features and their output types
- aggregation rules (what input type is allowed)
- optional dependency info

### Structure (simple)
```text
GlobalSymbolTable
├─ Sources
│  └─ transactions
│     ├─ amount: number
│     └─ ...
├─ Features
│  ├─ user_spend -> number
│  ├─ merchant_count -> integer
│  └─ fraud_score -> number
└─ Aggregations
   ├─ sum(number) -> number
   ├─ avg(number) -> number
   └─ count(any) -> integer
```

### Why needed
Without symbol table, analyzer cannot resolve names and types properly.
It is needed for:
- resolving `from transactions`
- checking `target amount`
- resolving derived feature source like `from user_spend`
- validating aggregation type compatibility

---

## 4) Validation checks (complete list)

### Program level
- duplicate feature names
- build dependency graph between features
- detect circular dependencies
- ensure validation order is correct (topological order)

### Feature level
- feature name valid and unique
- source is present
- required properties exist
- illegal combinations rejected

### Source check
- source exists as raw table OR existing feature
- if feature source, upstream feature should be valid

### Window check
- format valid (7d, 24h etc.)
- value > 0
- allowed unit only

### Aggregation check
- aggregation keyword supported
- valid for input type
- valid for source kind (raw/derived based on DSL rules)

### Target check
- for raw source: target column required (for sum/avg etc.)
- target column must exist
- target type must match aggregation
- for derived source: target behavior should follow DSL policy (usually not raw column-based)

---

## 5) Cross-node dependencies

Main dependencies:
- Aggregation -> Target type
- Target -> Source schema
- Source kind (raw/derived) -> allowed fields
- Upstream feature output type -> downstream aggregation validity
- Feature dependencies -> validation order
- Upstream validity -> downstream validity

Also yes, possible policy dependency:
- Aggregation -> Window

---

## 6) derived feature validation

Given:
- `user_spend` from `transactions`
- `fraud_score` from `user_spend` (derived feature)

New checks needed for `fraud_score` (not needed in basic raw feature case):

1. `user_spend` must exist as a feature symbol  
2. add dependency edge `fraud_score -> user_spend`  
3. detect cycles in feature graph  
4. validate in topological order  
5. if upstream invalid, downstream should fail too  
6. use upstream output type as input for `avg`  
7. enforce rule for aggregation over derived feature output  
8. window compatibility rule between upstream and downstream  
9. lineage tracking (`fraud_score -> user_spend -> transactions`)  
10. better error messages with upstream context

---

## 7) Schema registry design (compile-time types)

### Problem
Analyzer needs column types at compile time, but real schema is in external DB.

### Approaches

1. **Manual registration**  
   User writes schema in config/DSL.  
   Easy to start, but can become stale if DB changes.

2. **Automatic inference**  
   System connects to DB and fetches schema automatically.  
   Requires DB access at compile time.

### Production choice (hybrid)
Best practical choice is hybrid:
- auto-infer when DB is accessible
- manual schema declaration as fallback
- drift detection to catch stale manual schema