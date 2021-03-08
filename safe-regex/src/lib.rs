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
//! - Runtime is linear.
//! - Memory usage is constant.  Does not allocate.
//! - `no_std`
//! - Rust compiler checks and optimizes the matcher
//! - Supports basic regular expression syntax:
//!   - Any byte: `.`
//!   - Sequences: `abc`
//!   - Classes: `[-ab0-9]`, `[^ab]`
//!   - Repetition: `a?`, `a*`, `a+`, `a{1}`, `a{1,}`, `a{,1}`, `a{1,2}`, `a{,}`
//!   - Alternates: `a|b|c`
//!   - Capturing groups: `a(bc)?`
//!   - Non-capturing groups: `a(?:bc)?`
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
//!   | 1 | 30 | check PEM Base64 `[a-zA-Z0-9+/]{0,64}=*` |
//!   | 1 | 20-400 | substring search `.*(2G8H81RFNZ).*` |
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
//! use safe_regex::{regex, Matcher0};
//! let matcher: Matcher0<_> =
//!     regex!(br"[ab][0-9]*");
//! assert!(matcher.is_match(b"a42"));
//! assert!(!matcher.is_match(b"X"));
//! ```
//!
//! ```rust
//! use safe_regex::{regex, Matcher3};
//! let matcher: Matcher3<_> =
//!     regex!(br"([ab])([0-9]*)(suffix)?");
//! let (prefix, digits, suffix) =
//!     matcher.match_slices(b"a42").unwrap();
//! assert_eq!(b"a", prefix);
//! assert_eq!(b"42", digits);
//! assert_eq!(b"", suffix);
//! let (prefix_range, digits_r, suffix_r)
//!     = matcher.match_ranges(b"a42").unwrap();
//! assert_eq!(0..1_usize, prefix_range);
//! assert_eq!(1..3_usize, digits_r);
//! assert_eq!(0..0_usize, suffix_r);
//! ```
//!
//! # Changelog
//! - v0.2.3
//!   - Rename `match_all` -> `match_slices`.
//!   - Add `match_ranges`.
//! - v0.2.2 - Simplify `match_all` return type
//! - v0.2.1 - Non-capturing groups, bug fixes
//! - v0.2.0
//!   - Linear-time & constant-memory algorithm! :)
//!   - Work around rustc optimizer hang on regexes with exponential execution paths like "a{,30}".
//!     See `src/bin/uncompilable/main.rs`.
//! - v0.1.1 - Bug fixes and more tests.
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - 11+ capturing groups
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
use core::ops::Range;
pub use safe_regex_macro::regex;

/// Provides an `is_match` function.
pub trait IsMatch {
    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
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
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices(&self, data: &[u8]) -> Option<()> {
        (self.f)(data)
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(&self, data: &[u8]) -> Option<()> {
        (self.f)(data)
    }
}
impl<F: Fn(&[u8]) -> Option<()>> IsMatch for Matcher0<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 1 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher1<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 1]>,
{
    f: F,
}
impl<F> Matcher1<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 1]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(&self, data: &[u8]) -> Option<(Range<usize>,)> {
        let [r0] = (self.f)(data)?;
        Some((r0,))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(&self, data: &'d [u8]) -> Option<(&'d [u8],)> {
        let [r0] = (self.f)(data)?;
        Some((&data[r0],))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 1]>> IsMatch for Matcher1<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 2 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher2<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 2]>,
{
    f: F,
}
impl<F> Matcher2<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 2]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(&self, data: &[u8]) -> Option<(Range<usize>, Range<usize>)> {
        let [r0, r1] = (self.f)(data)?;
        Some((r0, r1))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(&self, data: &'d [u8]) -> Option<(&'d [u8], &'d [u8])> {
        let [r0, r1] = (self.f)(data)?;
        Some((&data[r0], &data[r1]))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 2]>> IsMatch for Matcher2<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 3 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher3<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 3]>,
{
    f: F,
}
impl<F> Matcher3<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 3]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(&self, data: &[u8]) -> Option<(Range<usize>, Range<usize>, Range<usize>)> {
        let [r0, r1, r2] = (self.f)(data)?;
        Some((r0, r1, r2))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(&self, data: &'d [u8]) -> Option<(&'d [u8], &'d [u8], &'d [u8])> {
        let [r0, r1, r2] = (self.f)(data)?;
        Some((&data[r0], &data[r1], &data[r2]))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 3]>> IsMatch for Matcher3<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 4 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher4<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 4]>,
{
    f: F,
}
impl<F> Matcher4<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 4]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(Range<usize>, Range<usize>, Range<usize>, Range<usize>)> {
        let [r0, r1, r2, r3] = (self.f)(data)?;
        Some((r0, r1, r2, r3))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(&'d [u8], &'d [u8], &'d [u8], &'d [u8])> {
        let [r0, r1, r2, r3] = (self.f)(data)?;
        Some((&data[r0], &data[r1], &data[r2], &data[r3]))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 4]>> IsMatch for Matcher4<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 5 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher5<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 5]>,
{
    f: F,
}
impl<F> Matcher5<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 5]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(&'d [u8], &'d [u8], &'d [u8], &'d [u8], &'d [u8])> {
        let [r0, r1, r2, r3, r4] = (self.f)(data)?;
        Some((&data[r0], &data[r1], &data[r2], &data[r3], &data[r4]))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 5]>> IsMatch for Matcher5<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 6 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher6<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 6]>,
{
    f: F,
}
impl<F> Matcher6<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 6]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4, r5] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4, r5))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(&'d [u8], &'d [u8], &'d [u8], &'d [u8], &'d [u8], &'d [u8])> {
        let [r0, r1, r2, r3, r4, r5] = (self.f)(data)?;
        Some((
            &data[r0], &data[r1], &data[r2], &data[r3], &data[r4], &data[r5],
        ))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 6]>> IsMatch for Matcher6<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 7 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher7<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 7]>,
{
    f: F,
}
impl<F> Matcher7<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 7]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4, r5, r6] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4, r5, r6))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
    )> {
        let [r0, r1, r2, r3, r4, r5, r6] = (self.f)(data)?;
        Some((
            &data[r0], &data[r1], &data[r2], &data[r3], &data[r4], &data[r5], &data[r6],
        ))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 7]>> IsMatch for Matcher7<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 8 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher8<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 8]>,
{
    f: F,
}
impl<F> Matcher8<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 8]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4, r5, r6, r7))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7] = (self.f)(data)?;
        Some((
            &data[r0], &data[r1], &data[r2], &data[r3], &data[r4], &data[r5], &data[r6], &data[r7],
        ))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 8]>> IsMatch for Matcher8<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 9 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher9<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 9]>,
{
    f: F,
}
impl<F> Matcher9<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 9]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7, r8] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4, r5, r6, r7, r8))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7, r8] = (self.f)(data)?;
        Some((
            &data[r0], &data[r1], &data[r2], &data[r3], &data[r4], &data[r5], &data[r6], &data[r7],
            &data[r8],
        ))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 9]>> IsMatch for Matcher9<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
    }
}

/// A compiled regular expression with 10 capturing groups.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This struct holds that type.
pub struct Matcher10<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 10]>,
{
    f: F,
}
impl<F> Matcher10<F>
where
    F: Fn(&[u8]) -> Option<[Range<usize>; 10]>,
{
    /// This is used internally by the `regex!` macro.
    #[must_use]
    pub fn new(f: F) -> Self {
        Self { f }
    }

    /// Returns `true` if `data` matches the regular expression,
    /// otherwise returns `false`.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher0};
    /// let matcher: Matcher0<_> = regex!(br"[abc][0-9]*");
    /// assert!(matcher.is_match(b"a42"));
    /// assert!(!matcher.is_match(b"X"));
    /// ```
    #[must_use]
    pub fn is_match(&self, data: &[u8]) -> bool {
        (self.f)(data).is_some()
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((Range<u32>,Range<u32>,...))` if the expression matched all of the bytes in `data`.
    /// The tuple fields are ranges of bytes in `data` that matched capturing
    /// groups in the expression.
    /// A capturing group that matches no bytes will produce as a zero-length
    /// range.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_ranges(b"a42").unwrap();
    /// assert_eq!(0..1_usize, prefix);
    /// assert_eq!(1..3_usize, digits);
    /// assert_eq!(0..0_usize, suffix);
    /// ```
    #[must_use]
    pub fn match_ranges(
        &self,
        data: &[u8],
    ) -> Option<(
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
        Range<usize>,
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9] = (self.f)(data)?;
        Some((r0, r1, r2, r3, r4, r5, r6, r7, r8, r9))
    }

    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some((&[u8],&[u8],...))`
    /// if the expression matched all of the bytes in `data`.
    /// The tuple fields are slices of `data` that matched
    /// capturing groups in the expression.
    ///
    /// This is a whole-string match.
    /// For sub-string search, put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    ///
    /// # Example
    /// ```rust
    /// use safe_regex::{regex, Matcher3};
    /// let matcher: Matcher3<_> = regex!(br"([abc])([0-9]*)(suffix)?");
    /// let (prefix, digits, suffix) = matcher.match_slices(b"a42").unwrap();
    /// assert_eq!(b"a", prefix);
    /// assert_eq!(b"42", digits);
    /// assert!(suffix.is_empty());
    /// ```
    #[must_use]
    pub fn match_slices<'d>(
        &self,
        data: &'d [u8],
    ) -> Option<(
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
        &'d [u8],
    )> {
        let [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9] = (self.f)(data)?;
        Some((
            &data[r0], &data[r1], &data[r2], &data[r3], &data[r4], &data[r5], &data[r6], &data[r7],
            &data[r8], &data[r9],
        ))
    }
}
impl<F: Fn(&[u8]) -> Option<[Range<usize>; 10]>> IsMatch for Matcher10<F> {
    fn is_match(&self, data: &[u8]) -> bool {
        self.is_match(data)
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
