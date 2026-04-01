# Semantic Analyzer Notes (my own understanding)

## Visitor Pattern — Traversal Order
I understand Visitor like this: analyzer walks AST node by node and checks meaning at each step.

For my DSL, traversal order looks like:

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

So simple flow is:
1. visit Program
2. visit each Feature
3. visit child nodes (source, window, aggregation, target, filter)
4. collect errors while walking

---

## Symbol Table Design
This is a lookup map the analyzer uses for names and types.

It stores:
- raw sources and column types
- feature definitions and output types
- aggregation rules
- feature dependencies

Structure with real DSL values:

```text
GlobalSymbolTable
├─ Sources
│  └─ transactions
│     ├─ amount: number
│     ├─ user_id: string
│     ├─ merchant_id: string
│     └─ ts: timestamp
├─ Features
│  ├─ user_spend
│  │  ├─ source: transactions
│  │  ├─ window: 30d
│  │  ├─ aggregation: sum
│  │  ├─ target: amount
│  │  └─ output_type: number
│  ├─ merchant_count
│  │  ├─ source: transactions
│  │  ├─ window: 7d
│  │  ├─ aggregation: count
│  │  ├─ target: amount
│  │  └─ output_type: integer
│  └─ fraud_score
│     ├─ source: user_spend
│     ├─ window: 7d
│     ├─ aggregation: avg
│     └─ output_type: number
└─ AggregationRules
   ├─ sum(number) -> number
   ├─ avg(number) -> number
   └─ count(any) -> integer
```

---

## Schema Registry
Problem: analyzer needs schema at compile time, but real data schema is in external DB.

### 1) Manual registration
User defines schema manually.
- pros: simple, no DB dependency
- cons: can become stale if DB changes

### 2) Automatic inference
System connects to DB and pulls schema.
- pros: accurate and updated
- cons: needs DB access during compile/validation

### 3) Hybrid (production choice)
Use both:
- auto-infer when DB is reachable
- manual schema as fallback
- drift detection for stale manual definitions

This is most practical in real systems.

---

## Validation Checks Per Node

### Feature node checks
- feature name exists and is unique
- source exists (raw source or feature)
- required fields are present
- invalid combinations are rejected
- if source is feature, dependency edge is added

### Window node checks
- format valid (like `7d`, `24h`)
- value > 0
- allowed unit only
- optional rule checks against upstream window

### Aggregation node checks
- aggregation keyword supported
- input type compatible
- output type inferred correctly
- allowed for raw/derived source as per DSL rule

### Filter node checks
- field/column exists
- operator valid for type
- literal value type matches field type
- on derived source, only available output bindings can be used

---

## Derived Feature Checks
For derived feature like:

- `user_spend` from `transactions`
- `fraud_score` from `user_spend`

Extra checks needed:

### Forward reference problem
If `fraud_score` appears before `user_spend`, analyzer still must resolve it.
Need two-pass resolution or dependency graph with topological order.

### Window inconsistency
Need rule for upstream vs downstream window compatibility.
Without this, re-windowing semantics can become ambiguous.

### Output binding vs column name
Derived source is feature output, not raw table columns.
Analyzer must bind to upstream output contract, not assume raw column access.

Also needed in practice:
- cycle detection in feature graph
- upstream invalid -> downstream invalid propagation
- clear lineage tracking (`fraud_score -> user_spend -> transactions`)

---

## Explicit Override Pattern
Default should be strict validation.
But sometimes business exceptions are real.

### When to use
- temporary migration
- known exception case
- backward compatibility

### How to use
- require explicit override flag/annotation
- emit warning when override is used
- keep override local and intentional
- never make relaxed behavior default

So my rule: **strict by default, override only explicitly.**