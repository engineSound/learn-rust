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

```visual
{
 "title": "Bindings, step by step",
 "regions": [
  {
   "id": "b",
   "label": "Bindings"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "let mut",
   "group": "How it works",
   "code": "let mut count = 0;\ncount += 1;\ncount += 1;\nprintln!(\"{count}\");",
   "check": {
    "output": "2"
   },
   "steps": [
    {
     "line": 1,
     "say": "`mut` says this binding may change.",
     "rule": "Immutable is the default; `mut` is opt-in.",
     "set": [
      {
       "id": "c",
       "in": "b",
       "title": "count",
       "value": "0",
       "tag": "mut i32",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "It changes in place…",
     "set": [
      {
       "id": "c",
       "value": "1",
       "state": "active"
      }
     ]
    },
    {
     "line": 3,
     "say": "…again.",
     "set": [
      {
       "id": "c",
       "value": "2"
      }
     ]
    },
    {
     "line": 4,
     "say": "Printed: 2.",
     "set": [
      {
       "id": "c",
       "state": "ok"
      },
      {
       "id": "o",
       "in": "out",
       "title": "2"
      }
     ]
    }
   ]
  },
  {
   "name": "Shadowing",
   "group": "How it works",
   "code": "let s = \" 42 \";\nlet s = s.trim();\nlet s: i64 = s.parse().unwrap();\nprintln!(\"{}\", s * 2);",
   "check": {
    "output": "84"
   },
   "steps": [
    {
     "line": 1,
     "say": "`s` is a text slice with spaces.",
     "set": [
      {
       "id": "s1",
       "in": "b",
       "title": "s",
       "value": "\" 42 \"",
       "tag": "&str",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "A **new** `s` shadows the old one. The old binding still exists, but its name now points to the new one.",
     "rule": "Shadowing makes a new binding; it isn't mutation.",
     "set": [
      {
       "id": "s1",
       "state": "faded",
       "tag": "shadowed"
      },
      {
       "id": "s2",
       "in": "b",
       "title": "s",
       "value": "\"42\"",
       "tag": "&str",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "And again, now with a different **type**: `i64`. `mut` could never change the type.",
     "rule": "Shadowing is idiomatic for parse-and-convert steps.",
     "set": [
      {
       "id": "s2",
       "state": "faded",
       "tag": "shadowed"
      },
      {
       "id": "s3",
       "in": "b",
       "title": "s",
       "value": "42",
       "tag": "i64",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "Arithmetic works on the number.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "84"
      }
     ]
    }
   ]
  },
  {
   "name": "Assign to immutable",
   "group": "What if it goes wrong",
   "code": "let reading = 23;\nreading = 24;",
   "check": {
    "error": "E0384"
   },
   "steps": [
    {
     "line": 1,
     "say": "`reading` is immutable: no `mut`.",
     "set": [
      {
       "id": "r",
       "in": "b",
       "title": "reading",
       "value": "23",
       "tag": "i32",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "Assigning a new value is refused.",
     "rule": "Add `mut` if it really must change.",
     "set": [
      {
       "id": "r",
       "state": "bad",
       "tag": "immutable!"
      }
     ],
     "error": "E0384: cannot assign twice to immutable variable `reading`"
    }
   ]
  },
  {
   "name": "Stray semicolon",
   "group": "What if it goes wrong",
   "code": "fn square(x: i32) -> i32 {\n    x * x;\n}",
   "check": {
    "error": "E0308"
   },
   "steps": [
    {
     "line": 1,
     "say": "`square` promises to return an `i32`.",
     "set": [
      {
       "id": "f",
       "in": "b",
       "title": "square(x)",
       "value": "-> i32",
       "tag": "promise",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "With `;`, `x * x` is a statement: computed, then thrown away.",
     "rule": "The last expression **without** `;` is the return value.",
     "set": [
      {
       "id": "e",
       "in": "b",
       "title": "x * x;",
       "tag": "discarded",
       "state": "bad"
      }
     ]
    },
    {
     "line": 3,
     "say": "The block returns `()`, not an `i32`.",
     "rule": "Remove the `;` and it compiles.",
     "set": [
      {
       "id": "f",
       "state": "bad",
       "value": "-> ()"
      }
     ],
     "error": "E0308: mismatched types: expected `i32`, found `()`"
    }
   ]
  }
 ]
}
```

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
