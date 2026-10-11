//! Runs a lesson's tests against the learner's code with `cargo test`.
//!
//! The lesson is copied to data/runs/<slug>/ (kept between runs) and built with a
//! shared target directory, so after the first build a run takes a second or two.

use crate::json::Json;
use crate::lessons::Lesson;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(120);
const MAX_OUTPUT: usize = 200_000;

/// One run at a time: runs share the scratch folder and the target directory.
static RUN_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, PartialEq)]
pub struct TestResult {
    pub name: String,
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub code: Option<String>,
    pub title: String,
    pub message: String,
}

pub fn run_lesson(lesson: &Lesson, code: &str, data_dir: &Path) -> std::io::Result<Json> {
    let _guard = RUN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let scratch = data_dir.join("runs").join(&lesson.slug);
    if scratch.exists() {
        fs::remove_dir_all(scratch.join("src")).ok();
    }
    copy_dir(&lesson.dir, &scratch)?;
    // Make the copy its own workspace so cargo doesn't look for the repo's workspace.
    let manifest = scratch.join("Cargo.toml");
    let mut toml = fs::read_to_string(&manifest)?;
    if !toml.contains("[workspace]") {
        toml.push_str("\n[workspace]\n");
        fs::write(&manifest, toml)?;
    }
    fs::write(scratch.join(&lesson.file), code)?;

    let started = Instant::now();
    let mut child = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["test", "--color", "never"])
        .current_dir(&scratch)
        .env("CARGO_TARGET_DIR", data_dir.join("target"))
        .env("CARGO_TERM_COLOR", "never")
        .env("RUST_BACKTRACE", "0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let out = read_async(child.stdout.take());
    let err = read_async(child.stderr.take());
    let mut timed_out = false;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if started.elapsed() > TIMEOUT {
            timed_out = true;
            child.kill().ok();
            child.wait().ok();
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    let stdout = out.join().unwrap_or_default();
    let stderr = err.join().unwrap_or_default();
    let duration_ms = started.elapsed().as_millis() as f64;

    let tests = parse_tests(&stdout);
    let diagnostics = parse_errors(&stderr);
    let passed = tests.iter().filter(|t| t.ok).count();
    let total = tests.len();
    let output = format!("{}\n{}", stderr.trim(), stdout.trim()).trim().to_string();

    Ok(Json::obj(vec![
        ("passed", Json::Num(passed as f64)),
        ("failed", Json::Num((total - passed) as f64)),
        ("total", Json::Num(total as f64)),
        (
            "tests",
            Json::Arr(
                tests
                    .into_iter()
                    .map(|t| Json::obj(vec![("name", Json::Str(t.name)), ("ok", Json::Bool(t.ok)), ("message", Json::Str(t.message))]))
                    .collect(),
            ),
        ),
        (
            "compileErrors",
            Json::Arr(
                diagnostics
                    .into_iter()
                    .map(|d| {
                        Json::obj(vec![
                            ("code", d.code.map(Json::Str).unwrap_or(Json::Null)),
                            ("title", Json::Str(d.title)),
                            ("message", Json::Str(d.message)),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("timedOut", Json::Bool(timed_out)),
        ("durationMs", Json::Num(duration_ms)),
        ("output", Json::Str(output)),
    ]))
}

fn read_async<R: Read + Send + 'static>(pipe: Option<R>) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut p) = pipe {
            let mut chunk = [0u8; 8192];
            while let Ok(n) = p.read(&mut chunk) {
                if n == 0 {
                    break;
                }
                if buf.len() < MAX_OUTPUT {
                    buf.extend_from_slice(&chunk[..n]);
                }
            }
        }
        String::from_utf8_lossy(&buf).into_owned()
    })
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "target" {
            continue;
        }
        let dest: PathBuf = to.join(&name);
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

/// Parse libtest's output: `test tests::name ... ok|FAILED` plus the failure sections.
pub fn parse_tests(stdout: &str) -> Vec<TestResult> {
    let mut tests: Vec<TestResult> = stdout
        .lines()
        .filter_map(|l| {
            let rest = l.strip_prefix("test ")?;
            let (name, status) = rest.rsplit_once(" ... ")?;
            let ok = match status.trim() {
                "ok" => true,
                "FAILED" => false,
                _ => return None,
            };
            Some(TestResult { name: name.to_string(), ok, message: String::new() })
        })
        .collect();

    // Sections look like: ---- tests::name stdout ----\n<panic message>\n
    let mut current: Option<String> = None;
    let mut buf: Vec<&str> = Vec::new();
    let flush = |name: &Option<String>, buf: &mut Vec<&str>, tests: &mut Vec<TestResult>| {
        if let Some(name) = name {
            if let Some(t) = tests.iter_mut().find(|t| &t.name == name) {
                t.message = clean_panic(buf);
            }
        }
        buf.clear();
    };
    for line in stdout.lines() {
        if let Some(name) = line.strip_prefix("---- ").and_then(|s| s.strip_suffix(" stdout ----")) {
            flush(&current, &mut buf, &mut tests);
            current = Some(name.to_string());
        } else if line == "failures:" || line.starts_with("test result:") {
            flush(&current, &mut buf, &mut tests);
            current = None;
        } else if current.is_some() {
            buf.push(line);
        }
    }
    flush(&current, &mut buf, &mut tests);

    for t in &mut tests {
        if let Some(short) = t.name.strip_prefix("tests::") {
            t.name = short.replace('_', " ");
        }
    }
    tests
}

fn clean_panic(lines: &[&str]) -> String {
    lines
        .iter()
        .filter(|l| !l.starts_with("note: "))
        .map(|l| {
            // "thread 'tests::x' (123) panicked at src/lib.rs:7:5:" -> "panicked at src/lib.rs:7:5:"
            match (l.starts_with("thread '"), l.find("panicked at ")) {
                (true, Some(i)) => &l[i..],
                _ => l,
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Pull compiler errors (not warnings) out of cargo's stderr.
pub fn parse_errors(stderr: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    let mut push = |block: &mut Vec<&str>| {
        if let Some(first) = block.first() {
            if let Some(d) = to_diagnostic(first, block) {
                out.push(d);
            }
        }
        block.clear();
    };
    for line in stderr.lines() {
        let starts_new = line.starts_with("error") || line.starts_with("warning") || line.starts_with("   Compiling") || line.starts_with("    Finished") || line.starts_with("     Running");
        if starts_new {
            push(&mut block);
        }
        block.push(line);
    }
    push(&mut block);
    out
}

fn to_diagnostic(first: &str, block: &[&str]) -> Option<Diagnostic> {
    let (code, title) = if let Some(rest) = first.strip_prefix("error[") {
        let (code, title) = rest.split_once("]: ")?;
        (Some(code.to_string()), title.to_string())
    } else if let Some(title) = first.strip_prefix("error: ") {
        // Cargo's own summary lines aren't useful to the learner.
        if title.starts_with("could not compile") || title.starts_with("test failed") || title.starts_with("aborting") {
            return None;
        }
        (None, title.to_string())
    } else {
        return None;
    };
    Some(Diagnostic { code, title, message: block.join("\n").trim_end().to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STDOUT: &str = "
running 2 tests
test tests::adds_numbers ... ok
test tests::subtracts ... FAILED

failures:

---- tests::subtracts stdout ----

thread 'tests::subtracts' (891) panicked at src/lib.rs:13:5:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::subtracts

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

    #[test]
    fn parses_results_and_panic_messages() {
        assert_eq!(
            parse_tests(STDOUT),
            vec![
                TestResult { name: "adds numbers".into(), ok: true, message: String::new() },
                TestResult { name: "subtracts".into(), ok: false, message: "panicked at src/lib.rs:13:5:\nnot yet implemented".into() },
            ]
        );
    }

    #[test]
    fn keeps_errors_with_codes_and_drops_warnings_and_summaries() {
        let stderr = "   Compiling lesson v0.1.0
warning: unused variable: `c`
 --> src/lib.rs:6:30

error[E0308]: mismatched types
 --> src/lib.rs:7:5
  |
7 |     c * 2.0;
  |             - help: remove this semicolon

error: could not compile `lesson` (lib test) due to 1 previous error
";
        let errs = parse_errors(stderr);
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].code.as_deref(), Some("E0308"));
        assert_eq!(errs[0].title, "mismatched types");
        assert!(errs[0].message.contains("remove this semicolon"));
    }

    /// End to end: copies lesson 01, runs real `cargo test`, and reads the results.
    #[test]
    fn runs_lesson_01_starter_broken_and_solved() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let lesson = crate::lessons::load(&root.join("lessons"), "01-bindings-expressions").unwrap().unwrap();
        let data = std::env::temp_dir().join(format!("learn-rust-test-{}", std::process::id()));
        let field = |j: &Json, key: &str| match j {
            Json::Obj(f) => f.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap(),
            _ => panic!("not an object"),
        };
        let len = |j: Json| match j {
            Json::Arr(a) => a.len(),
            _ => panic!("not an array"),
        };

        let starter = run_lesson(&lesson, &lesson.starter, &data).unwrap();
        assert_eq!(field(&starter, "passed"), Json::Num(0.0));
        assert_eq!(field(&starter, "total"), Json::Num(6.0));

        let broken = lesson.starter.replacen("    todo!()", "    c * 9.0 / 5.0 + 32.0;", 1);
        let r = run_lesson(&lesson, &broken, &data).unwrap();
        assert_eq!(len(field(&r, "compileErrors")), 1, "{}", r.to_string());
        assert!(r.to_string().contains("E0308"));

        let solved = lesson
            .starter
            .replacen("    todo!()", "    c * 9.0 / 5.0 + 32.0", 1)
            .replacen("    todo!()", r#"    if n < 0 { "negative" } else if n == 0 { "zero" } else { "positive" }"#, 1)
            .replacen("    todo!()", "    let mut t: u64 = 0; for i in 0..=n { if i % 2 == 0 { t += i as u64; } } t", 1)
            .replacen("    todo!()", "    let s = s.trim(); let s: i64 = s.parse().unwrap(); s * 2", 1);
        let r = run_lesson(&lesson, &solved, &data).unwrap();
        assert_eq!(field(&r, "passed"), Json::Num(6.0), "{}", r.to_string());
        assert_eq!(field(&r, "failed"), Json::Num(0.0));
        fs::remove_dir_all(&data).ok();
    }
}
