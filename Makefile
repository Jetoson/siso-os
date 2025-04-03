.PHONY: build run clean

build:
	cargo build --target x86_64-unknown-none

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/siso-os

clean:
	cargo clean
