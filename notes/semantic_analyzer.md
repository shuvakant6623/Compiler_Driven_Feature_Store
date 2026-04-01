# Semantic Analyzer Notes (My Understanding)

## What a Semantic Analyzer Does (my mental model)
Parser only checks syntax (shape of the DSL).
Semantic analyzer checks meaning.

So it answers:
- Does this source exist?
- Is this column valid for that source?
- Is aggregation valid for that target type?
- Are required fields present together?
- Are cross-feature rules valid?

In short: **parser says “valid sentence”, semantic analyzer says “valid idea”.**

---

## Visitor Pattern (how analyzer walks tree)
I use Visitor to walk each AST node and run checks node-by-node.

Typical order:

1. Program node
2. Each FeatureDeclaration
3. Inside feature:
   - source
   - window
   - aggregation
   - target
   - optional filters / groupings (if DSL has them)

Traversal sketch:

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
     ├─ Source(user_spend)   // derived source
     ├─ Window(7d)
     └─ Aggregation(avg)
```

---

## Symbol Table Design

### What it stores
- **Data sources** (raw tables): name + schema
- **Features**: name + output type + metadata
- **Columns per source**: column name -> type
- **Built-in aggregations** and supported input/output types
- Optional: scope, diagnostics locations

### Structure sketch

```text
GlobalSymbolTable
├─ Sources
│  └─ transactions
│     ├─ amount: number
│     ├─ merchant_id: string
│     └─ ...
├─ Features
│  ├─ user_spend
│  │  ├─ source: transactions
│  │  ├─ output_type: number
│  │  └─ window: 30d
│  ├─ merchant_count
│  │  └─ output_type: number
│  └─ fraud_score
│     └─ output_type: number (derived)
└─ AggregationRules
   ├─ sum(number) -> number
   ├─ avg(number) -> number
   └─ count(any) -> integer
```

### Answer to Question 2
A symbol table is needed so semantic checks can resolve names and types quickly:
- resolve `from transactions`
- resolve `target amount` in that source
- resolve `from user_spend` as feature symbol (derived)
- verify aggregation/type compatibility

Without symbol table, analyzer can’t do consistent cross-node validation.

---

## Complete Validation Checks (by node type)

### Program
- duplicate feature names
- dependency graph build
- cycle detection between features
- topological order for validation (base before derived)

### FeatureDeclaration
- feature name not empty / not duplicate
- source exists (raw source OR existing feature)
- required properties present for feature kind
- no forbidden property combos

### SourceReference (`from ...`)
- raw source exists in catalog **or**
- referenced feature exists
- if feature source: referenced feature already semantically valid (or sortable via topo pass)

### Window
- format valid (`7d`, `24h`, etc.)
- positive, non-zero
- unit allowed
- if derived-feature rules exist: derived window constraints checked

### Aggregation
- aggregation keyword exists
- aggregation allowed for source kind (raw vs derived)
- input type compatibility
- output type inferred correctly

### Target
- required for raw-source aggregations like `sum/avg/min/max` (depending DSL)
- column exists in chosen raw source
- column type compatible with aggregation
- for derived source, target may be forbidden or remapped to feature output (DSL rule dependent)

### Filters / GroupBy (if present)
- referenced columns exist
- predicate types valid
- group keys valid and compatible with aggregation semantics

---

## Cross Node Dependencies

Main dependencies found:

- **Aggregation -> Target type**
- **Target -> Source schema**
- **Source kind (raw/derived) -> allowed fields**
- **Feature output type -> downstream feature aggregation validity**
- **Window -> source grain / feature grain compatibility**
- **Program-level feature graph -> validation order**
- **Feature name -> symbol resolution in later nodes**

You already noted:
- Aggregation -> Window (possible policy dependency)

Other important ones:
- Source -> Target
- Source -> Aggregation
- Upstream Feature Output Type -> Downstream Aggregation
- Upstream Feature Validity -> Downstream Feature Validity

---

## Derived Feature Problem (Question 3)

Given:

- `user_spend` from raw `transactions`
- `fraud_score` from **feature** `user_spend`

What *new* checks are needed for `fraud_score` (not needed for `user_spend`)?

### New checks for derived feature (`fraud_score`)

1. **Source is feature check**
   - `user_spend` must exist as a feature symbol (not just table lookup)

2. **Dependency graph registration**
   - add edge `fraud_score -> user_spend`

3. **Cycle detection**
   - ensure no loop like `user_spend -> fraud_score -> user_spend`

4. **Topological validation order**
   - `user_spend` must be validated before `fraud_score`

5. **Upstream validity gate**
   - if `user_spend` has semantic errors, block/mark `fraud_score` invalid too

6. **Derived input type check**
   - use `user_spend` output type as input to `avg`
   - verify `avg(number)` is valid

7. **Derived schema model check**
   - derived feature usually has single value output, not raw columns
   - if DSL allows `target` on derived source, define meaning strictly; otherwise forbid it

8. **Window compatibility across levels**
   - validate policy like:
     - downstream window <= upstream window? or
     - allow any? or
     - require explicit resampling rule
   - This did not exist in plain raw-source feature checks

9. **Aggregation-on-aggregation policy**
   - decide whether re-aggregating already aggregated features is allowed
   - e.g., `avg(sum(...))` can be semantically risky without grain definition

10. **Lineage tracking**
   - store lineage `fraud_score -> user_spend -> transactions` for explainability/debugging

11. **Materialization/readiness constraints (if runtime-aware)**
   - upstream feature availability/refresh expectations before downstream compute

12. **Error propagation quality**
   - diagnostics should report both local error and upstream dependency context

So the key shift is:  
for raw features you validate against **table schema**,  
for derived features you also validate against a **feature dependency graph + upstream feature contracts**.