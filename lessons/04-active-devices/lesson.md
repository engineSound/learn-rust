---
id: rust-04-active-devices
track: rust
title: Pick active devices
topic: Structs & iterators
minutes: 10
concepts: [structs, iterators, closures, vec]
prereqs: [rust-03-battery-low]
contrast: java
edit: src/lib.rs
---

# Pick active devices

## Learn the idea

The dashboard lists devices that are online. A device is a **struct**, like a Java record:

```
pub struct Device { pub name: String, pub online: bool }
```

Iterators work like Java streams, with one difference that trips people up: they are **lazy**, and nothing happens until you consume them, usually with `.collect()`:

| Java streams | Rust iterators |
|---|---|
| `list.stream()` | `slice.iter()` (borrows each item) |
| `.filter(d -> d.online())` | `.filter(\|d\| d.online)` |
| `.map(Device::name)` | `.map(\|d\| d.name.clone())` |
| `.collect(toList())` | `.collect::<Vec<_>>()` |

`|d| …` is a closure (a lambda). And sorting needs a **mutable** vector: `let mut names = …; names.sort();`

```visual
{
 "title": "Watch the data flow through the iterator",
 "intro": "Lazy steps, then `collect` makes them run.",
 "scenarios": [
  {
   "name": "filter → map → collect",
   "group": "How it works",
   "regions": [
    {
     "id": "src",
     "label": "devices: &[Device]"
    },
    {
     "id": "f",
     "label": ".filter(online)"
    },
    {
     "id": "m",
     "label": ".map(name)"
    },
    {
     "id": "res",
     "label": "collect + sort"
    }
   ],
   "code": "let mut names: Vec<String> = devices\n    .iter()\n    .filter(|d| d.online)\n    .map(|d| d.name.clone())\n    .collect();\nnames.sort();",
   "check": {
    "prelude": "struct Device { name: String, online: bool }\nlet devices = vec![\n    Device { name: \"porch light\".into(), online: true },\n    Device { name: \"garage door\".into(), online: false },\n    Device { name: \"attic fan\".into(), online: true },\n];",
    "code": "let mut names: Vec<String> = devices.iter().filter(|d| d.online).map(|d| d.name.clone()).collect();\nnames.sort();\nprintln!(\"{names:?}\");",
    "output": "[\"attic fan\", \"porch light\"]"
   },
   "steps": [
    {
     "line": 2,
     "say": "`iter()` borrows each device in turn. Nothing is copied or moved.",
     "rule": "`iter()` = borrow; the caller keeps the data.",
     "set": [
      {
       "id": "d1",
       "in": "src",
       "title": "porch light",
       "tag": "online"
      },
      {
       "id": "d2",
       "in": "src",
       "title": "garage door",
       "tag": "offline"
      },
      {
       "id": "d3",
       "in": "src",
       "title": "attic fan",
       "tag": "online"
      }
     ]
    },
    {
     "line": 3,
     "say": "`filter` lets through the devices whose closure returns `true`.",
     "rule": "Iterators are lazy: these steps run only when something consumes them.",
     "set": [
      {
       "id": "d1",
       "in": "f",
       "state": "ok"
      },
      {
       "id": "d2",
       "state": "faded",
       "tag": "dropped"
      },
      {
       "id": "d3",
       "in": "f",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "`map` turns each borrowed device into an owned `String`: its name, cloned.",
     "rule": "`clone()` makes your own copy; the device keeps its name.",
     "set": [
      {
       "id": "d1",
       "in": "m",
       "title": "\"porch light\"",
       "tag": "String"
      },
      {
       "id": "d3",
       "in": "m",
       "title": "\"attic fan\"",
       "tag": "String"
      }
     ]
    },
    {
     "line": 5,
     "say": "`collect` pulls everything through and builds a `Vec<String>`.",
     "rule": "Nothing happens until you `collect` (or loop over the iterator).",
     "set": [
      {
       "id": "d1",
       "in": "res",
       "state": "ok"
      },
      {
       "id": "d3",
       "in": "res",
       "state": "ok"
      }
     ]
    },
    {
     "line": 6,
     "say": "`sort` reorders the vector in place. That's why it was declared `mut`.",
     "set": [
      {
       "id": "s1",
       "in": "res",
       "title": "\"attic fan\"",
       "state": "ok"
      },
      {
       "id": "s2",
       "in": "res",
       "title": "\"porch light\"",
       "state": "ok"
      }
     ],
     "drop": [
      "d1",
      "d3"
     ]
    }
   ]
  },
  {
   "name": "Forgot collect",
   "group": "What if it goes wrong",
   "regions": [
    {
     "id": "src",
     "label": "devices: &[Device]"
    },
    {
     "id": "f",
     "label": ".filter(online)"
    },
    {
     "id": "m",
     "label": ".map(name)"
    },
    {
     "id": "res",
     "label": "collect + sort"
    }
   ],
   "code": "fn names(devices: &[Device]) -> Vec<String> {\n    devices.iter().map(|d| d.name.clone())\n}",
   "check": {
    "prelude": "struct Device { name: String }",
    "error": "E0308"
   },
   "steps": [
    {
     "line": 1,
     "say": "The function promises a `Vec<String>`.",
     "set": [
      {
       "id": "p",
       "in": "res",
       "title": "returns",
       "value": "Vec<String>",
       "tag": "promise",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "But the body returns the iterator itself: a recipe, not the results.",
     "rule": "An iterator is a description of work; `collect()` runs it.",
     "set": [
      {
       "id": "it",
       "in": "m",
       "title": "Map<Iter<Device>, …>",
       "tag": "lazy iterator",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "it",
       "to": "p",
       "label": "✗",
       "state": "bad"
      }
     ],
     "error": "E0308: mismatched types: expected `Vec<String>`, found `Map<…>`"
    }
   ]
  },
  {
   "name": "Sort without mut",
   "group": "What if it goes wrong",
   "regions": [
    {
     "id": "src",
     "label": "devices: &[Device]"
    },
    {
     "id": "f",
     "label": ".filter(online)"
    },
    {
     "id": "m",
     "label": ".map(name)"
    },
    {
     "id": "res",
     "label": "collect + sort"
    }
   ],
   "code": "let names = vec![\"porch\", \"attic\"];\nnames.sort();",
   "check": {
    "error": "E0596"
   },
   "steps": [
    {
     "line": 1,
     "say": "`names` is immutable.",
     "set": [
      {
       "id": "n",
       "in": "res",
       "title": "names",
       "value": "[\"porch\", \"attic\"]",
       "tag": "Vec · immutable",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`sort` needs to change the vector in place, so it needs `mut`.",
     "rule": "Changing in place requires `let mut`.",
     "set": [
      {
       "id": "n",
       "state": "bad",
       "tag": "not mut!"
      }
     ],
     "error": "E0596: cannot borrow `names` as mutable, as it is not declared as mutable"
    }
   ]
  }
 ]
}
```

```run
let temps = vec![21.5, 19.0, 23.5];
let warm: Vec<f64> = temps.iter().filter(|t| **t > 20.0).copied().collect();
println!("{warm:?}");
let mut names = vec!["porch", "attic", "garage"];
names.sort();
println!("{names:?}");
```

## Quick check

```predict
let v = vec![3, 1, 2];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
println!("{:?} {:?}", v, doubled);
---
options: [3, 1, 2] [6, 2, 4] | [6, 2, 4] [6, 2, 4] | error: v was moved
answer: [3, 1, 2] [6, 2, 4]
why: `iter()` only borrows the items, so `v` is untouched and still usable.
```

## Your turn

Write `active_names(devices)`: return the **names** of the online devices, **sorted alphabetically**. The caller keeps its devices: you only get a borrowed slice.

## Example

```
active_names(&[porch light (online), garage door (offline), attic fan (online)])
  →  ["attic fan", "porch light"]
```

## Hints

1. Chain `devices.iter().filter(|d| d.online).map(|d| d.name.clone())`.
2. Finish with `.collect()` into a `Vec<String>`: you need `let mut names: Vec<String> = …;`.
3. Then `names.sort();` and return `names` as the last expression.

## Review

Tomorrow, solve this again, then sort by name length with `sort_by_key`.
