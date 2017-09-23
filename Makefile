.PHONY: debug release

debug:
	cargo build --features "clippy"

release:
	cargo rustc --release -- -C target-cpu=native && strip ./target/release/ahistorics
