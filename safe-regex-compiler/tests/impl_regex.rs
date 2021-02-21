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
    }
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"." }).unwrap())
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
