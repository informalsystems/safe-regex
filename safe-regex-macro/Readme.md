# safe-regex-macro

[![crates.io version](https://img.shields.io/crates/v/safe-regex-macro.svg)](https://crates.io/crates/safe-regex-macro)
[![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)

This crate provides the `regex!` macro used by the
[`safe-regex`](https://crates.io/crates/safe-regex) crate.

It is a thin wrapper around the
[`safe-regex-compiler`](https://crates.io/crates/safe-regex-compiler)
crate.

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

0/0        0/0          0/0    0/0     0/0      🔒  safe-regex-macro 0.1.1
0/0        0/0          0/0    0/0     0/0      🔒  ├── safe-proc-macro2 1.0.24
0/0        0/0          0/0    0/0     0/0      🔒  │   └── unicode-xid 0.2.1
0/0        0/0          0/0    0/0     0/0      🔒  └── safe-regex-compiler 0.1.1
0/0        0/0          0/0    0/0     0/0      🔒      ├── safe-proc-macro2 1.0.24
0/0        0/0          0/0    0/0     0/0      🔒      └── safe-quote 1.0.9
0/0        0/0          0/0    0/0     0/0      🔒          └── safe-proc-macro2 1.0.24

0/0        0/0          0/0    0/0     0/0    

```
## Changelog
- v0.1.1 - Bug fixes and more tests.
- v0.1.0 - First published version

## Release Process
1. Edit `Cargo.toml` and bump version number.
1. Run `../release.sh`

License: Apache-2.0
