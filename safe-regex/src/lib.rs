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
//! - Runtime is linear.  Memory usage is constant.
//!   Runtime and memory usage are both `O(n * r * g)` where
//!   - `n` is the length of the data to check
//!   - `r` is the length of the regex
//!   - `g` is the number of capturing groups in the regex
//! - Does not allocate
//! - `no_std`
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
//! - Partially optimized.  Runtime is about 10 times slower than
//!   [`regex`](https://crates.io/crates/regex) crate.
//!   Here are relative runtimes measured with
//!   [`safe-regex-rs/bench`](https://gitlab.com/leonhard-llc/safe-regex-rs/-/tree/main/bench)
//!   run on a 2018 Macbook Pro:
//!
//!   | `regex` | `safe_regex` | expression |
//!   | ----- | ---------- | ---------- |
//!   | 1 | 6 | find phone num `.*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*` |
//!   | 1 | 18 | find date time `.*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*` |
//!   | 1 | 0.9 | parse date time `([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)` |
//!   | 1 | 30 | check PEM Base64 `[a-zA-Z0-9+/=]{0,64}=*` |
//!   | 1 | 20-550 | substring search `.*(2G8H81RFNZ).*` |
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
//! use safe_regex::{regex, IsMatch, Matcher0};
//! let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
//! assert!(matcher.is_match(b"a42"));
//! assert!(!matcher.is_match(b"X"));
//! ```
//!
//! ```rust
//! use safe_regex::{regex, IsMatch, Matcher2};
//! let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
//! let (prefix, digits) = matcher.match_all(b"a42").unwrap();
//! assert_eq!(b"a", prefix.unwrap());
//! assert_eq!(b"42", digits.unwrap());
//! ```
//!
//! # Changelog
//! - v0.2.0
//!   - Linear-time & constant-memory algorithm! :)
//!   - Work around rustc optimizer hang on regexes with exponential execution paths like "a{,30}".
//!     See `src/bin/uncompilable/main.rs`.
//! - v0.1.1 - Bug fixes and more tests.
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about NFAs, <https://swtch.com/~rsc/regexp/>
//! - DONE - Design API
//! - DONE - Implement
//! - DONE - Add integration tests
//! - Simplify `match_all` return type
//! - Non-capturing groups
//! - >10 capturing groups
//! - Increase coverage
//! - Add fuzzing tests
//! - Common character classes: whitespace, letters, punctuation, etc.
//! - Match strings
//! - Implement optimizations explained in <https://swtch.com/%7Ersc/regexp/regexp3.html> .
//!   Some of the code already exists in `tests/dfa_single_pass.rs`
//!   and `tests/nfa_without_capturing.rs`.
//! - Once [const generics](https://github.com/rust-lang/rust/issues/44580)
//!   are stable, use the feature to simplify some types.
//! - Once
//!   [trait bounds on \`const fn\` parameters are stable](https://github.com/rust-lang/rust/issues/57563),
//!   make the `MatcherN::new` functions `const`.
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`

// https://swtch.com/~rsc/regexp/regexp1.html

#![forbid(unsafe_code)]
#![allow(clippy::type_complexity)]
pub use safe_regex_macro::regex;

/// Provides an `is_match` function.
pub trait IsMatch {
    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    fn is_match(&self, data: &[u8]) -> bool;
}

/// A compiled regular expression with no capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some` if the expression matched all of the bytes in `data`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
    #[must_use]
    pub fn match_all(&self, data: &[u8]) -> Option<()> {
        (self.f)(data)
    }
    /// This is used internally by the `regex!` macro.
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 1 capturing group.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
    #[must_use]
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<(Option<&'d [u8]>,)> {
        (self.f)(data)
    }
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 2 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
    #[must_use]
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>)> {
        (self.f)(data)
    }
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 3 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
    #[must_use]
    pub fn match_all<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(Option<&'d [u8]>, Option<&'d [u8]>, Option<&'d [u8]>)> {
        (self.f)(data)
    }
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 4 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 5 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 6 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 7 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 8 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 9 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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

/// A compiled regular expression with 10 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
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
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some(T)` if the expression matched all of the bytes in `data`.
    /// The value `T` is a tuple of captured group slices.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, IsMatch, Matcher2};
    /// let matcher: Matcher2<_> = regex!(br"([abc])([0-9]*)");
    /// let (prefix, digits) = matcher.match_all(b"a42").unwrap();
    /// assert_eq!(b"a", prefix.unwrap());
    /// assert_eq!(b"42", digits.unwrap());
    /// ```
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
    /// This is used internally by the `regex!` macro.
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
