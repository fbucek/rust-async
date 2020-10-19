.PHONY: all test mdbook serve sync init schema build check watchtest

all: test check build doc

build:
	cargo build --all-targets
test:
	cargo test
check:
	cargo check
	cargo clippy
	cargo fmt
doc:
	cargo doc --no-deps --document-private-items --open


