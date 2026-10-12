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
- [x] Learner profile (M3): concept mastery, spaced-repetition reviews, recurring mistakes and the next step, written into `PROGRESS.md` after every run
- [x] Adaptive path (M3.5): reviews use fresh drills; struggles get extra drills, easy wins get stretch exercises; rate each exercise too easy / about right / too hard
- [x] Tutor (M4, optional): hints for your code, a step-through visual of what your failing code really does, and new drills aimed at your mistakes, all checked before you see them
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

Without the app: `cd lessons/01-first-function && cargo test`.

## The tutor (optional)

Hints for your current code, a visual trace of your failing code, and new
drills written for your mistakes, from Claude. Without it, everything else still works.

1. Create an API key at https://console.anthropic.com/ (billed per use,
   separate from a Claude subscription; roughly 2–5 cents per hint, 5–15
   cents per visual trace and 10–30 cents per new drill).
2. `cp .env.example .env` and put the key after `ANTHROPIC_API_KEY=`.
   `.env` is never committed.
3. Restart `cargo run`. (The app calls the API with your Mac's built-in
   `curl`; nothing else to install.)

Then, under a failing result: **Show me what my code does**, which animates
your own code on the first failing check, step by step, up to where it goes
wrong (the app first runs the traced call on your real code, so the trace
can't claim anything your code doesn't do); **Ask** the tutor (optionally
type a question first); or
**Get a new exercise made for you**, which writes a drill aimed at what you find hard,
checks it against a working solution, and adds it to the lesson list.


## Docs

- [ROADMAP.md](ROADMAP.md): what to learn, in order, and the app milestones
- [docs/DESIGN.md](docs/DESIGN.md): how the app works
- [docs/SHARED-FORMAT.md](docs/SHARED-FORMAT.md): lesson and history formats shared with `learn-nodejs`
- [PROGRESS.md](PROGRESS.md): learning log; read by the tutor and by Claude
