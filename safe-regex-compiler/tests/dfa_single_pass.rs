//! This is an earlier version of the library.
//! It uses composable functions to build a DFA.
//!
//! It has a major flaw: no backtracking.
//! It performs a single pass through the input and pattern.
//! This means that it cannot match some valid regular expressions,
//! such as "a?a".
//!
//! Said another way, all optional or repeating elements are possessive.
//! Input bytes consumed by one part of the pattern are not available to
//! later parts of the pattern.
//! This means a pattern like `"a?a"` can never match.
//!
//! It supports a subset of regular expressions.
//!
//! Additionally, it gives no error when the user tries to use an unsupported
//! regex.  It just silently fails to match.
//!
//! One advantage of this version of the library is good performance:
//! - runtime O(`input_len`)
//! - memory O(1)
//!
//! I discovered this limitation and did more reading on the problem.
//! I came across this article which explains a solution:
//!   Regular Expression Matching Can Be Simple And Fast
//!   (but is slow in Java, Perl, PHP, Python, Ruby, ...)
//!   Russ Cox, rsc@swtch.com, January 2007
//!   <https://swtch.com/~rsc/regexp/regexp1.html>
//!
//! # Examples
//! ```rust
//! use safe_regex::simple;
//! use safe_regex::simple::Regex;
//!
//! // "."
//! simple::any_byte()
//!     .match_all(b"a")
//!     .unwrap();
//!
//! // "[0-9]"
//! (b'0'..=b'9').match_all(b"7").unwrap();
//!
//! // "[^0-9]"
//! simple::not(b'0'..=b'9')
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
//! simple::or("a", "b")
//!     .match_all(b"b")
//!     .unwrap();
//!
//! // "a|b|c|d|e"
//! simple::or5("a", "b", "c", "d", "e")
//!     .match_all(b"b").unwrap();
//!
//! // "(a|b)(c|d)"
//! simple::seq(
//!     simple::or("a", "b"),
//!     simple::or("c", "d"),
//! ).match_all(b"bc").unwrap();
//!
//! // "id([0-9]+)" capturing group
//! use std::cell::Cell;
//! let cell: Cell<Option<&[u8]>> =
//!     Cell::new(None);
//! simple::seq(
//!     "id",
//!     simple::group(
//!         &cell, (b'0'..b'9', 1..)
//! )).match_all(b"id42").unwrap();
//! assert_eq!(b"42", cell.get().unwrap());
//! ```
#![forbid(unsafe_code)]
use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// Implements regular expression matching on byte slices.
///
/// See the [module docs](index.html) for examples.
trait Regex<'d> {
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

// TO DO(https://github.com/rust-lang/rust/issues/44580)
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
struct Bytes<'d, T: AsRef<[u8]>> {
    bytes: T,
    phantom: PhantomData<&'d ()>,
}

impl<'d, T: AsRef<[u8]>> Bytes<'d, T> {
    // TO DO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    #[must_use]
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::bytes(b"ab")
///     .match_all(b"ab")
///     .unwrap();
/// ```
fn bytes<'d, T: AsRef<[u8]>>(b: T) -> Bytes<'d, T> {
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
struct Seq<'d, A: Regex<'d>, B: Regex<'d>> {
    a: A,
    b: B,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>, B: Regex<'d>> Seq<'d, A, B> {
    // TO DO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    #[must_use]
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::seq("a", "b")
///     .match_all(b"ab")
///     .unwrap();
/// ```
fn seq<'d, A: Regex<'d>, B: Regex<'d>>(a: A, b: B) -> Seq<'d, A, B> {
    Seq::new(a, b)
}

/// A sequence of three `Regex` patterns.
///
/// # Example
/// ```
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::seq3("a", "b", "c")
///     .match_all(b"abc")
///     .unwrap();
/// ```
fn seq3<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>>(
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::seq4("a", "b", "c", "d")
///     .match_all(b"abcd")
///     .unwrap();
/// ```
fn seq4<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>>(
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::seq5("a", "b", "c", "d", "e")
///     .match_all(b"abcde")
///     .unwrap();
/// ```
#[allow(clippy::many_single_char_names)]
#[allow(clippy::type_complexity)]
fn seq5<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>, E: Regex<'d>>(
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
struct AnyByte;

#[must_use]
/// Returns a `Regex` pattern that matches any byte.
///
/// # Example
/// ```
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::any_byte()
///     .match_all(b"a")
///     .unwrap();
/// ```
fn any_byte() -> AnyByte {
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
struct Not<'d, A: Regex<'d>> {
    re: A,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>> Not<'d, A> {
    // TO DO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    #[must_use]
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::not("x").match_all(b"b").unwrap();
/// assert_eq!(None, simple::not("x").match_all(b"x"));
/// ```
fn not<'d, A: Regex<'d>>(re: A) -> Not<'d, A> {
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
struct Or<'d, A: Regex<'d>, B: Regex<'d>> {
    a: A,
    b: B,
    phantom: PhantomData<&'d ()>,
}

impl<'d, A: Regex<'d>, B: Regex<'d>> Or<'d, A, B> {
    // TO DO(https://github.com/rust-lang/rust/issues/57563)
    //   Once `feature(const_fn)` is stable, make this `const fn`.
    #[must_use]
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::or("a", "b")
///     .match_all(b"b")
///     .unwrap();
/// ```
fn or<'d, A: Regex<'d>, B: Regex<'d>>(a: A, b: B) -> Or<'d, A, B> {
    Or::new(a, b)
}

/// Match any of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::or3("a", "b", "c")
///     .match_all(b"c")
///     .unwrap();
/// ```
fn or3<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>>(a: A, b: B, c: C) -> Or<'d, A, Or<'d, B, C>> {
    Or::new(a, Or::new(b, c))
}

/// Match any of the specified patterns.
///
/// # Example
/// ```
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::or4("a", "b", "c", "d")
///     .match_all(b"c")
///     .unwrap();
/// ```
fn or4<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>>(
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
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// simple::or5("a", "b", "c", "d", "e")
///     .match_all(b"c")
///     .unwrap();
/// ```
#[allow(clippy::many_single_char_names)]
#[allow(clippy::type_complexity)]
fn or5<'d, A: Regex<'d>, B: Regex<'d>, C: Regex<'d>, D: Regex<'d>, E: Regex<'d>>(
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
struct Group<'d, R: Regex<'d>>(&'d Cell<Option<&'d [u8]>>, R);

/// Wrap `re`.  Whenever `re` matches a slice, save that to `cell`.
///
/// This is a "capturing gorup".
///
/// # Example
/// ```
/// use safe_regex::simple;
/// use safe_regex::simple::Regex;
/// // "id([0-9]+)" capturing group
/// use std::cell::Cell;
/// let cell: Cell<Option<&[u8]>> =
///     Cell::new(None);
/// simple::seq(
///     "id",
///     simple::group(
///         &cell, (b'0'..b'9', 1..))
/// ).match_all(b"id42").unwrap();
/// assert_eq!(b"42", cell.get().unwrap());
/// ```
fn group<'d, R: Regex<'d>>(cell: &'d Cell<Option<&'d [u8]>>, re: R) -> Group<'d, R> {
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

#[test]
fn test_match_all() {
    assert_eq!(Some(()), "".match_all(b""));
    assert_eq!(None, "".match_all(b"a"));
    assert_eq!(Some(()), "b".match_all(b"b"));
    assert_eq!(None, "b".match_all(b"a"));
    assert_eq!(None, "b".match_all(b"ab"));
    assert_eq!(None, "b".match_all(b"bc"));
    assert_eq!(None, "b".match_all(b"abc"));
    assert_eq!(Some(()), "abc".match_all(b"abc"));
    assert_eq!(None, "abc".match_all(b"Xabc"));
    assert_eq!(None, "abc".match_all(b"abcY"));
    assert_eq!(None, "abc".match_all(b"XabcY"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"ac"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abc"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abbbc"));
}

#[test]
fn test_match_prefix() {
    assert_eq!(Some(0), "".match_prefix(b""));
    assert_eq!(Some(0), "".match_prefix(b"a"));
    assert_eq!(Some(1), "b".match_prefix(b"b"));
    assert_eq!(None, "b".match_prefix(b"a"));
    assert_eq!(None, "b".match_prefix(b"ab"));
    assert_eq!(Some(1), "b".match_prefix(b"bc"));
    assert_eq!(None, "b".match_prefix(b"abc"));
    assert_eq!(Some(3), "abc".match_prefix(b"abc"));
    assert_eq!(None, "abc".match_prefix(b"Xabc"));
    assert_eq!(Some(3), "abc".match_prefix(b"abcY"));
    assert_eq!(None, "abc".match_prefix(b"XabcY"));

    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"a"));
    assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"b"));
    assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"bY"));
    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xb"));
    assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"ab"));
    assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"abY"));
    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xab"));

    assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"a"));
    assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"ad"));
    assert_eq!(Some(2), seq("a", ("b", 0..)).match_prefix(b"ab"));
    assert_eq!(Some(3), seq("a", ("b", 0..)).match_prefix(b"abb"));
    assert_eq!(Some(4), seq("a", ("b", 0..)).match_prefix(b"abbb"));

    assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"ac"));
    assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"acd"));
    assert_eq!(Some(3), seq("a", seq(("b", 0..), "c")).match_prefix(b"abc"));
    assert_eq!(
        Some(3),
        seq("a", seq(("b", 0..), "c")).match_prefix(b"abcd")
    );
    assert_eq!(
        Some(4),
        seq("a", seq(("b", 0..), "c")).match_prefix(b"abbcd")
    );
}

#[test]
fn test_match_suffix() {
    assert_eq!(Some(0..0), "".match_suffix(b""));
    assert_eq!(Some(1..1), "".match_suffix(b"a"));
    assert_eq!(Some(0..1), "b".match_suffix(b"b"));
    assert_eq!(None, "b".match_suffix(b"a"));
    assert_eq!(Some(1..2), "b".match_suffix(b"ab"));
    assert_eq!(None, "b".match_suffix(b"bc"));
    assert_eq!(None, "b".match_suffix(b"abc"));
    assert_eq!(Some(0..3), "abc".match_suffix(b"abc"));
    assert_eq!(Some(1..4), "abc".match_suffix(b"Xabc"));
    assert_eq!(None, "abc".match_suffix(b"abcY"));
    assert_eq!(None, "abc".match_suffix(b"XabcY"));

    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"a"));
    assert_eq!(Some(0..1), seq(("a", 0..), "b").match_suffix(b"b"));
    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"bY"));
    assert_eq!(Some(1..2), seq(("a", 0..), "b").match_suffix(b"Xb"));
    assert_eq!(Some(0..2), seq(("a", 0..), "b").match_suffix(b"ab"));
    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"abY"));
    assert_eq!(Some(1..3), seq(("a", 0..), "b").match_suffix(b"Xab"));

    assert_eq!(Some(0..1), seq("a", ("b", 0..)).match_suffix(b"a"));
    assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"aY"));
    assert_eq!(Some(1..2), seq("a", ("b", 0..)).match_suffix(b"Xa"));
    assert_eq!(Some(0..2), seq("a", ("b", 0..)).match_suffix(b"ab"));
    assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"abY"));
    assert_eq!(Some(1..3), seq("a", ("b", 0..)).match_suffix(b"Xab"));
    assert_eq!(Some(0..3), seq("a", ("b", 0..)).match_suffix(b"abb"));
    assert_eq!(Some(0..4), seq("a", ("b", 0..)).match_suffix(b"abbb"));

    assert_eq!(
        Some(0..2),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"ac")
    );
    assert_eq!(None, seq("a", seq(("b", 0..), "c")).match_suffix(b"acY"));
    assert_eq!(
        Some(1..3),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xac")
    );
    assert_eq!(
        Some(0..3),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"abc")
    );
    assert_eq!(
        Some(1..4),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabc")
    );
    assert_eq!(
        Some(1..5),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabbc")
    );
}

#[test]
fn test_search() {
    // TO DO(mleonhard) Implement.
}

// TO DO(mleonhard) Change the rest of the tests to use match_prefix().

#[test]
fn test_impl_for_char() {
    assert_eq!(None, 'b'.search(b""));
    assert_eq!(None, 'b'.search(b"XY"));
    assert_eq!(Some(1..2), 'b'.search(b"abbc"));
}

#[test]
fn test_impl_for_str() {
    assert_eq!(Some(0..0), "".search(b""));
    assert_eq!(Some(1..3), "bb".search(b"abbc"));
}

#[test]
fn test_impl_for_string() {
    assert_eq!(Some(0..0), String::new().search(b""));
    assert_eq!(Some(1..3), String::from("bb").search(b"abbc"));
}

#[test]
fn test_impl_for_u8() {
    assert_eq!(None, b'b'.search(b""));
    assert_eq!(None, b'b'.search(b"a"));
    assert_eq!(Some(0..1), b'b'.search(b"b"));
    assert_eq!(Some(1..2), b'b'.search(b"ab"));
    assert_eq!(Some(1..2), b'b'.search(b"abc"));
}

#[test]
fn test_impl_for_u8_slice() {
    // Empty pattern
    assert_eq!(Some(0..0), b"".as_ref().search(b""));
    assert_eq!(Some(0..0), b"".as_ref().search(b"a"));
    // // Whole string matches
    assert_eq!(Some(0..2), b"bb".as_ref().search(b"bb"));
    // Matches at beginning
    assert_eq!(Some(0..2), b"bb".as_ref().search(b"bbc"));
    // Matches at end
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abb"));
    // Matches in middle
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbb"));
    // Does not accept partial matches
    assert_eq!(None, b"abc".as_ref().search(b"ab"));
    assert_eq!(None, b"abc".as_ref().search(b"abd"));
    assert_eq!(None, b"abc".as_ref().search(b"bc"));
    assert_eq!(None, b"abc".as_ref().search(b"bcd"));
    // Check returned range
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbcbbc"));
}

#[test]
fn test_bytes() {
    assert_eq!(Some(0..0), bytes(b"").search(b""));
    assert_eq!(Some(1..3), bytes(b"bb").search(b"abbc"));

    let value = bytes(b"abc");
    let value_copy = value; // Copy
    #[allow(clippy::clone_on_copy)]
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Bytes { bytes: [97, 98, 99], phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < bytes(b"def")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_seq() {
    assert_eq!(Some(0..4), seq("ab", "cd").search(b"abcdX"));
    assert_eq!(Some(1..5), seq("ab", "cd").search(b"Xabcd"));
    assert_eq!(None, seq("ab", "cd").search(b"abXcd"));
    assert_eq!(Some(1..4), seq("a", seq("b", "c")).search(b"XabcY"));

    let value = seq("a", "b");
    let value_copy = value; // Copy
    #[allow(clippy::clone_on_copy)]
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Seq { a: \"a\", b: \"b\", phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < seq("d", "d")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_seq3() {
    assert_eq!(Some(0..6), seq3("ab", "cd", "ef").search(b"abcdefX"));
    assert_eq!(Some(1..7), seq3("ab", "cd", "ef").search(b"Xabcdef"));
    assert_eq!(None, seq3("ab", "cd", "ef").search(b"abXcdef"));
    assert_eq!(Some(1..4), seq3("a", "b", "c").search(b"XabcY"));
}

#[test]
fn test_seq4() {
    assert_eq!(
        Some(0..8),
        seq4("ab", "cd", "ef", "gh").search(b"abcdefghX")
    );
    assert_eq!(
        Some(1..9),
        seq4("ab", "cd", "ef", "gh").search(b"Xabcdefgh")
    );
    assert_eq!(None, seq4("ab", "cd", "ef", "gh").search(b"abXcdefgh"));
    assert_eq!(Some(1..5), seq4("a", "b", "c", "d").search(b"XabcdY"));
}

#[test]
fn test_seq5() {
    assert_eq!(
        Some(0..10),
        seq5("ab", "cd", "ef", "gh", "ij").search(b"abcdefghijX")
    );
    assert_eq!(
        Some(1..11),
        seq5("ab", "cd", "ef", "gh", "ij").search(b"Xabcdefghij")
    );
    assert_eq!(
        None,
        seq5("ab", "cd", "ef", "gh", "ij").search(b"abXcdefghij")
    );
    assert_eq!(Some(1..6), seq5("a", "b", "c", "d", "e").search(b"XabcdeY"));
}

#[test]
fn test_range() {
    assert_eq!(None, (..).search(b""));
    assert_eq!(Some(0..1), (..).search(b"a"));

    assert_eq!(None, (b'b'..).search(b""));
    assert_eq!(None, (b'b'..).search(b"a"));
    assert_eq!(Some(0..1), (b'b'..).search(b"b"));
    assert_eq!(Some(0..1), (b'b'..).search(b"c"));

    assert_eq!(None, (..b'c').search(b""));
    assert_eq!(Some(0..1), (..b'c').search(b"a"));
    assert_eq!(Some(0..1), (..b'c').search(b"b"));
    assert_eq!(None, (..b'c').search(b"c"));

    assert_eq!(None, (..=b'b').search(b""));
    assert_eq!(Some(0..1), (..=b'b').search(b"a"));
    assert_eq!(Some(0..1), (..=b'b').search(b"b"));
    assert_eq!(None, (..=b'b').search(b"c"));

    assert_eq!(None, (b'b'..b'd').search(b""));
    assert_eq!(None, (b'b'..b'd').search(b"a"));
    assert_eq!(Some(0..1), (b'b'..b'd').search(b"b"));
    assert_eq!(Some(0..1), (b'b'..b'd').search(b"c"));
    assert_eq!(None, (b'b'..b'd').search(b"d"));

    assert_eq!(None, (b'b'..=b'c').search(b""));
    assert_eq!(None, (b'b'..=b'c').search(b"a"));
    assert_eq!(Some(0..1), (b'b'..=b'c').search(b"b"));
    assert_eq!(Some(0..1), (b'b'..=b'c').search(b"c"));
    assert_eq!(None, (b'b'..=b'c').search(b"d"));
}

#[test]
fn test_repeat_range() {
    // zero of, '{0}'
    assert_eq!(Some(0..0), ("b", ..=0).search(b""));
    assert_eq!(Some(0..0), (("b", ..=1), ..=1).search(b"a"));

    // zero or one, '?', '{0,1}'
    assert_eq!(Some(0..0), ("b", ..=1).search(b""));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"a"));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"ab"));
    assert_eq!(Some(0..1), ("b", ..=1).search(b"b"));
    assert_eq!(Some(0..1), ("b", ..=1).search(b"bb"));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b""));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b"a"));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", ..=1).search(b"bc"));
    assert_eq!(Some(0..2), ("bc", ..=1).search(b"bcbc"));

    // zero or more, '*', '{0,}'
    assert_eq!(Some(0..0), ("b", ..).search(b""));
    assert_eq!(Some(0..0), ("b", ..).search(b"a"));
    assert_eq!(Some(0..0), ("b", ..).search(b"ab"));
    assert_eq!(Some(0..1), ("b", ..).search(b"b"));
    assert_eq!(Some(0..4), ("b", ..).search(b"bbbb"));
    assert_eq!(Some(0..0), ("bc", ..).search(b""));
    assert_eq!(Some(0..0), ("bc", ..).search(b"a"));
    assert_eq!(Some(0..0), ("bc", ..).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", ..).search(b"bc"));
    assert_eq!(Some(0..4), ("bc", ..).search(b"bcbc"));

    // one or more, '+', '{1,}'
    assert_eq!(None, ("b", 1..).search(b""));
    assert_eq!(None, ("b", 1..).search(b"a"));
    assert_eq!(Some(1..2), ("b", 1..).search(b"ab"));
    assert_eq!(Some(0..1), ("b", 1..).search(b"b"));
    assert_eq!(Some(0..4), ("b", 1..).search(b"bbbb"));
    assert_eq!(None, ("bc", 1..).search(b""));
    assert_eq!(None, ("bc", 1..).search(b"a"));
    assert_eq!(Some(1..3), ("bc", 1..).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", 1..).search(b"bc"));
    assert_eq!(Some(1..3), ("bc", 1..).search(b"bbc"));
    assert_eq!(Some(0..4), ("bc", 1..).search(b"bcbc"));

    // n of, '{n}'
    assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));
    assert_eq!(None, ("b", 1..=1).search(b""));
    assert_eq!(Some(0..1), ("b", 1..=1).search(b"b"));

    assert_eq!(None, ("b", 2..=2).search(b""));
    assert_eq!(None, ("b", 2..=2).search(b"aaa"));
    assert_eq!(None, ("b", 2..=2).search(b"abaa"));
    assert_eq!(Some(1..3), ("b", 2..=2).search(b"abb"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bb"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbc"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbbb"));

    assert_eq!(None, ("bc", 2..=2).search(b""));
    assert_eq!(None, ("bc", 2..=2).search(b"aa"));
    assert_eq!(None, ("bc", 2..=2).search(b"abb"));
    assert_eq!(None, ("bc", 2..=2).search(b"ccd"));
    assert_eq!(Some(1..5), ("bc", 2..=2).search(b"abcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbc"));
    assert_eq!(Some(1..5), ("bc", 2..=2).search(b"bbcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbcbcbc"));

    // m to n of, '{m,n}'
    assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));

    assert_eq!(Some(0..0), ("b", 0..=1).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=1).search(b"a"));
    assert_eq!(Some(0..0), ("b", 0..=1).search(b"ab"));

    assert_eq!(None, ("b", 1..=2).search(b""));
    assert_eq!(None, ("b", 1..=2).search(b"a"));
    assert_eq!(Some(0..1), ("b", 1..=2).search(b"b"));
    assert_eq!(Some(0..1), ("b", 1..=2).search(b"bc"));
    assert_eq!(Some(1..2), ("b", 1..=2).search(b"ab"));
    assert_eq!(Some(1..2), ("b", 1..=2).search(b"abc"));
    assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbc"));
    assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbbbc"));

    assert_eq!(None, ("b", 2..=4).search(b""));
    assert_eq!(None, ("b", 2..=4).search(b"aa"));
    assert_eq!(None, ("b", 2..=4).search(b"ab"));
    assert_eq!(None, ("b", 2..=4).search(b"abc"));
    assert_eq!(Some(0..2), ("b", 2..=4).search(b"bb"));
    assert_eq!(Some(0..2), ("b", 2..=4).search(b"bbcc"));
    assert_eq!(Some(2..4), ("b", 2..=4).search(b"aabb"));
    assert_eq!(Some(1..3), ("b", 2..=4).search(b"abbc"));
    assert_eq!(Some(1..4), ("b", 2..=4).search(b"abbbc"));
    assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbc"));
    assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbbbbbc"));

    assert_eq!(None, ("bc", 2..=4).search(b""));
    assert_eq!(None, ("bc", 2..=4).search(b"aaaa"));
    assert_eq!(None, ("bc", 2..=4).search(b"abc"));
    assert_eq!(None, ("bc", 2..=4).search(b"abcb"));
    assert_eq!(None, ("bc", 2..=4).search(b"abcd"));
    assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbcdddd"));
    assert_eq!(Some(4..8), ("bc", 2..=4).search(b"aaaabcbc"));
    assert_eq!(Some(1..5), ("bc", 2..=4).search(b"abcbcd"));
    assert_eq!(Some(2..6), ("bc", 2..=4).search(b"abbcbc"));
    assert_eq!(Some(1..7), ("bc", 2..=4).search(b"abcbcbc"));
    assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbc"));
    assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbcbc"));

    assert_eq!(Some(0..0), ("b", ..).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..).search(b"abc"));
    assert_eq!(Some(0..0), ("b", ..2).search(b"abc"));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..2).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..=1).search(b"abc"));
}

#[test]
fn test_any_byte() {
    assert_eq!(None, any_byte().search(b""));
    assert_eq!(Some(0..1), any_byte().search(b"a"));
    assert_eq!(Some(0..1), any_byte().search(b"ab"));

    let value = any_byte();
    let value_copy = value; // Copy
    #[allow(clippy::clone_on_copy)]
    let _value_clone = value.clone(); // Clone
    assert_eq!("AnyByte", format!("{:?}", value)); // Debug
    assert_eq!(
        Some(core::cmp::Ordering::Equal),
        value.partial_cmp(&any_byte()) // PartialOrd
    );
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_not() {
    assert_eq!(None, not("X").search(b""));
    assert_eq!(None, not("X").search(b"X"));
    assert_eq!(None, not("X").search(b"XX"));
    assert_eq!(Some(0..1), not("X").search(b"ab"));
    assert_eq!(Some(1..2), not("X").search(b"Xab"));
    assert_eq!(Some(2..3), not("X").search(b"XXab"));
    assert_eq!(Some(0..1), not(seq(any_byte(), "X")).search(b"aX"));

    let value = not("X");
    let value_copy = value; // Copy
    #[allow(clippy::clone_on_copy)]
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Not { re: \"X\", phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < not("Y")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_or() {
    assert_eq!(None, or("a", "b").search(b""));
    assert_eq!(Some(1..2), or("a", "b").search(b"XaY"));
    assert_eq!(Some(1..2), or("a", "b").search(b"XbY"));
    assert_eq!(Some(0..1), or("a", "b").search(b"ab"));
    assert_eq!(None, or("a", "b").search(b"XY"));

    let value = or("a", "b");
    let value_copy = value; // Copy
    #[allow(clippy::clone_on_copy)]
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Or { a: \"a\", b: \"b\", phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < or("d", "d")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_or3() {
    assert_eq!(None, or3("a", "b", "c").search(b""));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XaY"));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XbY"));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XcY"));
    assert_eq!(Some(0..1), or3("a", "b", "c").search(b"abc"));
    assert_eq!(None, or3("a", "b", "c").search(b"XY"));
}

#[test]
fn test_or4() {
    assert_eq!(None, or4("a", "b", "c", "d").search(b""));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XaY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XbY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XcY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XdY"));
    assert_eq!(Some(0..1), or4("a", "b", "c", "d").search(b"abcd"));
    assert_eq!(None, or4("a", "b", "c", "d").search(b"XY"));
}

#[test]
fn test_or5() {
    assert_eq!(None, or5("a", "b", "c", "d", "e").search(b""));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XaY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XbY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XcY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XdY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XeY"));
    assert_eq!(Some(0..1), or5("a", "b", "c", "d", "e").search(b"abcde"));
    assert_eq!(None, or5("a", "b", "c", "d", "e").search(b"XY"));
}

#[test]
fn test_group() {
    let cell: Cell<Option<&[u8]>> = Cell::new(None);
    assert_eq!(Some(0..0), group(&cell, "").search(b""));
    assert_eq!(b"", cell.get().unwrap());
    cell.take();
    assert_eq!(None, group(&cell, "b").search(b"a"));
    assert_eq!(None, cell.get());
    cell.take();
    assert_eq!(Some(1..3), group(&cell, "bb").search(b"abb"));
    assert_eq!(b"bb", cell.get().unwrap());
    cell.take();
    assert_eq!(Some(0..2), group(&cell, "bb").search(b"bbc"));
    assert_eq!(b"bb", cell.get().unwrap());
    cell.take();
    assert_eq!(Some(1..3), group(&cell, "bb").search(b"abbc"));
    assert_eq!(b"bb", cell.get().unwrap());
    {
        let cell_b: Cell<Option<&[u8]>> = Cell::new(None);
        let cell_d: Cell<Option<&[u8]>> = Cell::new(None);
        assert_eq!(
            Some(1..8),
            seq5(
                "a",
                group(&cell_b, ("b", 1..)),
                ("c", 1..),
                group(&cell_d, ("d", 1..)),
                ("e", 1..),
            )
            .search(b"XabcdddeY")
        );
        assert_eq!(b"b", cell_b.get().unwrap());
        assert_eq!(b"ddd", cell_d.get().unwrap());
    }
    {
        let cell_b: Cell<Option<&[u8]>> = Cell::new(None);
        let cell_d: Cell<Option<&[u8]>> = Cell::new(None);
        assert_eq!(
            None,
            seq5(
                "a",
                group(&cell_b, ("b", 1..)),
                ("c", 1..),
                group(&cell_d, ("d", 1..)),
                ("e", 1..),
            )
            .search(b"abbde")
        );
        assert_eq!(b"bb", cell_b.get().unwrap());
        assert_eq!(None, cell_d.get());
    }

    {
        let cell: Cell<Option<&[u8]>> = Cell::new(None);
        let value = group(&cell, "a");
        let value_copy = value; // Copy
        #[allow(clippy::clone_on_copy)]
        let _value_clone = value.clone(); // Clone
        assert_eq!("Group(Cell { value: None }, \"a\")", format!("{:?}", value)); // Debug
        assert!(value < group(&cell, "b")); // PartialOrd
        assert_eq!(value, value_copy); // PartialEq
    }
}
