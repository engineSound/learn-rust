---
id: rust-05-incoming-message
track: rust
title: Read an incoming message
topic: Result & errors
minutes: 12
concepts: [result, errors, parsing]
prereqs: [rust-04-active-devices]
contrast: java
edit: src/lib.rs
---

# Read an incoming message

## Learn the idea

Devices send readings as text like `fan=3.5`. Some messages arrive broken.

**Rust has no exceptions.** A function that can fail returns `Result<T, E>`: either `Ok(value)` or `Err(error)`. Like `Option`, you can't get at the value without deciding what happens on `Err`.

- `"3.5".parse::<f64>()` returns `Result<f64, ParseFloatError>`.
- `text.split_once('=')` returns `Option<(&str, &str)>`; `.ok_or("missing =")` turns `None` into an `Err`.
- The **`?` operator** means "if this is an `Err`, return it from my function right now; otherwise give me the value". It replaces Java's `throws` plus try/catch boilerplate.
- `.map_err(|_| …)` swaps an error for your own message.

```visual
{
 "title": "Following a Result through ?",
 "regions": [
  {
   "id": "in",
   "label": "text"
  },
  {
   "id": "steps",
   "label": "Each step with ?"
  },
  {
   "id": "res",
   "label": "Result"
  }
 ],
 "scenarios": [
  {
   "name": "A good message",
   "group": "How it works",
   "code": "fn parse(text: &str) -> Result<f64, String> {\n    let (_, v) = text.split_once('=').ok_or(\"missing =\")?;\n    let v: f64 = v.parse().map_err(|_| \"bad value\")?;\n    Ok(v)\n}\nprintln!(\"{:?}\", parse(\"fan=3.5\"));",
   "check": {
    "output": "Ok(3.5)"
   },
   "steps": [
    {
     "line": 6,
     "say": "The text arrives.",
     "set": [
      {
       "id": "t",
       "in": "in",
       "title": "text",
       "value": "\"fan=3.5\"",
       "tag": "&str",
       "state": "ok"
      }
     ]
    },
    {
     "line": 2,
     "say": "`split_once` finds the `=`: `Some((\"fan\", \"3.5\"))`. `ok_or` makes it `Ok`, and `?` unwraps it.",
     "rule": "`?` on `Ok(x)` gives you `x` and carries on.",
     "set": [
      {
       "id": "s1",
       "in": "steps",
       "title": "split_once + ?",
       "value": "Ok((\"fan\", \"3.5\"))",
       "tag": "continue",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "t",
       "to": "s1",
       "state": "muted"
      }
     ]
    },
    {
     "line": 3,
     "say": "`parse` succeeds: `Ok(3.5)`, and `?` hands over 3.5.",
     "set": [
      {
       "id": "s2",
       "in": "steps",
       "title": "parse + ?",
       "value": "Ok(3.5)",
       "tag": "continue",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "s1",
       "to": "s2",
       "state": "muted"
      }
     ]
    },
    {
     "line": 4,
     "say": "Both steps worked, so the function returns `Ok(3.5)`.",
     "rule": "Wrap the success value in `Ok(…)`.",
     "set": [
      {
       "id": "r",
       "in": "res",
       "title": "result",
       "value": "Ok(3.5)",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "s2",
       "to": "r",
       "state": "ok"
      }
     ]
    }
   ]
  },
  {
   "name": "A bad value",
   "group": "How it works",
   "code": "fn parse(text: &str) -> Result<f64, String> {\n    let (_, v) = text.split_once('=').ok_or(\"missing =\")?;\n    let v: f64 = v.parse().map_err(|_| \"bad value\")?;\n    Ok(v)\n}\nprintln!(\"{:?}\", parse(\"fan=hot\"));",
   "check": {
    "output": "Err(\"bad value\")"
   },
   "steps": [
    {
     "line": 6,
     "say": "This message has a word where the number should be.",
     "set": [
      {
       "id": "t",
       "in": "in",
       "title": "text",
       "value": "\"fan=hot\"",
       "tag": "&str",
       "state": "info"
      }
     ]
    },
    {
     "line": 2,
     "say": "The `=` is there, so the first `?` carries on.",
     "set": [
      {
       "id": "s1",
       "in": "steps",
       "title": "split_once + ?",
       "value": "Ok((\"fan\", \"hot\"))",
       "tag": "continue",
       "state": "ok"
      }
     ],
     "arrows": [
      {
       "from": "t",
       "to": "s1",
       "state": "muted"
      }
     ]
    },
    {
     "line": 3,
     "say": "`parse` fails; `map_err` swaps the error for \"bad value\", and `?` **returns it immediately**.",
     "rule": "`?` on `Err(e)` returns `Err(e)` from the whole function. No exceptions needed.",
     "set": [
      {
       "id": "s2",
       "in": "steps",
       "title": "parse + ?",
       "value": "Err(\"bad value\")",
       "tag": "return early",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "s1",
       "to": "s2",
       "state": "muted"
      }
     ]
    },
    {
     "line": 4,
     "say": "`Ok(v)` is never reached. The caller gets the error as a value it must handle.",
     "set": [
      {
       "id": "r",
       "in": "res",
       "title": "result",
       "value": "Err(\"bad value\")",
       "state": "info"
      }
     ],
     "arrows": [
      {
       "from": "s2",
       "to": "r",
       "label": "?",
       "state": "bad",
       "style": "solid"
      }
     ]
    }
   ]
  },
  {
   "name": "unwrap() on Err",
   "group": "What if it goes wrong",
   "code": "let v: f64 = \"hot\".parse().unwrap();\nprintln!(\"{v}\");",
   "check": {
    "error": "unwrap"
   },
   "steps": [
    {
     "line": 1,
     "say": "Parsing \"hot\" gives an `Err`…",
     "set": [
      {
       "id": "t",
       "in": "in",
       "title": "text",
       "value": "\"hot\"",
       "tag": "&str"
      },
      {
       "id": "p",
       "in": "steps",
       "title": "parse()",
       "value": "Err(invalid float literal)",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "t",
       "to": "p",
       "state": "muted"
      }
     ]
    },
    {
     "line": 1,
     "say": "…and `unwrap()` turns that into a **panic**: the program stops.",
     "rule": "Use `?`, `match` or `unwrap_or` for input you don't control.",
     "set": [
      {
       "id": "r",
       "in": "res",
       "title": "unwrap()",
       "tag": "panic!",
       "state": "bad"
      }
     ],
     "arrows": [
      {
       "from": "p",
       "to": "r",
       "state": "bad"
      }
     ],
     "error": "panicked: called `Result::unwrap()` on an `Err` value"
    }
   ]
  }
 ]
}
```

```run
fn parse_value(text: &str) -> Result<f64, String> {
    let value: f64 = text.trim().parse().map_err(|_| format!("not a number: {text}"))?;
    Ok(value * 2.0)
}

println!("{:?}", parse_value("3.5"));
println!("{:?}", parse_value("oops"));
```

## Quick check

```predict
let r = "3.5x".parse::<f64>();
println!("{}", r.is_ok());
---
options: true | false | panic
answer: false
why: Parsing fails, but that's just an `Err` value, not a crash. Only `unwrap()` would panic.
```

## Your turn

Write `parse_reading(text)`. For `"fan=3.5"` return `Ok(("fan".to_string(), 3.5))`. Otherwise return an `Err` with exactly one of these messages: `"missing ="` (no `=`), `"missing device"` (nothing before `=`), `"bad value"` (the value isn't a number). Spaces around the parts are allowed.

## Example

```
parse_reading("fan=3.5")   →  Ok(("fan", 3.5))
parse_reading("fan 3.5")   →  Err("missing =")
parse_reading("fan=hot")   →  Err("bad value")
```

## Hints

1. `let (device, value) = text.split_once('=').ok_or("missing =")?;` handles the first case.
2. Check `device.trim().is_empty()` and `return Err("missing device".to_string());`.
3. Parse with `value.trim().parse::<f64>().map_err(|_| "bad value".to_string())?`, then `Ok((device.trim().to_string(), value))`.

## Review

Tomorrow, solve this again, then return a custom `enum ParseError` instead of `String`.
