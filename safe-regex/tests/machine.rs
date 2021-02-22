#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::hash::Hash;
use safe_regex::internal::escape_ascii;

#[test]
fn byte() {
    let re = {
        #[doc = "br\"a\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Byte0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Byte0(ranges), Some(97u8)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Byte0(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
}

#[test]
fn any_byte() {
    let re = {
        #[doc = "br\".\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            AnyByte0([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::AnyByte0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::AnyByte0(ranges), Some(_)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    re.match_all(b"X").unwrap();
    assert_eq!(None, re.match_all(b"XY"));
}

#[test]
fn class_inclusive() {
    let re = {
        #[doc = "br\"[abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Class0([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Class0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Class0(ranges), Some(b))
                        if b == 97u8 || b == 98u8 || b == 99u8 || (50u8..=52u8).contains(&b) =>
                    {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Class0(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    re.match_all(b"b").unwrap();
    re.match_all(b"c").unwrap();
    assert_eq!(None, re.match_all(b"1"));
    re.match_all(b"2").unwrap();
    re.match_all(b"3").unwrap();
    re.match_all(b"4").unwrap();
    assert_eq!(None, re.match_all(b"5"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"abc"));
}

#[test]
fn class_exclusive() {
    let re = {
        #[doc = "br\"[^abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Class0([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Class0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Class0(ranges), Some(b))
                        if b != 97u8 && b != 98u8 && b != 99u8 && !(50u8..=52u8).contains(&b) =>
                    {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Class0(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    re.match_all(b"X").unwrap();
    re.match_all(b"Y").unwrap();
    assert_eq!(None, re.match_all(b"XY"));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"b"));
    assert_eq!(None, re.match_all(b"c"));
    re.match_all(b"1").unwrap();
    assert_eq!(None, re.match_all(b"2"));
    assert_eq!(None, re.match_all(b"3"));
    assert_eq!(None, re.match_all(b"4"));
    re.match_all(b"5").unwrap();
}

#[test]
fn seq() {
    let re = {
        #[doc = "br\"aab\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0([core::ops::Range<u32>; 1usize]),
            Byte1([core::ops::Range<u32>; 1usize]),
            Byte2([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Byte0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Byte1(ranges_clone));
                    }
                    (Self::Byte0(_), Some(_)) => {}
                    (Self::Byte1(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Byte2(ranges_clone));
                    }
                    (Self::Byte1(_), Some(_)) => {}
                    (Self::Byte2(ranges), Some(b'b')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Byte2(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"aaa"));
    assert_eq!(None, re.match_all(b"aaX"));
    re.match_all(b"aab").unwrap();
    assert_eq!(None, re.match_all(b"Xaab"));
    assert_eq!(None, re.match_all(b"aXab"));
    assert_eq!(None, re.match_all(b"aaXb"));
    assert_eq!(None, re.match_all(b"aabX"));
    assert_eq!(None, re.match_all(b"aaba"));
    assert_eq!(None, re.match_all(b"aabaa"));
    assert_eq!(None, re.match_all(b"aabaab"));
}

#[test]
fn alternates() {
    let re = {
        #[doc = "br\"a|b\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Alt0([core::ops::Range<u32>; 1usize]),
            Alt0Byte0([core::ops::Range<u32>; 1usize]),
            Alt0Byte1([core::ops::Range<u32>; 1usize]),
            Alt0Matched([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Alt0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Alt0(ranges), Some(b)) => {
                        Self::Alt0Byte0(ranges.clone()).make_next_states(Some(b), n, next_states);
                        Self::Alt0Byte1(ranges.clone()).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Alt0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        Self::Alt0Matched(ranges_clone).make_next_states(None, n, next_states)
                    }
                    (Self::Alt0Byte0(_), Some(_)) => {}
                    (Self::Alt0Byte1(ranges), Some(b'b')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        Self::Alt0Matched(ranges_clone).make_next_states(None, n, next_states)
                    }
                    (Self::Alt0Byte1(_), Some(_)) => {}
                    (Self::Alt0Matched(ranges), None) => {
                        let ranges_clone = ranges.clone();
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    re.match_all(b"b").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));
}

#[test]
fn group() {
    let re = {
        #[doc = "br\"(a)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Group0([core::ops::Range<u32>; 2usize]),
            GroupMatched1([core::ops::Range<u32>; 2usize]),
            Byte2([core::ops::Range<u32>; 2usize]),
            Accept([core::ops::Range<u32>; 2usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 2usize];
            fn start() -> Self {
                Self::Group0([0..0, u32::MAX..u32::MAX])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Byte2(ranges), Some(97u8)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        Self::GroupMatched1(ranges_clone).make_next_states(None, n, next_states)
                    }
                    (Self::Byte2(_), Some(_)) => {}
                    (Self::Group0(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize] = n..n;
                        Self::Byte2(ranges_clone).make_next_states(Some(b), n, next_states);
                    }
                    (Self::GroupMatched1(ranges), None) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[1usize].end;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
}

#[test]
fn optional() {
    let re = {
        #[doc = "br\"a?\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Optional0([core::ops::Range<u32>; 1usize]),
            Optional0Byte0([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Optional0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Optional0(ranges) | Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Optional0(ranges), Some(b)) => {
                        // '?' matches.
                        Self::Optional0Byte0(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                        // '?' doesn't match.
                        Self::Accept(ranges.clone()).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Optional0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Optional0Byte0(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    re.match_all(b"").unwrap();
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
}

#[test]
fn optional_at_start() {
    let re = {
        #[doc = "br\"a?a\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Optional0([core::ops::Range<u32>; 1usize]),
            Optional0Byte1([core::ops::Range<u32>; 1usize]),
            Byte2([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Optional0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Optional0(ranges), Some(b)) => {
                        // '?' matches.
                        Self::Optional0Byte1(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                        // '?' doesn't match.
                        Self::Byte2(ranges.clone()).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Optional0Byte1(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Byte2(ranges_clone));
                    }
                    (Self::Optional0Byte1(_), Some(_)) => {}
                    (Self::Byte2(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Byte2(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaa"));
}

#[test]
fn optional_at_end() {
    let re = {
        #[doc = "br\"aa?\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0([core::ops::Range<u32>; 1usize]),
            Optional1([core::ops::Range<u32>; 1usize]),
            Optional1Byte1([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Byte0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Optional1(ranges) | Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Optional1(ranges_clone));
                    }
                    (Self::Byte0(_), Some(_)) => {}
                    (Self::Optional1(ranges), Some(b)) => {
                        // '?' matches.
                        Self::Optional1Byte1(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                        // '?' doesn't match.
                        Self::Accept(ranges.clone()).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Optional1Byte1(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Optional1Byte1(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaa"));
}

#[test]
fn star() {
    let re = {
        #[doc = "br\"a*aa\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Star0([core::ops::Range<u32>; 1usize]),
            Star0Byte0([core::ops::Range<u32>; 1usize]),
            Star0Matched([core::ops::Range<u32>; 1usize]),
            Byte1([core::ops::Range<u32>; 1usize]),
            Byte2([core::ops::Range<u32>; 1usize]),
            Accept([core::ops::Range<u32>; 1usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 1usize];
            fn start() -> Self {
                Self::Star0([0..0])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Star0(ranges) | Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Star0(ranges), Some(b)) => {
                        // '*' matches.
                        Self::Star0Byte0(ranges.clone()).make_next_states(Some(b), n, next_states);
                        // '*' doesn't match.
                        Self::Byte1(ranges.clone()).make_next_states(Some(b), n, next_states)
                    }
                    (Self::Star0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        Self::Star0Matched(ranges_clone).make_next_states(None, n, next_states)
                    }
                    (Self::Star0Byte0(_), Some(_)) => {}
                    (Self::Star0Matched(ranges), None) => {
                        next_states.insert(Self::Star0(ranges.clone()));
                    }
                    (Self::Byte1(ranges), Some(b'a')) => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                    (Self::Byte1(_), Some(_)) => {}
                    (Self::Byte2(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Byte2(_), Some(_)) => {}
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    re.match_all(b"").unwrap();
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaX"));
    re.match_all(b"aaa").unwrap();
    assert_eq!(None, re.match_all(b"Xaaa"));
    assert_eq!(None, re.match_all(b"aXaa"));
    assert_eq!(None, re.match_all(b"aaXa"));
    assert_eq!(None, re.match_all(b"aaaX"));
    re.match_all(b"aaaa").unwrap();
    assert_eq!(None, re.match_all(b"Xaaaa"));
    assert_eq!(None, re.match_all(b"aXaaa"));
    assert_eq!(None, re.match_all(b"aaXaa"));
    assert_eq!(None, re.match_all(b"aaaXa"));
    assert_eq!(None, re.match_all(b"aaaaX"));
    re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        .unwrap();
    assert_eq!(
        None,
        re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaX")
    );
}

#[test]
fn seq_in_group() {
    let re = {
        #[doc = "br\"(ab)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Group0([core::ops::Range<u32>; 2usize]),
            Group0Byte0([core::ops::Range<u32>; 2usize]),
            Group0Byte1([core::ops::Range<u32>; 2usize]),
            Group0Matched([core::ops::Range<u32>; 2usize]),
            Accept([core::ops::Range<u32>; 2usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 2usize];
            fn start() -> Self {
                Self::Group0([0..0, u32::MAX..u32::MAX])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Group0(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize] = n..n;
                        Self::Group0Byte0(ranges_clone).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Group0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        next_states.insert(Self::Group0Byte1(ranges_clone));
                    }
                    (Self::Group0Byte0(_), Some(_)) => {}
                    (Self::Group0Byte1(ranges), Some(b'b')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        Self::Group0Matched(ranges_clone).make_next_states(None, n, next_states);
                    }
                    (Self::Group0Byte1(_), Some(_)) => {}
                    (Self::Group0Matched(ranges), None) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[1usize].end;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abab"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!(0..2, groups.group_range(1).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));
}

#[test]
fn alternates_in_group() {
    let re = {
        #[doc = "br\"(a|b)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Group0([core::ops::Range<u32>; 2usize]),
            Group0Alt0([core::ops::Range<u32>; 2usize]),
            Group0Alt0Byte0([core::ops::Range<u32>; 2usize]),
            Group0Alt0Byte1([core::ops::Range<u32>; 2usize]),
            Group0Alt0Matched([core::ops::Range<u32>; 2usize]),
            Group0Matched([core::ops::Range<u32>; 2usize]),
            Accept([core::ops::Range<u32>; 2usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 2usize];
            fn start() -> Self {
                Self::Group0([0..0, u32::MAX..u32::MAX])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Group0(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize] = n..n;
                        Self::Group0Alt0(ranges_clone).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Group0Alt0(ranges), Some(b)) => {
                        Self::Group0Alt0Byte0(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                        Self::Group0Alt0Byte1(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                    }
                    (Self::Group0Alt0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        Self::Group0Alt0Matched(ranges_clone).make_next_states(
                            None,
                            n,
                            next_states,
                        );
                    }
                    (Self::Group0Alt0Byte0(_), Some(_)) => {}
                    (Self::Group0Alt0Byte1(ranges), Some(b'b')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        Self::Group0Alt0Matched(ranges_clone).make_next_states(
                            None,
                            n,
                            next_states,
                        );
                    }
                    (Self::Group0Alt0Byte1(_), Some(_)) => {}
                    (Self::Group0Alt0Matched(ranges), None) => {
                        Self::Group0Matched(ranges.clone()).make_next_states(None, n, next_states)
                    }
                    (Self::Group0Matched(ranges), None) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[1usize].end;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));

    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));

    let groups = re.match_all(b"b").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!("b", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(1).unwrap()));
}

#[test]
fn optionals_in_groups() {
    let re = {
        #[doc = "br\"(a?)(ab)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Group0Start([core::ops::Range<u32>; 3usize]),
            Group0Matched([core::ops::Range<u32>; 3usize]),
            Group0Optional0([core::ops::Range<u32>; 3usize]),
            Group0Optional0Byte0([core::ops::Range<u32>; 3usize]),
            Group1Start([core::ops::Range<u32>; 3usize]),
            Group1Matched([core::ops::Range<u32>; 3usize]),
            Group1Byte0([core::ops::Range<u32>; 3usize]),
            Group1Byte1([core::ops::Range<u32>; 3usize]),
            Accept([core::ops::Range<u32>; 3usize]),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 3usize];
            fn start() -> Self {
                Self::Group0Start([0..0, u32::MAX..u32::MAX, u32::MAX..u32::MAX])
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    (Self::Group0Start(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize] = n..n;
                        Self::Group0Optional0(ranges_clone).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                    }
                    (Self::Group0Optional0(ranges), Some(b)) => {
                        // '?' matches
                        Self::Group0Optional0Byte0(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                        // '?' doesn't match
                        Self::Group0Matched(ranges.clone()).make_next_states(
                            Some(b),
                            n,
                            next_states,
                        );
                    }
                    (Self::Group0Optional0Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[1usize].end = n + 1;
                        Self::Group0Matched(ranges_clone).make_next_states(None, n, next_states);
                    }
                    (Self::Group0Optional0Byte0(_), Some(_)) => {}
                    (Self::Group0Matched(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[1usize].end;
                        Self::Group1Start(ranges_clone).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Group0Matched(ranges), None) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[1usize].end;
                        next_states.insert(Self::Group1Start(ranges_clone));
                    }
                    (Self::Group1Start(ranges), Some(b)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[2usize] = n..n;
                        Self::Group1Byte0(ranges_clone).make_next_states(Some(b), n, next_states);
                    }
                    (Self::Group1Byte0(ranges), Some(b'a')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[2usize].end = n + 1;
                        next_states.insert(Self::Group1Byte1(ranges_clone));
                    }
                    (Self::Group1Byte0(_), Some(_)) => {}
                    (Self::Group1Byte1(ranges), Some(b'b')) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[2usize].end = n + 1;
                        Self::Group1Matched(ranges_clone).make_next_states(None, n, next_states);
                    }
                    (Self::Group1Byte1(_), Some(_)) => {}
                    (Self::Group1Matched(ranges), None) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = ranges_clone[2usize].end;
                        next_states.insert(Self::Accept(ranges_clone));
                    }
                    (Self::Accept(_), _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aabX"));
    assert_eq!(None, re.match_all(b"Xaab"));
    assert_eq!(None, re.match_all(b"aaXb"));
    assert_eq!(None, re.match_all(b"aXab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b"aaba"));
    assert_eq!(None, re.match_all(b"aabaa"));
    assert_eq!(None, re.match_all(b"aabaab"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(2).unwrap()));

    let groups = re.match_all(b"aab").unwrap();
    assert_eq!(0..3, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!(1..3, groups.group_range(2).unwrap());
    assert_eq!("aab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(2).unwrap()));
}
