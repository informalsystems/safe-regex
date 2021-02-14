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
use core::hash::Hash;
use core::ops::Range;
pub use safe_regex_macro::regex;
use std::collections::HashSet;
use std::marker::PhantomData;

pub struct Matcher<T> {
    phantom: PhantomData<T>,
}
impl<S, T> Matcher<T>
where
    S: AsRef<[std::ops::Range<u32>]> + Debug,
    T: internal::Machine<State = S> + Eq + Hash + Debug + Sized,
{
    /// We can make this function `const` when
    /// [trait bounds on \`const fn\` parameters are stable](https://github.com/rust-lang/rust/issues/57563).
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
    pub fn match_all<'d>(&self, data: &'d [u8]) -> Option<Groups<'d, T::State>> {
        T::match_all(data)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'d, T: AsRef<[Range<u32>]>> {
    ranges: T,
    data: &'d [u8],
}
impl<'d, T: AsRef<[Range<u32>]>> Groups<'d, T> {
    pub fn new(ranges: T, data: &'d [u8]) -> Self {
        Self { ranges, data }
    }

    pub fn group_range(&self, n: usize) -> Range<usize> {
        if let Some(r) = self.ranges.as_ref().get(n) {
            (r.start as usize)..(r.end as usize)
        } else {
            panic!("group {} not found in Match struct", n);
        }
    }

    pub fn group(&self, n: usize) -> &[u8] {
        &self.data[self.group_range(n)]
    }
}

pub mod internal {
    use core::fmt::Debug;
    use core::hash::Hash;
    use core::ops::Range;
    pub use safe_regex_macro::regex;
    use std::collections::HashSet;
    use std::marker::PhantomData;

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

    pub trait Machine {
        type State;
        fn start() -> Self;
        fn accept(&self) -> Option<Self::State>;
        fn make_next_states(&self, opt_b: Option<u8>, n: u32, next_states: &mut HashSet<Self>)
        where
            Self: Sized;

        fn match_all(data: &[u8]) -> Option<crate::Groups<Self::State>>
        where
            Self: Eq + Hash + Debug + Sized,
            Self::State: AsRef<[Range<u32>]> + Debug,
        {
            let start = Self::start();
            println!("match_all b\"{}\" {:?}", escape_ascii(data), start);
            if data.is_empty() {
                return start.accept().map(|s| crate::Groups::new(s, data));
            }
            // We store states in a set to eliminate duplicate states.
            // This is necessary for the algorithm to work in useful time and memory.
            let mut states: HashSet<Self> = HashSet::new();
            states.insert(start);
            let mut next_states: HashSet<Self> = HashSet::new();
            for (n, b) in data.iter().enumerate() {
                println!("process_byte {}", escape_ascii([*b]));
                // We call `HashSet::drain` to use less memory.
                // It might be faster to just use `iter()` and then call
                // `HashSet::clear` after the loop.  Let's test before changing it.
                for state in states.drain() {
                    state.make_next_states(Some(*b), n as u32, &mut next_states);
                }
                core::mem::swap(&mut states, &mut next_states);
                println!("states = {:?}", states);
                if states.is_empty() {
                    return None;
                }
            }
            for state in states {
                if let Some(accept) = state.accept() {
                    println!("accept = {:?}", accept);
                    return Some(crate::Groups::new(accept, data));
                }
            }
            None
        }
    }

    pub fn clone_and_set<T: AsMut<[Range<u32>]> + Clone>(array: &T, n: u32, item: Range<u32>) -> T {
        let mut array_clone = array.clone();
        array_clone.as_mut()[n as usize] = item;
        array_clone
    }

    pub fn clone_and_increment<T: AsMut<[Range<u32>]> + Clone>(array: &T, n: u32) -> T {
        let mut array_clone = array.clone();
        array_clone.as_mut()[n as usize].end += 1;
        array_clone
    }
}
