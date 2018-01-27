# [fungi][]

> _A bunch of stuff, metallic things. And rusts and smuts, fungi... (cit.)_

## Install and build

```
brew install rustup-init
rustup self update
rustup update
rustup install nightly
rustup default nightly
rustup show
rustup toolchain list
rustup component add rust-src
rustup run nightly cargo install rustfmt-nightly --force
export DYLD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$DYLD_LIBRARY_PATH
```

```
rustup update
rustup toolchain install nightly
rustup component add rustfmt-preview --toolchain=nightly
```

## Versions

```
rustc --version
rustc 1.25.0-nightly (51b0b3734 2018-01-12)

rustfmt --version
0.3.4-nightly ( )

rustup --version
rustup 1.9.0 ( )

cargo --version
cargo 0.25.0-nightly (a88fbace4 2017-12-29)

cargo-fmt --version
0.3.4-nightly ( )

racer --version
racer 2.0.12
racer complete std::io::B

```

[fungi]: https://github.com/zeroed/fungi
