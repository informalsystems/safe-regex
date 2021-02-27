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
//! - Checks input in a single pass.
//!   Runtime and memory usage are both `O(n * r * 2^g)` where
//!   - `n` is the length of the data to check
//!   - `r` is the length of the regex
//!   - `g` is the number of capturing groups in the regex
//!   TODO(mleonhard) Confirm this with a benchmark.
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
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about NFAs, <https://swtch.com/~rsc/regexp/>
//! - DONE - Design API
//! - DONE - Implement
//! - DONE - Add integration tests
//! - DONE - Add macro, `regex!(r"[a-z][0-9]")`
//! - Add fuzzing tests
//! - Add common character classes: whitespace, letters, punctuation, etc.
//! - Match strings
//! - Implement optimizations explained in <https://swtch.com/%7Ersc/regexp/regexp3.html> .
//!   Some of the code already exists in `tests/dfa_single_pass.rs`
//!   and `tests/nfa_without_capturing.rs`.
//!
//! # TO DO
//! - Add a memory-limited `match_all` fn, for use on untrusted data.
//!   Make it the default.
//! - Once [const generics](https://github.com/rust-lang/rust/issues/44580)
//!   are stable, use the feature to simplify `Repeat` and other types.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`

// https://swtch.com/~rsc/regexp/regexp1.html

#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::Range;
pub use safe_regex_macro::regex;

/// A compiled regular expression.
///
/// This is a zero-length type.
/// The `regex!` macro generates a Rust type that implements the regular expression.
/// This `Matcher` is just a holder for that type.
pub struct Matcher<T> {
    phantom: PhantomData<T>,
}
impl<S, T> Matcher<T>
where
    S: AsRef<[std::ops::Range<u32>]> + Debug,
    T: internal::Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
{
    /// Executes the regular expression against the byte string `data`.
    ///
    /// Returns `Some` if the expresison matched all of the bytes in `data`.
    ///
    /// This is not a sub-string search.
    /// If you need a sub-string search,
    /// put `.*` at the beginning and end of the regex.
    ///
    /// Returns `None` if the expression did not match `data`.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<Groups<'d, T::GroupRanges>> {
        T::match_all(data)
    }

    /// This is used internally by the `regex!` macro.
    ///
    /// We can make this function `const` when
    /// [trait bounds on \`const fn\` parameters are stable](https://github.com/rust-lang/rust/issues/57563).
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<S, T> Default for Matcher<T>
where
    S: AsRef<[std::ops::Range<u32>]> + Debug,
    T: internal::Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<S, T> Debug for Matcher<T>
where
    S: AsRef<[std::ops::Range<u32>]> + Debug,
    T: internal::Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            r#"Matcher(br"{}")"#,
            internal::escape_ascii(T::expression())
        )
    }
}

// TODO(mleonhard) Replace this run-time checking with compile-time checking.
#[derive(Clone, Debug, PartialEq)]
/// Groups captured by a regular expression.
pub struct Groups<'d, T: AsRef<[Range<u32>]>> {
    ranges: T,
    data: &'d [u8],
}
impl<'d, T: AsRef<[Range<u32>]>> Groups<'d, T> {
    /// Creates a new struct.
    ///
    /// `data` is the byte string the regular expression matched against.
    ///
    /// `ranges` is an array of ranges which are the regions inside `data`
    /// that matched capturing groups.
    pub fn new(ranges: T, data: &'d [u8]) -> Self {
        Self { ranges, data }
    }

    /// Get the range of capturing group number `n`.
    /// To find the `n` value for a particular group in a regex, count the
    /// number of open parenthesis `(` symbols that appear before the group.
    ///
    /// Note: Group 0 is the first group.
    /// It is NOT the matching portion of the string.
    ///
    /// Returns None if the group did not match any portion of the string.
    ///
    /// Panics if the regular expression does not have a capturing group `n`.
    ///
    /// # Examples
    /// ```rust
    /// use safe_regex::{regex, Matcher};
    /// let re: Matcher<_> = regex!(br"(a)(b)");
    /// let groups = re.match_all(b"ab").unwrap();
    /// assert_eq!(0..1, groups.group_range(0).unwrap());
    /// assert_eq!(1..2, groups.group_range(1).unwrap());
    /// // groups.group_range(2); // panics
    /// ```
    ///
    /// ```rust
    /// use safe_regex::{regex, Matcher};
    /// let re: Matcher<_> = regex!(br"(a)|(b)");
    /// let groups = re.match_all(b"b").unwrap();
    /// assert_eq!(None, groups.group_range(0));
    /// assert_eq!(Some(0..1), groups.group_range(1));
    /// ```
    pub fn group_range(&self, n: usize) -> Option<Range<usize>> {
        if let Some(r) = self.ranges.as_ref().get(n) {
            if *r == (u32::MAX..u32::MAX) {
                None
            } else {
                Some((r.start as usize)..(r.end as usize))
            }
        } else {
            panic!("group {} not found in Match struct", n)
        }
    }

    /// Gets the slice matched by capturing group number `n`.
    ///
    /// To find the `n` value for a particular group in a regex, count the
    /// number of open parenthesis `(` symbols that appear before the group.
    ///
    /// Note: Group 0 is the first group.
    /// It is NOT the matching portion of the string.
    ///
    /// Returns None if the group did not match any portion of the string.
    ///
    /// Panics if the regular expression does not have a capturing group `n`.
    ///
    /// # Examples
    /// ```rust
    /// use safe_regex::{regex, Matcher};
    /// let re: Matcher<_> = regex!(br"(a)(b)");
    /// let groups = re.match_all(b"ab").unwrap();
    /// assert_eq!(b"a", groups.group(0).unwrap());
    /// assert_eq!(b"b", groups.group(1).unwrap());
    /// // groups.group(2); // panics
    /// ```
    ///
    /// ```rust
    /// use safe_regex::{regex, Matcher};
    /// let re: Matcher<_> = regex!(br"(a)|(b)");
    /// let groups = re.match_all(b"b").unwrap();
    /// assert_eq!(None, groups.group(0));
    /// assert_eq!(b"b", groups.group(1).unwrap());
    /// ```
    pub fn group(&self, n: usize) -> Option<&[u8]> {
        Some(&self.data[self.group_range(n)?])
    }
}

pub mod internal {
    use core::convert::TryFrom;
    use core::fmt::Debug;
    use core::hash::Hash;
    use core::ops::Range;
    pub use safe_regex_macro::regex;
    use std::collections::HashSet;

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

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum InputByte {
        Available(u8, u32),
        Consumed(u32),
    }
    impl InputByte {
        #[must_use]
        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn byte(&self) -> Option<u8> {
            match self {
                InputByte::Available(b, _n) => Some(*b),
                InputByte::Consumed(_n) => None,
            }
        }
        #[must_use]
        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn index(&self) -> u32 {
            match self {
                InputByte::Available(_b, n) => *n,
                InputByte::Consumed(n) => *n,
            }
        }
        #[must_use]
        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn consume(self) -> Self {
            if let Self::Available(_b, n) = self {
                Self::Consumed(n + 1)
            } else {
                panic!("`consume()` called on {:?}", self)
            }
        }
    }
    impl core::fmt::Debug for InputByte {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            match self {
                InputByte::Available(b, n) => {
                    write!(f, "InputByte::Available(b'{}',{})", escape_ascii([*b]), n)
                }
                InputByte::Consumed(n) => write!(f, "InputByte::Consumed({})", n),
            }
        }
    }

    pub trait Machine {
        type GroupRanges;
        fn expression() -> &'static [u8];
        fn start(next_states: &mut HashSet<Self>)
        where
            Self: Sized;
        fn try_accept(&self) -> Option<Self::GroupRanges>;
        fn make_next_states(&self, b: u8, n: u32, next_states: &mut HashSet<Self>)
        where
            Self: Sized;

        #[must_use]
        fn match_all(data: &[u8]) -> Option<crate::Groups<Self::GroupRanges>>
        where
            Self: Eq + Hash + Debug + Sized,
            Self::GroupRanges: AsRef<[Range<u32>]> + Debug,
        {
            assert!(data.len() < u32::MAX as usize);
            // println!("match_all b\"{}\"", escape_ascii(data));
            // We store states in a set to eliminate duplicate states.
            // This is necessary for the algorithm to work in useful time and memory.
            let mut states: HashSet<Self> = HashSet::new();
            Self::start(&mut states);
            // println!("states = {:?}", states);
            let mut next_states: HashSet<Self> = HashSet::new();
            for (n, b) in data.iter().enumerate() {
                // println!("process_byte {}", escape_ascii([*b]));
                // We call `HashSet::drain` to use less memory.
                // It might be faster to just use `iter()` and then call
                // `HashSet::clear` after the loop.  Let's test before changing it.
                for state in states.drain() {
                    state.make_next_states(*b, u32::try_from(n).unwrap(), &mut next_states);
                }
                core::mem::swap(&mut states, &mut next_states);
                // println!("states = {:?}", states);
                if states.is_empty() {
                    return None;
                }
            }
            for state in states {
                if let Some(group_ranges) = state.try_accept() {
                    // println!("group_ranges = {:?}", group_ranges);
                    return Some(crate::Groups::new(group_ranges, data));
                }
            }
            None
        }
    }
}
