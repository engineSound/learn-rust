---
id: rust-01b-stretch-loops
track: rust
title: Loops that return values
kind: stretch
for: rust-01-bindings-expressions
minutes: 20
concepts: [expressions, let-mut, integer-types, loops]
prereqs: [rust-01-bindings-expressions]
contrast: java
edit: src/lib.rs
---

# Stretch: loops that return values

A harder exercise for when lesson 01 felt easy.

Rust has three loops: `for x in iter`, `while cond`, and `loop`. Unlike in
Java, `loop` is an expression: `break value;` makes the whole loop evaluate
to `value`.

```rust
let mut n = 1;
let first_big = loop {
    n *= 3;
    if n > 100 { break n; }
};   // first_big == 243
```

Integer overflow panics in debug builds (`attempt to multiply with overflow`)
instead of silently wrapping as in Java. Choose types wide enough, here `u64`.

## Exercise
Three functions in `src/lib.rs`. Use `loop` with `break value` at least once.
