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
```

`concepts` are the unit the tutor tracks. Name them in lowercase kebab-case
and reuse names across tracks when the idea is the same (e.g. `closures`,
`error-handling`, `async`). That reuse is how a future merged app notices
"you learned this in Node; here's the Rust version."

## 2. History format

One append-only event log, stored in SQLite (`data/history.db`):

```sql
CREATE TABLE events (
  id          INTEGER PRIMARY KEY,
  ts          TEXT    NOT NULL,          -- ISO 8601, UTC
  track       TEXT    NOT NULL,          -- nodejs | rust
  lesson_id   TEXT,                      -- from lesson front matter
  concept     TEXT,                      -- optional: a single concept
  kind        TEXT    NOT NULL,          -- see below
  duration_ms INTEGER,
  payload     TEXT                       -- JSON: code, error text, question...
);
```

`kind` is one of: `session_start`, `session_end`, `lesson_open`, `run`,
`pass`, `fail`, `hint`, `question`, `review`.

Everything else (mastery, review schedule, the learner profile) is
**derived** from events and can be rebuilt at any time. Merging the two apps
later means concatenating two event tables.

## 3. Learner profile

Regenerated after each session from the events and saved as
`data/profile.md`. The tutor reads it at the start of every session. It holds:

- per concept: mastery (0–1), last practiced, next review date
- recurring errors (e.g. Rust `E0382` seen 4 times)
- what to do next

Review timing uses spaced repetition, as in language learning: each
successful recall pushes the next review further out (1, 3, 7, 16, 35 days),
and a failure resets it.

## 4. Session shape (under 3 h/week, short daily sessions)

A ~20 minute session:

1. 3 min: one or two review items that are due
2. 15 min: the current lesson's exercise, run → feedback → fix
3. 2 min: one-line reflection ("what tripped me up"), saved as an event
