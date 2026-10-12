# learn-rust

A personal Rust tutor that runs on my Mac (later on AWS). It picks the next
exercise from my learning history, compiles and tests my code in seconds,
and explains compiler errors in light of what I've struggled with before.

Sister project: `learn-nodejs`. Both follow [docs/SHARED-FORMAT.md](docs/SHARED-FORMAT.md)
so they can merge into one multi-language tutor later.

## Status

- [x] Lesson format, history format, roadmap
- [x] Lesson 01
- [x] Web app (M1): lesson list, editor, Run button → compiler errors and test results in about a second
- [x] Learning history (M2): every run recorded; each lesson shows runs, best score and when you solved it
- [ ] Learner profile and review schedule
- [ ] Tutor (Claude API) reading the profile
- [ ] Docker setup; deploy to AWS

## Start the app

Needs Rust via [rustup](https://rustup.rs). There are no crates to download:
the app uses only the standard library.

```sh
cargo run                # then open http://localhost:3001
```

Pick a lesson, read it on the left, write code on the right, and press
**Ctrl+Enter** (or **⌘+Enter**) to compile and run the tests. Compiler errors
appear with their codes (like `E0308`). Your code is saved as you type, in
`data/work/` (not committed). After a lesson, add a line to
[PROGRESS.md](PROGRESS.md).

`cargo test` at the repo root tests the app itself (lesson tests are meant to
fail until you solve them).

Without the app: `cd lessons/01-bindings-expressions && cargo test`.

## Docs

- [ROADMAP.md](ROADMAP.md): what to learn, in order, and the app milestones
- [docs/DESIGN.md](docs/DESIGN.md): how the app works
- [docs/SHARED-FORMAT.md](docs/SHARED-FORMAT.md): lesson and history formats shared with `learn-nodejs`
- [PROGRESS.md](PROGRESS.md): learning log; read by the tutor and by Claude
