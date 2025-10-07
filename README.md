# Rust Simple Web Server

A compact, easy-to-understand Rust HTTP server suitable as a demo project on GitHub.
It demonstrates:
- Networking with `TcpListener` and `TcpStream`
- Basic HTTP parsing and response
- Concurrency with a custom thread pool
- Static file serving with MIME detection
- Logging with `log` + `env_logger`

## Features
- `GET /` serves `public/index.html`
- `GET /hello` returns a small JSON payload
- `GET /static/<file>` serves files from the `public/` directory (path-traversal safe)

## Build & Run
Make sure you have Rust installed (rustc + cargo).

```bash
cargo build --release
cargo run --release
```

Open `http://127.0.0.1:7878`

## Docker
```bash
docker build -t rust-simple-web-server .
docker run -p 7878:7878 rust-simple-web-server
```

## Testing & CI
A GitHub Actions workflow is included for:
- `cargo build --release`
- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`

## License
MIT
