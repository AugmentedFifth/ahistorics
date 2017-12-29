.PHONY: debug release native

debug:
	cargo build

release:
	cargo rustc --release -- -C target-feature=+crt-static && strip ./target/release/ahistorics

native:
	cargo rustc --release -- -C target-cpu=native && strip ./target/release/ahistorics
