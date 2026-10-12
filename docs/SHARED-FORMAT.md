# Shared format

`learn-nodejs` and `learn-rust` are independent apps. They follow the same
**lesson format** and **history format** so they can later merge into one
multi-language tutor without losing any learning history. Keep this file
identical in both repos; change it in both or neither.

## 1. Lesson format

Each exercise is a folder named `lessons/NN-slug/` with:

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
kind: core                     # core (default) | drill | stretch
for: node-01-values-functions  # drills and stretches only: the core lesson they practise
```

**Kinds.** A *core* lesson moves the learner forward along the roadmap. Each
core lesson should have:
- one or more **drills** (`kind: drill`, ~10 min, a single concept or two,
  *new* code): used for reviews and as extra practice after a struggle;
- optionally a **stretch** (`kind: stretch`, harder, combines concepts):
  offered when the learner found the core lesson easy.

Name them after their core lesson so they sort next to it: `01a-drill-…`,
`01b-stretch-…`, then `02-…`. Section 5 says when each is offered.

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
| `GET /api/lessons` | | `[{ slug, id, title, minutes, concepts, kind, for, stats }]` |
| `GET /api/lessons/:slug` | | `{ slug, meta, body, file, code, saved, stats }` |
| `PUT /api/lessons/:slug/code` | code as plain text | `{ saved: true }` |
| `DELETE /api/lessons/:slug/code` | | `{ code, stats, next }` (back to the starter) |
| `POST /api/lessons/:slug/run` | code as plain text | run result (below) plus `stats`, `next` |
| `POST /api/lessons/:slug/feedback` | `easy`, `right` or `hard` | `{ stats, next }` |
| `GET /api/profile` | | `{ profile, markdown, next }` (section 4) |

`next` is the adaptive next step (section 5) with the `slug` of its lesson
folder added, or `slug: null`.

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
| `feedback` | the learner rates a solved exercise | `{ difficulty: "easy" \| "right" \| "hard" }` |

Run payload: `{ passed, failed, total, failing: [test names],
errors: [compiler error codes or titles], timed_out, code }`. `code` is
what was run, capped at 64 KB.

Reserved for the tutor (M4): `hint` and `question`. Reviews aren't a
separate kind: a passing run on a lesson that's due counts as a review. Sessions aren't recorded as events; a session is
derived as a run of events with no gap longer than 30 minutes.

Everything else (per-lesson stats, mastery, the review schedule, the learner
profile) is **derived** from events and can be rebuilt at any time. Merging
the two apps later means concatenating the two files and sorting by `ts`.

The API returns per-lesson `stats` derived from events:
`{ runs, first_pass_run, last_result, last_ts, best_passed, total }`, where
`total` is the largest test count seen (a run that doesn't compile reports 0
tests).

## 4. Learner profile

Derived from the events after every run and reset, and written into
`PROGRESS.md` between `<!-- profile:start -->` and `<!-- profile:end -->`
(everything outside that block is left alone). `PROGRESS.md` is committed,
so the tutor and any Claude session can read it; the raw events stay local.
`GET /api/profile` returns `{ profile, markdown }`.

Both apps must produce **exactly** the output in `docs/fixtures/`: every case
in `profile-cases.json` (one per adaptive path), and `profile-expected.md` for
the first case. Each app's tests check this. Change the rules in both apps and
regenerate the fixtures together.

**Review schedule (spaced repetition, per core lesson).** Intervals are 1,
3, 7, 16 and 35 days, as with language-learning flashcards.

- The first passing run solves the lesson: review step 0, due 1 day later.
- Once the lesson is solved, runs on its drills and stretches count as runs
  on the lesson for reviewing it. Drills and stretches have no schedule of
  their own.
- Runs before the due time don't count as reviews.
- Once due, a passing run moves up one step (capped at the last) and sets the
  next due date from that run. A failing run marks the review as lapsed; the
  next pass then restarts at step 0 (due 1 day later).

**Mastery (0–1, per concept).** Average over the lessons that list the
concept and have at least one run:

| Exercise state (any kind) | Score |
|--------------|-------|
| started, not solved | 0.2 |
| solved on run 1 / runs 2–3 / run 4 or later | 0.7 / 0.6 / 0.5 |
| plus each review step reached (core lessons) | +0.1 |
| review currently lapsed | −0.2 |

Clamped to 0–1 and rounded to 2 decimals; `null` if never practiced.

**Recurring mistakes.** Across failing runs, count each compiler error code
(or title) and each failing test name; keep those seen at least twice, most
frequent first (ties alphabetical), top 5.

**Sessions.** Events with gaps of at most 30 minutes form a session; total
minutes is the sum of session spans. The streak counts consecutive UTC days
with any event, ending today or yesterday.

**Pace.** The median of "solved on run" over solved core lessons, and how
many times each rating was given.

The lesson `stats` in the API also carry `review_step`, `due` and `due_now`
from the profile. The UI marks due lessons with ↻.

## 5. Adaptive next step

The app always recommends one next step, chosen from the history in this
order. The UI opens it on start and shows it after every run and rating;
`PROGRESS.md` records it.

1. **Review.** If core lessons are due, take the one due longest. Practise it
   with one of its drills (never-run first, then the one run longest ago),
   so the review is fresh code rather than a replay. With no drill, review
   the lesson itself (Reset, then solve from memory).
2. **React to the last solved core lesson**, unless the learner has already
   started a later core lesson:
   - it was **hard** (rated "too hard", or solved on run 4 or later without
     a "too easy" rating) → its first unsolved drill, as extra practice;
   - it was **easy** (rated "too easy") → its first unsolved stretch.
   A first-try solve without a rating just moves on: with under 3 hours a
   week, extra exercises are offered only when the learner signals for them.
3. **Current lesson.** The first unsolved core lesson. After 3 or more runs
   without solving it, offer its first unsolved drill as a warm-up instead.
4. **Done.** All core lessons solved: say when the next review is due.

`next` is `{ kind, lesson_id, title, reason }`, where `kind` is `review`,
`drill`, `stretch`, `lesson` or `done` (then `lesson_id` and `title` are
`null`). `reason` is a short human-readable explanation shown in the UI.

What's not adaptive yet, and comes with the tutor (M4): generating *new*
drills aimed at recurring mistakes, and explanations in the learner's style.
Those need an LLM; see `ROADMAP.md`.

## 6. Session shape (under 3 h/week, short daily sessions)

A ~20 minute session:

1. 3 min: one or two review items that are due
2. 15 min: the current lesson's exercise, run → feedback → fix
3. 2 min: one-line reflection ("what tripped me up"), saved as an event
