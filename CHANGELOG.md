# parse_duration2 (fork)

## 3.0.0 (2022-07-02)
- Forked from zeta12ti
- Drop bigint support, add overflow error variant
- Fix security issue https://rustsec.org/advisories/RUSTSEC-2021-0041
- Drop travis CI

# zeta12ti's parse_duration (pre-fork)
---------------------------------------

# 2.1.0 (2020-02-21)
- Make the `parse` module public, allowing `parse::Error` to be matched on.

# 2.0.1 (2019-09-23)
- Clean up the crate-level docs.

# 2.0.0 (2019-09-22)
- Add minimum Rust version policy.
- Rewrite error enum to be more idiomatic.
- Fix some logic regarding exponents in the parser.

# 1.0.3 (2019-09-08)
Just some code clean up.
