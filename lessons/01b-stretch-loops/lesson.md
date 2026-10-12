---
id: rust-01b-stretch-loops
track: rust
title: Loops that return values
kind: stretch
for: rust-01-bindings-expressions
topic: Loops & integers
minutes: 20
concepts: [expressions, let-mut, integer-types, loops]
prereqs: [rust-01-bindings-expressions]
contrast: java
edit: src/lib.rs
---

# Loops that return values

## Recall

A harder one, for when *Bindings and expressions* felt easy.

Rust has three loops: `for x in iter`, `while cond`, and `loop`. Unlike in Java, `loop` is an expression: `break value;` makes the whole loop evaluate to `value`.

```run
let mut n = 1;
let first_big = loop {
    n *= 3;
    if n > 100 {
        break n;
    }
};
println!("first power of 3 above 100: {first_big}");
```

Integer overflow **panics** in debug builds (`attempt to multiply with overflow`) instead of silently wrapping as in Java. Choose types wide enough, here `u64`.

## Quick check

```predict
let mut steps = 0;
let mut n = 6;
while n != 1 {
    n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    steps += 1;
}
println!("{steps}");
---
options: 6 | 8 | 16
answer: 8
why: 6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 is eight steps (the Collatz sequence).
```

## Your turn

Three functions: `collatz_steps`, `digit_sum` and `next_power_of_two`. Use `loop` with `break value` at least once.

## Example

```
collatz_steps(6)          →  8
digit_sum(1234)           →  10
next_power_of_two(5)      →  8
```

## Hints

1. `collatz_steps`: shadow `n` as mutable (`let mut n = n;`) and count until it's 1.
2. `digit_sum`: add `n % 10`, then `n /= 10`, while `n > 0`.
3. `next_power_of_two`: start `p` at 1 and `loop { if p >= n { break p; } p *= 2; }`.
