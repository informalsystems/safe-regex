# safe-regex

[![crates.io version](https://img.shields.io/crates/v/essie-tls.svg)](https://crates.io/crates/safe-regex)
[![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)

A safe regular expression library.

## Features
- `forbid(unsafe_code)`
- `no_std` (depends only on `core`)
- Good test coverage (100%)
- Lets the Rust compiler optimize the pattern (no DFA).

## Limitations
- Only works on byte slices, not strings.
- You must write expressions using Rust syntax.
  For example, to match the expression `r"[a-z][0-9]"` write
  `safe_regex::seq(b'a'..b'z', b'0'..b'9')`.

## Documentation
<https://docs.rs/safe-regex-rs>

## Examples
```rust
use safe_regex;
use safe_regex::Regex;

// "."
safe_regex::any_byte()
    .match_all(b"a")
    .unwrap();

// "[0-9]"
(b'0'..=b'9').match_all(b"7").unwrap();

// "a?"
("a", ..=1).match_all(b"").unwrap();
("a", ..=1).match_all(b"a").unwrap();

// "a+"
("a", 1..).match_all(b"a").unwrap();
("a", 1..).match_all(b"aaa").unwrap();

// "a{3}"
("a", 3..=3).match_all(b"aaa").unwrap();

// "a{2,3}"
("a", 2..=3).match_all(b"aa").unwrap();
("a", 2..=3).match_all(b"aaa").unwrap();

// "a|b"
safe_regex::or("a", "b")
    .match_all(b"b")
    .unwrap();

// "a|b|c|d|e"
safe_regex::or5("a", "b", "c", "d", "e")
    .match_all(b"b").unwrap();

// "(a|b)(c|d)"
safe_regex::seq(
    safe_regex::or("a", "b"),
    safe_regex::or("c", "d"),
).match_all(b"bc").unwrap();

// "id([0-9]+)" capturing group
use std::cell::Cell;
let cell: Cell<Option<&[u8]>> =
    Cell::new(None);
safe_regex::seq(
    "id",
    safe_regex::group(
        &cell, (b'0'..b'9', 1..)
)).match_all(b"id42").unwrap();
assert_eq!(b"42", cell.get().unwrap());
```

## Alternatives
- [`regex`](https://crates.io/crates/regex)
  - Mature
  - Popular
  - Maintained by the core Rust language developers
  - Contains `unsafe` code.
- [`pcre2`](https://crates.io/crates/pcre2)
  - Uses PCRE library which is written in unsafe C.
- [`regular-expression`](https://crates.io/crates/regular-expression)
  - No documentation
- [`rec`](https://crates.io/crates/rec)

## Changelog
- v0.1.0 - First published version

## TO DO
- DONE - Match byte slices
- Match strings
- Macro, `regex!(r"[a-z][0-9]")`
- Common character classes: whitespace, letters, punctuation, etc.

## Release Process
1. Edit `Cargo.toml` and bump version number.
1. Run `./release.sh`

License: Apache-2.0
