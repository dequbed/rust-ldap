# Default to be built
default: debug

# Build a release candidate
release:
	cargo build --release

# Build a debug candidate
debug:
	cargo build

test:
	cargo test

bench:
	cargo bench
