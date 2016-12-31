default: build

build:
	cargo build

run: build
	./target/debug/bronze

release:
	cargo build --release

run-release: release
	./target/debug/bronze

clean:
	cargo clean

.PHONY: clean run run-release