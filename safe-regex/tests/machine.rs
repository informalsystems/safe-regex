#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::hash::Hash;
use safe_regex::internal::escape_ascii;

#[test]
fn byte() {
    let re = {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\".\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            AnyByte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn any_byte0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("any_byte0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    None => {
                        next_states.insert(Self::AnyByte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::any_byte0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::AnyByte0(ranges) => Self::any_byte0(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"[abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Class0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn class0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("class0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(b)
                        if b == 97u8 || b == 98u8 || b == 99u8 || (50u8..=52u8).contains(&b) =>
                    {
                        Self::accept(
                            &ranges.clone().skip_past(0usize, n),
                            None,
                            n + 1,
                            next_states,
                        )
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Class0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::class0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Class0(ranges) => Self::class0(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"[^abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Class0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn class0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("class0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(b)
                        if b != 97u8 && b != 98u8 && b != 99u8 && !(50u8..=52u8).contains(&b) =>
                    {
                        Self::accept(
                            &ranges.clone().skip_past(0usize, n),
                            None,
                            n + 1,
                            next_states,
                        )
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Class0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::class0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Class0(ranges) => Self::class0(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"aab\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Byte1(Ranges_),
            Byte2(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::byte1(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::byte2(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(98u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, Some(b), n, next_states),
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a|b\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Byte2(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn alt0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("alt0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(ranges, opt_b, n, next_states);
                Self::byte2(ranges, opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(98u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::alt0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_([core::ops::Range<u32>; 1usize]);
        impl Ranges_ {
            pub fn new() -> Self {
                Self([u32::MAX..u32::MAX])
            }
            pub fn enter(mut self, group: usize, n: u32) -> Self {
                self.0[group].start = n;
                self.0[group].end = n;
                self
            }
            pub fn skip_past(mut self, group: usize, n: u32) -> Self {
                self.0[group].end = n + 1;
                self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 1usize] {
                &self.0
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"(a)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn group_start0(
                ranges: &Ranges_,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut States_,
            ) {
                println!("group_start0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(&ranges.clone().enter(0usize, n), opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::group_end2(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn group_end2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("group_end2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn optional() {
    let re = {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a?\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn optional0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("optional0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(ranges, opt_b, n, next_states);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::optional0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a?a\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Byte2(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn optional0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("optional0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(ranges, opt_b, n, next_states);
                Self::byte2(ranges, opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::byte2(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::optional0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"aa?\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Byte2(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::optional1(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn optional1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("optional1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte2(ranges, opt_b, n, next_states);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::accept(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, Some(b), n, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                self
            }
            pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                []
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a*\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn star0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("star0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(ranges, opt_b, n, next_states);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::star0(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn start(next_states: &mut States_) {
                Self::star0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_([core::ops::Range<u32>; 1usize]);
        impl Ranges_ {
            pub fn new() -> Self {
                Self([u32::MAX..u32::MAX])
            }
            pub fn enter(mut self, group: usize, n: u32) -> Self {
                self.0[group].start = n;
                self.0[group].end = n;
                self
            }
            pub fn skip_past(mut self, group: usize, n: u32) -> Self {
                self.0[group].end = n + 1;
                self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 1usize] {
                &self.0
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"(ab)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte1(Ranges_),
            Byte2(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn group_start0(
                ranges: &Ranges_,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut States_,
            ) {
                println!("group_start0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte1(&ranges.clone().enter(0usize, n), opt_b, n, next_states);
            }
            fn byte1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::byte2(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(98u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("group_end3 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, Some(b), n, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn alternates_in_group() {
    let re = {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_([core::ops::Range<u32>; 1usize]);
        impl Ranges_ {
            pub fn new() -> Self {
                Self([u32::MAX..u32::MAX])
            }
            pub fn enter(mut self, group: usize, n: u32) -> Self {
                self.0[group].start = n;
                self.0[group].end = n;
                self
            }
            pub fn skip_past(mut self, group: usize, n: u32) -> Self {
                self.0[group].end = n + 1;
                self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 1usize] {
                &self.0
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"(a|b)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte2(Ranges_),
            Byte3(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn group_start0(
                ranges: &Ranges_,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut States_,
            ) {
                println!("group_start0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::alt1(&ranges.clone().enter(0usize, n), opt_b, n, next_states);
            }
            fn alt1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("alt1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte2(ranges, opt_b, n, next_states);
                Self::byte3(ranges, opt_b, n, next_states);
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn byte3(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte3 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(98u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte3(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("group_end3 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Byte3(ranges) => Self::byte3(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));

    let groups = re.match_all(b"b").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!("b", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn optionals_in_groups() {
    let re = {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_([core::ops::Range<u32>; 2usize]);
        impl Ranges_ {
            pub fn new() -> Self {
                Self([u32::MAX..u32::MAX, u32::MAX..u32::MAX])
            }
            pub fn enter(mut self, group: usize, n: u32) -> Self {
                self.0[group].start = n;
                self.0[group].end = n;
                self
            }
            pub fn skip_past(mut self, group: usize, n: u32) -> Self {
                self.0[group].end = n + 1;
                self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 2usize] {
                &self.0
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"(a?)(ab)\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte2(Ranges_),
            Byte5(Ranges_),
            Byte6(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn group_start0(
                ranges: &Ranges_,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut States_,
            ) {
                println!("group_start0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::optional1(&ranges.clone().enter(0usize, n), opt_b, n, next_states);
            }
            fn optional1(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("optional1 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte2(ranges, opt_b, n, next_states);
                Self::group_end3(ranges, opt_b, n, next_states);
            }
            fn byte2(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte2 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("group_end3 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::group_start4(ranges, opt_b, n, next_states);
            }
            fn group_start4(
                ranges: &Ranges_,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut States_,
            ) {
                println!("group_start4 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::byte5(&ranges.clone().enter(1usize, n), opt_b, n, next_states);
            }
            fn byte5(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte5 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(97u8) => Self::byte6(
                        &ranges.clone().skip_past(1usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte5(ranges.clone()));
                    }
                }
            }
            fn byte6(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("byte6 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(98u8) => Self::group_end7(
                        &ranges.clone().skip_past(1usize, n),
                        None,
                        n + 1,
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte6(ranges.clone()));
                    }
                }
            }
            fn group_end7(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("group_end7 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                Self::accept(ranges, opt_b, n, next_states);
            }
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 2usize];
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, Some(b), n, next_states),
                    Self::Byte5(ranges) => Self::byte5(ranges, Some(b), n, next_states),
                    Self::Byte6(ranges) => Self::byte6(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
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
    assert_eq!("", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));

    let groups = re.match_all(b"aab").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(1..3, groups.group_range(1).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));
}
