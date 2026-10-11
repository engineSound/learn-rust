//! learn-rust web app. Standard library only: no dependencies to download.
//! Start with `cargo run`, then open http://localhost:3001.

mod json;
mod lessons;
mod runner;

use json::Json;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

const MAX_BODY: usize = 256 * 1024;

struct Paths {
    lessons: PathBuf,
    data: PathBuf,
    public: PathBuf,
}

struct Request {
    method: String,
    path: String,
    body: String,
}

struct Response {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

impl Response {
    fn json(status: u16, value: Json) -> Response {
        Response { status, content_type: "application/json; charset=utf-8", body: value.to_string().into_bytes() }
    }

    fn error(status: u16, message: &str) -> Response {
        Response::json(status, Json::obj(vec![("error", Json::str(message))]))
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").to_path_buf();
    let paths: &'static Paths = Box::leak(Box::new(Paths {
        lessons: root.join("lessons"),
        data: root.join("data"),
        public: root.join("app").join("public"),
    }));
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".into());
    let listener = TcpListener::bind(format!("{host}:{port}")).unwrap_or_else(|e| panic!("can't listen on {host}:{port}: {e}"));
    println!("Learn Rust running at http://{}:{port}", if host == "0.0.0.0" { "localhost" } else { &host });

    for stream in listener.incoming().flatten() {
        thread::spawn(move || {
            if let Err(e) = serve(stream, paths) {
                eprintln!("request failed: {e}");
            }
        });
    }
}

fn serve(mut stream: TcpStream, paths: &Paths) -> std::io::Result<()> {
    let response = match read_request(&stream) {
        Ok(req) => handle(&req, paths),
        Err(msg) => Response::error(400, &msg),
    };
    let reason = match response.status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        _ => "Internal Server Error",
    };
    write!(
        stream,
        "HTTP/1.1 {} {reason}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        response.status,
        response.content_type,
        response.body.len()
    )?;
    stream.write_all(&response.body)?;
    stream.flush()
}

fn read_request(stream: &TcpStream) -> Result<Request, String> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|e| e.to_string())?;
    let mut parts = line.split_whitespace();
    let method = parts.next().ok_or("empty request")?.to_string();
    let target = parts.next().ok_or("no path")?;
    let path = target.split('?').next().unwrap_or("/").to_string();

    let mut length = 0usize;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).map_err(|e| e.to_string())?;
        let header = header.trim_end();
        if header.is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                length = value.trim().parse().map_err(|_| "bad Content-Length")?;
            }
        }
    }
    if length > MAX_BODY {
        return Err("Code is too large".into());
    }
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body).map_err(|e| e.to_string())?;
    let body = String::from_utf8(body).map_err(|_| "body is not UTF-8")?;
    Ok(Request { method, path, body })
}

fn work_path(paths: &Paths, lesson: &lessons::Lesson) -> PathBuf {
    paths.data.join("work").join(&lesson.slug).join(&lesson.file)
}

fn save_code(paths: &Paths, lesson: &lessons::Lesson, code: &str) -> std::io::Result<()> {
    let p = work_path(paths, lesson);
    fs::create_dir_all(p.parent().expect("work dir"))?;
    fs::write(p, code)
}

fn handle(req: &Request, paths: &Paths) -> Response {
    let method = req.method.as_str();
    let path = req.path.as_str();

    if method == "GET" && path == "/api/info" {
        return Response::json(
            200,
            Json::obj(vec![("track", Json::str("rust")), ("title", Json::str("Learn Rust")), ("accent", Json::str("#b4470f"))]),
        );
    }
    if method == "GET" && path == "/api/lessons" {
        let list = lessons::list(&paths.lessons)
            .into_iter()
            .map(|l| {
                Json::obj(vec![
                    ("slug", Json::str(l.slug.clone())),
                    ("id", l.get("id").cloned().unwrap_or(Json::Null)),
                    ("title", l.get("title").cloned().unwrap_or_else(|| Json::str(l.slug.clone()))),
                    ("minutes", l.get("minutes").cloned().unwrap_or(Json::Null)),
                    ("concepts", l.get("concepts").cloned().unwrap_or(Json::Arr(vec![]))),
                ])
            })
            .collect();
        return Response::json(200, Json::Arr(list));
    }

    if let Some(rest) = path.strip_prefix("/api/lessons/") {
        let (slug, action) = match rest.split_once('/') {
            Some((s, a)) => (s, a),
            None => (rest, ""),
        };
        let lesson = match lessons::load(&paths.lessons, slug) {
            Ok(Some(l)) => l,
            Ok(None) => return Response::error(404, "No such lesson"),
            Err(e) => return Response::error(500, &e),
        };
        let result: std::io::Result<Response> = match (method, action) {
            ("GET", "") => {
                let saved = fs::read_to_string(work_path(paths, &lesson)).ok();
                Ok(Response::json(
                    200,
                    Json::obj(vec![
                        ("slug", Json::str(lesson.slug.clone())),
                        ("meta", Json::Obj(lesson.meta.clone())),
                        ("body", Json::str(lesson.body.clone())),
                        ("file", Json::str(lesson.file.clone())),
                        ("saved", Json::Bool(saved.is_some())),
                        ("code", Json::Str(saved.unwrap_or_else(|| lesson.starter.clone()))),
                    ]),
                ))
            }
            ("PUT", "code") => save_code(paths, &lesson, &req.body).map(|_| Response::json(200, Json::obj(vec![("saved", Json::Bool(true))]))),
            ("DELETE", "code") => {
                fs::remove_file(work_path(paths, &lesson)).ok();
                Ok(Response::json(200, Json::obj(vec![("code", Json::str(lesson.starter.clone()))])))
            }
            ("POST", "run") => save_code(paths, &lesson, &req.body)
                .and_then(|_| runner::run_lesson(&lesson, &req.body, &paths.data))
                .map(|r| Response::json(200, r)),
            _ => Ok(Response::error(405, "Method not allowed")),
        };
        return result.unwrap_or_else(|e| Response::error(500, &e.to_string()));
    }

    if method == "GET" {
        let file = if path == "/" { "index.html" } else { &path[1..] };
        let safe = !file.is_empty() && file.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') && !file.starts_with('.');
        if safe {
            if let Ok(body) = fs::read(paths.public.join(file)) {
                let content_type = match file.rsplit('.').next() {
                    Some("html") => "text/html; charset=utf-8",
                    Some("css") => "text/css; charset=utf-8",
                    Some("js") => "text/javascript; charset=utf-8",
                    _ => "application/octet-stream",
                };
                return Response { status: 200, content_type, body };
            }
        }
    }
    Response::error(404, "Not found")
}
