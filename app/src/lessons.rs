//! Reads lessons/NN-slug/lesson.md (format: docs/SHARED-FORMAT.md).

use crate::json::Json;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Lesson {
    pub slug: String,
    pub dir: PathBuf,
    pub meta: Vec<(String, Json)>,
    pub body: String,
    pub file: String,
    pub starter: String,
}

impl Lesson {
    pub fn get(&self, key: &str) -> Option<&Json> {
        self.meta.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

pub fn valid_slug(slug: &str) -> bool {
    let mut chars = slug.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Parse the small YAML subset used in lesson front matter.
pub fn parse_front_matter(text: &str) -> (Vec<(String, Json)>, String) {
    let text = text.replace("\r\n", "\n");
    let Some(rest) = text.strip_prefix("---\n") else {
        return (Vec::new(), text);
    };
    let Some(end) = rest.find("\n---") else {
        return (Vec::new(), text);
    };
    let mut meta = Vec::new();
    for raw in rest[..end].lines() {
        let line = match raw.find(" #") {
            Some(i) => &raw[..i],
            None => raw,
        }
        .trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                meta.push((key.to_string(), parse_value(value.trim())));
            }
        }
    }
    let body = rest[end + 4..].trim_start_matches('-').trim_start_matches('\n').to_string();
    (meta, body)
}

fn parse_value(v: &str) -> Json {
    if v.starts_with('[') && v.ends_with(']') {
        let inner = v[1..v.len() - 1].trim();
        if inner.is_empty() {
            return Json::Arr(Vec::new());
        }
        return Json::Arr(inner.split(',').map(|s| parse_value(s.trim())).collect());
    }
    if let Ok(n) = v.parse::<f64>() {
        return Json::Num(n);
    }
    let unquoted = v
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
        .unwrap_or(v);
    Json::str(unquoted)
}

pub fn list(lessons_dir: &Path) -> Vec<Lesson> {
    let Ok(entries) = fs::read_dir(lessons_dir) else {
        return Vec::new();
    };
    let mut slugs: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("lesson.md").is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|s| valid_slug(s))
        .collect();
    slugs.sort();
    slugs.iter().filter_map(|s| load(lessons_dir, s).ok().flatten()).collect()
}

/// Load one lesson. Ok(None) if there's no such lesson; Err if it's malformed.
pub fn load(lessons_dir: &Path, slug: &str) -> Result<Option<Lesson>, String> {
    if !valid_slug(slug) {
        return Ok(None);
    }
    let dir = lessons_dir.join(slug);
    let Ok(text) = fs::read_to_string(dir.join("lesson.md")) else {
        return Ok(None);
    };
    let (meta, body) = parse_front_matter(&text);
    let file = match meta.iter().find(|(k, _)| k == "edit") {
        Some((_, Json::Str(f))) if !f.contains("..") && !f.starts_with('/') => f.clone(),
        _ => return Err(format!("lesson {slug}: front matter needs an \"edit:\" file inside the lesson folder")),
    };
    let starter = fs::read_to_string(dir.join(&file)).map_err(|e| format!("lesson {slug}: can't read {file}: {e}"))?;
    Ok(Some(Lesson { slug: slug.to_string(), dir, meta, body, file, starter }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_matter_strings_numbers_lists_comments() {
        let (meta, body) = parse_front_matter("---\nid: x-01  # comment\nminutes: 20\nconcepts: [a, b]\nprereqs: []\n---\n# Hi\n");
        assert_eq!(
            meta,
            vec![
                ("id".to_string(), Json::str("x-01")),
                ("minutes".to_string(), Json::Num(20.0)),
                ("concepts".to_string(), Json::Arr(vec![Json::str("a"), Json::str("b")])),
                ("prereqs".to_string(), Json::Arr(vec![])),
            ]
        );
        assert_eq!(body, "# Hi\n");
    }

    #[test]
    fn slugs_reject_path_tricks() {
        assert!(valid_slug("01-bindings-expressions"));
        assert!(!valid_slug("../app"));
        assert!(!valid_slug(""));
        assert!(!valid_slug("Lesson"));
    }

    #[test]
    fn every_lesson_has_required_front_matter() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lessons");
        let lessons = list(&dir);
        assert!(!lessons.is_empty());
        for l in &lessons {
            for key in ["id", "track", "title", "minutes", "concepts", "edit"] {
                assert!(l.get(key).is_some(), "{} is missing {key}", l.slug);
            }
            assert_eq!(l.get("track"), Some(&Json::str("rust")));
            let kind = l.get("kind").and_then(Json::as_str).unwrap_or("core");
            assert!(["core", "drill", "stretch"].contains(&kind), "{}: unknown kind {kind}", l.slug);
            if kind != "core" {
                let parent = l.get("for");
                assert!(
                    lessons.iter().any(|o| o.get("id") == parent && o.get("kind").and_then(Json::as_str).unwrap_or("core") == "core"),
                    "{}: \"for\" must name a core lesson",
                    l.slug
                );
            }
        }
    }
}
