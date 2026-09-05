//! Le serveur de `genesis atlas` : l'arbre d'evolution du projet, la todo, la boite a idees.
//! GET et POST, un thread par requete, aucune dependance externe (meme esprit que
//! `http_serve.rs`, qui reste GET seul et sert les mondes en direct -- volontairement separe,
//! celui-ci ne doit jamais toucher a un monde qui tourne).
//!
//! Pas de multipart pour l'upload : le nom de fichier voyage dans l'en-tete `X-Filename`, le
//! corps de la requete est le fichier brut (`fetch(url, { headers: {'X-Filename': f.name},
//! body: f })`, un `File` est deja un `Blob`).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};

const TEMPLATE: &str = include_str!("atlas_template.html");

/// Lance le serveur dans un thread detache. Cree l'arborescence de donnees si absente.
pub fn spawn(root: PathBuf, port: u16) -> std::io::Result<u16> {
    std::fs::create_dir_all(root.join("inbox").join("files"))?;
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let real = listener.local_addr()?.port();
    // Un seul ecrivain a la fois sur les fichiers JSON : evite qu'un depot dans la boite a
    // idees et une sauvegarde de l'arbre, arrives en meme temps, ne s'ecrasent l'un l'autre.
    let lock = Arc::new(Mutex::new(()));
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let root = root.clone();
            let lock = lock.clone();
            std::thread::spawn(move || {
                let _ = handle(stream, &root, &lock);
            });
        }
    });
    Ok(real)
}

fn handle(mut stream: TcpStream, root: &Path, lock: &Mutex<()>) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let raw_path = parts.next().unwrap_or("/").to_string();

    let mut content_length: usize = 0;
    let mut filename_header: Option<String> = None;
    let mut h = String::new();
    loop {
        h.clear();
        if reader.read_line(&mut h)? == 0 || h == "\r\n" || h == "\n" {
            break;
        }
        let lower = h.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            content_length = v.trim().parse().unwrap_or(0);
        }
        if lower.starts_with("x-filename:") {
            filename_header = h.splitn(2, ':').nth(1).map(|v| v.trim().to_string());
        }
    }

    let path = raw_path.split(['?', '#']).next().unwrap_or("/").to_string();

    if method == "GET" {
        return handle_get(&mut stream, root, &path);
    }
    if method != "POST" {
        return respond(&mut stream, 405, "text/plain", b"method not allowed");
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    let _guard = lock.lock().unwrap_or_else(|e| e.into_inner());
    match path.as_str() {
        "/api/tree" => save_json(&mut stream, root, "data.json", &body),
        "/api/todo" => save_json(&mut stream, root, "todo.json", &body),
        "/api/inbox/note" => add_note(&mut stream, root, &body),
        "/api/inbox/upload" => add_upload(&mut stream, root, &body, filename_header.as_deref()),
        _ => respond(&mut stream, 404, "text/plain", b"not found"),
    }
}

fn handle_get(stream: &mut TcpStream, root: &Path, path: &str) -> std::io::Result<()> {
    match path {
        "/" | "" => respond(stream, 200, "text/html; charset=utf-8", TEMPLATE.as_bytes()),
        "/api/tree" => serve_file_or(stream, root, "data.json", b"{\"nodes\":{}}"),
        "/api/todo" => serve_file_or(stream, root, "todo.json", b"[]"),
        "/api/inbox" => serve_file_or(stream, root, "inbox/index.json", b"[]"),
        p if p.starts_with("/inbox/files/") => serve_upload_file(stream, root, p),
        _ => respond(stream, 404, "text/plain", b"not found"),
    }
}

fn safe_join(root: &Path, rel: &str) -> Option<PathBuf> {
    let mut safe = PathBuf::new();
    for c in Path::new(rel.trim_start_matches('/')).components() {
        match c {
            Component::Normal(p) => safe.push(p),
            _ => return None,
        }
    }
    let full = root.join(&safe);
    if full.starts_with(root) {
        Some(full)
    } else {
        None
    }
}

fn serve_file_or(stream: &mut TcpStream, root: &Path, rel: &str, default: &[u8]) -> std::io::Result<()> {
    let full = root.join(rel);
    let body = std::fs::read(&full).unwrap_or_else(|_| default.to_vec());
    respond(stream, 200, "application/json; charset=utf-8", &body)
}

fn serve_upload_file(stream: &mut TcpStream, root: &Path, path: &str) -> std::io::Result<()> {
    let Some(full) = safe_join(root, path) else {
        return respond(stream, 400, "text/plain", b"bad path");
    };
    if !full.is_file() {
        return respond(stream, 404, "text/plain", b"not found");
    }
    let body = std::fs::read(&full)?;
    let ct = content_type(&full);
    respond(stream, 200, ct, &body)
}

fn save_json(stream: &mut TcpStream, root: &Path, name: &str, body: &[u8]) -> std::io::Result<()> {
    if serde_json::from_slice::<serde_json::Value>(body).is_err() {
        return respond(stream, 400, "text/plain", b"invalid json");
    }
    std::fs::write(root.join(name), body)?;
    respond(stream, 200, "application/json; charset=utf-8", b"{\"ok\":true}")
}

fn add_note(stream: &mut TcpStream, root: &Path, body: &[u8]) -> std::io::Result<()> {
    #[derive(serde::Deserialize)]
    struct NoteIn {
        text: String,
    }
    let Ok(input) = serde_json::from_slice::<NoteIn>(body) else {
        return respond(stream, 400, "text/plain", b"invalid json");
    };
    let mut items = load_inbox(root);
    let id = format!("i{}", now_ms());
    items.push(serde_json::json!({
        "id": id, "kind": "note", "name": serde_json::Value::Null, "path": serde_json::Value::Null,
        "text": input.text, "added": now_ms(), "triaged": false
    }));
    write_inbox(root, &items)?;
    respond(stream, 200, "application/json; charset=utf-8", b"{\"ok\":true}")
}

fn add_upload(
    stream: &mut TcpStream,
    root: &Path,
    body: &[u8],
    filename: Option<&str>,
) -> std::io::Result<()> {
    let raw_name = filename.unwrap_or("fichier");
    let clean = sanitize_filename(raw_name);
    let id = format!("i{}", now_ms());
    let stored = format!("{id}_{clean}");
    std::fs::write(root.join("inbox").join("files").join(&stored), body)?;
    let mut items = load_inbox(root);
    items.push(serde_json::json!({
        "id": id, "kind": "file", "name": raw_name, "path": format!("files/{stored}"),
        "text": serde_json::Value::Null, "added": now_ms(), "triaged": false
    }));
    write_inbox(root, &items)?;
    respond(stream, 200, "application/json; charset=utf-8", b"{\"ok\":true}")
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if cleaned.is_empty() {
        "fichier".to_string()
    } else {
        cleaned
    }
}

fn now_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn load_inbox(root: &Path) -> Vec<serde_json::Value> {
    std::fs::read(root.join("inbox").join("index.json"))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}

fn write_inbox(root: &Path, items: &[serde_json::Value]) -> std::io::Result<()> {
    let body = serde_json::to_vec_pretty(items).unwrap_or_default();
    std::fs::write(root.join("inbox").join("index.json"), body)
}

fn respond(stream: &mut TcpStream, code: u16, ct: &str, body: &[u8]) -> std::io::Result<()> {
    let reason = match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "OK",
    };
    write!(
        stream,
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ct}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)?;
    stream.flush()
}

fn content_type(p: &Path) -> &'static str {
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "html" | "htm" => "text/html; charset=utf-8",
        "md" | "txt" => "text/plain; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "xml" => "application/xml",
        "csv" => "text/csv",
        "xlsx" | "xls" => "application/vnd.ms-excel",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        _ => "application/octet-stream",
    }
}
