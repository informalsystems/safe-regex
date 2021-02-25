#![forbid(unsafe_code)]
use safe_proc_macro2::TokenStream;
use safe_quote::quote;
use safe_regex_compiler::impl_regex;

#[test]
fn syntax_errors() {
    fn to_s(s: TokenStream) -> String {
        format!("{}", s)
    }
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
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
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
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a" }).unwrap())
    );
}

#[test]
fn any_byte() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
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
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"." }).unwrap())
    );
}

#[test]
fn class_inclusive() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
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
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[abc2-4]" }).unwrap())
    );
}

#[test]
fn class_exclusive() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
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
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[^abc2-4]" }).unwrap())
    );
}

#[test]
fn seq() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(byte0), ib, ranges);
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
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte0(ranges) => Self::byte0(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"aab" }).unwrap())
    );
}

#[test]
fn alt() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(alt0), ib, ranges);
                Self::byte1(ranges, ib, next_states);
                Self::byte2(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a|b" }).unwrap())
    );
}

#[test]
fn group() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte1), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::byte1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn group_end0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("{} {:?} {:?}", stringify!(group_end0), ib, ranges);
                Self::accept(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a)" }).unwrap())
    );
}

#[test]
fn empty_group_in_seq() {
    let expected = quote! { {
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
                println!("{} {:?} {:?}", stringify!(byte2), ib, ranges);
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
                println!("{} {:?} {:?}", stringify!(empty1), ib, ranges);
                Self::group_end0(
                    ranges,
                    ib,
                    next_states, //
                );
            }
            fn group_start0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("{} {:?} {:?}", stringify!(group_start0), ib, ranges);
                Self::empty1(&ranges.clone().enter(0usize, ib.index()), ib, next_states);
            }
            fn group_end0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("{} {:?} {:?}", stringify!(group_end0), ib, ranges);
                Self::byte2(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                println!("accept {:?} {:?}", ib, ranges);
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
                println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    Self::Byte2(ranges) => Self::byte2(ranges, ib, next_states),
                    Self::Empty1(ranges) => Self::empty1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"()a" }).unwrap())
    );
}
