//! [![crates.io version](https://img.shields.io/crates/v/essie-tls.svg)](https://crates.io/crates/safe-regex)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! A safe regular expression library.
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - `no_std` (depends only on `core`)
//! - Good test coverage (100%)
//! - Lets the Rust compiler optimize the pattern (no DFA).
//!
//! # Limitations
//! - Only works on byte slices, not strings.
//! - You must write expressions using Rust syntax.
//!   For example, to match the expression `r"[a-z][0-9]"` write
//!   `safe_regex::seq(b'a'..b'z', b'0'..b'9')`.
//!
//! # Cargo Geiger Safety Report
//! `update_readme.sh` generates `Readme.md`
//! and replaces this section with the report.
//!
//! # Documentation
//! <https://docs.rs/safe-regex-rs>
//!
//! # Examples
//! ```rust
//! use safe_regex;
//! use safe_regex::Regex;
//!
//! // "."
//! safe_regex::any_byte()
//!     .match_all(b"a")
//!     .unwrap();
//!
//! // "[0-9]"
//! (b'0'..=b'9').match_all(b"7").unwrap();
//!
//! // "[^0-9]"
//! safe_regex::not(b'0'..=b'9')
//!     .match_all(b"a")
//!     .unwrap();
//!
//! // "a?"
//! ("a", ..=1).match_all(b"").unwrap();
//! ("a", ..=1).match_all(b"a").unwrap();
//!
//! // "a+"
//! ("a", 1..).match_all(b"a").unwrap();
//! ("a", 1..).match_all(b"aaa").unwrap();
//!
//! // "a{3}"
//! ("a", 3..=3).match_all(b"aaa").unwrap();
//!
//! // "a{2,3}"
//! ("a", 2..=3).match_all(b"aa").unwrap();
//! ("a", 2..=3).match_all(b"aaa").unwrap();
//!
//! // "a|b"
//! safe_regex::or("a", "b")
//!     .match_all(b"b")
//!     .unwrap();
//!
//! // "a|b|c|d|e"
//! safe_regex::or5("a", "b", "c", "d", "e")
//!     .match_all(b"b").unwrap();
//!
//! // "(a|b)(c|d)"
//! safe_regex::seq(
//!     safe_regex::or("a", "b"),
//!     safe_regex::or("c", "d"),
//! ).match_all(b"bc").unwrap();
//!
//! // "id([0-9]+)" capturing group
//! use std::cell::Cell;
//! let cell: Cell<Option<&[u8]>> =
//!     Cell::new(None);
//! safe_regex::seq(
//!     "id",
//!     safe_regex::group(
//!         &cell, (b'0'..b'9', 1..)
//! )).match_all(b"id42").unwrap();
//! assert_eq!(b"42", cell.get().unwrap());
//! ```
//!
//! # Alternatives
//! - [`regex`](https://crates.io/crates/regex)
//!   - Mature
//!   - Popular
//!   - Maintained by the core Rust language developers
//!   - Contains `unsafe` code.
//! - [`pcre2`](https://crates.io/crates/pcre2)
//!   - Uses PCRE library which is written in unsafe C.
//! - [`regular-expression`](https://crates.io/crates/regular-expression)
//!   - No documentation
//! - [`rec`](https://crates.io/crates/rec)
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Match byte slices
//! - Match strings
//! - Macro, `regex!(r"[a-z][0-9]")`
//! - Common character classes: whitespace, letters, punctuation, etc.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]

use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// Implements regular expression matching on byte slices.
///
/// See the [crate docs](index.html) for examples.
pub trait Regex<'d> {
    /// Checks if `data` starts with the pattern.
    ///
    /// Returns the number of bytes of `data` that match the pattern.
    ///
    /// Returns `None` if the pattern did not match.
    ///
    /// This is equivalent to adding '^' to the start of a regular expression.
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize>;

    /// Checks if `data` ends with the pattern.
    ///
    /// Returns the number of bytes of `data` that match the pattern.
    ///
    /// Returns `None` if the pattern did not match.
    ///
    /// This is equivalent to adding '$' to the end of a regular expression.
    fn match_suffix(&self, data: &'d [u8]) -> Option<Range<usize>> {
        let mut n: usize = 0;
        loop {
            if let Some(num_matched) = self.match_prefix(&data[n..]) {
                if n + num_matched == data.len() {
                    return Some(n..data.len());
                }
            }
            if data.len() <= n {
                return None;
            }
            n += 1;
        }
    }

    /// Checks if all of the bytes in `data` match the pattern.
    ///
    /// This is equivalent to adding '^' and '$' to the start and end of a
    /// regular expression.
    fn match_all(&self, data: &'d [u8]) -> Option<()> {
        let num_matched = self.match_prefix(data)?;
        if num_matched == data.len() {
            Some(())
        } else {
            None
        }
    }

    /// Searches for the pattern inside `data`.
    ///
    /// Returns the range of bytes in `data` that match the pattern.
    ///
    /// Returns `None` if the pattern did not match any sub-slice of `data`.
    fn search(&self, data: &'d [u8]) -> Option<Range<usize>> {
        let mut n: usize = 0;
        loop {
            if let Some(num_matched) = self.match_prefix(&data[n..]) {
                return Some(n..n + num_matched);
            }
            if data.len() <= n {
                return None;
            }
            n += 1;
        }
    }
}

fn check_range<R: RangeBounds<u8>>(range: &R, data: &[u8]) -> Option<usize> {
    if range.contains(data.get(0)?) {
        Some(1)
    } else {
        None
    }
}

impl<'d> Regex<'d> for Range<u8> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for RangeFrom<u8> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for RangeFull {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for RangeInclusive<u8> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for RangeTo<u8> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for RangeToInclusive<u8> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        check_range(self, data)
    }
}

impl<'d> Regex<'d> for char {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        let mut buf = [0_u8; 8];
        let s: &str = self.encode_utf8(&mut buf);
        s.match_prefix(data)
    }
}

impl<'d> Regex<'d> for &str {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        self.as_bytes().match_prefix(data)
    }
}

impl<'d> Regex<'d> for String {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        self.as_bytes().match_prefix(data)
    }
}

impl<'d> Regex<'d> for u8 {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        if !data.is_empty() && data[0] == *self {
            Some(1)
        } else {
            None
        }
    }
}

impl<'d> Regex<'d> for &[u8] {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        let slice_len = self.len();
        if slice_len <= data.len() && *self == &data[..slice_len] {
            Some(slice_len)
        } else {
            None
        }
    }
}

// TODO(https://github.com/rust-lang/rust/issues/44580)
//   Once const generics are stable, uncomment this and deprecate `Bytes`.
// impl<'d, const N: usize> Regex<'_> for [u8; N] {
//     fn check(&self, data: &'_ [u8]) -> Option<usize> {
//         (&self[..]).check(data)
//     }
// }

/// A wrapper type that implements `Regex` for structs that implement
/// `AsRef<[u8]>`.
///
/// [`bytes`](#method.bytes) returns this.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Bytes<'d, T: AsRef<[u8]>> {
    bytes: T,
    phantom: PhantomData<&'d ()>,
}

impl<'d, T: AsRef<[u8]>> Bytes<'d, T> {
    // TODO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    pub fn new(b: T) -> Self {
        Self {
            bytes: b,
            phantom: PhantomData,
        }
    }
}

/// Use an array as a `Regex` pattern.
/// This works with any type that implements `AsRef<[u8]>`.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::bytes(b"ab")
///     .match_all(b"ab")
///     .unwrap();
/// ```
pub fn bytes<'d, T: AsRef<[u8]>>(b: T) -> Bytes<'d, T> {
    Bytes::new(b)
}

impl<'d, T: AsRef<[u8]>> Regex<'d> for Bytes<'d, T> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        self.bytes.as_ref().match_prefix(data)
    }
}

/// A sequence of two `Regex` patterns.
///
/// [`seq`](#method.seq), [`seq3`](#method.seq3), [`seq4`](#method.seq4),
/// and [`seq5`](#method.seq5) return this.
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Seq<'d, A: Regex<'d>, B: Regex<'d>> {
    a: A,
    b: B,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>, B: Regex<'d>> Seq<'d, A, B> {
    // TODO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }
}

/// A sequence of two `Regex` patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::seq("a", "b")
///     .match_all(b"ab")
///     .unwrap();
/// ```
pub fn seq<'d, A: Regex<'d>, B: Regex<'d>>(a: A, b: B) -> Seq<'d, A, B> {
    Seq::new(a, b)
}

/// A sequence of three `Regex` patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::seq3("a", "b", "c")
///     .match_all(b"abc")
///     .unwrap();
/// ```
pub fn seq3<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>>(
    a: A,
    b: B,
    c: C,
) -> Seq<'d, A, Seq<'d, B, C>> {
    Seq::new(a, Seq::new(b, c))
}

/// A sequence of four `Regex` patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::seq4("a", "b", "c", "d")
///     .match_all(b"abcd")
///     .unwrap();
/// ```
pub fn seq4<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>>(
    a: A,
    b: B,
    c: C,
    d: D,
) -> Seq<'d, A, Seq<'d, B, Seq<'d, C, D>>> {
    Seq::new(a, Seq::new(b, Seq::new(c, d)))
}

/// A sequence of five `Regex` patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::seq5("a", "b", "c", "d", "e")
///     .match_all(b"abcde")
///     .unwrap();
/// ```
#[allow(clippy::many_single_char_names)]
#[allow(clippy::type_complexity)]
pub fn seq5<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>, E: Regex<'d>>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> Seq<'d, A, Seq<'d, B, Seq<'d, C, Seq<'d, D, E>>>> {
    Seq::new(a, Seq::new(b, Seq::new(c, Seq::new(d, e))))
}

impl<'d, A: Regex<'d>, B: Regex<'d>> Regex<'d> for Seq<'d, A, B> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        let num_matched1 = self.a.match_prefix(data)?;
        let num_matched2 = self.b.match_prefix(&data[num_matched1..])?;
        Some(num_matched1 + num_matched2)
    }
}

/// A `Regex` pattern that matches any byte.
///
/// [`any_byte`](#method.any_byte) returns this.
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct AnyByte;

#[must_use]
/// Returns a `Regex` pattern that matches any byte.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::any_byte()
///     .match_all(b"a")
///     .unwrap();
/// ```
pub fn any_byte() -> AnyByte {
    AnyByte {}
}

impl<'d> Regex<'d> for AnyByte {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        if data.is_empty() {
            None
        } else {
            Some(1)
        }
    }
}

/// A `Regex` pattern that matches a single byte, if that byte doesn't match
/// the specified `re`.
///
/// [`not`](#method.not) return this.
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Not<'d, A: Regex<'d>> {
    re: A,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>> Not<'d, A> {
    // TODO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    pub fn new(re: A) -> Self {
        Self {
            re,
            phantom: PhantomData,
        }
    }
}

/// Match any single byte that does not match `re`.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::not("x").match_all(b"b").unwrap();
/// assert_eq!(None, safe_regex::not("x").match_all(b"x"));
/// ```
pub fn not<'d, A: Regex<'d>>(re: A) -> Not<'d, A> {
    Not::new(re)
}

//#[allow(clippy::option_if_let_else)]
impl<'d, A: Regex<'d>> Regex<'d> for Not<'d, A> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        if !data.is_empty() && self.re.match_prefix(&data[..1]).is_none() {
            Some(1)
        } else {
            None
        }
    }
}

/// A `Regex` pattern that can match either of two patterns.
///
/// [`or`](#method.or), [`or3`](#method.or3), [`or4`](#method.or4),
/// and [`or5`](#method.or5) return this.
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Or<'d, A: Regex<'d>, B: Regex<'d>> {
    a: A,
    b: B,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>, B: Regex<'d>> Or<'d, A, B> {
    // TODO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }
}

/// Match either of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::or("a", "b")
///     .match_all(b"b")
///     .unwrap();
/// ```
pub fn or<'d, A: Regex<'d>, B: Regex<'d>>(a: A, b: B) -> Or<'d, A, B> {
    Or::new(a, b)
}

/// Match any of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::or3("a", "b", "c")
///     .match_all(b"c")
///     .unwrap();
/// ```
pub fn or3<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>>(
    a: A,
    b: B,
    c: C,
) -> Or<'d, A, Or<'d, B, C>> {
    Or::new(a, Or::new(b, c))
}

/// Match any of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::or4("a", "b", "c", "d")
///     .match_all(b"c")
///     .unwrap();
/// ```
pub fn or4<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>>(
    a: A,
    b: B,
    c: C,
    d: D,
) -> Or<'d, A, Or<'d, B, Or<'d, C, D>>> {
    Or::new(a, Or::new(b, Or::new(c, d)))
}

/// Match any of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// safe_regex::or5("a", "b", "c", "d", "e")
///     .match_all(b"c")
///     .unwrap();
/// ```
#[allow(clippy::many_single_char_names)]
#[allow(clippy::type_complexity)]
pub fn or5<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>, E: Regex<'d>>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> Or<'d, A, Or<'d, B, Or<'d, C, Or<'d, D, E>>>> {
    Or::new(a, Or::new(b, Or::new(c, Or::new(d, e))))
}

#[allow(clippy::option_if_let_else)]
impl<'d, A: Regex<'d>, B: Regex<'d>> Regex<'d> for Or<'d, A, B> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        if let Some(n) = self.a.match_prefix(data) {
            Some(n)
        } else if let Some(n) = self.b.match_prefix(data) {
            Some(n)
        } else {
            None
        }
    }
}

/// Wraps a `Regex` pattern and saves the slice that it matches.
///
/// [`group`](#method.group) returns this.
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Group<'d, R: Regex<'d>>(&'d Cell<Option<&'d [u8]>>, R);

/// Wrap `re`.  Whenever `re` matches a slice, save that to `cell`.
///
/// This is a "capturing gorup".
///
/// # Example
/// ```
/// use safe_regex;
/// use safe_regex::Regex;
/// // "id([0-9]+)" capturing group
/// use std::cell::Cell;
/// let cell: Cell<Option<&[u8]>> =
///     Cell::new(None);
/// safe_regex::seq(
///     "id",
///     safe_regex::group(
///         &cell, (b'0'..b'9', 1..))
/// ).match_all(b"id42").unwrap();
/// assert_eq!(b"42", cell.get().unwrap());
/// ```
pub fn group<'d, R: Regex<'d>>(cell: &'d Cell<Option<&'d [u8]>>, re: R) -> Group<'d, R> {
    Group(cell, re)
}

impl<'d, R: Regex<'d>> Regex<'d> for Group<'d, R> {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        let num_bytes_matched = self.1.match_prefix(data)?;
        self.0.set(Some(&data[0..num_bytes_matched]));
        Some(num_bytes_matched)
    }
}

impl<'d, T: Regex<'d>, R: RangeBounds<usize>> Regex<'d> for (T, R) {
    fn match_prefix(&self, data: &'d [u8]) -> Option<usize> {
        let max_incl = match self.1.end_bound() {
            Bound::Included(end_incl) => *end_incl,
            Bound::Excluded(end_excl) => *end_excl - 1,
            Bound::Unbounded => usize::MAX,
        };
        let mut num_found: usize = 0;
        let mut n: usize = 0;
        while n < data.len() && num_found < max_incl {
            let unchecked_data = &data[n..];
            if let Some(num_bytes_matched) = self.0.match_prefix(unchecked_data) {
                assert!(num_bytes_matched <= unchecked_data.len());
                num_found += 1;
                n += num_bytes_matched;
                assert!(n <= data.len());
                if num_bytes_matched == 0 {
                    // Zero-length match.  Can match an arbitrary number of times.
                    return Some(n);
                }
            } else {
                break;
            }
        }
        if self.1.contains(&num_found) {
            Some(n)
        } else {
            None
        }
    }
}
