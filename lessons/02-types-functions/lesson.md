---
id: rust-02-types-functions
track: rust
title: Types, tuples and slices
minutes: 20
concepts: [integer-types, casting, tuples, slices, functions]
prereqs: [rust-01-bindings-expressions]
contrast: java
edit: src/lib.rs
---

# Types, tuples and slices

## Coming from Java

| Java | Rust |
|------|------|
| `int` silently widens to `long` | no implicit conversion: `x as i64` or `i64::from(x)` |
| `(byte) 300` wraps quietly | `300_i32 as u8` also wraps (to 44), so clamp first if that's wrong |
| overloads `max(int, int)`, `max(long, long)` | no overloading: one name, one signature (generics come later) |
| return an array or a small class for two values | return a tuple: `fn f() -> (i32, i32)`, destructure with `let (a, b) = f();` |
| `int[] values` | a slice `&[i32]`: a borrowed view of an array or `Vec` |

## `as` conversions
`as` never fails: it truncates, wraps or saturates (float to int saturates:
`300.7_f64 as u8 == 255`). It's convenient but lossy, so make the intent clear
in your code, for example by clamping first.

## Tuples
```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
let (lo, hi) = min_max(7, 3);
```

## Slices
`&[f64]` lets a function read any run of `f64`s without taking ownership.
Loop with `for v in values { ... }` (each `v` is a `&f64`; use `*v` or let
arithmetic deref it), and get the count with `values.len()`, which is a
`usize`, so convert it before dividing an `f64` by it.

## Exercise
Four functions in `src/lib.rs`.
