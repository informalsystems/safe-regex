#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::hash::Hash;
use safe_regex::internal::escape_ascii;

#[test]
fn byte() {
    let re = {
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a"
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\".\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(_) => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"."
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"[abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(b)
                        if b == 97u8 || b == 98u8 || b == 99u8 || (50u8..=52u8).contains(&b) =>
                    {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"[abc2-4]"
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"[^abc2-4]\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(b)
                        if b != 97u8 && b != 98u8 && b != 99u8 && !(50u8..=52u8).contains(&b) =>
                    {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"[^abc2-4]"
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"aab\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte2(Ranges_),
            Byte1(Ranges_),
            Byte0(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 98u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::byte2(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::byte1(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"aab"
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
fn alt() {
    let re = {
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 98u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn alt0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::alt0_b(ranges, ib, next_states)
            }
            fn alt0_b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::alt0_final(ranges, ib, next_states)
            }
            fn alt0_final(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(alt0), ib, ranges);
                Self::byte1(ranges, ib, next_states);
                Self::byte2(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a|b"
            }
            fn start(next_states: &mut States_) {
                Self::alt0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
    #[allow(dead_code)]
    let re = {
        use safe_regex::internal::InputByte;
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
            pub fn exit(mut self, group: usize, n: u32) -> Self {
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
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::group_end0(
                            &ranges.clone().skip_past(0usize, ib.index()),
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::byte1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn group_end0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end0), ib, ranges);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn expression() -> &'static [u8] {
                br"(a)"
            }
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn optional0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional0_b(ranges, ib, next_states)
            }
            fn optional0_b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional0_final(ranges, ib, next_states)
            }
            fn optional0_final(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(optional0), ib, ranges);
                Self::byte1(ranges, ib, next_states);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a?"
            }
            fn start(next_states: &mut States_) {
                Self::optional0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::byte2(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn optional0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(optional0), ib, ranges);
                Self::byte1(ranges, ib, next_states);
                Self::byte2(ranges, ib, next_states);
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a?a"
            }
            fn start(next_states: &mut States_) {
                Self::optional0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::optional1(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte0(ranges.clone()));
                    }
                }
            }
            fn optional1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(optional1), ib, ranges);
                Self::byte2(ranges, ib, next_states);
                Self::accept(ranges, ib, next_states);
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(97u8) => Self::accept(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"aa?"
            }
            fn start(next_states: &mut States_) {
                Self::byte0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
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
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::star0(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn star0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::star0_b(ranges, ib, next_states)
            }
            fn star0_b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::star0_final(ranges, ib, next_states)
            }
            fn star0_final(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(star0), ib, ranges);
                Self::byte1(ranges, ib, next_states);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a*"
            }
            fn start(next_states: &mut States_) {
                Self::star0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
fn empty_group_in_seq() {
    #[allow(dead_code)]
    let re = {
        use safe_regex::internal::InputByte;
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
            pub fn exit(mut self, group: usize, n: u32) -> Self {
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
        #[doc = "br\"()a\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte2(Ranges_),
            Empty1(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(b) if b == 97u8 => {
                        Self::accept(
                            ranges,
                            ib.consume(),
                            next_states, //
                        ) //
                    }
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn empty1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(empty1), ib, ranges);
                Self::group_end0(
                    ranges,
                    ib,
                    next_states, //
                );
            }
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::empty1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn group_end0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end0), ib, ranges);
                Self::byte2(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn expression() -> &'static [u8] {
                br"()a"
            }
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Empty1(ranges) => Self::empty1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
    assert_eq!(0..0, groups.group_range(0).unwrap());
    assert_eq!("", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn seq_in_group() {
    #[allow(dead_code)]
    let re = {
        use safe_regex::internal::InputByte;
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
            pub fn exit(mut self, group: usize, n: u32) -> Self {
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
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::byte1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
                match ib.byte() {
                    Some(97u8) => Self::byte2(
                        &ranges.clone().skip_past(0usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(98u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end3), ib, ranges);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn expression() -> &'static [u8] {
                b"(ab)"
            }
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
fn alt_in_group() {
    #[allow(dead_code)]
    let re = {
        use safe_regex::internal::InputByte;
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
            pub fn exit(mut self, group: usize, n: u32) -> Self {
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
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::alt1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn alt1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(alt1), ib, ranges);
                Self::byte2(ranges, ib, next_states);
                Self::byte3(ranges, ib, next_states);
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(97u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn byte3(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte3), ib, ranges);
                match ib.byte() {
                    Some(98u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte3(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end3), ib, ranges);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 1usize];
            fn expression() -> &'static [u8] {
                b"(a|b)"
            }
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Byte3(ranges) => Self::byte3(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
    #[allow(dead_code)]
    let re = {
        use safe_regex::internal::InputByte;
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
            pub fn exit(mut self, group: usize, n: u32) -> Self {
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
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::optional1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn optional1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(optional1), ib, ranges);
                Self::byte2(ranges, ib, next_states);
                Self::group_end3(ranges, ib, next_states);
            }
            fn byte2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
                match ib.byte() {
                    Some(97u8) => Self::group_end3(
                        &ranges.clone().skip_past(0usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte2(ranges.clone()));
                    }
                }
            }
            fn group_end3(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end3), ib, ranges);
                Self::group_start4(ranges, ib, next_states);
            }
            fn group_start4(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_start4), ib, ranges);
                Self::byte5(&ranges.clone().enter(1usize, ib.index()), ib, next_states);
            }
            fn byte5(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte5), ib, ranges);
                match ib.byte() {
                    Some(97u8) => Self::byte6(
                        &ranges.clone().skip_past(1usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte5(ranges.clone()));
                    }
                }
            }
            fn byte6(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(byte6), ib, ranges);
                match ib.byte() {
                    Some(98u8) => Self::group_end7(
                        &ranges.clone().skip_past(1usize, ib.index()),
                        ib.consume(),
                        next_states,
                    ),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte6(ranges.clone()));
                    }
                }
            }
            fn group_end7(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("{} {:?} {:?}", stringify!(group_end7), ib, ranges);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 2usize];
            fn expression() -> &'static [u8] {
                b"(a?)(ab)"
            }
            fn start(next_states: &mut States_) {
                Self::group_start0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Byte5(ranges) => Self::byte5(ranges, ib, next_states),
                    Self::Byte6(ranges) => Self::byte6(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
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
