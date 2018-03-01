.PHONY: updatetools

updatetools:
	rustup update
	rustup toolchain install nightly
	rustup component add rustfmt-preview --toolchain=nightly
	cargo +nightly install clippy --force
