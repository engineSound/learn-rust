---
id: rust-02-types-functions
track: rust
title: Types, tuples and slices
topic: Types, tuples & slices
minutes: 20
concepts: [integer-types, casting, tuples, slices, functions]
prereqs: [rust-01-bindings-expressions]
contrast: java
edit: src/lib.rs
---

# Types, tuples and slices

## Learn the idea

Device readings come in many numeric types, and Rust never converts between them silently.

| Java | Rust |
|------|------|
| `int` silently widens to `long` | no implicit conversion: `x as i64` or `i64::from(x)` |
| `(byte) 300` wraps quietly | `300_i32 as u8` also wraps (to 44), so clamp first if that's wrong |
| overloads `max(int, int)`, `max(long, long)` | no overloading: one name, one signature |
| return a small class for two values | return a tuple: `fn f() -> (i32, i32)`, destructure with `let (a, b) = f();` |
| `int[] values` | a slice `&[i32]`: a borrowed view of an array or `Vec` |

```visual
{
 "title": "Casts and slices, visualised",
 "scenarios": [
  {
   "name": "as u8 vs clamp",
   "group": "How it works",
   "regions": [
    {
     "id": "in",
     "label": "i32"
    },
    {
     "id": "cast",
     "label": "Conversion"
    },
    {
     "id": "res",
     "label": "u8 (0–255)"
    }
   ],
   "code": "let raw: i32 = 300;\nlet wrapped = raw as u8;\nlet clamped = raw.clamp(0, 255) as u8;\nprintln!(\"{wrapped} {clamped}\");",
   "check": {
    "output": "44 255"
   },
   "steps": [
    {
     "line": 1,
     "say": "A reading of 300 in an `i32`.",
     "set": [
      {
       "id": "r",
       "in": "in",
       "title": "raw",
       "value": "300",
       "tag": "i32",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`as u8` keeps only the lowest 8 bits: 300 − 256 = 44. No error, no warning.",
     "rule": "`as` never fails: it truncates, wraps or saturates.",
     "set": [
      {
       "id": "c1",
       "in": "cast",
       "title": "as u8",
       "tag": "wraps"
      },
      {
       "id": "w",
       "in": "res",
       "title": "wrapped",
       "value": "44",
       "tag": "surprise!",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "c1",
       "state": "muted"
      },
      {
       "from": "c1",
       "to": "w",
       "state": "bad"
      }
     ]
    },
    {
     "line": 3,
     "say": "Clamping first keeps the value inside 0–255, so the cast is safe.",
     "rule": "Make lossy conversions explicit: clamp, then cast.",
     "set": [
      {
       "id": "c2",
       "in": "cast",
       "title": ".clamp(0, 255) as u8",
       "tag": "safe"
      },
      {
       "id": "cl",
       "in": "res",
       "title": "clamped",
       "value": "255",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "c2",
       "state": "muted"
      },
      {
       "from": "c2",
       "to": "cl",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "Printed: 44 255."
    }
   ]
  },
  {
   "name": "A slice borrows",
   "group": "How it works",
   "regions": [
    {
     "id": "stack",
     "label": "Stack · your variables"
    },
    {
     "id": "heap",
     "label": "Heap · the actual data"
    }
   ],
   "code": "let readings = vec![20.5, 21.0, 22.5];\nlet first_two = &readings[0..2];\nprintln!(\"{:?} {}\", first_two, readings.len());",
   "check": {
    "output": "[20.5, 21.0] 3"
   },
   "steps": [
    {
     "line": 1,
     "say": "A `Vec` owns three readings on the heap.",
     "set": [
      {
       "id": "v",
       "in": "stack",
       "title": "readings",
       "tag": "owner",
       "sub": "Vec<f64>",
       "state": "ok"
      },
      {
       "id": "h",
       "in": "heap",
       "title": "[20.5, 21.0, 22.5]"
      }
     ],
     "arrows": [
      {
       "from": "v",
       "to": "h",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`&readings[0..2]` is a **slice**: a borrowed view of part of the data. Nothing is copied.",
     "rule": "A slice `&[T]` is a pointer plus a length.",
     "set": [
      {
       "id": "s",
       "in": "stack",
       "title": "first_two",
       "tag": "&[f64] · len 2",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "v",
       "to": "h",
       "state": "ok"
      },
      {
       "from": "s",
       "to": "h",
       "label": "[0..2]",
       "state": "accent"
      }
     ]
    },
    {
     "line": 3,
     "say": "Both are usable: the slice only borrowed.",
     "rule": "Functions that only read should take `&[T]`, not `Vec<T>`.",
     "set": [
      {
       "id": "s",
       "state": "normal"
      }
     ]
    }
   ]
  }
 ]
}
```

```run
let raw: i32 = 300;
println!("as u8 wraps: {}", raw as u8);
println!("clamped first: {}", raw.clamp(0, 255) as u8);

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
let (lo, hi) = min_max(7, 3);
println!("lo = {lo}, hi = {hi}");

let readings = vec![20.5, 21.0, 22.5];
println!("{} readings", readings.len());
```

`values.len()` is a `usize`, so convert it (`as f64`) before dividing a float by it.

## Quick check

```predict
let x: u8 = 250;
let y = x as i32 + 10;
println!("{y}");
---
options: 260 | 4 | error
answer: 260
why: `x as i32` widens first, and there's room for 260 in an `i32`. Adding 10 to the `u8` itself would overflow and panic in a debug build.
```

## Your turn

Four functions: `to_byte`, `min_max3`, `split_seconds` and `mean`. The `///` comments say what each returns.

## Example

```
to_byte(300)                →  255
min_max3(3, 1, 2)           →  (1, 3)
split_seconds(3725)         →  (1, 2, 5)
mean(&[1.0, 2.0, 4.5])      →  2.5
```

## Hints

1. `to_byte`: `x.clamp(0, 255) as u8`.
2. `split_seconds`: use `/` and `%` with 3600 and 60, and return the tuple `(h, m, s)`.
3. `mean`: return `0.0` early for an empty slice; otherwise sum in a `for` loop and divide by `values.len() as f64`.

## Review

Tomorrow, solve this again, then make `mean` return `None` for an empty slice once you've met `Option` (lesson 05).
