## 🧠 Why Compilers Use Trees (and Why It Matters for ML Systems)

A compiler does not use a flat list of tokens — it uses a **tree**.

Because a list loses relationships between tokens.

---

### ❌ Problem with Lists

In a flat list:
- Tokens are placed one after another  
- Relationships are not explicit  
- You have to scan and search to understand structure  

Example:
```
feature aggregation operation sum
```
There is no clear connection between these elements.

---

### ✅ Tree-Based Approach

A tree encodes relationships using **parent → child connections**.

Example:
```
Feature
└── Aggregation
└── Operation: sum
```
Now:
- No searching required  
- Relationships are directly encoded  
- Navigation is straightforward  

---

### ⚡ Performance Insight

- List approach → **O(n)**  
- Tree approach → **O(depth)**  

At scale (e.g., 500+ features):
- Lists become slow and inefficient  
- Trees remain fast and structured  

---

### 🚀 Why This Matters (Feature Stores)

In ML systems:
- Features depend on transformations  
- Transformations depend on operations  

Using a list:
- Complex  
- Hard to debug  
- Not scalable  

Using a tree:
- Deterministic  
- Structured  
- Scalable  

---

### 💡 Key Idea

> Don’t search for relationships — encode them.

This is the difference between writing scripts and building systems.