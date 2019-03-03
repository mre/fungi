.PHONY: updatetools

# https://github.com/racer-rust/emacs-racer
racer:
	rustup component add rust-src
	cargo install racer

# cargo +nightly install clippy --force
updatetools:
	rustup update
	rustup toolchain install nightly
	rustup component add rustfmt-preview --toolchain=nightly

