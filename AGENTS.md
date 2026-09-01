# AGENTS.md — serversite

## What this is
Rust static site generator that produces a responsive server/service dashboard from a JSON config file, using Tera templates and Pico CSS.

## Stack
- Rust (edition 2021)
- Tera (templating)
- serde_json (JSON config)
- ureq (HTTP client for CouchDB mode)

## Build
```bash
make release
# or: cargo build --release
```

## Run
```bash
./target/release/serversite-gen
# or: ./serversite-gen /path/to/config.json
```
Serves generated `public/index.html` with any web server.

## Structure
- `src/main.rs` — generator logic
- `templates/index.html.tera` — HTML template
- `config.json` — site configuration
- `Cargo.toml` — Rust dependencies
- `Makefile` — build shortcuts
- `public/` — generated output
- `assets/` — service icons (PNG)
- `dockers/` — nginx docker setup

## Conventions
- No comments in code unless asked.
- Verify: `cargo check && cargo build`
