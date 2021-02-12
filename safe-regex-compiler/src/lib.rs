//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-compiler.svg)](https://crates.io/crates/safe-regex-compiler)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! A regular expression compiler.
//!
//! If you want to use regular expressions in your software, use the
//! [`safe_regex`](https://crates.io/crates/safe-regex) crate.
//!
//! # Cargo Geiger Safety Report
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about parsing
//! - DONE - Implement `parser`
//! - DONE - Add tests for `parser`
//! - Implement `macro_generator`
//! - Add tests for `macro_generator`
//! - Add unwrap functions for other `FinalNode` variants
//! - Add fuzzing tests
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]
pub mod parser;
