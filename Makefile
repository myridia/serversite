.PHONY: api

dev:
	cargo watch -x run -w src -w Cargo.toml -w config.json 
release:
	git pull;\
	cargo build --release
test:
	target/release/serversite-gen -o salamander

default: dev
