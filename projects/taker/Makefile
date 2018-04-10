SHELL := /bin/bash

.PHONY: build run sample

default: sample

build:
	RUST_LOG=taker cargo build

run:
	RUST_LOG=taker cargo run

clean:
	@echo "Removing 'test'"
	@rm -rf ~/Downloads/test/
	@echo "Removing 'tset'"
	@rm -rf ~/Downloads/tset/

sample: build clean run
