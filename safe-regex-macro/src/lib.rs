//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-macro.svg)](https://crates.io/crates/safe-regex-macro)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! This crate provides the `regex!` macro used by the
//! [`safe-regex`](https://crates.io/crates/safe-regex) crate.
//!
//! It is a thin wrapper around the
//! [`safe-regex-compiler`](https://crates.io/crates/safe-regex-compiler)
//! crate.
//!
//! # Cargo Geiger Safety Report
//!
//! # Changelog
//! - v0.1.1 - Bug fixes and more tests.
//! - v0.1.0 - First published version
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`
#![forbid(unsafe_code)]

/// Compiles a regular expression into a Rust type.
///
/// Returns a `Matcher` struct
/// with a `match_all` function that takes a byte string.
///
/// The `Matcher<T>` struct is zero-length.  Its parameter `T` is the regex type.
///
/// # Examples
/// ```rust
/// use safe_regex::{regex, Matcher};
/// let re: Matcher<_> = regex!(br"(ab)?c");
/// assert_eq!(None, re.match_all(b""));
/// assert_eq!(None, re.match_all(b"abcX"));
///
/// let groups1 = re.match_all(b"abc").unwrap();
/// assert_eq!(b"ab", groups1.group(0).unwrap());
/// assert_eq!(0..2, groups1.group_range(0).unwrap());
///
/// let groups2 = re.match_all(b"c").unwrap();
/// assert_eq!(None, groups2.group(0));
/// assert_eq!(None, groups2.group_range(0));
///
/// // groups2.group(1); // panics
/// ```
#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2 = safe_proc_macro2::TokenStream::from(input);
    let output2 = match safe_regex_compiler::impl_regex(input2) {
        Ok(output2) => output2,
        Err(reason) => panic!("{}", reason),
    };
    proc_macro::TokenStream::from(output2)
}
