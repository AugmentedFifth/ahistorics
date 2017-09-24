.PHONY: debug release

debug:
	cargo clippy && cargo build

release:
	cargo rustc --release -- -C target-cpu=native && strip ./target/release/ahistorics
