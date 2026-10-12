---
id: rust-02a-drill-expressions
track: rust
title: Expressions and shadowing, again
kind: drill
for: rust-02-convert-temperature
topic: Expressions & shadowing
minutes: 10
concepts: [expressions, shadowing]
prereqs: [rust-02-convert-temperature]
contrast: java
edit: src/lib.rs
---

# Expressions and shadowing, again

## Recall

A short practice round on *Convert a temperature*, with new code so you recall the idea rather than an old answer.

- `if` is an expression: `let x = if c { a } else { b };`
- The last expression in a block, **without** `;`, is its value. A stray `;` gives E0308.
- Shadowing (`let s = s.trim();`) makes a new binding, possibly of a new type.

## Quick check

```predict
let level = 42;
let label = if level > 50 { "high" } else { "normal" };
println!("{label}");
---
options: high | normal | 42
answer: normal
why: 42 is not above 50, so the `else` branch's value becomes `label`.
```

## Your turn

Three functions in the editor: `grade`, `distance` and `parse_sum`. Aim to finish in under 10 minutes.

## Example

```
grade(89)            →  'B'
distance(10, 3)      →  7
parse_sum("3, 4 ,5") →  12
```

## Hints

1. `grade` is one `if / else if` chain whose branches are `'A'`, `'B'`, `'C'`, `'F'`.
2. `distance`: `if a > b { a - b } else { b - a }`.
3. `parse_sum`: loop over `s.split(',')`, shadow `let piece = piece.trim();`, then parse.
