# safe-regex

[![crates.io version](https://img.shields.io/crates/v/safe-regex.svg)](https://crates.io/crates/safe-regex)
[![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)

A safe regular expression library.

## Features
- `forbid(unsafe_code)`
- Good test coverage (~80%)
- Runtime is linear.
- Memory usage is constant.  Does not allocate.
- `no_std`
- Rust compiler checks and optimizes the matcher
- Supports basic regular expression syntax:
  - Any byte: `.`
  - Sequences: `abc`
  - Classes: `[-ab0-9]`, `[^ab]`
  - Repetition: `a?`, `a*`, `a+`, `a{1}`, `a{1,}`, `a{,1}`, `a{1,2}`, `a{,}`
  - Alternates: `a|b|c`
  - Capturing groups: `a(bc)?`
  - Non-capturing groups: `a(?:bc)?`

## Limitations
- Only works on byte slices, not strings.
- Partially optimized.  Runtime is about 10 times slower than
  [`regex`](https://crates.io/crates/regex) crate.
  Here are relative runtimes measured with
  [`safe-regex-rs/bench`](https://gitlab.com/leonhard-llc/safe-regex-rs/-/tree/main/bench)
  run on a 2018 Macbook Pro:

  | `regex` | `safe_regex` | expression |
  | ----- | ---------- | ---------- |
  | 1 | 6 | find phone num `.*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*` |
  | 1 | 20 | find date time `.*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*` |
  | 1 | 0.75 | parse date time `([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)` |
  | 1 | 50 | check PEM Base64 `[a-zA-Z0-9+/]{0,64}=*` |
  | 1 | 20-500 | substring search `.*(2G8H81RFNZ).*` |

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

0/0        0/0          0/0    0/0     0/0      🔒  safe-regex 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒  └── safe-regex-macro 0.2.3
0/0        0/0          0/0    0/0     0/0      🔒      ├── safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒      │   └── unicode-xid 0.2.2
0/0        0/0          0/0    0/0     0/0      🔒      └── safe-regex-compiler 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒          ├── safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒          └── safe-quote 1.0.15
0/0        0/0          0/0    0/0     0/0      🔒              └── safe-proc-macro2 1.0.36

0/0        0/0          0/0    0/0     0/0    

```
## Examples
```rust
use safe_regex::{regex, Matcher0};
let matcher: Matcher0<_> =
    regex!(br"[ab][0-9]*");
assert!(matcher.is_match(b"a42"));
assert!(!matcher.is_match(b"X"));
```

```rust
use safe_regex::{regex, Matcher3};
let matcher: Matcher3<_> =
    regex!(br"([ab])([0-9]*)(suffix)?");
let (prefix, digits, suffix) =
    matcher.match_slices(b"a42").unwrap();
assert_eq!(b"a", prefix);
assert_eq!(b"42", digits);
assert_eq!(b"", suffix);
let (prefix_range, digits_r, suffix_r)
    = matcher.match_ranges(b"a42").unwrap();
assert_eq!(0..1_usize, prefix_range);
assert_eq!(1..3_usize, digits_r);
assert_eq!(0..0_usize, suffix_r);
```

## Changelog
- v0.2.4
  - Bug fixes, reducing performance.
  - Optimize non-match runtime.
- v0.2.3
  - Rename `match_all` -> `match_slices`.
  - Add `match_ranges`.
- v0.2.2 - Simplify `match_all` return type
- v0.2.1 - Non-capturing groups, bug fixes
- v0.2.0
  - Linear-time & constant-memory algorithm! :)
  - Work around rustc optimizer hang on regexes with exponential execution paths like "a{,30}".
    See `src/bin/uncompilable/main.rs`.
- v0.1.1 - Bug fixes and more tests.
- v0.1.0 - First published version

## TO DO
- 11+ capturing groups
- Increase coverage
- Add fuzzing tests
- Common character classes: whitespace, letters, punctuation, etc.
- Match strings
- Implement optimizations explained in <https://swtch.com/%7Ersc/regexp/regexp3.html> .
  Some of the code already exists in `tests/dfa_single_pass.rs`
  and `tests/nfa_without_capturing.rs`.
- Once [const generics](https://github.com/rust-lang/rust/issues/44580)
  are stable, use the feature to simplify some types.
- Once
  [trait bounds on `const fn` parameters are stable](https://github.com/rust-lang/rust/issues/57563),
  make the `MatcherN::new` functions `const`.
## Release Process
1. Edit `Cargo.toml` and bump version number.
1. Run `../release.sh`

License: Apache-2.0
