---
id: rust-01-bindings-expressions
track: rust
title: Bindings and expressions
topic: Bindings & expressions
minutes: 20
concepts: [cargo, let-mut, shadowing, expressions, integer-types]
prereqs: []
contrast: java
edit: src/lib.rs
---

# Bindings and expressions

## Learn the idea

You're writing the firmware helpers for a small fleet of home devices. Rust starts with two habits that differ from Java.

**1. Everything is immutable unless you say `mut`.** `let x = 1;` is like Java's `final int x = 1;`. To change it: `let mut x = 1; x += 1;`

**2. `if` and blocks are expressions.** They produce values, like Java's `cond ? a : b`. The last line of a block, **without a semicolon**, is its value, so functions don't need `return`:

```rust
fn square(x: i32) -> i32 {
    x * x          // no `;` → this is the return value
}
```

Add a `;` after `x * x` and the function returns `()` (nothing). The compiler then reports **E0308**, mismatched types. It's the most common beginner error, so expect it.

Change the reading below and press **Run**:

```run
let reading = 23;
let status = if reading > 25 { "hot" } else { "fine" };
println!("{reading}°C is {status}");

let mut count = 0;
count += 1;
println!("readings so far: {count}");
```

**Shadowing:** you can declare a *new* binding with the same name, even with a new type, which is handy for parse-and-convert steps: `let s = s.trim(); let s: i64 = s.parse().unwrap();`

## Quick check

```predict
let x = 5;
let x = x * 2;
{
    let x = 0;
}
println!("{x}");
---
options: 0 | 5 | 10
answer: 10
why: The second `let x` shadows the first (10). The `let x = 0` inside the braces is a separate binding that disappears at the `}`.
```

## Your turn

Four small device helpers in the editor: convert Celsius to Fahrenheit, classify a number, sum the even numbers up to `n`, and parse-and-double a text value. Each `///` comment says exactly what to return. Replace each `todo!()`.

## Example

```
celsius_to_fahrenheit(100.0)  →  212.0
classify(-7)                  →  "negative"
sum_of_evens(10)              →  30
parse_and_double(" 21 ")      →  42
```

## Hints

1. `celsius_to_fahrenheit` is one expression: `c * 9.0 / 5.0 + 32.0`, with no `;`.
2. `classify`: `if n < 0 { "negative" } else if n == 0 { "zero" } else { "positive" }`. The whole `if` is the return value.
3. `sum_of_evens`: `let mut total: u64 = 0;`, loop `for i in 0..=n`, add `i as u64` when `i % 2 == 0`, and end with `total`.

## Review

Tomorrow, solve this again, then add a `;` after one return expression on purpose and read what E0308 says.
