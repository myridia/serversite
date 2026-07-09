.PHONY: api

dev:
	cargo watch -x run -w src -w Cargo.toml -w config.json 
release:
	git pull;\
	cargo build --release 
default: dev
