---
id: rust-02b-stretch-loops
track: rust
title: Loops that return values
kind: stretch
for: rust-02-convert-temperature
topic: Loops & integers
minutes: 20
concepts: [expressions, let-mut, integer-types, loops]
prereqs: [rust-02-convert-temperature]
contrast: java
edit: src/lib.rs
---

# Loops that return values

## Recall

A harder one, for when *Convert a temperature* felt easy.

Rust has three loops: `for x in iter`, `while cond`, and `loop`. Unlike in Java, `loop` is an expression: `break value;` makes the whole loop evaluate to `value`.

```visual
{
 "title": "A loop that returns a value",
 "regions": [
  {
   "id": "vars",
   "label": "Variables"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "break with a value",
   "group": "How it works",
   "code": "let mut n = 1;\nlet first_big = loop {\n    n *= 3;\n    if n > 100 {\n        break n;\n    }\n};\nprintln!(\"{first_big}\");",
   "check": {
    "output": "243"
   },
   "steps": [
    {
     "line": 1,
     "say": "`n` starts at 1.",
     "set": [
      {
       "id": "n",
       "in": "vars",
       "title": "n",
       "value": "1",
       "tag": "mut",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "`n *= 3` → 3. Not above 100 yet, so the loop goes round again.",
     "set": [
      {
       "id": "n",
       "value": "3",
       "state": "active"
      }
     ]
    },
    {
     "line": 3,
     "say": "`n *= 3` → 9. Not above 100 yet, so the loop goes round again.",
     "set": [
      {
       "id": "n",
       "value": "9",
       "state": "active"
      }
     ]
    },
    {
     "line": 3,
     "say": "`n *= 3` → 27. Not above 100 yet, so the loop goes round again.",
     "set": [
      {
       "id": "n",
       "value": "27",
       "state": "active"
      }
     ]
    },
    {
     "line": 3,
     "say": "`n *= 3` → 81. Not above 100 yet, so the loop goes round again.",
     "set": [
      {
       "id": "n",
       "value": "81",
       "state": "active"
      }
     ]
    },
    {
     "line": 5,
     "say": "243 is above 100: `break n` ends the loop **and** hands back the value.",
     "rule": "`loop` is an expression: `break value` is its result.",
     "set": [
      {
       "id": "n",
       "value": "243"
      },
      {
       "id": "fb",
       "in": "vars",
       "title": "first_big",
       "value": "243",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "n",
       "to": "fb",
       "label": "break n",
       "state": "ok"
      }
     ]
    },
    {
     "line": 8,
     "say": "Printed: 243.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "243"
      }
     ]
    }
   ]
  },
  {
   "name": "Overflow",
   "group": "What if it goes wrong",
   "code": "let readings: [u8; 2] = [250, 10];\nlet mut total: u8 = 0;\nfor r in readings {\n    total += r;\n}",
   "check": {
    "error": "overflow"
   },
   "steps": [
    {
     "line": 2,
     "say": "`total` is a `u8`: 0 to 255.",
     "set": [
      {
       "id": "t",
       "in": "vars",
       "title": "total",
       "value": "0",
       "tag": "u8",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "+250 → 250.",
     "set": [
      {
       "id": "t",
       "value": "250",
       "state": "active"
      }
     ]
    },
    {
     "line": 4,
     "say": "+10 would be 260, too big for a `u8`. In a debug build Rust **panics**; Java would silently wrap.",
     "rule": "Pick a type wide enough (here `u32`), or use `checked_add`.",
     "set": [
      {
       "id": "t",
       "value": "260?",
       "state": "bad"
      }
     ],
     "error": "panicked: attempt to add with overflow"
    }
   ]
  }
 ]
}
```

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
