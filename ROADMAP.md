# Roadmap

**Learner:** professional Java developer, intermediate Python, language
learner and researcher. **Goal:** general fluency plus backend/APIs.
**Pace:** short daily sessions (~20 min), under 3 hours a week. Rust takes
longer than Node; week numbers assume that pace, and slower is fine.

The order below is the *core* path. The app adapts around it (drills, stretches,
reviews; see `docs/SHARED-FORMAT.md`, section 5). Lessons contrast with Java first. Rust's hard part for Java developers is
ownership, so Module 2 gets the most time and the most review.

## Learning path

### Module 1: Rust from scratch (weeks 1–4), built ✅
One practical skill per exercise (~8–12 min), in the same running story as
`learn-nodejs`: the backend for a small fleet of home devices. Each has an
animated step-through visual (including what goes wrong, with the real
compiler error code), a runnable example, a quick check, the exercise and
three hints.

| # | Exercise | Skill | Key difference from Java |
|---|----------|-------|--------------------------|
| 01 | Your first Rust function | functions, `format!`, `&str` vs `String` | last expression is the return value; a stray `;` is E0308 |
| 02 | Convert a temperature | `f64`, `let mut`, shadowing | immutable by default; no silent number conversions |
| 03 | Decide when a battery is low | `Option`, `match` | no `null`; `if` needs a real `bool` |
| 04 | Pick active devices | structs, slices, iterators | `filter`/`map` are lazy until `collect` |
| 05 | Read an incoming message | `Result`, `?` | no exceptions; errors are values |
| 06 | Who owns the reading? | ownership, `&`, `&mut` | no GC: one owner, moves, borrows |
| 07 | Describe a device status | enums with data, exhaustive `match` | variants carry data; missing arm is E0004 |
| 08 | Count readings per device | `HashMap`, entry API | `map[key]` panics; `get` returns `Option` |
| 09 | Print a reading nicely | traits, `Display` | no inherited `toString()`; E0277 |
| 10 | Summarize the day | structs + `HashMap` + `Option` | `f64` has no `max()` (no `Ord`) |

Practice and challenge exercises so far: a drill and a challenge for 02; the
tutor generates more on demand.

### Module 2: Ownership in depth (weeks 5–7), the biggest shift from Java
| # | Lesson | |
|---|--------|---|
| 11 | The borrow checker's rules | one writer or many readers; E0499, E0502 |
| 12 | `String` vs `&str`, slices | owned vs borrowed data, returning slices |
| 13 | Iterating without fighting the borrow checker | `iter`, `iter_mut`, `into_iter` |
| 14 | Lifetimes, intro | what `'a` actually says |
| 15 | Smart pointers | `Box`, `Rc`, `RefCell`, when you need them |

### Module 3: Abstraction (weeks 8–10)
| # | Lesson | |
|---|--------|---|
| 16 | Traits in depth | default methods, impls for foreign types |
| 17 | Generics and trait bounds | monomorphization vs Java's type erasure |
| 18 | Closures and iterator adapters | `Fn`/`FnMut`/`FnOnce`, zero-cost chains |
| 19 | Error handling | custom errors, `From`, `thiserror` / `anyhow` |
| 20 | Modules and crates | `mod`, `pub`, `Cargo.toml` dependencies |

### Module 4: Backend (weeks 11–15)
| # | Lesson | |
|---|--------|---|
| 21 | `async` and Tokio | futures are lazy; compare with Node's event loop |
| 22 | `serde` | JSON in and out |
| 23 | Axum | routes, extractors, shared state |
| 24 | SQLite with `rusqlite` / `sqlx` | queries, migrations |
| 25 | Testing an API | integration tests |

### Capstone (week 16 onward)
A **vocabulary API with spaced repetition**, the same project as the
`learn-nodejs` capstone, so you can compare the two languages directly.

## App milestones

| Milestone | What you get |
|-----------|--------------|
| M1 ✅ | Website lists lessons; editor; **Run** shows compiler and test output |
| M2 ✅ | Every run, pass and fail saved to the history log (`data/events.jsonl`) |
| M3 ✅ | Learner profile generated after each session; review items scheduled |
| M3.5 ✅ | Adaptive path: reviews with fresh drills, extra drills after a struggle, stretches after an easy win, your own difficulty ratings |
| M4 ✅ | **Tutor (Claude API)**: compiler-error explanations and hints in your style; generates new drills aimed at your recurring mistakes, verified against a reference solution before you see them |
| M5 | Docker; deploy to an existing EC2 instance behind a login |
