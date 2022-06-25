.PHONY: all test mdbook serve sync init schema build check watchtest

all: test check build doc

build:
	cargo build --all-targets
test:
	cargo test
check:
	cargo check
	cargo fmt
	cargo clippy
	cargo fix --allow-dirty
	cargo audit
	cargo +nightly udeps
doc:
	cargo doc --no-deps --document-private-items --open


