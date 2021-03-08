//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-macro.svg)](https://crates.io/crates/safe-regex-macro)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
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
//! See [`safe_regex`](https://crates.io/crates/safe-regex) create.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`
#![forbid(unsafe_code)]

/// Compiles a regular expression into a Rust type.
///
/// Returns a `MatcherN` struct where `N` is the number of capturing groups.
///
/// Specify the type of the expected matcher so your editor can
/// show its functions and documentation:
/// `let matcher: Matcher0<_> = regex!(br".")`.
///
/// # Examples
/// ```rust
/// use safe_regex::{regex, Matcher0};
/// let matcher: Matcher0<_> =
///     regex!(br"[ab][0-9]*");
/// assert!(matcher.is_match(b"a42"));
/// assert!(!matcher.is_match(b"X"));
/// ```
///
/// ```rust
/// use safe_regex::{regex, Matcher3};
/// let matcher: Matcher3<_> =
///     regex!(br"([ab])([0-9]*)(suffix)?");
/// let (prefix, digits, suffix) =
///     matcher.match_slices(b"a42").unwrap();
/// assert_eq!(b"a", prefix);
/// assert_eq!(b"42", digits);
/// assert_eq!(b"", suffix);
/// let (prefix_range, digits_r, suffix_r)
///     = matcher.match_ranges(b"a42").unwrap();
/// assert_eq!(0..1_usize, prefix_range);
/// assert_eq!(1..3_usize, digits_r);
/// assert_eq!(0..0_usize, suffix_r);
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
