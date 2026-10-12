---
id: rust-02-convert-temperature
track: rust
title: Convert a temperature
topic: Numbers & bindings
minutes: 8
concepts: [let-mut, numbers, shadowing]
prereqs: [rust-01-first-function]
contrast: java
edit: src/lib.rs
---

# Convert a temperature

## Learn the idea

Your thermostats report Celsius; some users want Fahrenheit.

- **Bindings are immutable by default.** `let x = 1;` is like Java's `final`. Write `let mut x` if it must change.
- **Number types are explicit and never mix silently.** `f64` is Java's `double`; `i32` is `int`. `21 * 9 / 5` is **integer** division (37, not 37.8), exactly as in Java, and `f64 * i32` doesn't even compile. Use float literals: `9.0 / 5.0`.
- **Rounding:** `x.round()` rounds to a whole number, so `(x * 10.0).round() / 10.0` rounds to one decimal and stays an `f64`.
- **Shadowing:** `let x = …` again makes a *new* binding with the same name, even of a new type.

```visual
{
 "title": "Follow the number",
 "regions": [
  {
   "id": "vars",
   "label": "Variables"
  },
  {
   "id": "calc",
   "label": "Working it out"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Convert and round",
   "group": "How it works",
   "code": "let c: f64 = 21.46;\nlet raw = c * 9.0 / 5.0 + 32.0;\nlet rounded = (raw * 10.0).round() / 10.0;\nprintln!(\"{rounded}\");",
   "check": {
    "output": "70.6"
   },
   "steps": [
    {
     "line": 1,
     "say": "`c` is an `f64`. The annotation matters: a bare `21.46` has no exact type yet, and `.round()` needs one (E0689).",
     "rule": "Immutable by default: `c` can't change.",
     "set": [
      {
       "id": "c",
       "in": "vars",
       "title": "c",
       "value": "21.46",
       "tag": "f64",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "Float arithmetic all the way: 21.46 × 9.0 / 5.0 = 38.628.",
     "set": [
      {
       "id": "w1",
       "in": "calc",
       "title": "c * 9.0 / 5.0",
       "value": "38.628",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "c",
       "to": "w1",
       "state": "muted"
      }
     ]
    },
    {
     "line": 2,
     "say": "+ 32.0 gives `raw` = 70.628.",
     "set": [
      {
       "id": "w1",
       "state": "faded"
      },
      {
       "id": "raw",
       "in": "vars",
       "title": "raw",
       "value": "70.628",
       "tag": "f64",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "w1",
       "to": "raw",
       "label": "+ 32.0",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "× 10.0, then `.round()`: 706.28 → 706.0.",
     "rule": "`round()` returns an `f64`: still a float.",
     "set": [
      {
       "id": "w2",
       "in": "calc",
       "title": "(raw * 10.0).round()",
       "value": "706.0",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "raw",
       "to": "w2",
       "state": "muted"
      }
     ]
    },
    {
     "line": 3,
     "say": "/ 10.0 gives 70.6.",
     "set": [
      {
       "id": "w2",
       "state": "faded"
      },
      {
       "id": "r",
       "in": "vars",
       "title": "rounded",
       "value": "70.6",
       "tag": "f64",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "w2",
       "to": "r",
       "label": "/ 10.0",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "Printed: 70.6.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "70.6"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "Shadowing",
   "group": "How it works",
   "code": "let reading = \" 21.5 \";\nlet reading = reading.trim();\nlet reading: f64 = reading.parse().unwrap();\nprintln!(\"{}\", reading + 1.0);",
   "check": {
    "output": "22.5"
   },
   "steps": [
    {
     "line": 1,
     "say": "Text from a sensor, with spaces.",
     "set": [
      {
       "id": "r1",
       "in": "vars",
       "title": "reading",
       "value": "\" 21.5 \"",
       "tag": "&str",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "A **new** `reading` shadows the old one: same name, new binding.",
     "rule": "Shadowing is not mutation: each `let` makes a new variable.",
     "set": [
      {
       "id": "r1",
       "state": "faded",
       "tag": "shadowed"
      },
      {
       "id": "r2",
       "in": "vars",
       "title": "reading",
       "value": "\"21.5\"",
       "tag": "&str",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "And again with a different **type**, `f64`. `mut` could never change the type.",
     "rule": "Idiomatic for parse-and-convert steps.",
     "set": [
      {
       "id": "r2",
       "state": "faded",
       "tag": "shadowed"
      },
      {
       "id": "r3",
       "in": "vars",
       "title": "reading",
       "value": "21.5",
       "tag": "f64",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "Now arithmetic works.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "22.5"
      }
     ]
    }
   ]
  },
  {
   "name": "Integer division",
   "group": "What if it goes wrong",
   "code": "let c: i32 = 21;\nlet f = c * 9 / 5 + 32;\nprintln!(\"{f}\");",
   "check": {
    "output": "69"
   },
   "steps": [
    {
     "line": 1,
     "say": "`c` is an integer.",
     "set": [
      {
       "id": "c",
       "in": "vars",
       "title": "c",
       "value": "21",
       "tag": "i32",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "21 × 9 = 189, and 189 / 5 is **37**: integer division drops the .8.",
     "rule": "Integers divide to integers. Use `f64` for measurements.",
     "set": [
      {
       "id": "w",
       "in": "calc",
       "title": "c * 9 / 5",
       "value": "37",
       "tag": "was 37.8",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "c",
       "to": "w",
       "state": "bad"
      }
     ]
    },
    {
     "line": 3,
     "say": "The answer is 69 instead of 69.8, and nothing warns you.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "69",
       "state": "bad"
      }
     ],
     "error": "69 instead of 69.8: the fraction was lost"
    }
   ]
  },
  {
   "name": "Change an immutable binding",
   "group": "What if it goes wrong",
   "code": "let reading = 21.5;\nreading = 22.0;",
   "check": {
    "error": "E0384"
   },
   "steps": [
    {
     "line": 1,
     "say": "`reading` has no `mut`.",
     "set": [
      {
       "id": "r",
       "in": "vars",
       "title": "reading",
       "value": "21.5",
       "tag": "f64",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "Assigning a new value is refused at compile time.",
     "rule": "Add `mut` if it really must change; otherwise, make a new binding.",
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
  }
 ]
}
```

```run
let c: f64 = 21.46;
let raw = c * 9.0 / 5.0 + 32.0;
let rounded = (raw * 10.0).round() / 10.0;
println!("{raw} → {rounded}");
println!("integer division: {}", 21 * 9 / 5);
```

## Quick check

```predict
println!("{} {}", 7 / 2, 7.0 / 2.0);
---
options: 3 3.5 | 3.5 3.5 | 3 3
answer: 3 3.5
why: Integers divide to integers (like Java); floats keep the fraction.
```

## Your turn

Write `to_fahrenheit(celsius)`. Use F = C × 9/5 + 32, and round the result to **one decimal place**.

## Example

```
to_fahrenheit(21.5)  →  70.7
to_fahrenheit(100.0) →  212.0
```

## Hints

1. Compute the raw value with float literals: `celsius * 9.0 / 5.0 + 32.0`.
2. Round to one decimal: multiply by `10.0`, `.round()`, divide by `10.0`.
3. You can shadow: `let f = …;` then `let f = (f * 10.0).round() / 10.0;` and end with `f`.

## Review

Tomorrow, solve this again, then write `to_celsius` without looking.
