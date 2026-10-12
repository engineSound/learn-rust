---
id: rust-03-battery-low
track: rust
title: Decide when a battery is low
topic: Option & match
minutes: 10
concepts: [option, match, booleans]
prereqs: [rust-02-convert-temperature]
contrast: java
edit: src/lib.rs
---

# Decide when a battery is low

## Learn the idea

Sensors run on batteries; sometimes the level is unknown. In Java that would be a nullable `Integer`, and a `NullPointerException` waiting to happen.

**Rust has no `null`.** A value that might be missing has type `Option<T>`: either `Some(value)` or `None`. You can't use it as a number until you've handled `None`, and the compiler checks that you did.

```
match level {
    Some(l) => l < 20,   // l is the u8 inside
    None => false,       // must be handled, or it won't compile
}
```

Also: `if` needs a real `bool`. There's no "truthy" `0` or empty string as in JavaScript: `if level { … }` doesn't compile.

```visual
{
 "title": "Option instead of null",
 "regions": [
  {
   "id": "in",
   "label": "level: Option<u8>"
  },
  {
   "id": "match",
   "label": "match level"
  },
  {
   "id": "res",
   "label": "Result"
  }
 ],
 "scenarios": [
  {
   "name": "A known level",
   "group": "How it works",
   "code": "let level: Option<u8> = Some(15);\nlet low = match level {\n    Some(l) => l < 20,\n    None => false,\n};\nprintln!(\"{low}\");",
   "check": {
    "output": "true"
   },
   "steps": [
    {
     "line": 1,
     "say": "The level is known: `Some(15)`. The 15 is wrapped inside.",
     "rule": "`Option<T>` is `Some(value)` or `None`. There is no `null`.",
     "set": [
      {
       "id": "lv",
       "in": "in",
       "title": "level",
       "value": "Some(15)",
       "tag": "Option<u8>",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "`match` checks the arms in order. `Some(l)` fits and unwraps 15 into `l`.",
     "rule": "A pattern can take the value out of `Some`.",
     "set": [
      {
       "id": "a1",
       "in": "match",
       "title": "Some(l) => l < 20",
       "tag": "matches · l = 15",
       "state": "active"
      },
      {
       "id": "a2",
       "in": "match",
       "title": "None => false",
       "tag": "skipped",
       "state": "faded"
      }
     ],
     "arrows": [
      {
       "from": "lv",
       "to": "a1",
       "state": "accent"
      }
     ]
    },
    {
     "line": 3,
     "say": "15 < 20, so the result is `true`.",
     "set": [
      {
       "id": "a1",
       "state": "ok"
      },
      {
       "id": "r",
       "in": "res",
       "title": "low",
       "value": "true",
       "tag": "bool",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "a1",
       "to": "r",
       "state": "ok"
      }
     ]
    }
   ]
  },
  {
   "name": "Unknown level",
   "group": "How it works",
   "code": "let level: Option<u8> = None;\nlet low = match level {\n    Some(l) => l < 20,\n    None => false,\n};\nprintln!(\"{low}\");",
   "check": {
    "output": "false"
   },
   "steps": [
    {
     "line": 1,
     "say": "No reading came in: `None`.",
     "set": [
      {
       "id": "lv",
       "in": "in",
       "title": "level",
       "value": "None",
       "tag": "Option<u8>",
       "state": "info"
      }
     ]
    },
    {
     "line": 4,
     "say": "`Some(l)` doesn't fit, so the `None` arm runs.",
     "rule": "The compiler insists on the `None` arm: you can't forget it.",
     "set": [
      {
       "id": "a1",
       "in": "match",
       "title": "Some(l) => l < 20",
       "tag": "skipped",
       "state": "faded"
      },
      {
       "id": "a2",
       "in": "match",
       "title": "None => false",
       "tag": "matches",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "lv",
       "to": "a2",
       "state": "accent"
      }
     ]
    },
    {
     "line": 4,
     "say": "Result: `false`. No NullPointerException possible.",
     "set": [
      {
       "id": "a2",
       "state": "ok"
      },
      {
       "id": "r",
       "in": "res",
       "title": "low",
       "value": "false",
       "tag": "bool",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "a2",
       "to": "r",
       "state": "ok"
      }
     ]
    }
   ]
  },
  {
   "name": "unwrap() on None",
   "group": "What if it goes wrong",
   "code": "let level: Option<u8> = None;\nlet l = level.unwrap();\nprintln!(\"{l}\");",
   "check": {
    "error": "unwrap"
   },
   "steps": [
    {
     "line": 1,
     "say": "The level is unknown.",
     "set": [
      {
       "id": "lv",
       "in": "in",
       "title": "level",
       "value": "None",
       "tag": "Option<u8>",
       "state": "info"
      }
     ]
    },
    {
     "line": 2,
     "say": "`unwrap()` says \"I promise there's a value\". There isn't, so the program **panics** and stops.",
     "rule": "`unwrap` is Rust's NullPointerException, but you have to ask for it. Prefer `match`, `if let` or `unwrap_or`.",
     "set": [
      {
       "id": "u",
       "in": "match",
       "title": "level.unwrap()",
       "tag": "panic!",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "lv",
       "to": "u",
       "state": "bad"
      }
     ],
     "error": "panicked: called `Option::unwrap()` on a `None` value"
    }
   ]
  },
  {
   "name": "if needs a bool",
   "group": "What if it goes wrong",
   "code": "let level: u8 = 15;\nif level {\n    println!(\"has charge\");\n}",
   "check": {
    "error": "E0308"
   },
   "steps": [
    {
     "line": 1,
     "say": "A plain number.",
     "set": [
      {
       "id": "lv",
       "in": "in",
       "title": "level",
       "value": "15",
       "tag": "u8",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "In JavaScript a number can stand in for true/false. In Rust, `if` only takes a `bool`.",
     "rule": "Say what you mean: `if level > 0`.",
     "set": [
      {
       "id": "i",
       "in": "match",
       "title": "if level",
       "tag": "not a bool",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "lv",
       "to": "i",
       "state": "bad"
      }
     ],
     "error": "E0308: mismatched types: expected `bool`, found `u8`"
    }
   ]
  }
 ]
}
```

```run
let levels: [Option<u8>; 3] = [Some(15), Some(80), None];
for level in levels {
    let text = match level {
        Some(l) if l < 20 => format!("{l}% is low"),
        Some(l) => format!("{l}% is fine"),
        None => String::from("unknown"),
    };
    println!("{text}");
}
```

## Quick check

```predict
let level: Option<u8> = None;
println!("{}", level.unwrap_or(100));
---
options: 0 | 100 | None
answer: 100
why: `unwrap_or` gives the value inside `Some`, or the fallback when it's `None`.
```

## Your turn

Write `is_battery_low(level, charging)`. A battery is low when the level is **known**, **under 20**, and it's **not charging**. An unknown level (`None`) is not low: we can't tell.

## Example

```
is_battery_low(Some(15), false)  →  true
is_battery_low(Some(15), true)   →  false
is_battery_low(None, false)      →  false
```

## Hints

1. Start with `match level { … }` and two arms: `Some(l) => …` and `None => …`.
2. In the `Some(l)` arm, combine two conditions with `&&`: `l < 20` and `!charging`.
3. The `None` arm is just `false`.

## Review

Tomorrow, solve this again, then rewrite it with `matches!` or `if let` and compare.
