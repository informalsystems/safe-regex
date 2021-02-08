//! [![crates.io version](https://img.shields.io/crates/v/safe-regex.svg)](https://crates.io/crates/safe-regex)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! A safe regular expression library.
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Good test coverage (??%) - TODO(mleonhard) Update.
//! - `no_std`, depends only on `core`
//! - Does not allocate
//! - Checks input in a single pass
//! - No recursion, no risk of stack overflow
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
//! `update_readme.sh` generates `Readme.md`
//! and replaces this section with the report.
//!
//! # Examples
//! ```rust
//! // use safe_regex::simple;
//! // use safe_regex::simple::Regex;
//! //
//! // // "."
//! // simple::any_byte()
//! //     .match_all(b"a")
//! //     .unwrap();
//! //
//! // // "[0-9]"
//! // (b'0'..=b'9').match_all(b"7").unwrap();
//! //
//! // // "[^0-9]"
//! // simple::not(b'0'..=b'9')
//! //     .match_all(b"a")
//! //     .unwrap();
//! //
//! // // "a?"
//! // ("a", ..=1).match_all(b"").unwrap();
//! // ("a", ..=1).match_all(b"a").unwrap();
//! //
//! // // "a+"
//! // ("a", 1..).match_all(b"a").unwrap();
//! // ("a", 1..).match_all(b"aaa").unwrap();
//! //
//! // // "a{3}"
//! // ("a", 3..=3).match_all(b"aaa").unwrap();
//! //
//! // // "a{2,3}"
//! // ("a", 2..=3).match_all(b"aa").unwrap();
//! // ("a", 2..=3).match_all(b"aaa").unwrap();
//! //
//! // // "a|b"
//! // simple::or("a", "b")
//! //     .match_all(b"b")
//! //     .unwrap();
//! //
//! // // "a|b|c|d|e"
//! // simple::or5("a", "b", "c", "d", "e")
//! //     .match_all(b"b").unwrap();
//! //
//! // // "(a|b)(c|d)"
//! // simple::seq(
//! //     simple::or("a", "b"),
//! //     simple::or("c", "d"),
//! // ).match_all(b"bc").unwrap();
//! //
//! // // "id([0-9]+)" capturing group
//! // use std::cell::Cell;
//! // let cell: Cell<Option<&[u8]>> =
//! //     Cell::new(None);
//! // simple::seq(
//! //     "id",
//! //     simple::group(
//! //         &cell, (b'0'..b'9', 1..)
//! // )).match_all(b"id42").unwrap();
//! // assert_eq!(b"42", cell.get().unwrap());
//! ```
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about NFAs, <https://swtch.com/~rsc/regexp/>
//! - Design API
//! - Implement
//! - Add integration tests
//! - Add macro, `regex!(r"[a-z][0-9]")`
//! - Add fuzzing tests
//! - Add common character classes: whitespace, letters, punctuation, etc.
//! - Match strings
//!
//! # TO DO
//! - Once [const generics](https://github.com/rust-lang/rust/issues/44580)
//!   are stable, use the feature to simplify `Repeat` and other types.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`

// https://swtch.com/~rsc/regexp/regexp1.html

#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Range;
use safe_regex_parser::escape_ascii;

pub trait RangeTrait {
    fn is_discarding_range(&self) -> bool;
    fn extend(self, r: &Range<usize>) -> Self;
    fn end(&self) -> usize;
    fn range(&self) -> Range<usize>;
}

#[derive(Clone, PartialEq)]
pub struct DiscardingRange;
impl RangeTrait for DiscardingRange {
    fn is_discarding_range(&self) -> bool {
        true
    }

    fn extend(self, _r: &Range<usize>) -> Self {
        self
    }

    fn end(&self) -> usize {
        0
    }
    fn range(&self) -> Range<usize> {
        0..0
    }
}
impl Debug for DiscardingRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "DiscardingRange")
    }
}

#[derive(Clone)]
pub struct MatchRange<R: RangeTrait + Clone + Debug> {
    start: usize,
    end: usize,
    outer: R,
}
impl<R: RangeTrait + Clone + Debug> MatchRange<R> {
    #[must_use]
    pub fn new(outer: R, n: usize) -> Self {
        if outer.is_discarding_range() {
            Self {
                start: n,
                end: n,
                outer,
            }
        } else {
            let end = outer.range().end;
            if end != n {
                panic!("{} is not immediately after {:?}", n, outer);
            }
            Self {
                start: end,
                end,
                outer,
            }
        }
    }
    pub fn into_outer(self) -> R {
        self.outer
    }
}
impl<R: RangeTrait + Clone + Debug> RangeTrait for MatchRange<R> {
    fn is_discarding_range(&self) -> bool {
        false
    }

    fn extend(mut self, r: &Range<usize>) -> Self {
        println!("extend {:?} + {:?}", &self, &r);
        if self.end != 0 && self.end != r.start {
            panic!("{:?} is not immediately after {:?}", &r, &self);
        }
        if r.end < r.start {
            panic!("bad range: {:?}", &r);
        }
        self.end = r.end;
        self
    }

    fn end(&self) -> usize {
        self.end
    }
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}
impl<R: RangeTrait + Clone + Debug> Debug for MatchRange<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "MatchRange({}..{},{:?})",
            self.start, self.end, self.outer
        )
    }
}

pub trait Matcher {
    type RangeType;
    fn reset(&mut self);
    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType>;
    // This method takes `&mut self` so we can eliminate the `AsRef<[T]>` type
    // param on `Repeat`.
    fn matches_empty(&mut self) -> bool;
}

pub fn match_all<T: Matcher<RangeType = DiscardingRange> + Debug>(
    matcher: &mut T,
    data: &[u8],
) -> bool {
    if data.is_empty() {
        return matcher.matches_empty();
    }
    matcher.reset();
    println!("{:?}", &matcher);
    println!("process_byte {}", escape_ascii([data[0]]));
    let mut result = matcher
        .process_byte(Some(DiscardingRange), data[0], 0)
        .is_some();
    println!("{:?}", &matcher);
    println!("result = {}", result);
    for (n, b) in data.iter().copied().enumerate().skip(1) {
        println!("process_byte {}", escape_ascii([b]));
        result = matcher.process_byte(None, b, n).is_some();
        println!("{:?}", &matcher);
        println!("result = {}", result);
    }
    result
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Byte<R: RangeTrait + Debug> {
    value: u8,
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> Byte<R> {
    #[must_use]
    pub fn new(b: u8) -> Self {
        Self {
            value: b,
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for Byte<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        if b == self.value {
            println!(
                "{:?} extend {:?} + {}..{}",
                self,
                prev_matching_range,
                n,
                n + 1
            );
            #[allow(clippy::range_plus_one)]
            Some(prev_matching_range.extend(&(n..n + 1)))
        } else {
            None
        }
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for Byte<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Byte({})", escape_ascii([self.value]))
    }
}

pub struct AnyByte<R: RangeTrait + Debug> {
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> AnyByte<R> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for AnyByte<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        _b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        println!(
            "{:?} extend {:?} + {}..{}",
            self,
            prev_matching_range,
            n,
            n + 1
        );
        #[allow(clippy::range_plus_one)]
        Some(prev_matching_range.extend(&(n..n + 1)))
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for AnyByte<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "AnyByte")
    }
}

pub struct Class<R: RangeTrait + Debug> {
    incl: bool,
    bytes: &'static [u8],
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> Class<R> {
    #[must_use]
    pub fn new(incl: bool, bytes: &'static [u8]) -> Self {
        Self {
            incl,
            bytes,
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for Class<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        let contains = self.bytes.contains(&b);
        if (self.incl && contains) || (!self.incl && !contains) {
            println!(
                "{:?} extend {:?} + {}..{}",
                self,
                prev_matching_range,
                n,
                n + 1
            );
            #[allow(clippy::range_plus_one)]
            Some(prev_matching_range.extend(&(n..n + 1)))
        } else {
            None
        }
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for Class<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Class({})", escape_ascii(self.bytes))
    }
}

pub struct Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    a: A,
    b: B,
    prev_a_state: Option<R>,
}
impl<R, A, B> Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            prev_a_state: None,
        }
    }

    fn reset_impl(&mut self) {
        self.a.reset();
        self.b.reset();
        self.prev_a_state = None;
    }

    fn process_byte_impl(&mut self, prev_state: Option<R>, b: u8, n: usize) -> Option<R> {
        let prev_a_state_clone = self.prev_a_state.clone();
        let result = self.b.process_byte(self.prev_a_state.take(), b, n);
        println!(
            "Seq.process_byte({},{}) {:?} --{:?}-> {:?}",
            escape_ascii([b]),
            n,
            prev_a_state_clone,
            self.b,
            result
        );

        let prev_state_clone = prev_state.clone();
        self.prev_a_state = self.a.process_byte(prev_state, b, n);
        println!(
            "Seq.process_byte({},{}) {:?} --{:?}-> {:?}",
            escape_ascii([b]),
            n,
            prev_state_clone,
            self.a,
            self.prev_a_state
        );
        result
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.a.matches_empty() && self.b.matches_empty()
    }
}
impl<R, A, B> Matcher for Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Matcher for &mut Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Debug for Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Seq({:?},{:?},{:?})", self.a, self.prev_a_state, self.b)
    }
}

pub struct Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    a: A,
    b: B,
    phantom: PhantomData<R>,
}
impl<R, A, B> Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }

    fn reset_impl(&mut self) {
        self.a.reset();
        self.b.reset();
    }

    fn process_byte_impl(&mut self, prev_state: Option<R>, b: u8, n: usize) -> Option<R> {
        let prev_state_clone = prev_state.clone();
        if let Some(match_range) = self.a.process_byte(prev_state, b, n) {
            return Some(match_range);
        }
        self.b.process_byte(prev_state_clone, b, n)
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.a.matches_empty() || self.b.matches_empty()
    }
}
impl<R, A, B> Matcher for Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Matcher for &mut Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Debug for Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Either({:?},{:?})", self.a, self.b)
    }
}

#[derive(Clone)]
pub struct CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    range: Option<Range<usize>>,
    inner: T,
    phantom: PhantomData<R>,
}
impl<R, T> CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            range: None,
            inner,
            phantom: PhantomData,
        }
    }

    pub fn range(&self) -> Option<Range<usize>> {
        self.range.clone()
    }

    fn reset_impl(&mut self) {
        self.inner.reset();
        self.range = None;
    }

    fn process_byte_impl(
        &mut self,
        prev_outer_state: Option<<CapturingGroup<R, T> as Matcher>::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<<CapturingGroup<R, T> as Matcher>::RangeType> {
        let prev_state = prev_outer_state.map(|p| MatchRange::new(p, n));
        let state = self.inner.process_byte(prev_state, b, n);
        let match_range = state?;
        let range = match_range.range();
        let outer_range = match_range.into_outer().extend(&range);
        self.range = Some(range);
        Some(outer_range)
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.inner.matches_empty()
    }
}
impl<R, T> Matcher for CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Matcher for &mut CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Debug for CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "CapturingGroup({:?})", self.inner)
    }
}
