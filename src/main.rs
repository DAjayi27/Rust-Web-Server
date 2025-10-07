use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::Path;

mod threadpool;
mod static_files;

use threadpool::ThreadPool;
use log::{info, warn, error};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 2048];
    let size = match stream.read(&mut buffer) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to read from connection: {}", e);
            return;
        }
    };
    if size == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..size]).to_string();
    let request_line = request.lines().next().unwrap_or("");
    info!("Request: {}", request_line);

    // Very simple parsing: METHOD PATH HTTP/1.1
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    if method != "GET" {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write_all(response.as_bytes());
        return;
    }

    // Route handling
    match path {
        "/" => {
            match static_files::read_file("public/index.html") {
                Ok((body, mime)) => {
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
                        body.len(),
                        mime
                    );
                    let _ = stream.write_all(header.as_bytes());
                    let _ = stream.write_all(&body);
                }
                Err(e) => {
                    warn!("Failed to read index.html: {}", e);
                    let body = b"404 Not Found";
                    let response = format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.write_all(body);
                }
            }
        }
        "/hello" => {
            let body = b"{\"message\": \"Hello, world!\"}\n";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(body);
        }
        p if p.starts_with("/static/") => {
            // strip leading /static/
            let rel = &p[8..];
            let path = Path::new("public").join(rel);
            match static_files::read_file(path) {
                Ok((body, mime)) => {
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
                        body.len(),
                        mime
                    );
                    let _ = stream.write_all(header.as_bytes());
                    let _ = stream.write_all(&body);
                }
                Err(e) => {
                    warn!("Static file error for {}: {}", p, e);
                    let body = b"404 Not Found";
                    let response = format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.write_all(body);
                }
            }
        }
        _ => {
            let body = b"404 Not Found";
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(body);
        }
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server on http://127.0.0.1:7878");

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                error!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
