# AGENTS.md — serversite

## What this is
`serversite` — a small Rust static site generator that turns one `config.json` into a responsive dashboard HTML page. Used to publish the internal site index (salamander-jewelry.net services).

## Stack
- **Rust** (edition 2021), crate name `serversite-gen`
- Dependencies: `serde`, `serde_json` (with `preserve_order`), `tera` (templating), `ureq` (HTTP, for the CouchDB pull mode)
- **No runtime deps** — single compiled binary

## Structure
- `src/main.rs` — generator logic (config load, Tera render, file output)
- `templates/index.html.tera` — the single HTML template; receives the whole config as variable `cfg`
- `config.json` — site content (header, section arrays like `Websites`, `Servers`, `APIs`, contact, footer)
- `public/` — generated output (`public/index.html`)
- `index.html` — plain redirect to `public/index.html`
- `assets/` — technology logos referenced by the template
- `dockers/` — docker-compose + clean script for hosting the result
- `Makefile` — `make dev` = cargo watch, `make release` = release build

## How it works
1. Config source: `./serversite-gen [path-to-config.json]` (default `./config.json`).
2. CouchDB mode: `./serversite-gen -o <user> -d <doc>` fetches the doc from `https://cb.neriene.com/userdb-<hex-of-user>/<doc>` (default doc `config`), strips `_`-prefixed fields, saves it as `config.json`, then renders. Falls back to local config on fetch failure.
3. If no config file exists, writes and uses the built-in `DEFAULT_CONFIG`.
4. Tera renders `index.html.tera` with the config as `cfg`, writes `public/index.html` and the redirect `index.html`.

## Conventions
- Config shape is semi-fixed: `header`, section arrays (any key = section title), `contact`, `footer`. The template iterates known `Websites` / `Servers` / `APIs` sections; new section keys need template support.
- Emojis in config are HTML entities (e.g. `&#x1f4e1;`).
- Keep `config.json` and `templates/index.html.tera` in sync when adding sections.
- Verify with `cargo check` (run on the host — this dev container has no cargo).

## Environment caveat
No docker/cargo in this dev container — builds and compose commands must be run on the target host.