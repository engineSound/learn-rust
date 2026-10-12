---
id: rust-10-daily-summary
track: rust
title: Summarize the day
topic: Putting it together
minutes: 12
concepts: [structs, hashmap, option, iteration]
prereqs: [rust-09-format-reading]
contrast: java
edit: src/lib.rs
---

# Summarize the day

## Learn the idea

End of the day: the hub sends one summary. This exercise combines what you've built: structs, `HashMap`, `Option` and borrowing.

One new thing. **Floats can't be compared with `max()`.** In Java, `Collections.max` on `Double`s works; in Rust, `f64` doesn't implement `Ord`, because `NaN` isn't greater, smaller or equal to anything. So `iter().max()` on floats doesn't compile (E0277). Two ways out:

- Keep the best so far in an `Option` and compare with `>` yourself.
- Or `.max_by(|a, b| a.total_cmp(b))`: `total_cmp` gives floats a total order.

Keeping "the best so far" in an `Option` is a handy pattern: `None` means "nothing seen yet", so an empty day is handled without special cases.

```visual
{
 "title": "Keeping the best so far",
 "intro": "`hottest` starts as `None` and is replaced each time a reading beats it.",
 "regions": [
  {
   "id": "in",
   "label": "readings"
  },
  {
   "id": "acc",
   "label": "Building the summary"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Find the hottest",
   "group": "How it works",
   "code": "let temps = [(\"hall\", 21.5), (\"kitchen\", 25.0), (\"hall\", 22.0)];\nlet mut hottest: Option<(&str, f64)> = None;\nfor (device, c) in temps {\n    if hottest.map_or(true, |(_, h)| c > h) {\n        hottest = Some((device, c));\n    }\n}\nprintln!(\"{hottest:?}\");",
   "check": {
    "output": "Some((\"kitchen\", 25.0))"
   },
   "steps": [
    {
     "line": 1,
     "say": "Three readings.",
     "set": [
      {
       "id": "r1",
       "in": "in",
       "title": "hall",
       "value": "21.5"
      },
      {
       "id": "r2",
       "in": "in",
       "title": "kitchen",
       "value": "25.0"
      },
      {
       "id": "r3",
       "in": "in",
       "title": "hall",
       "value": "22.0"
      }
     ]
    },
    {
     "line": 2,
     "say": "Nothing seen yet, so `hottest` is `None`.",
     "rule": "`None` = \"no best yet\". No fake starting value like `Double.MIN_VALUE`.",
     "set": [
      {
       "id": "h",
       "in": "acc",
       "title": "hottest",
       "value": "None",
       "tag": "Option<(&str, f64)>",
       "state": "info"
      }
     ]
    },
    {
     "line": 5,
     "say": "hall 21.5: `hottest` is `None`, so `map_or` returns `true`. Store it.",
     "rule": "`map_or(default, f)`: `default` for `None`, `f(value)` for `Some`.",
     "set": [
      {
       "id": "r1",
       "state": "active"
      },
      {
       "id": "h",
       "value": "Some(hall, 21.5)",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "r1",
       "to": "h",
       "label": "first one",
       "state": "ok"
      }
     ]
    },
    {
     "line": 5,
     "say": "kitchen 25.0 > 21.5: replace.",
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
       "id": "h",
       "value": "Some(kitchen, 25.0)",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "r2",
       "to": "h",
       "label": "25.0 > 21.5",
       "state": "ok"
      }
     ]
    },
    {
     "line": 4,
     "say": "hall 22.0 > 25.0? No. Keep kitchen.",
     "rule": "A strict `>` keeps the first reading on a tie.",
     "set": [
      {
       "id": "r2",
       "state": "faded"
      },
      {
       "id": "r3",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "r3",
       "to": "h",
       "label": "22.0 < 25.0",
       "state": "muted",
       "style": "dashed"
      }
     ]
    },
    {
     "line": 8,
     "say": "Done.",
     "set": [
      {
       "id": "r3",
       "state": "faded"
      },
      {
       "id": "o",
       "in": "out",
       "title": "Some((\"kitchen\", 25.0))"
      }
     ],
     "arrows": [
      {
       "from": "h",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "max() on floats",
   "group": "What if it goes wrong",
   "code": "let temps = vec![21.5, 25.0, 22.0];\nlet hottest = temps.iter().max();\nprintln!(\"{hottest:?}\");",
   "check": {
    "error": "E0277"
   },
   "steps": [
    {
     "line": 1,
     "say": "A `Vec<f64>`.",
     "set": [
      {
       "id": "t",
       "in": "in",
       "title": "temps",
       "value": "[21.5, 25.0, 22.0]",
       "tag": "Vec<f64>"
      }
     ]
    },
    {
     "line": 2,
     "say": "`max()` needs the items to implement `Ord`, a total order. `f64` doesn't: `NaN` can't be ordered.",
     "rule": "Use `max_by(|a, b| a.total_cmp(b))`, or compare with `>` yourself.",
     "set": [
      {
       "id": "m",
       "in": "acc",
       "title": ".max()",
       "value": "needs Ord",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "t",
       "to": "m",
       "label": "f64: Ord?",
       "state": "bad",
       "style": "dashed"
      }
     ],
     "error": "E0277: the trait bound `f64: Ord` is not satisfied"
    }
   ]
  },
  {
   "name": "NaN compares false",
   "group": "What if it goes wrong",
   "code": "let broken = f64::NAN;\nprintln!(\"{} {} {}\", broken > 1.0, broken < 1.0, broken == broken);",
   "check": {
    "output": "false false false"
   },
   "steps": [
    {
     "line": 1,
     "say": "A broken sensor reports NaN (\"not a number\").",
     "set": [
      {
       "id": "n",
       "in": "in",
       "title": "broken",
       "value": "NaN",
       "tag": "f64",
       "state": "bad"
      }
     ]
    },
    {
     "line": 2,
     "say": "Every comparison with NaN is `false`, even with itself. That's why `f64` has no `Ord`.",
     "rule": "Same as Java's `double`: Rust just makes you notice.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "false false false",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "n",
       "to": "o",
       "state": "bad"
      }
     ]
    }
   ]
  }
 ]
}
```

```run
let temps = [21.5, 25.0, 19.0];
let mut hottest: Option<f64> = None;
for t in temps {
    match hottest {
        Some(h) if h >= t => {}
        _ => hottest = Some(t),
    }
}
println!("{hottest:?}");
let best = temps.iter().copied().max_by(|a, b| a.total_cmp(b));
println!("{best:?}");
```

## Quick check

```predict
let temps: [f64; 0] = [];
let best = temps.iter().copied().max_by(|a, b| a.total_cmp(b));
println!("{best:?}");
---
options: 0.0 | None | panic
answer: None
why: An empty iterator has no maximum, so `max_by` returns `None`. No crash, no fake zero.
```

## Your turn

Write `summarize(readings)`. Return a `Summary` with the **total** number of readings, the **count per device**, and the **hottest** reading as `Some((device, celsius))`, or `None` when there are no readings. If two readings tie, keep the first.

## Example

```
[hall 21.5, kitchen 25.0, hall 22.0]
  →  Summary { total: 3, by_device: {"hall": 2, "kitchen": 1},
               hottest: Some(("kitchen", 25.0)) }
```

## Hints

1. Start with `let mut by_device = HashMap::new();` and `let mut hottest: Option<(String, f64)> = None;`, then loop over `readings`.
2. In the loop: count with the entry API, then replace `hottest` when it's `None` **or** the reading is strictly hotter than the stored one.
3. The update can be: `if hottest.as_ref().map_or(true, |(_, c)| r.celsius > *c) { hottest = Some((r.device.clone(), r.celsius)); }`. End with `Summary { total: readings.len(), by_device, hottest }`.

## Review

Tomorrow, solve this again, then add `coldest` using `min_by` and `total_cmp`.
