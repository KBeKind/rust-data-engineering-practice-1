format:
	cargo fmt --quiet

link:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

all:
	format lint test run