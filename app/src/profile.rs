//! Learner profile: derived from the event log (format: docs/SHARED-FORMAT.md, section 4).
//! learn-nodejs/app/profile.js implements the same rules; docs/fixtures/ keeps them in step.

use crate::history::iso_from_millis;
use crate::json::Json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const DAY: i64 = 86_400_000;
const SESSION_GAP: i64 = 30 * 60_000;
pub const REVIEW_DAYS: [i64; 5] = [1, 3, 7, 16, 35];
const START: &str = "<!-- profile:start -->";
const END: &str = "<!-- profile:end -->";

pub struct LessonRef {
    pub id: String,
    pub title: String,
    pub concepts: Vec<String>,
}

#[derive(Default)]
struct LessonState {
    solved: bool,
    step: usize,
    due: i64,
    lapsed: bool,
    runs: usize,
    first_pass: Option<usize>,
}

/// Milliseconds since the epoch for "YYYY-MM-DDTHH:MM:SS(.mmm)Z"; None if malformed.
pub fn parse_iso(ts: &str) -> Option<i64> {
    let b = ts.as_bytes();
    if b.len() < 20 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':' || b[16] != b':' {
        return None;
    }
    let n = |r: std::ops::Range<usize>| ts.get(r)?.parse::<i64>().ok();
    let (y, mo, d, h, mi, s) = (n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?);
    let millis = match ts.get(19..20) {
        Some(".") => n(20..23)?,
        _ => 0,
    };
    // Days-from-civil (Howard Hinnant's algorithm).
    let y = if mo <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = if mo > 2 { mo - 3 } else { mo + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * DAY + ((h * 60 + mi) * 60 + s) * 1000 + millis)
}

fn str_of<'a>(e: &'a Json, key: &str) -> Option<&'a str> {
    e.get(key).and_then(Json::as_str)
}

fn day(ts: &str) -> &str {
    ts.get(..10).unwrap_or(ts)
}

fn opt_num(n: Option<usize>) -> Json {
    n.map(|v| Json::Num(v as f64)).unwrap_or(Json::Null)
}

/// Build the profile. `lessons` are in lesson order, `events` oldest first.
pub fn build_profile(track: &str, lessons: &[LessonRef], events: &[Json], now: &str) -> Json {
    let now_ms = parse_iso(now).unwrap_or(0);
    let runs: Vec<(&Json, i64)> = events
        .iter()
        .filter(|e| matches!(str_of(e, "kind"), Some("pass" | "fail")))
        .filter_map(|e| Some((e, parse_iso(str_of(e, "ts")?)?)))
        .collect();

    let states: Vec<LessonState> = lessons
        .iter()
        .map(|l| {
            let mut st = LessonState::default();
            for (e, t) in runs.iter().filter(|(e, _)| str_of(e, "lesson_id") == Some(l.id.as_str())) {
                st.runs += 1;
                let pass = str_of(e, "kind") == Some("pass");
                if !st.solved {
                    if pass {
                        st.solved = true;
                        st.first_pass = Some(st.runs);
                        st.step = 0;
                        st.due = t + REVIEW_DAYS[0] * DAY;
                    }
                } else if *t >= st.due {
                    if pass {
                        st.step = if st.lapsed { 0 } else { (st.step + 1).min(REVIEW_DAYS.len() - 1) };
                        st.lapsed = false;
                        st.due = t + REVIEW_DAYS[st.step] * DAY;
                    } else {
                        st.lapsed = true;
                    }
                }
            }
            st
        })
        .collect();

    let score = |s: &LessonState| -> Option<f64> {
        if s.runs == 0 {
            return None;
        }
        if !s.solved {
            return Some(0.2);
        }
        let base = match s.first_pass {
            Some(1) => 0.7,
            Some(n) if n <= 3 => 0.6,
            _ => 0.5,
        };
        Some((base + 0.1 * s.step as f64 - if s.lapsed { 0.2 } else { 0.0 }).clamp(0.0, 1.0))
    };
    let mut concepts: Vec<(String, Vec<f64>, Vec<String>)> = Vec::new();
    for (l, s) in lessons.iter().zip(&states) {
        for name in &l.concepts {
            let i = match concepts.iter().position(|c| &c.0 == name) {
                Some(i) => i,
                None => {
                    concepts.push((name.clone(), Vec::new(), Vec::new()));
                    concepts.len() - 1
                }
            };
            concepts[i].2.push(l.id.clone());
            if let Some(v) = score(s) {
                concepts[i].1.push(v);
            }
        }
    }

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (e, _) in runs.iter().filter(|(e, _)| str_of(e, "kind") == Some("fail")) {
        let Some(p) = e.get("payload") else { continue };
        let errors = p.get("errors").map(Json::as_array).unwrap_or(&[]);
        let failing = p.get("failing").map(Json::as_array).unwrap_or(&[]);
        for what in errors.iter().chain(failing).filter_map(Json::as_str) {
            *counts.entry(what.to_string()).or_default() += 1;
        }
    }
    let mut recurring: Vec<(String, usize)> = counts.into_iter().filter(|(_, n)| *n >= 2).collect();
    recurring.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    recurring.truncate(5);

    let times: Vec<i64> = events.iter().filter_map(|e| parse_iso(str_of(e, "ts")?)).collect();
    let (mut sessions, mut span, mut start, mut prev) = (0, 0i64, 0i64, None::<i64>);
    for &t in &times {
        if prev.map_or(true, |p| t - p > SESSION_GAP) {
            if let Some(p) = prev {
                span += p - start;
            }
            sessions += 1;
            start = t;
        }
        prev = Some(t);
    }
    if let Some(p) = prev {
        span += p - start;
    }
    let days: BTreeSet<String> = events.iter().filter_map(|e| str_of(e, "ts")).map(|t| day(t).to_string()).collect();
    let mut cursor = if days.contains(day(now)) { now_ms } else { now_ms - DAY };
    let mut streak = 0;
    while days.contains(day(&iso_from_millis(cursor))) {
        streak += 1;
        cursor -= DAY;
    }

    struct Out<'a> {
        l: &'a LessonRef,
        s: &'a LessonState,
        due: Option<String>,
        due_now: bool,
    }
    let out: Vec<Out> = lessons
        .iter()
        .zip(&states)
        .map(|(l, s)| Out { l, s, due: s.solved.then(|| iso_from_millis(s.due)), due_now: s.solved && now_ms >= s.due })
        .collect();
    let status = |s: &LessonState| if s.solved { "solved" } else if s.runs > 0 { "started" } else { "new" };

    let mut due_now: Vec<&Out> = out.iter().filter(|o| o.due_now).collect();
    due_now.sort_by(|a, b| a.due.cmp(&b.due));
    let next = if let Some(o) = due_now.first() {
        ("review", Json::str(o.l.id.clone()), Json::str(o.l.title.clone()), format!("due since {}", day(o.due.as_deref().unwrap_or(""))))
    } else if let Some(o) = out.iter().find(|o| !o.s.solved) {
        let reason = if o.s.runs > 0 { "in progress" } else { "next new lesson" };
        ("lesson", Json::str(o.l.id.clone()), Json::str(o.l.title.clone()), reason.to_string())
    } else {
        let earliest = out.iter().filter_map(|o| o.due.clone()).min();
        let reason = match earliest {
            Some(d) => format!("all lessons solved; next review {}", day(&d)),
            None => "all lessons solved".to_string(),
        };
        ("done", Json::Null, Json::Null, reason)
    };

    Json::obj(vec![
        ("generated_at", Json::str(now)),
        ("track", Json::str(track)),
        (
            "sessions",
            Json::obj(vec![
                ("count", Json::Num(sessions as f64)),
                ("streak_days", Json::Num(streak as f64)),
                ("minutes_total", Json::Num((span as f64 / 60_000.0).round())),
                ("last_ts", events.last().and_then(|e| e.get("ts")).cloned().unwrap_or(Json::Null)),
            ]),
        ),
        (
            "lessons",
            Json::Arr(
                out.iter()
                    .map(|o| {
                        Json::obj(vec![
                            ("id", Json::str(o.l.id.clone())),
                            ("title", Json::str(o.l.title.clone())),
                            ("status", Json::str(status(o.s))),
                            ("runs", Json::Num(o.s.runs as f64)),
                            ("first_pass_run", opt_num(o.s.first_pass)),
                            ("review_step", opt_num(o.s.solved.then_some(o.s.step))),
                            ("due", o.due.clone().map(Json::Str).unwrap_or(Json::Null)),
                            ("due_now", Json::Bool(o.due_now)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "concepts",
            Json::Arr(
                concepts
                    .into_iter()
                    .map(|(name, scores, ids)| {
                        let mastery = if scores.is_empty() {
                            Json::Null
                        } else {
                            Json::Num((scores.iter().sum::<f64>() / scores.len() as f64 * 100.0).round() / 100.0)
                        };
                        Json::obj(vec![("name", Json::Str(name)), ("mastery", mastery), ("lessons", Json::Arr(ids.into_iter().map(Json::Str).collect()))])
                    })
                    .collect(),
            ),
        ),
        (
            "recurring",
            Json::Arr(recurring.into_iter().map(|(what, count)| Json::obj(vec![("what", Json::Str(what)), ("count", Json::Num(count as f64))])).collect()),
        ),
        ("next", Json::obj(vec![("kind", Json::str(next.0)), ("lesson_id", next.1), ("title", next.2), ("reason", Json::Str(next.3))])),
    ])
}

pub fn profile_markdown(p: &Json) -> String {
    let s = |j: Option<&Json>| j.and_then(Json::as_str).unwrap_or("").to_string();
    let n = |j: Option<&Json>| j.and_then(Json::as_f64).unwrap_or(0.0);
    let generated = s(p.get("generated_at"));
    let mut out = vec![
        START.to_string(),
        "<!-- Generated by the app from data/events.jsonl. Edits inside this block are overwritten. -->".to_string(),
        "## Learner profile".to_string(),
        String::new(),
        format!("Updated {} {} UTC.", generated.get(..10).unwrap_or(""), generated.get(11..16).unwrap_or("")),
        String::new(),
    ];
    let next = p.get("next");
    let (kind, title, reason) = (s(next.and_then(|x| x.get("kind"))), s(next.and_then(|x| x.get("title"))), s(next.and_then(|x| x.get("reason"))));
    out.push(match kind.as_str() {
        "review" => format!("**Next:** review \"{title}\" ({reason})."),
        "lesson" => format!("**Next:** \"{title}\" ({reason})."),
        _ => format!("**Next:** {reason}."),
    });
    out.push(String::new());
    let sess = p.get("sessions");
    let count = n(sess.and_then(|x| x.get("count")));
    out.push(if count > 0.0 {
        let streak = n(sess.and_then(|x| x.get("streak_days")));
        format!(
            "**Sessions:** {}, {} min in total, streak {} {}, last on {}.",
            count,
            n(sess.and_then(|x| x.get("minutes_total"))),
            streak,
            if streak == 1.0 { "day" } else { "days" },
            day(&s(sess.and_then(|x| x.get("last_ts"))))
        )
    } else {
        "**Sessions:** none yet.".to_string()
    });
    out.push(String::new());
    out.push("| Lesson | Status | Runs | Solved on run | Review |".to_string());
    out.push("|--------|--------|------|---------------|--------|".to_string());
    for l in p.get("lessons").map(Json::as_array).unwrap_or(&[]) {
        let review = if l.get("due_now").and_then(Json::as_bool) == Some(true) {
            "due now".to_string()
        } else {
            match l.get("due").and_then(Json::as_str) {
                Some(d) => format!("next {}", day(d)),
                None => "–".to_string(),
            }
        };
        let first = match l.get("first_pass_run").and_then(Json::as_f64) {
            Some(v) => v.to_string(),
            None => "–".to_string(),
        };
        out.push(format!("| {} | {} | {} | {first} | {review} |", s(l.get("title")), s(l.get("status")), n(l.get("runs"))));
    }
    out.push(String::new());
    let practiced: Vec<&Json> = p.get("concepts").map(Json::as_array).unwrap_or(&[]).iter().filter(|c| c.get("mastery").and_then(Json::as_f64).is_some()).collect();
    if practiced.is_empty() {
        out.push("No concepts practiced yet.".to_string());
    } else {
        out.push("| Concept | Mastery |".to_string());
        out.push("|---------|---------|".to_string());
        for c in practiced {
            out.push(format!("| {} | {:.2} |", s(c.get("name")), n(c.get("mastery"))));
        }
    }
    out.push(String::new());
    let recurring: Vec<String> = p
        .get("recurring")
        .map(Json::as_array)
        .unwrap_or(&[])
        .iter()
        .map(|r| format!("`{}` ×{}", s(r.get("what")), n(r.get("count"))))
        .collect();
    out.push(if recurring.is_empty() { "**Recurring mistakes:** none yet.".to_string() } else { format!("**Recurring mistakes:** {}.", recurring.join(", ")) });
    out.push(END.to_string());
    out.join("\n")
}

/// Replace the generated block in PROGRESS.md (or insert it before "## Log").
pub fn write_progress(file: &Path, markdown: &str) -> std::io::Result<()> {
    let text = fs::read_to_string(file).unwrap_or_default();
    let next = match (text.find(START), text.find(END)) {
        (Some(a), Some(b)) if b > a => format!("{}{}{}", &text[..a], markdown, &text[b + END.len()..]),
        _ if text.contains("\n## Log") => text.replacen("\n## Log", &format!("\n{markdown}\n\n## Log"), 1),
        _ => {
            let sep = if text.is_empty() { "" } else if text.ends_with('\n') { "\n" } else { "\n\n" };
            format!("{text}{sep}{markdown}\n")
        }
    };
    if next != text {
        fs::write(file, next)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::parse;

    fn fixture(name: &str) -> String {
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/fixtures").join(name)).unwrap()
    }

    fn lessons_from(j: &Json) -> Vec<LessonRef> {
        j.as_array()
            .iter()
            .map(|l| LessonRef {
                id: l.get("id").and_then(Json::as_str).unwrap().to_string(),
                title: l.get("title").and_then(Json::as_str).unwrap().to_string(),
                concepts: l.get("concepts").map(Json::as_array).unwrap_or(&[]).iter().filter_map(Json::as_str).map(String::from).collect(),
            })
            .collect()
    }

    #[test]
    fn iso_round_trip() {
        for ts in ["1970-01-01T00:00:00.000Z", "2000-02-29T23:59:59.999Z", "2026-10-05T21:00:00.000Z"] {
            assert_eq!(iso_from_millis(parse_iso(ts).unwrap()), ts);
        }
        assert_eq!(parse_iso("2026-10-05T21:00:00Z"), parse_iso("2026-10-05T21:00:00.000Z"));
        assert_eq!(parse_iso("yesterday"), None);
    }

    #[test]
    fn matches_the_shared_fixture() {
        let input = parse(&fixture("profile-input.json")).unwrap();
        let events = input.get("events").unwrap().as_array().to_vec();
        let profile = build_profile(
            input.get("track").and_then(Json::as_str).unwrap(),
            &lessons_from(input.get("lessons").unwrap()),
            &events,
            input.get("now").and_then(Json::as_str).unwrap(),
        );
        assert_eq!(profile, parse(&fixture("profile-expected.json")).unwrap());
        assert_eq!(profile_markdown(&profile) + "\n", fixture("profile-expected.md"));
    }

    #[test]
    fn empty_history_and_passing_reviews() {
        let lessons = vec![LessonRef { id: "a".into(), title: "A".into(), concepts: vec!["c".into()] }];
        let empty = build_profile("t", &lessons, &[], "2026-01-01T00:00:00.000Z");
        assert_eq!(empty.get("next").and_then(|n| n.get("reason")), Some(&Json::str("next new lesson")));
        assert_eq!(empty.get("sessions").and_then(|s| s.get("count")), Some(&Json::Num(0.0)));

        let ev = |ts: &str| parse(&format!(r#"{{"ts":"{ts}","lesson_id":"a","kind":"pass","payload":{{"failing":[],"errors":[]}}}}"#)).unwrap();
        let reviewed = build_profile("t", &lessons, &[ev("2026-01-01T10:00:00.000Z"), ev("2026-01-02T11:00:00.000Z"), ev("2026-01-05T12:00:00.000Z")], "2026-01-06T00:00:00.000Z");
        let lesson = &reviewed.get("lessons").unwrap().as_array()[0];
        assert_eq!(lesson.get("review_step"), Some(&Json::Num(2.0)));
        assert_eq!(lesson.get("due"), Some(&Json::str("2026-01-12T12:00:00.000Z")));
        assert_eq!(reviewed.get("concepts").unwrap().as_array()[0].get("mastery"), Some(&Json::Num(0.9)));
        assert_eq!(reviewed.get("next").and_then(|n| n.get("reason")), Some(&Json::str("all lessons solved; next review 2026-01-12")));
    }

    #[test]
    fn write_progress_replaces_block_and_keeps_the_rest() {
        let file = std::env::temp_dir().join(format!("learn-prog-{}.md", std::process::id()));
        fs::write(&file, "# Progress\n\nintro\n\n## Log\n| a |\n").unwrap();
        write_progress(&file, "<!-- profile:start -->\nONE\n<!-- profile:end -->").unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "# Progress\n\nintro\n\n<!-- profile:start -->\nONE\n<!-- profile:end -->\n\n## Log\n| a |\n");
        write_progress(&file, "<!-- profile:start -->\nTWO\n<!-- profile:end -->").unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "# Progress\n\nintro\n\n<!-- profile:start -->\nTWO\n<!-- profile:end -->\n\n## Log\n| a |\n");
        fs::remove_file(&file).ok();
    }
}
