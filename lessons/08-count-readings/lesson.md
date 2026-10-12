---
id: rust-08-count-readings
track: rust
title: Count readings per device
topic: HashMap & the entry API
minutes: 10
concepts: [hashmap, entry-api, iteration]
prereqs: [rust-07-device-status]
contrast: java
edit: src/lib.rs
---

# Count readings per device

## Learn the idea

The hub receives readings from many devices, and you need a count per device. In Java: `Map<String, Integer>` and `counts.merge(name, 1, Integer::sum)`.

Rust's `HashMap` lives in `std::collections` (`use` it first). The idiomatic way to update-or-insert is the **entry API**:

```
*counts.entry(name).or_insert(0) += 1;
```

- `entry(key)` finds the slot for the key, present or not.
- `or_insert(0)` puts `0` there if it's empty, and returns a **mutable reference** (`&mut usize`) to the value.
- `*… += 1` changes the value through that reference (the `*` follows it).

Reading: `counts.get("hall")` returns an `Option<&usize>`, `None` if the key is missing. `counts["hall"]` **panics** if it's missing, like an unchecked `get` that throws.

```visual
{
 "title": "Counting with the entry API",
 "intro": "Watch the map fill up: `entry` finds the slot, `or_insert` fills it only when empty, `+= 1` updates it in place.",
 "regions": [
  {
   "id": "in",
   "label": "readings"
  },
  {
   "id": "map",
   "label": "counts: HashMap"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Three readings",
   "group": "How it works",
   "code": "use std::collections::HashMap;\nlet readings = [\"hall\", \"kitchen\", \"hall\"];\nlet mut counts: HashMap<&str, usize> = HashMap::new();\nfor device in readings {\n    *counts.entry(device).or_insert(0) += 1;\n}\nprintln!(\"{}\", counts[\"hall\"]);",
   "check": {
    "output": "2"
   },
   "steps": [
    {
     "line": 2,
     "say": "Three readings arrive.",
     "set": [
      {
       "id": "r1",
       "in": "in",
       "title": "\"hall\""
      },
      {
       "id": "r2",
       "in": "in",
       "title": "\"kitchen\""
      },
      {
       "id": "r3",
       "in": "in",
       "title": "\"hall\""
      }
     ]
    },
    {
     "line": 3,
     "say": "An empty map, declared `mut` because we will change it.",
     "rule": "Like Java, a map is empty until you add keys.",
     "set": [
      {
       "id": "m",
       "in": "map",
       "title": "counts",
       "value": "{}",
       "tag": "HashMap<&str, usize>"
      }
     ]
    },
    {
     "line": 5,
     "say": "\"hall\" isn't there yet: `or_insert(0)` adds 0, then `+= 1` makes it 1.",
     "rule": "`or_insert` gives a `&mut` to the value, new or existing.",
     "set": [
      {
       "id": "r1",
       "state": "active"
      },
      {
       "id": "hall",
       "in": "map",
       "title": "\"hall\"",
       "value": "1",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "r1",
       "to": "hall",
       "label": "insert 0, +1",
       "state": "ok"
      }
     ]
    },
    {
     "line": 5,
     "say": "\"kitchen\" is new too: 0 + 1.",
     "set": [
      {
       "id": "r1",
       "state": "faded"
      },
      {
       "id": "r2",
       "state": "active"
      },
      {
       "id": "kit",
       "in": "map",
       "title": "\"kitchen\"",
       "value": "1",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "r2",
       "to": "kit",
       "label": "insert 0, +1",
       "state": "ok"
      }
     ]
    },
    {
     "line": 5,
     "say": "\"hall\" exists: `or_insert` leaves it alone, and `+= 1` makes 2.",
     "rule": "Same line, two cases: no `containsKey` check needed.",
     "set": [
      {
       "id": "r2",
       "state": "faded"
      },
      {
       "id": "r3",
       "state": "active"
      },
      {
       "id": "hall",
       "value": "2",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "r3",
       "to": "hall",
       "label": "+1",
       "state": "accent"
      }
     ]
    },
    {
     "line": 7,
     "say": "`counts[\"hall\"]` reads the value: 2.",
     "set": [
      {
       "id": "r3",
       "state": "faded"
      },
      {
       "id": "hall",
       "state": "ok"
      },
      {
       "id": "o",
       "in": "out",
       "title": "2"
      }
     ],
     "arrows": [
      {
       "from": "hall",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "Index a missing key",
   "group": "What if it goes wrong",
   "code": "use std::collections::HashMap;\nlet mut counts: HashMap<&str, usize> = HashMap::new();\ncounts.insert(\"hall\", 2);\nprintln!(\"{}\", counts[\"garage\"]);",
   "check": {
    "error": "panicked"
   },
   "steps": [
    {
     "line": 3,
     "say": "The map has one key.",
     "set": [
      {
       "id": "hall",
       "in": "map",
       "title": "\"hall\"",
       "value": "2",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "`counts[\"garage\"]`: there is no such key, so the program **panics** at runtime.",
     "rule": "`map[key]` is for keys you know exist. Otherwise use `get`, which returns an `Option`.",
     "set": [
      {
       "id": "g",
       "in": "map",
       "title": "\"garage\"",
       "value": "missing",
       "state": "bad"
      }
     ],
     "error": "panicked: key not found in the map"
    }
   ]
  },
  {
   "name": "The safe way: get",
   "group": "What if it goes wrong",
   "code": "use std::collections::HashMap;\nlet mut counts: HashMap<&str, usize> = HashMap::new();\ncounts.insert(\"hall\", 2);\nlet n = counts.get(\"garage\").copied().unwrap_or(0);\nprintln!(\"{n}\");",
   "check": {
    "output": "0"
   },
   "steps": [
    {
     "line": 3,
     "say": "Same map.",
     "set": [
      {
       "id": "hall",
       "in": "map",
       "title": "\"hall\"",
       "value": "2",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "`get` returns `None` instead of crashing…",
     "rule": "`get` → `Option<&V>`: `Some(&value)` or `None`.",
     "set": [
      {
       "id": "g",
       "in": "map",
       "title": "get(\"garage\")",
       "value": "None",
       "tag": "Option<&usize>",
       "state": "info"
      }
     ]
    },
    {
     "line": 4,
     "say": "…and `.unwrap_or(0)` picks the fallback. (`.copied()` turns `&usize` into `usize`.)",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "0",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "g",
       "to": "o",
       "label": "unwrap_or(0)",
       "state": "ok"
      }
     ]
    }
   ]
  }
 ]
}
```

```run
use std::collections::HashMap;

let words = ["hall", "kitchen", "hall", "hall"];
let mut counts: HashMap<&str, usize> = HashMap::new();
for w in words {
    *counts.entry(w).or_insert(0) += 1;
}
println!("hall: {}", counts["hall"]);
println!("kitchen: {:?}", counts.get("kitchen"));
println!("garage: {:?}", counts.get("garage"));
```

## Quick check

```predict
use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("hall", 1);
m.insert("hall", 5);
println!("{} {}", m.len(), m["hall"]);
---
options: 2 1 | 1 5 | 1 1
answer: 1 5
why: `insert` on an existing key replaces the value, as `put` does in Java.
```

## Your turn

Write `count_by_device(readings)`. Each reading is a `(device, value)` pair; return how many readings each device sent.

## Example

```
[("hall", 21.5), ("kitchen", 19.0), ("hall", 22.0)]
  →  {"hall": 2, "kitchen": 1}
```

## Hints

1. Create `let mut counts = HashMap::new();` and loop with `for (device, _) in readings`.
2. The key type is `String`, so the map needs an owned copy of the name: `device.clone()`.
3. Inside the loop: `*counts.entry(device.clone()).or_insert(0) += 1;` and return `counts` at the end.

## Review

Tomorrow, solve this again, then write `average_by_device` returning `HashMap<String, f64>`.
