---
id: rust-03-ownership
track: rust
title: Who owns the reading?
topic: Ownership & borrowing
minutes: 12
concepts: [ownership, borrowing, references]
prereqs: [rust-02-types-functions]
contrast: java
edit: src/lib.rs
---

# Who owns the reading?

## Learn the idea

In Java, objects live on the heap and the garbage collector frees them once nothing refers to them. Rust has **no garbage collector**. Instead:

1. **Every value has exactly one owner.** When the owner goes out of scope, the value is freed (*dropped*).
2. **Assigning or passing a `String` moves it.** The new variable owns it and the old one can't be used any more; the compiler says **E0382**.
3. **To use a value without taking it, borrow it.** You can have any number of readers (`&T`) **or** exactly one writer (`&mut T`), never both at once.

Step through each scenario below, including the ones that go wrong.

```visual
{
 "title": "See it in action",
 "intro": "Pick a scenario and step through it. The left side is the stack (your variables), the right side is the heap (the actual data). Arrows show who points at what.",
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
 "scenarios": [
  {
   "name": "Move",
   "group": "How it works",
   "code": "let a = String::from(\"hi\");\nlet b = a;\nprintln!(\"{}\", b);\n}  // end of main",
   "check": {
    "code": "let a = String::from(\"hi\");\nlet b = a;\nprintln!(\"{}\", b);",
    "output": "hi"
   },
   "steps": [
    {
     "line": 1,
     "say": "`String::from` asks the heap for room and stores \"hi\" there. The variable `a` holds the pointer and **owns** that memory.",
     "rule": "Rule 1: every value has exactly one owner.",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner",
       "sub": "String",
       "state": "ok"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\"",
       "tag": "bytes",
       "sub": "owned by `a`"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`let b = a` **moves** ownership to `b`. Only the pointer is copied; the bytes stay put. `a` is now unusable.",
     "rule": "Rule 2: assigning a `String` moves it. Java would copy the reference and keep both alive.",
     "set": [
      {
       "id": "a",
       "state": "moved",
       "tag": "moved"
      },
      {
       "id": "b",
       "in": "stack",
       "title": "b",
       "tag": "owner",
       "sub": "String",
       "state": "ok"
      },
      {
       "id": "hi",
       "sub": "owned by `b`"
      }
     ],
     "arrows": [
      {
       "from": "b",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "`b` is the owner, so using it is fine.",
     "set": [
      {
       "id": "b",
       "state": "active"
      }
     ]
    },
    {
     "line": 4,
     "say": "At the end of `main`, the owner `b` goes out of scope: Rust frees the heap memory right there. No garbage collector.",
     "rule": "When the owner goes out of scope, the value is dropped.",
     "drop": [
      "a",
      "b",
      "hi"
     ],
     "arrows": []
    }
   ]
  },
  {
   "name": "Share a borrow (&)",
   "group": "How it works",
   "code": "let a = String::from(\"hi\");\nlet r1 = &a;\nlet r2 = &a;\nprintln!(\"{} {} {}\", a, r1, r2);",
   "check": {
    "output": "hi hi hi"
   },
   "steps": [
    {
     "line": 1,
     "say": "`a` owns the string.",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner",
       "sub": "String",
       "state": "ok"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\"",
       "sub": "owned by `a`"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`&a` **borrows** it: `r1` can read, but owns nothing.",
     "rule": "Rule 3: borrowing lets you use a value without taking it.",
     "set": [
      {
       "id": "r1",
       "in": "stack",
       "title": "r1",
       "tag": "&String · reader",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      },
      {
       "from": "r1",
       "to": "a",
       "label": "&",
       "state": "accent"
      }
     ]
    },
    {
     "line": 3,
     "say": "Another reader is fine: any number of `&` borrows can exist together.",
     "rule": "Many readers (`&T`) at once: allowed.",
     "set": [
      {
       "id": "r1",
       "state": "normal"
      },
      {
       "id": "r2",
       "in": "stack",
       "title": "r2",
       "tag": "&String · reader",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      },
      {
       "from": "r1",
       "to": "a",
       "label": "&",
       "state": "accent"
      },
      {
       "from": "r2",
       "to": "a",
       "label": "&",
       "state": "accent"
      }
     ]
    },
    {
     "line": 4,
     "say": "Owner and readers can all read at the same time.",
     "rule": "Readers can't change the value, so sharing is safe.",
     "set": [
      {
       "id": "r2",
       "state": "normal"
      }
     ]
    }
   ]
  },
  {
   "name": "Lend a writer (&mut)",
   "group": "How it works",
   "code": "let mut a = String::from(\"hi\");\nlet w = &mut a;\nw.push_str(\"!\");\nprintln!(\"{}\", a);",
   "check": {
    "output": "hi!"
   },
   "steps": [
    {
     "line": 1,
     "say": "`a` owns the string, and is declared `mut`, so it may change.",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner · mut",
       "sub": "String",
       "state": "ok"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\"",
       "sub": "owned by `a`"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`&mut a` lends a **writer**. While `w` is in use, nobody else may touch `a`, not even `a` itself.",
     "rule": "Exactly one writer (`&mut T`) at a time, and no readers meanwhile.",
     "set": [
      {
       "id": "w",
       "in": "stack",
       "title": "w",
       "tag": "&mut String · writer",
       "state": "active"
      },
      {
       "id": "a",
       "state": "faded",
       "tag": "lent out"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "muted"
      },
      {
       "from": "w",
       "to": "a",
       "label": "&mut",
       "state": "accent"
      }
     ]
    },
    {
     "line": 3,
     "say": "Through the writer, the string grows to \"hi!\".",
     "set": [
      {
       "id": "hi",
       "title": "\"hi!\"",
       "state": "active"
      }
     ]
    },
    {
     "line": 4,
     "say": "`w` is never used again, so the borrow is over and `a` is back in charge.",
     "rule": "A borrow lasts until its last use.",
     "set": [
      {
       "id": "a",
       "state": "ok",
       "tag": "owner · mut"
      },
      {
       "id": "hi",
       "state": "normal"
      }
     ],
     "drop": [
      "w"
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      }
     ]
    }
   ]
  },
  {
   "name": "Use after move",
   "group": "What if it goes wrong",
   "code": "let a = String::from(\"hi\");\nlet b = a;\nprintln!(\"{}\", a);",
   "check": {
    "error": "E0382"
   },
   "steps": [
    {
     "line": 1,
     "say": "`a` owns \"hi\".",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner",
       "state": "ok"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\"",
       "sub": "owned by `a`"
      }
     ],
     "arrows": [
      {
       "from": "a",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "Ownership moves to `b`.",
     "set": [
      {
       "id": "a",
       "state": "moved",
       "tag": "moved"
      },
      {
       "id": "b",
       "in": "stack",
       "title": "b",
       "tag": "owner",
       "state": "ok"
      },
      {
       "id": "hi",
       "sub": "owned by `b`"
      }
     ],
     "arrows": [
      {
       "from": "b",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "Using `a` again is refused at compile time: it no longer owns anything.",
     "rule": "If you need two owners, `clone()` makes a second copy.",
     "set": [
      {
       "id": "a",
       "state": "bad",
       "tag": "moved!"
      }
     ],
     "arrows": [
      {
       "from": "b",
       "to": "hi",
       "state": "ok"
      },
      {
       "from": "a",
       "to": "hi",
       "label": "✗",
       "state": "bad"
      }
     ],
     "error": "E0382: borrow of moved value: `a`"
    }
   ]
  },
  {
   "name": "Two writers",
   "group": "What if it goes wrong",
   "code": "let mut a = String::from(\"hi\");\nlet w1 = &mut a;\nlet w2 = &mut a;\nw1.push_str(\"!\");\nw2.push_str(\"?\");",
   "check": {
    "error": "E0499"
   },
   "steps": [
    {
     "line": 2,
     "say": "`w1` borrows `a` as a writer.",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner · mut",
       "state": "faded"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\""
      },
      {
       "id": "w1",
       "in": "stack",
       "title": "w1",
       "tag": "&mut",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "w1",
       "to": "a",
       "label": "&mut",
       "state": "accent"
      },
      {
       "from": "a",
       "to": "hi",
       "state": "muted"
      }
     ]
    },
    {
     "line": 3,
     "say": "A second writer while `w1` is still going to be used: two writers could change the same data at once.",
     "rule": "One writer at a time. This rule prevents data races at compile time.",
     "set": [
      {
       "id": "w2",
       "in": "stack",
       "title": "w2",
       "tag": "&mut",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "w1",
       "to": "a",
       "label": "&mut",
       "state": "accent"
      },
      {
       "from": "w2",
       "to": "a",
       "label": "&mut ✗",
       "state": "bad"
      },
      {
       "from": "a",
       "to": "hi",
       "state": "muted"
      }
     ],
     "error": "E0499: cannot borrow `a` as mutable more than once at a time"
    }
   ]
  },
  {
   "name": "Read while writing",
   "group": "What if it goes wrong",
   "code": "let mut a = String::from(\"hi\");\nlet w = &mut a;\nprintln!(\"{}\", a);\nw.push_str(\"!\");",
   "check": {
    "error": "E0502"
   },
   "steps": [
    {
     "line": 2,
     "say": "`w` is a writer, and it will be used again on line 4.",
     "set": [
      {
       "id": "a",
       "in": "stack",
       "title": "a",
       "tag": "owner · mut",
       "state": "faded"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\""
      },
      {
       "id": "w",
       "in": "stack",
       "title": "w",
       "tag": "&mut",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "w",
       "to": "a",
       "label": "&mut",
       "state": "accent"
      },
      {
       "from": "a",
       "to": "hi",
       "state": "muted"
      }
     ]
    },
    {
     "line": 3,
     "say": "Reading `a` here would see the data while it might be half-changed.",
     "rule": "No readers while a writer is active.",
     "set": [
      {
       "id": "a",
       "state": "bad",
       "tag": "read ✗"
      }
     ],
     "error": "E0502: cannot borrow `a` as immutable because it is also borrowed as mutable"
    }
   ]
  },
  {
   "name": "Dangling reference",
   "group": "What if it goes wrong",
   "code": "fn dangle() -> &String {\n    let s = String::from(\"hi\");\n    &s\n}",
   "check": {
    "error": "E0106"
   },
   "steps": [
    {
     "line": 2,
     "say": "Inside the function, `s` owns a new string.",
     "set": [
      {
       "id": "s",
       "in": "stack",
       "title": "s",
       "tag": "owner",
       "sub": "inside dangle()",
       "state": "ok"
      },
      {
       "id": "hi",
       "in": "heap",
       "title": "\"hi\"",
       "sub": "owned by `s`"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "hi",
       "state": "ok"
      }
     ]
    },
    {
     "line": 3,
     "say": "The function tries to return a reference to it…",
     "set": [
      {
       "id": "ret",
       "in": "stack",
       "title": "returned &String",
       "tag": "reference",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "hi",
       "state": "ok"
      },
      {
       "from": "ret",
       "to": "s",
       "label": "&",
       "state": "accent"
      }
     ]
    },
    {
     "line": 4,
     "say": "…but `s` is dropped when the function ends, so the reference would point at freed memory. Rust refuses to compile it.",
     "rule": "A reference can never outlive what it points to. Return the `String` itself instead.",
     "set": [
      {
       "id": "ret",
       "state": "bad",
       "tag": "points at nothing"
      }
     ],
     "drop": [
      "s",
      "hi"
     ],
     "arrows": [],
     "error": "E0106: missing lifetime specifier (it would be a dangling reference)"
    }
   ]
  }
 ]
}
```

```run
fn shout(text: &str) -> String {
    text.to_uppercase()
}

let reading = String::from("21.5°C ok");
let loud = shout(&reading);   // borrow: `reading` is still ours
println!("{reading} / {loud}");

let moved = reading;          // move: `reading` can't be used after this
println!("{moved}");
```

## Quick check

```predict
let a = String::from("fan");
let b = a.clone();
let c = b;
println!("{a} {c}");
---
options: fan fan | error: a was moved | error: b was moved
answer: fan fan
why: `clone` makes a second `String`, so `a` keeps its own. `b` moves into `c`, which is fine because `b` isn't used afterwards.
```

## Your turn

Three helpers for the device log. Each takes **exactly what it needs**:

- `with_unit(reading: &str) -> String` borrows the text and returns a new owned `String` with `°C` added.
- `total(readings: &[f64]) -> f64` borrows the readings and adds them up, so the caller keeps its `Vec`.
- `record(log: &mut Vec<f64>, value: f64)` borrows the log as a writer, appends the value, and keeps only the **last 3** readings.

## Example

```
with_unit("21.5")                        →  "21.5°C"
total(&vec![1.0, 2.5, 3.0])              →  6.5
record(&mut log, 4.0)  // log was [1.0, 2.0, 3.0]  →  log is [2.0, 3.0, 4.0]
```

## Hints

1. `with_unit`: `format!("{reading}°C")` builds a new `String`; the borrowed `&str` is only read.
2. `total`: `readings.iter().sum()` works, or a `for r in readings` loop adding `*r`.
3. `record`: `log.push(value);` then, if `log.len() > 3`, `log.remove(0);`. Through `&mut` you change the caller's vector directly.

## Review

Tomorrow, solve this again, then change `total` to take `Vec<f64>` (no `&`) and read the error the tests now give you.
