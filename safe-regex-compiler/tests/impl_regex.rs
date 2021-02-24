#![forbid(unsafe_code)]
use safe_proc_macro2::TokenStream;
use safe_quote::quote;
use safe_regex_compiler::impl_regex;

fn to_s(s: TokenStream) -> String {
    format!("{}", s)
}

#[test]
fn syntax_errors() {
    let err = Err("expected a raw byte string, like br\"abc\"".to_string());
    assert_eq!(err, impl_regex(quote! {"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {r"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {b"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {'a}).map(to_s));
    assert_eq!(err, impl_regex(quote! {b'b'}).map(to_s));
    assert_eq!(err, impl_regex(quote! {1}).map(to_s));
    assert_eq!(err, impl_regex(quote! {(br"a")}).map(to_s));
    assert_eq!(err, impl_regex(quote! {br"a";}).map(to_s));
    assert_eq!(err, impl_regex(quote! {br"a" br"b"}).map(to_s));
}

// TODO(mleonhard) Test macro with comment.

#[test]
fn byte() {
    let expected = quote! {
    {
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
                    Some(97u8) => Self::accept(&ranges.clone(), None, n + 1, next_states,),
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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a" }).unwrap())
    );
}

#[test]
fn any_byte() {
    let expected = quote! {
    {
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
            fn anybyte0(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("anybyte0 opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => Self::accept(&ranges.clone(), None, n + 1, next_states,),
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
                Self::anybyte0(&Ranges_::new(), None, 0, next_states);
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
                    Self::AnyByte0(ranges) => Self::anybyte0(ranges, Some(b), n, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"." }).unwrap())
    );
}

#[test]
fn class_inclusive() {
    let expected = quote! {
    {
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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[abc2-4]" }).unwrap())
    );
}

#[test]
fn class_exclusive() {
    let expected = quote! {
    {
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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[^abc2-4]" }).unwrap())
    );
}

#[test]
fn seq() {
    let expected = quote! {
    {
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
                    (Self::Byte0(ranges), Some(97u8)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Byte1(ranges_clone));
                    }
                    (Self::Byte0(_), Some(_)) => {}
                    (Self::Byte1(ranges), Some(97u8)) => {
                        let mut ranges_clone = ranges.clone();
                        ranges_clone[0usize].end = n + 1;
                        next_states.insert(Self::Byte2(ranges_clone));
                    }
                    (Self::Byte1(_), Some(_)) => {}
                    (Self::Byte2(ranges), Some(98u8)) => {
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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"aab" }).unwrap())
    );
}

#[test]
fn group() {
    let expected = quote! {
    {
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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a)" }).unwrap())
    );
}
