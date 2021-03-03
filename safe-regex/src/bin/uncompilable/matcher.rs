use core::convert::TryFrom;
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::Range;
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
    fn match_all(data: &[u8]) -> Option<Groups<Self::GroupRanges>>
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
                return Some(Groups::new(group_ranges, data));
            }
        }
        None
    }
}

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
    T: Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
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
    T: Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<S, T> Debug for Matcher<T>
where
    S: AsRef<[std::ops::Range<u32>]> + Debug,
    T: Machine<GroupRanges = S> + Eq + Hash + Debug + Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, r#"Matcher(br"{}")"#, escape_ascii(T::expression()))
    }
}

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
}
