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

## Overview

## Date and Time Spans

`Timespan` can be used to create a time zone aware span consisting of `chrono::DateTime`s.

Currently the `DateTimeSpan` supports serialization and deserialization for the `chrono::Utc`,
`chrono::Local` and `chrono::FixedOffset` time zones. For support of other time zone types
please refer to the [documentation](https://docs.rs/timespan/).

When the `with-serde` feature is enabled `DateTimeSpan` has support for
[`serde`](https://github.com/serde-rs/serde) serialization and deserialization.

```rust
use timespan::DateTimeSpan;
use chrono::Utc;

let a: DateTimeSpan<Utc> = "2017-01-01T15:10:00 +0200 - 2017-01-02T09:30:00 +0200"
   .parse().unwrap();

assert!(
    format!("{}", a.format("{start} to {end}", "%c", "%c")) ==
    "Sun Jan  1 13:10:00 2017 to Mon Jan  2 07:30:00 2017"
);
```

## Individual Date Spans

A `DateSpan` can be used to create a time zone aware span consisting of `chrono::Date`s.

Currently the `DateSpan` does *not* support serialization and deserialization from strings.

```rust
use timespan::DateSpan;
use chrono_tz::Europe::Paris;

let a = DateSpan::from_utc_datespan(
    &"1789-06-17 - 1799-11-10".parse().unwrap(),
    &Paris,
);

let f = a.format(
    "The french revolution lasted from the {start} to the {end}.",
    "%eth of %B %Y",
    "%eth of %B %Y",
);
assert!(
    format!("{}", f) ==
    "The french revolution lasted from the 17th of June 1789 to the 10th of November 1799."
);
```

## Naive Date and Time Spans

The `NaiveDateSpan`, `NaiveTimeSpan` and `NaiveDateTimeSpan` are all not aware of time zones
and can be used for simple time spans.

All naive spans have full support for serialization and deserialization from strings.

When the `with-serde` feature is enabled all naive spans have support for
[`serde`](https://github.com/serde-rs/serde) serialization and deserialization.

```rust
use timespan::NaiveDateSpan;

let a: NaiveDateSpan = "2017-04-15 - 2017-08-15".parse().unwrap();
let b = NaiveDateSpan::parse_from_str(
    "15.04.17 - 15.08.17",
    "{start} - {end}",
    "%d.%m.%y", "%d.%m.%y"
).unwrap();

let f = a.format("from {start} to {end}", "%m/%d", "%m/%d");
assert!(format!("{}", f) == "from 04/15 to 08/15");
assert!(a == b);
```

```rust
use timespan::NaiveTimeSpan;

let a: NaiveTimeSpan = "17:30:00 - 19:15:00".parse().unwrap();
let b = NaiveTimeSpan::parse_from_str(
    "05.30 PM - 07.15 PM",
    "{start} - {end}",
    "%I.%M %P", "%I.%M %P"
).unwrap();

let f = a.format("from {start} to {end}", "%R", "%R");
assert!(format!("{}", f) == "from 17:30 to 19:15");
assert!(a == b);
```

```rust
use timespan::NaiveDateTimeSpan;

let a: NaiveDateTimeSpan = "2017-02-20T11:30:00 - 2017-02-23T18:00:00".parse().unwrap();
let b = NaiveDateTimeSpan::parse_from_str(
    "02/20/17 11.30 am - 02/23/17 06.00 pm",
    "{start} - {end}",
    "%D %I.%M %p", "%D %I.%M %p"
).unwrap();

let f = a.format("from {start} to {end}", "%R on %A", "%R on %A");
assert!(format!("{}", f) == "from 11:30 on Monday to 18:00 on Thursday");
assert!(a == b);
```

## How to Run the Examples

In order to run an example from the `example` folder issue the following command.

```sh
$ cargo run --example <name>
```

### The `convert` Example

Convert `from 10.30 to 14.00` to `10:30 - 14:00`:

```sh
$ cargo run --example convert -- "from 10.30 to 14.00" \
    "from {start} to {end}" "%H.%M" "%H.%M" \
    "{start} - {end}" "%R" "%R"
```

## License

This project is licensed under the GPL-v3 license - see the [LICENSE](LICENSE) file for details.

### duration
 - `NaiveDateTimeSpan::parse_from_str` of s, fmt, start, end -> `NaiveDateTimeSpan::duration`

### contains
 - `NaiveDateTimeSpan::parse_from_str` of s, fmt, start, end -> `NaiveDateTimeSpan::contains`
