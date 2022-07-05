//! This crate provides a function [`parse`](fn.parse.html) for parsing strings into durations.
//! The parser is based on the standard set by
//! [systemd.time](https://www.freedesktop.org/software/systemd/man/systemd.time.html#Parsing%20Time%20Spans),
//! but extends it significantly.
//! For example, negative numbers, decimals and exponents are allowed.
//!
//! ```
//! use ::parse_duration0::parse;
//! use ::std::time::Duration;
//!
//! // One hour less than a day
//! assert_eq!(parse("1 day -1 hour"), Ok(Duration::new(82_800, 0)));
//! // Using exponents
//! assert_eq!(parse("0.126 days"), Ok(Duration::new(10_886, 400_000_000)));
//! // Extra things will be ignored
//! assert_eq!(
//!     parse("Duration: 1 hour, 15 minutes and 29 seconds"),
//!     Ok(Duration::new(4529, 0))
//! );
//! ```
//!
//! # Syntax
//!
//! Generally, the accepted syntax is a sequence of `[value]` `[unit]` pairs, such as
//! `"15 days 20 seconds 100 milliseconds"`.
//! Spaces are not needed as in `"15days20seconds100milliseconds"`.
//! Order doesn't matter at all.
//!
//! Characters other than alphanumeric (actually all word characters as defined by the regex crate)
//! are ignored,
//! other than the fact that they act as a word boundary.
//! So `".:++++]][][[][15[]][seconds][]:}}}}"` is the same as `"15 seconds"`.
//!
//! Any words with no corresponding value are ignored.
//! So in `"14 days seconds"`, `"seconds"` would be ignored.
//!
//! Any value without a unit will produce an error, unless *only* that unit is passed
//! (besides non-word characters).
//! In that case, the value is interpreted as seconds.
//! For example, `".:++++]][][[][15[]][][]:}}}}"` would be interpreted as 15 seconds.
//!
//! If the same unit is specified more than once, the sum of the values is used.
//!
//! ```
//! use ::parse_duration0::parse;
//! use ::std::time::Duration;
//!
//! assert_eq!(
//!     parse("15 days 20 seconds 100 milliseconds"),
//!     Ok(Duration::new(1_296_020, 100_000_000))
//! );
//! assert_eq!(
//!     parse("15days20seconds100milliseconds"),
//!     Ok(Duration::new(1_296_020, 100_000_000))
//! );
//!
//! assert_eq!(parse(".:++++]][][[][15[]][seconds][]:}}}}"), Ok(Duration::new(15, 0)));
//!
//! assert_eq!(parse("14 days seconds"), Ok(Duration::new(1_209_600, 0)));
//!
//! assert_eq!(parse(".:++++]][][[][15[]][][]:}}}}"), Ok(Duration::new(15, 0)));
//!
//! assert_eq!(parse("10 seconds 20 seconds"), Ok(Duration::new(30, 0)));
//! ```
//!
//! # Units
//!
//! The following units are accepted:
//!
//! - nanoseconds
//! - microseconds
//! - milliseconds
//! - seconds
//! - minutes
//! - hours
//! - days
//! - weeks
//! - months
//! - years
//!
//! Years are defined using the average over 400 years in the Gregorian calendar.
//! As such, a year is equivalent to 365.2425 days. A month is defined as one twelfth of a year.
//!
//! Abbreviations for each of these units are accepted.
//! The general rule is that any initial segment of the full name is accepted as long as it's not
//! ambiguous.
//! Additionally, the parser is generally case-insensitive.
//! The exception to both these rules is that `"m"` (or `"mi"` or `"min"`...)
//! is accepted for minutes
//! and `"M"` (or `"Mo"` or `"Mon"`...) is accepted for months.
//! Initial segments for other abbreviations
//! (`"nsecs"`, `"usecs"`, `"μsecs"`, `"msecs"`, `"secs"`, `"mins"`, `"hrs"`, `"wks"`, `"yrs"`)
//! are also accepted.
//!
//!
//! ```
//! use ::parse_duration0::parse;
//! use ::std::time::Duration;
//!
//! // Full names may be used
//! assert_eq!(parse("10 days 1 nanoseconds 15 years"), Ok(Duration::new(474_218_280, 1)));
//! // or very short names
//! assert_eq!(parse("10d1n15y"), Ok(Duration::new(474_218_280, 1)));
//! ```
//!
//! # Values
//!
//! The values may be an integer, a decimal, or a mantissa with an exponent.
//! They may be as large as desired as long as the final duration is less than
//! 2<sup>64</sup> seconds.
//!
//! Negatives are allowed, but the negative sign must be directly adjacent to the value:
//! `"-15 seconds"`, not `"- 15 seconds"`.
//! When using negative values, the sum must end up non-negative, since `Duration`s are positive
//! durations.
//!
//! Decimals are accurate up to nanosecond precision.
//! They will be rounded down to the nearest nanosecond if necessary.
//!
//! ```
//! use ::parse_duration0::parse;
//! use ::std::time::Duration;
//!
//! assert_eq!(parse("1 day -1 hour"), Ok(Duration::new(82_800, 0)));
//!
//! assert_eq!(parse("1844670000000000000 seconds"), Ok(Duration::new(1_844_670_000_000_000_000, 0)));
//! assert_eq!(
//!     parse("18446700000000000 nanoseconds"),
//!     Ok(Duration::new(18_446_700, 0))
//! );
//! ```
//!
//! # Errors
//!
//! The error `enum` has different variants for particular sorts of errors.
//! See [the documentation for the error `enum`](parse/enum.Error.html) for more information.

extern crate regex;
#[macro_use]
extern crate lazy_static;

/// This module contains the parse function and the error `enum`.
///
/// See the [module level documentation](index.html) for more.
pub mod parse;

pub use self::parse::parse;

#[cfg(test)]
mod tests;
