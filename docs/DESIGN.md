# Design

## What it is
A small web app I run on my Mac and later on AWS. It shows the next lesson,
lets me write Rust in the browser, compiles and tests it, and remembers
everything so a tutor (Claude) can adapt to me.

## Parts

```
browser ──► Axum server ──► runner (cargo test in a lesson crate, 60 s timeout)
                 │
                 ├──► SQLite: data/history.db  (event log, see SHARED-FORMAT.md)
                 ├──► profile builder → data/profile.md
                 └──► tutor: Claude API (profile + lesson + my code + compiler output)
```

- **Web UI:** server-rendered pages plus a code editor (CodeMirror). No JS build step.
- **Runner:** copies the lesson crate to a scratch directory, writes my code,
  runs `cargo test --message-format=json` with a shared target directory so
  rebuilds take seconds, and returns errors (with codes like `E0382`) and
  test results as JSON.
- **History:** append-only `events` table; everything else is derived.
  Compiler error codes are stored with each `fail` event, because repeated
  codes are the clearest sign of what to review.
- **Profile builder:** after each session, recomputes per-concept mastery and
  review dates (spaced repetition) and writes `data/profile.md`.
- **Tutor:** called only when I ask, or after repeated failures. Prompt =
  profile + lesson.md + my code + compiler output. API key in `.env`
  (`ANTHROPIC_API_KEY`), never committed.

## Running it
- **Mac (now):** `cargo run -p app`, open http://localhost:3001.
- **AWS (later):** the same app in a Docker container (with the Rust
  toolchain) on an existing EC2 instance, behind HTTPS and a login, because it
  compiles and runs code.

## Safety
The runner executes code I wrote. Only I use it, but it still runs with a
timeout, in a scratch directory, and in a container on AWS.
