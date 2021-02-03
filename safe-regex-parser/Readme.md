# safe-regex-parser

[![crates.io version](https://img.shields.io/crates/v/safe-regex-parser.svg)](https://crates.io/crates/safe-regex-parser)
[![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-parser/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-parser/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)

This crate is used by the
[`safe_regex`](https://crates.io/crates/safe-regex) crate.
If you want to use regular expressions in your software, use that crate.

## Features
- Provides a `parse` function that converts a regular expression string
  into a `FinalNode` struct which is the root of an abstract syntax tree
- Implements a straightforward
  [contex-free grammar parser](https://www.cs.umd.edu/class/summer2015/cmsc330/parsing/)
- Parses in a single pass
- No recursion, no risk of stack overflow
- Depends only on `core` (it's `no_std`)
- Good test coverage (93%)

## Limitations
- Parses only raw byte strings, `br"..."`.
- Allocates.  Uses `Vec` and `String`.

## Alternatives
- [`regex-syntax`](https://crates.io/crates/regex-syntax)
  - Mature
  - Popular
  - Maintained by the core Rust language developers
  - Full of features
- [`regular-expression`](https://crates.io/crates/regular-expression)
  - No documentation

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

0/0        0/0          0/0    0/0     0/0      🔒  safe-regex-parser 0.1.0

0/0        0/0          0/0    0/0     0/0    

```
## Changelog
- v0.1.0 - First published version

## TO DO
- DONE - Read about regular expressions
- DONE - Read about parsing
- DONE - Implement `parse`
- DONE - Add integration tests
- Add unwrap functions for other FinalNode variants
- Add fuzzing tests

## Release Process
1. Edit `Cargo.toml` and bump version number.
1. Run `./release.sh`

License: Apache-2.0
