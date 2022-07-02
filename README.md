# parse_duration2
[![Crates.io](https://img.shields.io/crates/v/parse_duration2.svg)](https://crates.io/crates/parse_duration2)

This is a fork of [zeta12ti's `parse_duration`](https://github.com/zeta12ti/parse_duration) that fixes a DOS issue and does a few small updates.

---

This crate provides a function `parse` for parsing strings into durations.
The parser is based on the standard set by
[systemd.time](https://www.freedesktop.org/software/systemd/man/systemd.time.html#Parsing%20Time%20Spans),
but extends it significantly.
For example, negative numbers, decimals and exponents are allowed.

```
extern crate parse_duration;

use parse_duration2::parse;
use std::time::Duration;

// One hour less than a day
assert_eq!(parse("1 day -1 hour"), Ok(Duration::new(82_800, 0)));
// Using exponents
assert_eq!(parse("1.26e-1 days"), Ok(Duration::new(10_886, 400_000_000)));
// Extra things will be ignored
assert_eq!(
    parse("Duration: 1 hour, 15 minutes and 29 seconds"),
    Ok(Duration::new(4529, 0))
);
```
