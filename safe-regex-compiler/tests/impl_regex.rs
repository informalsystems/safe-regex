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
            Byte0,
            Accept,
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 0usize];
            fn start() -> Self {
                Self::Byte0
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept => Some([]),
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
                println!(
                    "make_next_states {} {} {:?}",
                    opt_b.map_or(String::from("None"), |b| format!(
                        "Some({})",
                        safe_regex::internal::escape_ascii(&[b])
                    )),
                    n,
                    self,
                );
                match (self, opt_b) {
                    (Self::Byte0, Some(97u8)) => {
                        next_states.insert(Self::Accept);
                    }
                    (Self::Byte0, Some(_)) => {}
                    (Self::Accept, _) => {}
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
            AnyByte0,
            Accept,
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = [core::ops::Range<u32>; 0usize];
            fn start() -> Self {
                Self::AnyByte0
            }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept => Some([]),
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
                println!(
                    "make_next_states {} {} {:?}",
                    opt_b.map_or(String::from("None"), |b| format!(
                        "Some({})",
                        safe_regex::internal::escape_ascii(&[b])
                    )),
                    n,
                    self,
                );
                match (self, opt_b) {
                    (Self::AnyByte0, Some(_)) => {
                        next_states.insert(Self::Accept);
                    }
                    (Self::Accept, _) => {}
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

// Literal { kind: Str, symbol: "abc€\\x20\\u{00A2}", suffix: None, span: #0 bytes(61..81) }
// regex!("abc€\x20\u{00A2}");

// Literal { kind: StrRaw(0), symbol: "abc€\\x20\\u{00A2}", suffix: None, span: #0 bytes(61..82)
// regex!(r"abc€\x20\u{00A2}");

// Literal { kind: ByteStr, symbol: "abc\\x20", suffix: None, span: #0 bytes(338..348) }
// regex!(b"abc\x20");

// Literal { kind: ByteStrRaw(0), symbol: "abc\\x20", suffix: None, span: #0 bytes(461..472) }
// regex!(br"abc\x20");

// regex!(b"abc\x20\n").match_all("abc").unwrap();
// regex!(br#"\n\r\t\\\0\'\""#).match_all("abc").unwrap();
// regex!(br"\n\?[abc]\x01");
//regex!(br"abc\x20");
