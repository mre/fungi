SHELL := /bin/bash

.PHONY: build run clean

default: build

build:
	RUST_LOG=taker cargo build

run:
	RUST_LOG=taker cargo run

install:
	cargo build && cargo build --release && cargo install --path . --force

clean:
	@echo "Removing 'test'"
	@rm -rf ~/Downloads/test/
	@echo "Removing 'tset'"
	@rm -rf ~/Downloads/tset/

# sample: build clean run
