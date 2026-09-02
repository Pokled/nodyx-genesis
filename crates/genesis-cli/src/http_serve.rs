//! Un serveur de fichiers minimal pour `genesis serve --port` : de quoi qu'OBS charge
//! `http://localhost:PORT/stream.html` et que l'overlay puisse `fetch` `live.json` et
//! `scene.json` (le `file://` bloque `fetch` pour cause de CORS).
//!
//! GET seulement, un thread par requete, aucune dependance. Ne sert que le dossier du monde,
//! et refuse tout chemin qui remonte hors de ce dossier.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};

/// Lance le serveur dans un thread detache. Renvoie le port reel ecoute (utile si `port` = 0).
pub fn spawn(root: PathBuf, port: u16) -> std::io::Result<u16> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let real = listener.local_addr()?.port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let root = root.clone();
            std::thread::spawn(move || {
                let _ = handle(stream, &root);
            });
        }
    });
    Ok(real)
}

fn handle(mut stream: TcpStream, root: &Path) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // On vide le reste des en-tetes.
    let mut h = String::new();
    loop {
        h.clear();
        if reader.read_line(&mut h)? == 0 || h == "\r\n" || h == "\n" {
            break;
        }
    }

    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_path = parts.next().unwrap_or("/");
    if method != "GET" && method != "HEAD" {
        return respond(&mut stream, 405, "text/plain", b"method not allowed", method == "HEAD");
    }

    let path = raw_path.split(['?', '#']).next().unwrap_or("/");
    let rel = path.trim_start_matches('/');
    let rel = if rel.is_empty() { "stream.html" } else { rel };

    // Anti remontee : aucun `..`, aucune racine absolue.
    let mut safe = PathBuf::new();
    for c in Path::new(rel).components() {
        match c {
            Component::Normal(p) => safe.push(p),
            _ => return respond(&mut stream, 400, "text/plain", b"bad path", method == "HEAD"),
        }
    }
    let full = root.join(&safe);
    if !full.starts_with(root) || !full.is_file() {
        return respond(&mut stream, 404, "text/plain", b"not found", method == "HEAD");
    }

    let mut body = Vec::new();
    std::fs::File::open(&full)?.read_to_end(&mut body)?;
    let ct = content_type(&full);
    write_head(&mut stream, 200, ct, body.len(), is_volatile(&full))?;
    if method != "HEAD" {
        stream.write_all(&body)?;
    }
    stream.flush()
}

fn respond(
    stream: &mut TcpStream,
    code: u16,
    ct: &str,
    body: &[u8],
    head_only: bool,
) -> std::io::Result<()> {
    write_head(stream, code, ct, body.len(), true)?;
    if !head_only {
        stream.write_all(body)?;
    }
    stream.flush()
}

fn write_head(
    stream: &mut TcpStream,
    code: u16,
    ct: &str,
    len: usize,
    no_cache: bool,
) -> std::io::Result<()> {
    let reason = match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "OK",
    };
    let cache = if no_cache {
        "Cache-Control: no-store\r\n"
    } else {
        "Cache-Control: max-age=5\r\n"
    };
    write!(
        stream,
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ct}\r\nContent-Length: {len}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n{cache}\r\n"
    )
}

/// Les fichiers que `serve` reecrit sans cesse : jamais de cache.
fn is_volatile(p: &Path) -> bool {
    matches!(
        p.file_name().and_then(|n| n.to_str()),
        Some("live.json" | "scene.json" | "records.json")
    )
}

fn content_type(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("json") | Some("jsonl") => "application/json; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("toml") | Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
