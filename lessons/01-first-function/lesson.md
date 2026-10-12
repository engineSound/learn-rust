---
id: rust-01-first-function
track: rust
title: Your first Rust function
topic: Functions & strings
minutes: 8
concepts: [functions, expressions, strings]
prereqs: []
contrast: java
edit: src/lib.rs
---

# Your first Rust function

## Learn the idea

You're writing the firmware helpers for a small fleet of home devices. Everything starts with functions.

A Rust function declares its parameter and return types, like a Java method, but it doesn't need a class around it:

| Java | Rust |
|------|------|
| `static String greet(String name) { … }` | `fn greet(name: &str) -> String { … }` |
| `String.format("Hi, %s!", name)` | `format!("Hi, {name}!")` |
| `return x;` | the last expression, **without `;`**, is the return value |

Two kinds of text: **`&str`** is borrowed text you only read (like a view), and **`String`** is text you own and can return or change. A function that builds new text takes `&str` and returns `String`.

```visual
{
 "title": "What happens when you call a function",
 "intro": "Step through a call: the argument goes in, a new `String` comes back.",
 "regions": [
  {
   "id": "caller",
   "label": "Your code"
  },
  {
   "id": "fn",
   "label": "Inside greeting(name)"
  },
  {
   "id": "out",
   "label": "Console",
   "kind": "output"
  }
 ],
 "scenarios": [
  {
   "name": "Call and return",
   "group": "How it works",
   "code": "fn greeting(name: &str) -> String {\n    format!(\"Hello, {name}!\")\n}\nlet text = greeting(\"Sam\");\nprintln!(\"{text}\");",
   "check": {
    "output": "Hello, Sam!"
   },
   "steps": [
    {
     "line": 4,
     "say": "Your code calls `greeting` with the text `\"Sam\"`.",
     "rule": "Calling a function runs its body with the arguments you pass.",
     "set": [
      {
       "id": "call",
       "in": "caller",
       "title": "greeting(\"Sam\")",
       "tag": "calling",
       "state": "active"
      }
     ]
    },
    {
     "line": 1,
     "say": "`name` **borrows** the text: it can read `\"Sam\"` but doesn't own it.",
     "rule": "`&str` = borrowed text. The caller keeps ownership.",
     "set": [
      {
       "id": "call",
       "state": "normal"
      },
      {
       "id": "name",
       "in": "fn",
       "title": "name",
       "value": "\"Sam\"",
       "tag": "&str · borrowed",
       "state": "active"
      }
     ],
     "arrows": [
      {
       "from": "name",
       "to": "call",
       "label": "&",
       "state": "accent"
      }
     ]
    },
    {
     "line": 2,
     "say": "`format!` builds a brand-new `String`, owned by the function for now.",
     "rule": "`format!` is like `String.format`, with `{name}` placeholders.",
     "set": [
      {
       "id": "name",
       "state": "normal"
      },
      {
       "id": "s",
       "in": "fn",
       "title": "format!(…)",
       "value": "\"Hello, Sam!\"",
       "tag": "String · owned",
       "state": "ok"
      }
     ],
     "arrows": []
    },
    {
     "line": 2,
     "say": "It's the last expression, with no `;`, so it's the return value. Ownership moves to the caller.",
     "rule": "The last expression without `;` is what the function returns.",
     "set": [
      {
       "id": "s",
       "state": "faded",
       "tag": "moved out"
      },
      {
       "id": "text",
       "in": "caller",
       "title": "text",
       "value": "\"Hello, Sam!\"",
       "tag": "String · owner",
       "state": "ok"
      },
      {
       "id": "call",
       "state": "faded",
       "tag": "done"
      }
     ],
     "arrows": [
      {
       "from": "s",
       "to": "text",
       "label": "return",
       "state": "ok",
       "style": "solid"
      }
     ]
    },
    {
     "line": 5,
     "say": "The function is done; `name` is gone. `text` owns the result.",
     "set": [
      {
       "id": "o",
       "in": "out",
       "title": "Hello, Sam!"
      }
     ],
     "drop": [
      "name",
      "s"
     ],
     "arrows": [
      {
       "from": "text",
       "to": "o",
       "state": "muted"
      }
     ]
    }
   ]
  },
  {
   "name": "Stray semicolon",
   "group": "What if it goes wrong",
   "code": "fn greeting(name: &str) -> String {\n    format!(\"Hello, {name}!\");\n}",
   "check": {
    "error": "E0308"
   },
   "steps": [
    {
     "line": 1,
     "say": "`greeting` promises to return a `String`.",
     "set": [
      {
       "id": "f",
       "in": "fn",
       "title": "greeting",
       "value": "-> String",
       "tag": "promise",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "With `;`, the `format!` line becomes a statement: the String is built, then dropped.",
     "rule": "A `;` turns an expression into a statement that returns nothing.",
     "set": [
      {
       "id": "s",
       "in": "fn",
       "title": "format!(…);",
       "value": "\"Hello, Sam!\"",
       "tag": "dropped",
       "state": "bad"
      }
     ]
    },
    {
     "line": 3,
     "say": "The body now returns `()` (nothing), not a `String`. The compiler refuses.",
     "rule": "Remove the `;` on the last line to return its value.",
     "set": [
      {
       "id": "f",
       "state": "bad",
       "value": "-> ()"
      }
     ],
     "error": "E0308: mismatched types: expected `String`, found `()`"
    }
   ]
  }
 ]
}
```

```run
fn greet(name: &str) -> String {
    format!("Hi, {name}! Your thermostat is online.")
}

println!("{}", greet("Ada"));
println!("{}", greet("Linus"));
```

## Quick check

```predict
let device = "kitchen sensor";
println!("Hello, {}", device.to_uppercase());
---
options: Hello, kitchen sensor | Hello, KITCHEN SENSOR | error
answer: Hello, KITCHEN SENSOR
why: `to_uppercase()` returns a new `String`; `{}` in `println!` prints it.
```

## Your turn

Every device greets its owner when it starts. Write `greeting(name)` so it **returns** `"Hello, "` followed by the name and an exclamation mark.

## Example

```
greeting("Sam")  →  "Hello, Sam!"
```

## Hints

1. Use `format!`: it works like `println!` but gives you the `String` instead of printing it.
2. Inside the string, `{name}` puts the value of `name` there.
3. `format!("Hello, {name}!")` is the whole body. No `return`, no `;`.

## Review

Tomorrow, solve this again, then add a `;` at the end on purpose and read what E0308 says.
