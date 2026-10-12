---
id: rust-09-format-reading
track: rust
title: Print a reading nicely
topic: Traits: Display
minutes: 10
concepts: [traits, display, formatting]
prereqs: [rust-08-count-readings]
contrast: java
edit: src/lib.rs
---

# Print a reading nicely

## Learn the idea

The dashboard shows readings as `hall: 21.5°C`. In Java you'd override `toString()`. Rust has no base `Object` class; instead, a type **implements a trait**, Rust's version of an interface.

To make `{}` work for your type, implement `std::fmt::Display`:

```
impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:.1}°C", self.device, self.celsius)
    }
}
```

- `write!` is like `format!`, but writes into the formatter `f`.
- `{:.1}` prints one decimal place (like Java's `%.1f`).
- Once `Display` is implemented, you also get `.to_string()` for free.

`{:?}` is the other one, **`Debug`**: a developer view you usually get with `#[derive(Debug)]`. `{}` needs `Display`, which you always write yourself: the compiler can't guess how users want to see your type.

```visual
{
 "title": "What {} does with your type",
 "intro": "`println!(\"{}\", r)` asks the type how to show itself, by calling its `Display` implementation.",
 "regions": [
  {
   "id": "val",
   "label": "Your value"
  },
  {
   "id": "impl",
   "label": "impl Display for Reading"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Print a reading",
   "group": "How it works",
   "code": "use std::fmt;\nstruct Reading {\n    device: String,\n    celsius: f64,\n}\nimpl fmt::Display for Reading {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"{}: {:.1}°C\", self.device, self.celsius)\n    }\n}\nlet r = Reading { device: String::from(\"hall\"), celsius: 21.46 };\nprintln!(\"{r}\");",
   "check": {
    "output": "hall: 21.5°C"
   },
   "steps": [
    {
     "line": 11,
     "say": "A `Reading` value with two fields.",
     "set": [
      {
       "id": "r",
       "in": "val",
       "title": "r",
       "value": "hall · 21.46",
       "tag": "Reading",
       "state": "active"
      }
     ]
    },
    {
     "line": 12,
     "say": "`{r}` needs `Display`. Rust finds `impl Display for Reading` and calls its `fmt`.",
     "rule": "A trait is a contract; `impl Trait for Type` is where the type fulfils it.",
     "set": [
      {
       "id": "r",
       "state": "normal"
      },
      {
       "id": "fmt",
       "in": "impl",
       "title": "fmt(&self, f)",
       "value": "self = &r",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "fmt",
       "label": "&self",
       "state": "accent"
      }
     ]
    },
    {
     "line": 8,
     "say": "`write!` fills the text in: `{:.1}` turns 21.46 into `21.5`.",
     "rule": "`{:.1}` = one decimal, rounded. Same as `%.1f` in Java.",
     "set": [
      {
       "id": "fmt",
       "state": "normal"
      },
      {
       "id": "w",
       "in": "impl",
       "title": "write!(f, …)",
       "value": "\"hall: 21.5°C\"",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "fmt",
       "to": "w",
       "state": "muted"
      }
     ]
    },
    {
     "line": 8,
     "say": "It returns `Ok(())`, and `println!` prints what was written.",
     "rule": "`fmt::Result` is `Ok(())` or an error from the output.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "hall: 21.5°C"
      }
     ],
     "arrows": [
      {
       "from": "w",
       "to": "o",
       "state": "ok"
      }
     ]
    }
   ]
  },
  {
   "name": "Forget to implement Display",
   "group": "What if it goes wrong",
   "code": "struct Reading {\n    device: String,\n    celsius: f64,\n}\nlet r = Reading { device: String::from(\"hall\"), celsius: 21.5 };\nprintln!(\"{r}\");",
   "check": {
    "error": "E0277"
   },
   "steps": [
    {
     "line": 5,
     "say": "A `Reading`, but no `impl Display` anywhere.",
     "set": [
      {
       "id": "r",
       "in": "val",
       "title": "r",
       "value": "hall · 21.5",
       "tag": "Reading"
      }
     ]
    },
    {
     "line": 6,
     "say": "`{r}` asks for `Display`. There is none, so it doesn't compile.",
     "rule": "Unlike Java's inherited `toString()`, there is no default. Implement `Display`, or use `{:?}` with `#[derive(Debug)]`.",
     "set": [
      {
       "id": "none",
       "in": "impl",
       "title": "impl Display for Reading",
       "value": "missing",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "none",
       "label": "?",
       "state": "bad",
       "style": "dashed"
      }
     ],
     "error": "E0277: `Reading` doesn't implement `std::fmt::Display`"
    }
   ]
  },
  {
   "name": "Debug instead",
   "group": "What if it goes wrong",
   "code": "#[derive(Debug)]\nstruct Reading {\n    device: String,\n    celsius: f64,\n}\nlet r = Reading { device: String::from(\"hall\"), celsius: 21.5 };\nprintln!(\"{r:?}\");",
   "check": {
    "output": "Reading { device: \"hall\", celsius: 21.5 }"
   },
   "steps": [
    {
     "line": 1,
     "say": "`#[derive(Debug)]` asks the compiler to write a `Debug` implementation for you.",
     "rule": "`derive` generates trait code, like Lombok or a Java record's `toString`.",
     "set": [
      {
       "id": "d",
       "in": "impl",
       "title": "impl Debug (derived)",
       "value": "generated",
       "state": "info"
      }
     ]
    },
    {
     "line": 7,
     "say": "`{:?}` uses it: a developer view with field names and quotes. Good for logs, not for users.",
     "set": [
      {
       "id": "r",
       "in": "val",
       "title": "r",
       "value": "hall · 21.5",
       "tag": "Reading"
      },
      {
       "id": "o",
       "in": "out",
       "title": "Reading { device: \"hall\", celsius: 21.5 }"
      }
     ],
     "arrows": [
      {
       "from": "r",
       "to": "d",
       "label": "{:?}",
       "state": "accent"
      },
      {
       "from": "d",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  }
 ]
}
```

```run
use std::fmt;

struct Battery {
    percent: u8,
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "battery {}%", self.percent)
    }
}

let b = Battery { percent: 42 };
println!("{b}");
let text: String = b.to_string();
println!("{} chars", text.len());
```

## Quick check

```predict
println!("{:.1} {:.2}", 21.0, 3.14159);
---
options: 21 3.14 | 21.0 3.14 | 21.0 3.15
answer: 21.0 3.14
why: `{:.N}` always prints exactly N decimals and rounds: 3.14159 → 3.14.
```

## Your turn

Implement `Display` for `Reading` so it prints as `device: value°C` with **one** decimal.

## Example

```
Reading { device: "hall", celsius: 21.46 }  →  "hall: 21.5°C"
Reading { device: "attic", celsius: -3.0 }  →  "attic: -3.0°C"
```

## Hints

1. The method signature is fixed: `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`.
2. Use `write!(f, …)` with the same placeholders as `println!`; read the fields with `self.device` and `self.celsius`.
3. `write!(f, "{}: {:.1}°C", self.device, self.celsius)` (no `;`, it's the return value).

## Review

Tomorrow, solve this again, then add `#[derive(Debug)]` and compare `{}` with `{:?}`.
