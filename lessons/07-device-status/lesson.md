---
id: rust-07-device-status
track: rust
title: Describe a device status
topic: Enums & match
minutes: 10
concepts: [enums, match, pattern-binding]
prereqs: [rust-06-ownership]
contrast: java
edit: src/lib.rs
---

# Describe a device status

## Learn the idea

A device is online, offline since some time, or in an error state with a message. In Java you'd write an `enum` plus extra fields, or a sealed interface with three records.

A Rust `enum` does this directly. **Each variant can carry its own data:**

```
enum Status {
    Online,                          // no data
    Offline { since_minutes: u32 },  // named fields, like a record
    Error(String),                   // a positional value
}
```

`match` takes the value apart and **binds** the data inside each variant to a name:

| Java 21 | Rust |
|------|------|
| `switch (s) { case Offline o -> o.sinceMinutes(); … }` | `match s { Status::Offline { since_minutes } => …, … }` |
| missing case → compile error only for sealed types | missing variant → **always** a compile error (E0004) |

`match` is an expression, so each arm gives a value and the whole `match` is the result.

```visual
{
 "title": "How match picks an arm",
 "intro": "`match` tries each arm from the top. The first pattern that fits wins, and its names are bound to the data inside.",
 "regions": [
  {
   "id": "val",
   "label": "The value"
  },
  {
   "id": "match",
   "label": "match status"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Offline for 5 minutes",
   "group": "How it works",
   "code": "enum Status {\n    Online,\n    Offline { since_minutes: u32 },\n    Error(String),\n}\nlet s = Status::Offline { since_minutes: 5 };\nlet text = match &s {\n    Status::Online => String::from(\"online\"),\n    Status::Offline { since_minutes } => format!(\"offline for {since_minutes} min\"),\n    Status::Error(msg) => format!(\"error: {msg}\"),\n};\nprintln!(\"{text}\");",
   "check": {
    "output": "offline for 5 min"
   },
   "steps": [
    {
     "line": 6,
     "say": "One value of type `Status`: the `Offline` variant, carrying `since_minutes: 5`.",
     "rule": "An enum value is exactly one variant, plus that variant's data.",
     "set": [
      {
       "id": "s",
       "in": "val",
       "title": "s",
       "value": "Offline { since_minutes: 5 }",
       "tag": "Status",
       "state": "active"
      }
     ]
    },
    {
     "line": 8,
     "say": "First arm: `Status::Online`. Wrong variant, so skip it.",
     "rule": "Arms are tried from top to bottom.",
     "set": [
      {
       "id": "s",
       "state": "normal"
      },
      {
       "id": "a1",
       "in": "match",
       "title": "Status::Online",
       "value": "no match",
       "state": "faded"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "a1",
       "state": "muted",
       "style": "dashed"
      }
     ]
    },
    {
     "line": 9,
     "say": "Second arm fits. `since_minutes` is bound to the 5 inside.",
     "rule": "Patterns take values apart, like a record deconstruction in Java 21.",
     "set": [
      {
       "id": "a2",
       "in": "match",
       "title": "Status::Offline { since_minutes }",
       "value": "since_minutes = 5",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "a2",
       "label": "matches",
       "state": "ok"
      }
     ]
    },
    {
     "line": 9,
     "say": "The arm's expression builds the text; it becomes the value of the whole `match`.",
     "rule": "`match` is an expression: no `return`, no temporary variable.",
     "set": [
      {
       "id": "t",
       "in": "val",
       "title": "text",
       "value": "\"offline for 5 min\"",
       "tag": "String",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "a2",
       "to": "t",
       "label": "value",
       "state": "ok"
      }
     ]
    },
    {
     "line": 12,
     "say": "Printed. The `Error` arm was never tried.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "offline for 5 min"
      }
     ],
     "arrows": [
      {
       "from": "t",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "Error with a message",
   "group": "How it works",
   "code": "enum Status {\n    Online,\n    Offline { since_minutes: u32 },\n    Error(String),\n}\nlet s = Status::Error(String::from(\"overheated\"));\nif let Status::Error(msg) = &s {\n    println!(\"error: {msg}\");\n}",
   "check": {
    "output": "error: overheated"
   },
   "steps": [
    {
     "line": 6,
     "say": "An `Error` variant that owns a `String`.",
     "set": [
      {
       "id": "s",
       "in": "val",
       "title": "s",
       "value": "Error(\"overheated\")",
       "tag": "Status",
       "state": "active"
      }
     ]
    },
    {
     "line": 7,
     "say": "When you only care about **one** variant, `if let` is shorter than a full `match`.",
     "rule": "`if let PATTERN = value { … }` runs only if the pattern fits.",
     "set": [
      {
       "id": "s",
       "state": "normal"
      },
      {
       "id": "a",
       "in": "match",
       "title": "if let Status::Error(msg)",
       "value": "msg = \"overheated\"",
       "tag": "&String",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "a",
       "label": "matches",
       "state": "ok"
      }
     ]
    },
    {
     "line": 8,
     "say": "`msg` borrows the text inside `s` (we matched on `&s`).",
     "rule": "Matching on a reference binds references: nothing is moved out.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "error: overheated"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "Forget a variant",
   "group": "What if it goes wrong",
   "code": "enum Status {\n    Online,\n    Offline { since_minutes: u32 },\n    Error(String),\n}\nfn describe(s: &Status) -> String {\n    match s {\n        Status::Online => String::from(\"online\"),\n        Status::Error(msg) => format!(\"error: {msg}\"),\n    }\n}",
   "check": {
    "error": "E0004"
   },
   "steps": [
    {
     "line": 7,
     "say": "`describe` must return a `String` for **every** possible `Status`.",
     "set": [
      {
       "id": "f",
       "in": "val",
       "title": "describe",
       "value": "Online | Offline | Error",
       "tag": "all variants"
      }
     ]
    },
    {
     "line": 8,
     "say": "The `match` covers `Online` and `Error`…",
     "set": [
      {
       "id": "a1",
       "in": "match",
       "title": "Status::Online",
       "state": "ok"
      },
      {
       "id": "a2",
       "in": "match",
       "title": "Status::Error(msg)",
       "state": "ok"
      }
     ]
    },
    {
     "line": 8,
     "say": "…but not `Offline`. Rust refuses to compile, and names the missing pattern.",
     "rule": "A `match` must be exhaustive. Add the arm, or `_ =>` if you really mean \"anything else\".",
     "set": [
      {
       "id": "miss",
       "in": "match",
       "title": "Status::Offline { .. }",
       "value": "not covered",
       "state": "bad"
      }
     ],
     "error": "E0004: non-exhaustive patterns: `&Status::Offline { .. }` not covered"
    }
   ]
  }
 ]
}
```

```run
enum Status {
    Online,
    Offline { since_minutes: u32 },
    Error(String),
}

fn label(s: &Status) -> String {
    match s {
        Status::Online => String::from("up"),
        Status::Offline { since_minutes } => format!("down {since_minutes}m"),
        Status::Error(msg) => format!("broken: {msg}"),
    }
}

let all = [Status::Online, Status::Offline { since_minutes: 12 }, Status::Error(String::from("no signal"))];
for s in &all {
    println!("{}", label(s));
}
```

## Quick check

```predict
enum Status {
    Online,
    Offline { since_minutes: u32 },
    Error(String),
}
let s = Status::Offline { since_minutes: 3 };
let n = match s {
    Status::Offline { since_minutes } => since_minutes * 60,
    _ => 0,
};
println!("{n}");
---
options: 3 | 180 | 0
answer: 180
why: The `Offline` arm matches and binds `since_minutes` to 3; `_` catches every other variant.
```

## Your turn

Write `describe(status)`. It returns `"online"`, `"offline for N min"`, or `"error: MESSAGE"`.

## Example

```
describe(&Status::Online)                              →  "online"
describe(&Status::Offline { since_minutes: 5 })        →  "offline for 5 min"
describe(&Status::Error(String::from("overheated")))   →  "error: overheated"
```

## Hints

1. Start with `match status { … }` and one arm per variant: `Status::Online`, `Status::Offline { since_minutes }` and `Status::Error(msg)`.
2. Each arm is an expression. The first one is `String::from("online")`; the others use `format!`.
3. `Status::Offline { since_minutes } => format!("offline for {since_minutes} min"),` and `Status::Error(msg) => format!("error: {msg}"),`

## Review

Tomorrow, add a fourth variant `Updating(u8)` (percent done) and let the compiler show you every `match` you must fix.
