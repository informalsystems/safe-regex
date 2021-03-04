//! [![crates.io version](https://img.shields.io/crates/v/safe-regex.svg)](https://crates.io/crates/safe-regex)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! A safe regular expression library.
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Good test coverage (~80%)
//! - Checks input in a single pass.
//!   Runtime and memory usage are both `O(n * r * 2^g)` where
//!   - `n` is the length of the data to check
//!   - `r` is the length of the regex
//!   - `g` is the number of capturing groups in the regex
//!   - TODO(mleonhard) Confirm this with a benchmark.
//! - Rust compiler checks and optimizes the matcher
//! - Supports basic regular expression syntax:
//!   - Any byte: `.`
//!   - Sequences: `abc`
//!   - Classes: `[-ab0-9]`, `[^ab]`
//!   - Repetition: `a?`, `a*`, `a+`, `a{1}`, `a{1,}`, `a{,1}`, `a{1,2}`, `a{,}`
//!   - Alternates: `a|b|c`
//!   - Capturing groups: `a(b*)?`
//!
//! # Limitations
//! - Only works on byte slices, not strings.
//! - Allocates.  Uses
//!   [`std::collections::HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html)
//!   during matching.
//! - Not optimized.
//!   For comparison, this crate takes 10 times more CPU time than the
//!   [`regex`](https://crates.io/crates/regex) crate to match complex expressions.
//!   And it takes 1,000 times more CPU time to match simple expressions.
//!   See [`safe-regex-rs/bench`](https://gitlab.com/leonhard-llc/safe-regex-rs/-/tree/main/bench).
//!
//! # Alternatives
//! - [`regex`](https://crates.io/crates/regex)
//!   - Mature & Popular
//!   - Maintained by the core Rust language developers
//!   - Contains `unsafe` code.
//! - [`pcre2`](https://crates.io/crates/pcre2)
//!   - Uses PCRE library which is written in unsafe C.
//! - [`regular-expression`](https://crates.io/crates/regular-expression)
//!   - No documentation
//! - [`rec`](https://crates.io/crates/rec)
//!
//! # Cargo Geiger Safety Report
//!
//! # Examples
//! ```rust
//! use safe_regex::{regex, Matcher};
//! let re: Matcher<_> = regex!(br"(ab)?c");
//! assert_eq!(None, re.match_all(b""));
//! assert_eq!(None, re.match_all(b"abcX"));
//!
//! let groups1 = re.match_all(b"abc").unwrap();
//! assert_eq!(b"ab", groups1.group(0).unwrap());
//! assert_eq!(0..2, groups1.group_range(0).unwrap());
//!
//! let groups2 = re.match_all(b"c").unwrap();
//! assert_eq!(None, groups2.group(0));
//! assert_eq!(None, groups2.group_range(0));
//!
//! // groups2.group(1); // panics
//! ```
//!
//! # Changelog
//! - v0.1.1 - Bug fixes and more tests.
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about NFAs, <https://swtch.com/~rsc/regexp/>
//! - DONE - Design API
//! - DONE - Implement
//! - DONE - Add integration tests
//! - Support more than 10 matching groups
//! - Increase coverage
//! - Add fuzzing tests
//! - Add common character classes: whitespace, letters, punctuation, etc.
//! - Match strings
//! - Implement optimizations explained in <https://swtch.com/%7Ersc/regexp/regexp3.html> .
//!   Some of the code already exists in `tests/dfa_single_pass.rs`
//!   and `tests/nfa_without_capturing.rs`.
//! - Add a memory-limited `match_all` fn, for use on untrusted data.
//!   Make it the default.
//! - Once [const generics](https://github.com/rust-lang/rust/issues/44580)
//!   are stable, use the feature to simplify some types.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`

// https://swtch.com/~rsc/regexp/regexp1.html

#![forbid(unsafe_code)]
pub use safe_regex_macro::regex;

pub trait IsMatch {
    fn is_match(&self, data: &[u8]) -> bool;
}

pub struct Matcher0<F>
where
    F: Fn(&[u8]) -> Option<()>,
{
    f: F,
}
impl<F> Matcher0<F>
where
    F: Fn(&[u8]) -> Option<()>,
{
    #[must_use]
    pub fn match_all(&self, data: &[u8]) -> Option<()> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher0<F>
where
    F: Fn(&[u8]) -> Option<()>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher1<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>,)>,
{
    f: F,
}
impl<F> Matcher1<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>,)>,
{
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<(Option<&'d [u8]>,)> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher1<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>,)>,
{
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher2<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    f: F,
}
impl<F> Matcher2<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>)> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher2<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher3<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    f: F,
}
impl<F> Matcher3<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>, Option<&'d [u8]>)> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher3<F>
where
    F: for<'d> Fn(&'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>, Option<&'d [u8]>)>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher4<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher4<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher4<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher5<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher5<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher5<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher6<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher6<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher6<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher7<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher7<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher7<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher8<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher8<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher8<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher9<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher9<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher9<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub struct Matcher10<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    f: F,
}
impl<F> Matcher10<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )> {
        (self.f)(data)
    }
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F> IsMatch for Matcher10<F>
where
    F: for<'d> Fn(
        &'d [u8],
    ) -> Option<(
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
        Option<&'d [u8]>,
    )>,
{
    #[must_use]
    fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }
}

pub mod internal {
    /// Converts the bytes into an ASCII string.
    pub fn escape_ascii(input: impl AsRef<[u8]>) -> String {
        let mut result = String::new();
        for byte in input.as_ref() {
            for ascii_byte in core::ascii::escape_default(*byte) {
                result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
            }
        }
        result
    }
}
