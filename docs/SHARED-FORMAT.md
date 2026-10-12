# Shared format

`learn-nodejs` and `learn-rust` are independent apps. They follow the same
**lesson format** and **history format** so they can later merge into one
multi-language tutor without losing any learning history. Keep this file
identical in both repos; change it in both or neither.

## 1. Lesson format

Each lesson is a folder named `lessons/NN-slug/` with:

| File | Purpose |
|------|---------|
| `lesson.md` | YAML front matter (below) + a short explanation, ~5 minutes of reading |
| exercise file(s) | Starter code with `TODO`s for the learner to fill in |
| test file(s) | Tests that decide pass/fail; the app only runs these |

Front matter:

```yaml
id: node-01-values-functions   # <track>-<NN>-<slug>, globally unique
track: nodejs                  # nodejs | rust | (future languages)
title: Values, equality and functions
minutes: 20                    # target length of one short daily session
concepts: [equality, let-const, arrow-functions, array-methods]
prereqs: []                    # lesson ids
contrast: java                 # language the explanation compares against
edit: exercise.js              # the file the learner edits (Rust: src/lib.rs)
```

`concepts` are the unit the tutor tracks. Name them in lowercase kebab-case
and reuse names across tracks when the idea is the same (e.g. `closures`,
`error-handling`, `async`). That reuse is how a future merged app notices
"you learned this in Node; here's the Rust version."

## 2. App API and front end

Both apps serve the same JSON API, and `app/public/` (the browser UI) is
**byte-for-byte identical** in both repos. Change it in both or neither.

| Method and path | Body | Returns |
|-----------------|------|---------|
| `GET /api/info` | | `{ track, title, accent }` |
| `GET /api/lessons` | | `[{ slug, id, title, minutes, concepts }]` |
| `GET /api/lessons/:slug` | | `{ slug, meta, body, file, code, saved }` |
| `PUT /api/lessons/:slug/code` | code as plain text | `{ saved: true }` |
| `DELETE /api/lessons/:slug/code` | | `{ code }` (the starter) |
| `POST /api/lessons/:slug/run` | code as plain text | run result (below) |

Run result: `{ passed, failed, total, tests: [{ name, ok, message }],
compileErrors: [{ code, title, message }], timedOut, durationMs, output }`.
`compileErrors[].code` is a compiler error code such as `E0308`, or `null`.

The learner's code is saved to `data/work/<slug>/<edit file>` (gitignored).

## 3. History format

One append-only event log in [JSON Lines](https://jsonlines.org):
`data/events.jsonl`, one event per line, oldest first. It is plain text (you
can read it in any editor) and needs no database or dependencies. The app
skips any line that isn't valid JSON, such as a half-written last line after
a crash.

```json
{"ts":"2026-10-12T09:30:00.000Z","track":"rust","lesson_id":"rust-01-bindings-expressions","concept":null,"kind":"fail","duration_ms":352,"payload":{"passed":0,"failed":0,"total":0,"failing":[],"errors":["E0308"],"timed_out":false,"code":"..."}}
```

| Field | Meaning |
|-------|---------|
| `ts` | ISO 8601, UTC, with milliseconds |
| `track` | `nodejs`, `rust`, … |
| `lesson_id` | from lesson front matter, or `null` |
| `concept` | a single concept, or `null` |
| `kind` | see below |
| `duration_ms` | how long the run took, or `null` |
| `payload` | kind-specific object, or `null` |

Kinds recorded today:

| `kind` | When | `payload` |
|--------|------|-----------|
| `lesson_open` | a lesson is opened | `null` |
| `pass` | a run where every test passes | run payload |
| `fail` | any other run (failures, compile errors, timeout) | run payload |
| `reset` | code reset to the starter | `null` |

Run payload: `{ passed, failed, total, failing: [test names],
errors: [compiler error codes or titles], timed_out, code }`. `code` is
what was run, capped at 64 KB.

Reserved for later milestones: `hint`, `question` (tutor, M4) and `review`
(spaced repetition, M3). Sessions aren't recorded as events; a session is
derived as a run of events with no gap longer than 30 minutes.

Everything else (per-lesson stats, mastery, the review schedule, the learner
profile) is **derived** from events and can be rebuilt at any time. Merging
the two apps later means concatenating the two files and sorting by `ts`.

The API returns per-lesson `stats` derived from events:
`{ runs, first_pass_run, last_result, last_ts, best_passed, total }`, where
`total` is the largest test count seen (a run that doesn't compile reports 0
tests).

## 4. Learner profile

Regenerated after each session from the events and saved as
`data/profile.md`. The tutor reads it at the start of every session. It holds:

- per concept: mastery (0–1), last practiced, next review date
- recurring errors (e.g. Rust `E0382` seen 4 times)
- what to do next

Review timing uses spaced repetition, as in language learning: each
successful recall pushes the next review further out (1, 3, 7, 16, 35 days),
and a failure resets it.

## 5. Session shape (under 3 h/week, short daily sessions)

A ~20 minute session:

1. 3 min: one or two review items that are due
2. 15 min: the current lesson's exercise, run → feedback → fix
3. 2 min: one-line reflection ("what tripped me up"), saved as an event
