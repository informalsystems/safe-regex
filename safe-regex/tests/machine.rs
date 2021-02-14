#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;
use safe_regex::internal::{clone_and_increment, clone_and_set, escape_ascii, Machine};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

fn opt_b_to_s(opt_b: Option<u8>) -> String {
    opt_b.map_or(String::from("None"), |b| {
        format!("Some({})", escape_ascii([b]))
    })
}

#[test]
fn byte() {
    // "a"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Byte0,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Byte0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Byte0, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Byte0, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Byte0));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"aa"));
}

#[test]
fn class_inclusive() {
    // "[abc]"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Class0,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Class0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Class0, Some(b)) if b"abc".contains(&b) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Class0, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Class0));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    Re::match_all(b"b").unwrap();
    Re::match_all(b"c").unwrap();
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"aa"));
    assert_eq!(None, Re::match_all(b"abc"));
}

#[test]
fn class_exclusive() {
    // "[^abc]"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Class0,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Class0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Class0, Some(b)) if !b"abc".contains(&b) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Class0, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Class0));
    assert_eq!(None, Re::match_all(b""));
    Re::match_all(b"X").unwrap();
    Re::match_all(b"Y").unwrap();
    assert_eq!(None, Re::match_all(b"XY"));
    assert_eq!(None, Re::match_all(b"a"));
    assert_eq!(None, Re::match_all(b"b"));
    assert_eq!(None, Re::match_all(b"c"));
}

#[test]
fn seq() {
    // "aab"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Byte0,
        Byte1,
        Byte2,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Byte0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Byte0, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Byte1);
                }
                (Self::Byte0, Some(_)) => {}
                (Self::Byte1, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Byte2);
                }
                (Self::Byte1, Some(_)) => {}
                (Self::Byte2, Some(b'b')) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Byte2, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Byte0));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"aa"));
    assert_eq!(None, Re::match_all(b"Xaa"));
    assert_eq!(None, Re::match_all(b"aXa"));
    assert_eq!(None, Re::match_all(b"aaX"));
    assert_eq!(None, Re::match_all(b"aaa"));
    assert_eq!(None, Re::match_all(b"aaX"));
    Re::match_all(b"aab").unwrap();
    assert_eq!(None, Re::match_all(b"Xaab"));
    assert_eq!(None, Re::match_all(b"aXab"));
    assert_eq!(None, Re::match_all(b"aaXb"));
    assert_eq!(None, Re::match_all(b"aabX"));
    assert_eq!(None, Re::match_all(b"aaba"));
    assert_eq!(None, Re::match_all(b"aabaa"));
    assert_eq!(None, Re::match_all(b"aabaab"));
}

#[test]
fn alternates() {
    // "a|b"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Alt0,
        Alt0Byte0,
        Alt0Byte1,
        Alt0Matched,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Alt0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            if let Self::Accept = self {
                Some([])
            } else {
                None
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Alt0, Some(b)) => {
                    Self::Alt0Byte0.make_next_states(Some(b), n, next_states);
                    Self::Alt0Byte1.make_next_states(Some(b), n, next_states);
                }
                (Self::Alt0Byte0, Some(b'a')) => {
                    Self::Alt0Matched.make_next_states(None, n, next_states)
                }
                (Self::Alt0Byte0, Some(_)) => {}
                (Self::Alt0Byte1, Some(b'b')) => {
                    Self::Alt0Matched.make_next_states(None, n, next_states)
                }
                (Self::Alt0Byte1, Some(_)) => {}
                (Self::Alt0Matched, None) => {
                    next_states.insert(Self::Accept);
                }
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    Re::match_all(b"b").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"bX"));
    assert_eq!(None, Re::match_all(b"Xb"));
    assert_eq!(None, Re::match_all(b"ab"));
    assert_eq!(None, Re::match_all(b"aa"));
    assert_eq!(None, Re::match_all(b"ba"));
    assert_eq!(None, Re::match_all(b"bb"));
}

#[test]
fn group() {
    // "(ab)"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Group0([Range<u32>; 1]),
        Group0Byte0([Range<u32>; 1]),
        Group0Byte1([Range<u32>; 1]),
        Group0Matched([Range<u32>; 1]),
        Accept([Range<u32>; 1]),
    }
    impl Machine for Re {
        type State = [Range<u32>; 1];
        fn start() -> Self {
            Self::Group0([u32::MAX..u32::MAX])
        }
        fn accept(&self) -> Option<Self::State> {
            if let Self::Accept(range) = self {
                Some(range.clone())
            } else {
                None
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Group0(prev_ranges), Some(b)) => {
                    let ranges = clone_and_set(prev_ranges, 0, n..n);
                    Self::Group0Byte0(ranges).make_next_states(Some(b), n, next_states);
                }
                (Self::Group0Byte0(prev_ranges), Some(b'a')) => {
                    let ranges = clone_and_increment(prev_ranges, 0);
                    next_states.insert(Self::Group0Byte1(ranges));
                }
                (Self::Group0Byte0(_), Some(_)) => {}
                (Self::Group0Byte1(prev_ranges), Some(b'b')) => {
                    let ranges = clone_and_increment(prev_ranges, 0);
                    Self::Group0Matched(ranges).make_next_states(None, n, next_states);
                }
                (Self::Group0Byte1(_), Some(_)) => {}
                (Self::Group0Matched(ranges), None) => {
                    next_states.insert(Self::Accept(ranges.clone()));
                }
                (Self::Accept(_), _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"bX"));
    assert_eq!(None, Re::match_all(b"Xb"));
    assert_eq!(None, Re::match_all(b"Xab"));
    assert_eq!(None, Re::match_all(b"aXb"));
    assert_eq!(None, Re::match_all(b"abX"));
    assert_eq!(None, Re::match_all(b"aba"));
    assert_eq!(None, Re::match_all(b"abab"));

    let groups = Re::match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0));
    assert_eq!("ab", escape_ascii(groups.group(0)));
}

#[test]
fn optional() {
    // "a?"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Optional0,
        Optional0Byte0,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Optional0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Optional0 => Some([]),
                Self::Accept => Some([]),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Optional0, Some(b)) => {
                    // '?' matches.
                    Self::Optional0Byte0.make_next_states(Some(b), n, next_states);
                    // '?' doesn't match.
                    Self::Accept.make_next_states(Some(b), n, next_states);
                }
                (Self::Optional0Byte0, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Optional0Byte0, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Optional0));
    Re::match_all(b"").unwrap();
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"aa"));
}

#[test]
fn optional_at_start() {
    // "a?a"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Optional0,
        Optional0Byte1,
        Byte2,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Optional0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            if let Self::Accept = self {
                Some([])
            } else {
                None
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Optional0, Some(b)) => {
                    // '?' matches.
                    Self::Optional0Byte1.make_next_states(Some(b), n, next_states);
                    // '?' doesn't match.
                    Self::Byte2.make_next_states(Some(b), n, next_states);
                }
                (Self::Optional0Byte1, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Byte2);
                }
                (Self::Optional0Byte1, Some(_)) => {}
                (Self::Byte2, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Byte2, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    Re::match_all(b"aa").unwrap();
    assert_eq!(None, Re::match_all(b"aaX"));
    assert_eq!(None, Re::match_all(b"Xaa"));
    assert_eq!(None, Re::match_all(b"aXa"));
    assert_eq!(None, Re::match_all(b"aaa"));
}

#[test]
fn optional_at_end() {
    // "aa?"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Byte0,
        Optional1,
        Optional1Byte1,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Byte0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Optional1 | Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Byte0, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Optional1);
                }
                (Self::Byte0, Some(_)) => {}
                (Self::Optional1, Some(b)) => {
                    // '?' matches.
                    Self::Optional1Byte1.make_next_states(Some(b), n, next_states);
                    // '?' doesn't match.
                    Self::Accept.make_next_states(Some(b), n, next_states);
                }
                (Self::Optional1Byte1, Some(b'a')) => {
                    // Consume byte.
                    next_states.insert(Self::Accept);
                }
                (Self::Optional1Byte1, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::Byte0));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    Re::match_all(b"aa").unwrap();
    assert_eq!(None, Re::match_all(b"aaX"));
    assert_eq!(None, Re::match_all(b"Xaa"));
    assert_eq!(None, Re::match_all(b"aXa"));
    assert_eq!(None, Re::match_all(b"aaa"));
}

#[test]
fn star() {
    // "a*aa"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Star0,
        Star0Byte0,
        Star0Matched,
        Byte1,
        Byte2,
        Accept,
    }
    impl Machine for Re {
        type State = [Range<u32>; 0];
        fn start() -> Self {
            Self::Star0
        }
        fn accept(&self) -> Option<[Range<u32>; 0]> {
            match self {
                Self::Star0 => Some([]),
                Self::Accept => Some([]),
                _ => None,
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Star0, Some(b)) => {
                    // '*' matches.
                    Self::Star0Byte0.make_next_states(Some(b), n, next_states);
                    // '*' doesn't match.
                    Self::Byte1.make_next_states(Some(b), n, next_states)
                }
                (Self::Star0Byte0, Some(b'a')) => {
                    Self::Star0Matched.make_next_states(None, n, next_states)
                }
                (Self::Star0Byte0, Some(_)) => {}
                (Self::Star0Matched, None) => {
                    next_states.insert(Self::Star0);
                }
                (Self::Byte1, Some(b'a')) => {
                    next_states.insert(Self::Byte2);
                }
                (Self::Byte1, Some(_)) => {}
                (Self::Byte2, Some(b'a')) => {
                    next_states.insert(Self::Accept);
                }
                (Self::Byte2, Some(_)) => {}
                (Self::Accept, _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    Re::match_all(b"").unwrap();
    assert_eq!(None, Re::match_all(b"X"));
    Re::match_all(b"a").unwrap();
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    Re::match_all(b"aa").unwrap();
    assert_eq!(None, Re::match_all(b"Xaa"));
    assert_eq!(None, Re::match_all(b"aXa"));
    assert_eq!(None, Re::match_all(b"aaX"));
    Re::match_all(b"aaa").unwrap();
    assert_eq!(None, Re::match_all(b"Xaaa"));
    assert_eq!(None, Re::match_all(b"aXaa"));
    assert_eq!(None, Re::match_all(b"aaXa"));
    assert_eq!(None, Re::match_all(b"aaaX"));
    Re::match_all(b"aaaa").unwrap();
    assert_eq!(None, Re::match_all(b"Xaaaa"));
    assert_eq!(None, Re::match_all(b"aXaaa"));
    assert_eq!(None, Re::match_all(b"aaXaa"));
    assert_eq!(None, Re::match_all(b"aaaXa"));
    assert_eq!(None, Re::match_all(b"aaaaX"));
    Re::match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    assert_eq!(
        None,
        Re::match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaX")
    );
}

#[test]
fn alternates_in_group() {
    // "(a|b)"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Group0([Range<u32>; 1]),
        Group0Alt0([Range<u32>; 1]),
        Group0Alt0Byte0([Range<u32>; 1]),
        Group0Alt0Byte1([Range<u32>; 1]),
        Group0Alt0Matched([Range<u32>; 1]),
        Group0Matched([Range<u32>; 1]),
        Accept([Range<u32>; 1]),
    }
    impl Machine for Re {
        type State = [Range<u32>; 1];
        fn start() -> Self {
            Self::Group0([u32::MAX..u32::MAX])
        }
        fn accept(&self) -> Option<Self::State> {
            if let Self::Accept(range) = self {
                Some(range.clone())
            } else {
                None
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Group0(prev_ranges), Some(b)) => {
                    let ranges = clone_and_set(prev_ranges, 0, n..n);
                    Self::Group0Alt0(ranges).make_next_states(Some(b), n, next_states);
                }
                (Self::Group0Alt0(ranges), Some(b)) => {
                    Self::Group0Alt0Byte0(ranges.clone()).make_next_states(Some(b), n, next_states);
                    Self::Group0Alt0Byte1(ranges.clone()).make_next_states(Some(b), n, next_states);
                }
                (Self::Group0Alt0Byte0(prev_ranges), Some(b'a')) => {
                    let ranges = clone_and_increment(prev_ranges, 0);
                    Self::Group0Alt0Matched(ranges).make_next_states(None, n, next_states);
                }
                (Self::Group0Alt0Byte0(_), Some(_)) => {}
                (Self::Group0Alt0Byte1(prev_ranges), Some(b'b')) => {
                    let ranges = clone_and_increment(prev_ranges, 0);
                    Self::Group0Alt0Matched(ranges).make_next_states(None, n, next_states);
                }
                (Self::Group0Alt0Byte1(_), Some(_)) => {}
                (Self::Group0Alt0Matched(ranges), None) => {
                    Self::Group0Matched(ranges.clone()).make_next_states(None, n, next_states)
                }
                (Self::Group0Matched(ranges), None) => {
                    next_states.insert(Self::Accept(ranges.clone()));
                }
                (Self::Accept(_), _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }
    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"bX"));
    assert_eq!(None, Re::match_all(b"Xb"));
    assert_eq!(None, Re::match_all(b"ab"));
    assert_eq!(None, Re::match_all(b"aa"));
    assert_eq!(None, Re::match_all(b"ba"));
    assert_eq!(None, Re::match_all(b"bb"));

    let groups = Re::match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0));
    assert_eq!("a", escape_ascii(groups.group(0)));

    let groups = Re::match_all(b"b").unwrap();
    assert_eq!(0..1, groups.group_range(0));
    assert_eq!("b", escape_ascii(groups.group(0)));
}

#[test]
fn optionals_in_groups() {
    // "(a?)(ab)"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Re {
        Group0Start([Range<u32>; 2]),
        Group0Optional0([Range<u32>; 2]),
        Group0Optional0Byte0([Range<u32>; 2]),
        Group1Start([Range<u32>; 2]),
        Group1Byte0([Range<u32>; 2]),
        Group1Byte1([Range<u32>; 2]),
        Accept([Range<u32>; 2]),
    }
    impl Machine for Re {
        type State = [Range<u32>; 2];
        fn start() -> Self {
            Self::Group0Start([u32::MAX..u32::MAX, u32::MAX..u32::MAX])
        }
        fn accept(&self) -> Option<Self::State> {
            if let Self::Accept(range) = self {
                Some(range.clone())
            } else {
                None
            }
        }
        fn make_next_states(
            &self,
            opt_b: Option<u8>,
            n: u32,
            next_states: &mut HashSet<Self, RandomState>,
        ) {
            println!("make_next_states {} {} {:?}", opt_b_to_s(opt_b), n, self);
            match (self, opt_b) {
                (Self::Group0Start(prev_ranges), Some(b)) => {
                    let ranges = clone_and_set(prev_ranges, 0, n..n);
                    Self::Group0Optional0(ranges).make_next_states(Some(b), n, next_states);
                }
                (Self::Group0Optional0(ranges), Some(b)) => {
                    // '?' matches
                    Self::Group0Optional0Byte0(ranges.clone()).make_next_states(
                        Some(b),
                        n,
                        next_states,
                    );
                    // '?' doesn't match
                    Self::Group1Start(ranges.clone()).make_next_states(Some(b), n, next_states);
                }
                (Self::Group0Optional0Byte0(prev_ranges), Some(b'a')) => {
                    // Consume byte.
                    let ranges = clone_and_increment(prev_ranges, 0);
                    next_states.insert(Self::Group1Start(ranges));
                }
                (Self::Group0Optional0Byte0(_), Some(_)) => {}
                (Self::Group1Start(prev_ranges), Some(b)) => {
                    let ranges = clone_and_set(prev_ranges, 1, n..n);
                    Self::Group1Byte0(ranges).make_next_states(Some(b), n, next_states);
                }
                (Self::Group1Byte0(prev_ranges), Some(b'a')) => {
                    // Consume byte.
                    let ranges = clone_and_increment(prev_ranges, 1);
                    next_states.insert(Self::Group1Byte1(ranges));
                }
                (Self::Group1Byte0(_), Some(_)) => {}
                (Self::Group1Byte1(prev_ranges), Some(b'b')) => {
                    // Consume byte.
                    let ranges = clone_and_increment(prev_ranges, 1);
                    next_states.insert(Self::Accept(ranges));
                }
                (Self::Group1Byte1(_), Some(_)) => {}
                (Self::Accept(_), _) => {}
                other => panic!("invalid state transition {:?}", other),
            }
        }
    }

    println!("size {} bytes", core::mem::size_of_val(&Re::start()));
    assert_eq!(None, Re::match_all(b""));
    assert_eq!(None, Re::match_all(b"X"));
    assert_eq!(None, Re::match_all(b"aX"));
    assert_eq!(None, Re::match_all(b"Xa"));
    assert_eq!(None, Re::match_all(b"abX"));
    assert_eq!(None, Re::match_all(b"Xab"));
    assert_eq!(None, Re::match_all(b"aXb"));
    assert_eq!(None, Re::match_all(b"aaX"));
    assert_eq!(None, Re::match_all(b"Xaa"));
    assert_eq!(None, Re::match_all(b"aXa"));
    assert_eq!(None, Re::match_all(b"aabX"));
    assert_eq!(None, Re::match_all(b"Xaab"));
    assert_eq!(None, Re::match_all(b"aaXb"));
    assert_eq!(None, Re::match_all(b"aXab"));
    assert_eq!(None, Re::match_all(b"aba"));
    assert_eq!(None, Re::match_all(b"abab"));
    assert_eq!(None, Re::match_all(b"aaba"));
    assert_eq!(None, Re::match_all(b"aabaa"));
    assert_eq!(None, Re::match_all(b"aabaab"));

    let groups = Re::match_all(b"ab").unwrap();
    assert_eq!("", escape_ascii(groups.group(0)));
    assert_eq!("ab", escape_ascii(groups.group(1)));

    let groups = Re::match_all(b"aab").unwrap();
    assert_eq!(0..1, groups.group_range(0));
    assert_eq!(1..3, groups.group_range(1));
    assert_eq!("a", escape_ascii(groups.group(0)));
    assert_eq!("ab", escape_ascii(groups.group(1)));
}
