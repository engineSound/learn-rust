# Roadmap

**Learner:** professional Java developer, intermediate Python, language
learner and researcher. **Goal:** general fluency plus backend/APIs.
**Pace:** short daily sessions (~20 min), under 3 hours a week. Rust takes
longer than Node; week numbers assume that pace, and slower is fine.

The order below is the *core* path. The app adapts around it (drills, stretches,
reviews; see `docs/SHARED-FORMAT.md`, section 5). Lessons contrast with Java first. Rust's hard part for Java developers is
ownership, so Phase 2 gets the most time and the most review.

## Learning path

### Phase 1: Basics (weeks 1–3)
| # | Lesson | Key difference from Java |
|---|--------|--------------------------|
| 01 | Bindings and expressions | immutable by default; `if` and blocks return values |
| 02 | Types and functions | sized integers, no implicit conversion, no overloading |
| 03 | Structs and `impl` | data and methods without classes or inheritance |
| 04 | Enums and `match` | enums carry data; `match` must be exhaustive |
| 05 | `Option` and `Result` | no `null`, no exceptions |

### Phase 2: Ownership (weeks 4–7), the biggest shift from Java
| # | Lesson | |
|---|--------|---|
| 06 | Moves | assignment moves; the borrow checker's first errors |
| 07 | Borrowing | `&` and `&mut`; one writer or many readers |
| 08 | `String` vs `&str`, slices | owned vs borrowed data |
| 09 | Collections | `Vec`, `HashMap`, iterating without fighting the borrow checker |
| 10 | Lifetimes, intro | what `'a` actually says |

### Phase 3: Abstraction (weeks 8–10)
| # | Lesson | |
|---|--------|---|
| 11 | Traits | like interfaces, plus default methods and impls for foreign types |
| 12 | Generics and trait bounds | monomorphization vs Java's type erasure |
| 13 | Closures and iterators | `map`/`filter`/`collect`, zero-cost |
| 14 | Error handling | `?`, custom errors, `thiserror` / `anyhow` |
| 15 | Modules and crates | `mod`, `pub`, `Cargo.toml` dependencies |

### Phase 4: Backend (weeks 11–15)
| # | Lesson | |
|---|--------|---|
| 16 | `async` and Tokio | futures are lazy; compare with Node's event loop |
| 17 | `serde` | JSON in and out |
| 18 | Axum | routes, extractors, shared state |
| 19 | SQLite with `rusqlite` / `sqlx` | queries, migrations |
| 20 | Testing an API | integration tests |

### Phase 5: Capstone (week 16 onward)
A **vocabulary API with spaced repetition**, the same project as the
`learn-nodejs` capstone, so you can compare the two languages directly.

## App milestones

| Milestone | What you get |
|-----------|--------------|
| M1 ✅ | Website lists lessons; editor; **Run** shows compiler and test output |
| M2 ✅ | Every run, pass and fail saved to the history log (`data/events.jsonl`) |
| M3 ✅ | Learner profile generated after each session; review items scheduled |
| M3.5 ✅ | Adaptive path: reviews with fresh drills, extra drills after a struggle, stretches after an easy win, your own difficulty ratings |
| M4 | **Tutor (Claude API)**: compiler-error explanations and hints in your style; generates new drills aimed at your recurring mistakes, verified against a reference solution before you see them |
| M5 | Docker; deploy to an existing EC2 instance behind a login |
