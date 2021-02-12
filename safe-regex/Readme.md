# safe-regex

[![crates.io version](https://img.shields.io/crates/v/safe-regex.svg)](https://crates.io/crates/safe-regex)
[![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)

A safe regular expression library.

## Features
- `forbid(unsafe_code)`
- Good test coverage (??%) - TODO(mleonhard) Update.
- Checks input in a single pass.
  Runtime and memory usage are both `O(n * r * 2^g)` where
  - `n` is the length of the data to check
  - `r` is the length of the regex
  - `g` is the number of capturing groups in the regex
  TODO(mleonhard) Confirm this with a benchmark.
- Rust compiler checks and optimizes the matcher
- Supports basic regular expression syntax:
  - Any byte: `.`
  - Sequences: `abc`
  - Classes: `[-ab0-9]`, `[^ab]`
  - Repetition: `a?`, `a*`, `a+`, `a{1}`, `a{1,}`, `a{,1}`, `a{1,2}`, `a{,}`
  - Alternates: `a|b|c`
  - Capturing groups: `a(b*)?`

## Limitations
- Only works on byte slices, not strings.
- Allocates.  Uses
  [`std::collections::HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html)
  during matching.

## Alternatives
- [`regex`](https://crates.io/crates/regex)
  - Mature & Popular
  - Maintained by the core Rust language developers
  - Contains `unsafe` code.
- [`pcre2`](https://crates.io/crates/pcre2)
  - Uses PCRE library which is written in unsafe C.
- [`regular-expression`](https://crates.io/crates/regular-expression)
  - No documentation
- [`rec`](https://crates.io/crates/rec)

## Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒  safe-regex 0.1.0
0/0        0/0          0/0    0/0     0/0      🔒  └── safe-regex-macro 0.1.0
0/0        0/0          0/0    0/0     0/0      ❓      ├── proc-macro2 1.0.24
0/0        0/0          0/0    0/0     0/0      🔒      │   └── unicode-xid 0.2.1
0/0        0/0          0/0    0/0     0/0      🔒      └── safe-regex-compiler 0.1.0
0/0        0/0          0/0    0/0     0/0      ❓          ├── proc-macro2 1.0.24
0/0        0/0          0/0    0/0     0/0      🔒          └── quote 1.0.8
0/0        0/0          0/0    0/0     0/0      ❓              └── proc-macro2 1.0.24

0/0        0/0          0/0    0/0     0/0    

```
## Examples
```rust
// use safe_regex::simple;
// use safe_regex::simple::Regex;
//
// // "."
// simple::any_byte()
//     .match_all(b"a")
//     .unwrap();
//
// // "[0-9]"
// (b'0'..=b'9').match_all(b"7").unwrap();
//
// // "[^0-9]"
// simple::not(b'0'..=b'9')
//     .match_all(b"a")
//     .unwrap();
//
// // "a?"
// ("a", ..=1).match_all(b"").unwrap();
// ("a", ..=1).match_all(b"a").unwrap();
//
// // "a+"
// ("a", 1..).match_all(b"a").unwrap();
// ("a", 1..).match_all(b"aaa").unwrap();
//
// // "a{3}"
// ("a", 3..=3).match_all(b"aaa").unwrap();
//
// // "a{2,3}"
// ("a", 2..=3).match_all(b"aa").unwrap();
// ("a", 2..=3).match_all(b"aaa").unwrap();
//
// // "a|b"
// simple::or("a", "b")
//     .match_all(b"b")
//     .unwrap();
//
// // "a|b|c|d|e"
// simple::or5("a", "b", "c", "d", "e")
//     .match_all(b"b").unwrap();
//
// // "(a|b)(c|d)"
// simple::seq(
//     simple::or("a", "b"),
//     simple::or("c", "d"),
// ).match_all(b"bc").unwrap();
//
// // "id([0-9]+)" capturing group
// use std::cell::Cell;
// let cell: Cell<Option<&[u8]>> =
//     Cell::new(None);
// simple::seq(
//     "id",
//     simple::group(
//         &cell, (b'0'..b'9', 1..)
// )).match_all(b"id42").unwrap();
// assert_eq!(b"42", cell.get().unwrap());
```

## Changelog
- v0.1.0 - First published version

## TO DO
- DONE - Read about regular expressions
- DONE - Read about NFAs, <https://swtch.com/~rsc/regexp/>
- Design API
- Implement
- Add integration tests
- Add macro, `regex!(r"[a-z][0-9]")`
- Add fuzzing tests
- Add common character classes: whitespace, letters, punctuation, etc.
- Match strings

## TO DO
- Once [const generics](https://github.com/rust-lang/rust/issues/44580)
  are stable, use the feature to simplify `Repeat` and other types.

## Release Process
1. Edit `Cargo.toml` and bump version number.
1. Run `./release.sh`

License: Apache-2.0
