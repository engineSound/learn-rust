---
id: rust-01-bindings-expressions
track: rust
title: Bindings and expressions
minutes: 20
concepts: [cargo, let-mut, shadowing, expressions, integer-types]
prereqs: []
contrast: java
---

# Bindings and expressions

## Coming from Java

| Java | Rust |
|------|------|
| `int x = 1;` (mutable) | `let x = 1;` (**immutable** by default) |
| `int x = 1; x++;` | `let mut x = 1; x += 1;` |
| `final` | the default; `mut` is the exception you opt into |
| `int`, `long` | `i32`, `i64`, `u32`, `usize`… sizes are explicit, no silent widening |
| `cond ? a : b` | `if cond { a } else { b }`, since `if` is an expression |
| `return x;` | the last expression in a block, **without** `;`, is its value |
| `mvn test` / `gradle test` | `cargo test` |

## Expressions, not statements
Blocks and `if` produce values:

```rust
let sign = if n < 0 { "negative" } else { "non-negative" };
fn square(x: i32) -> i32 { x * x }   // no `return`, no `;`
```

Add a `;` after `x * x` and the function returns `()` (nothing). The
compiler then reports a type mismatch, `E0308`. That's one of the most
common beginner errors, so expect to see it.

## Shadowing
You can declare a new binding with the same name, even with a new type:

```rust
let input = " 42 ";
let input = input.trim();              // still &str
let input: i64 = input.parse().unwrap(); // now an i64
```

This differs from `mut`: each `let` creates a new variable. It's idiomatic
for parse-and-convert steps.

## No implicit numeric conversion
`let total: u64 = some_u32;` doesn't compile. Convert explicitly:
`some_u32 as u64` or `u64::from(some_u32)`.

## Exercise
Open `src/lib.rs`, replace each `todo!()`, and run `cargo test` in this
folder. Read the compiler's messages fully; they usually tell you the fix.
