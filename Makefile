.PHONY: debug release

debug:
	cargo clippy && cargo build

release:
	cargo rustc --release -- -C debuginfo=0 -C opt-level=3 -C target-cpu=native && strip ./target/release/ahistorics
