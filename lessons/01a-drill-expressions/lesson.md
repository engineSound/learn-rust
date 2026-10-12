---
id: rust-01a-drill-expressions
track: rust
title: Expressions and shadowing, again
kind: drill
for: rust-01-bindings-expressions
minutes: 10
concepts: [expressions, shadowing]
prereqs: [rust-01-bindings-expressions]
contrast: java
edit: src/lib.rs
---

# Drill: expressions and shadowing

A short drill on the ideas from *Bindings and expressions*, with new code so
you recall the ideas rather than the old solution.

Remember:
- `if` is an expression: `let x = if c { a } else { b };`
- The last expression in a block, **without** `;`, is its value. A stray `;`
  gives `E0308` (expected a value, found `()`).
- Shadowing (`let s = s.trim();`) makes a new binding, possibly of a new type.

## Exercise
Three functions in `src/lib.rs`. Aim to finish in under 10 minutes.
