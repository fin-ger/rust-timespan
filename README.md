# Timespan Library for Rust
[![crates.io](https://img.shields.io/crates/v/timespan.svg)](https://crates.io/crates/timespan)
[![Latest Tag](https://img.shields.io/github/tag/fin-ger/rust-timespan.svg)](https://github.com/fin-ger/rust-timespan/releases)
[![Build Status](https://travis-ci.org/fin-ger/rust-timespan.svg?branch=master)](https://travis-ci.org/fin-ger/rust-timespan)
[![Documentation](https://docs.rs/timespan/badge.svg)](https://docs.rs/timespan/)
[![Homepage](https://img.shields.io/badge/github.io-homepage-blue.svg)](https://fin-ger.github.io/rust-timespan/)

A simple timespan for chrono times.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
timespan = "^0"
```

Or, if you want [Serde](https://github.com/serde-rs/serde) support, include it like this:

```toml
[dependencies]
timespan = { version = "^0", features = ["with-serde"] }
```

Then put this in your crate root:

```rust
extern crate timespan;
```

## How to Run the Examples

In order to run an example from the `example` folder issue the following command.

```
$ cargo run --example <name>
```

## License

This project is licensed under the GPL-v3 license - see the [LICENSE](LICENSE) file for details.
