# learn-rust

A personal Rust tutor that runs on my Mac (later on AWS). It picks the next
exercise from my learning history, compiles and tests my code in seconds,
and explains compiler errors in light of what I've struggled with before.

Sister project: `learn-nodejs`. Both follow [docs/SHARED-FORMAT.md](docs/SHARED-FORMAT.md)
so they can merge into one multi-language tutor later.

## Status

- [x] Lesson format, history format, roadmap
- [x] Lesson 01 (works from the command line today)
- [ ] Web app: lesson list, editor, Run button → compiler and test results
- [ ] History database + learner profile
- [ ] Tutor (Claude API) reading the profile
- [ ] Docker setup; deploy to AWS

## Doing a lesson today, before the app exists

Needs Rust via [rustup](https://rustup.rs).

```sh
cd lessons/01-bindings-expressions
cat lesson.md            # read (~5 min)
$EDITOR src/lib.rs       # replace each todo!()
cargo test               # repeat until green; read the compiler errors closely
```

Then add a line to [PROGRESS.md](PROGRESS.md).

## Docs

- [ROADMAP.md](ROADMAP.md): what to learn, in order, and the app milestones
- [docs/DESIGN.md](docs/DESIGN.md): how the app works
- [docs/SHARED-FORMAT.md](docs/SHARED-FORMAT.md): lesson and history formats shared with `learn-nodejs`
- [PROGRESS.md](PROGRESS.md): learning log; read by the tutor and by Claude
