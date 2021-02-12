#![forbid(unsafe_code)]
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use safe_regex_compiler::impl_regex;

fn stream_to_s(s: TokenStream) -> String {
    s.into_iter().map(|tree| format!("{} ", tree)).collect()
}

#[test]
fn byte() {
    let input = quote! {
        regex!(enum Re = br"a")
    };
    let expected = quote! {
        /// "a"
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum Re {
            Byte0,
            Accept,
        }
        impl safe_regex::Regex for Re {
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
                match (self, opt_b) {
                    (Self::Byte0, Some(b'a')) => {
                        next_states.insert(Self::Accept);
                    }
                    (Self::Byte0, Some(_)) => {}
                    (Self::Accept, _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }
    };
    assert_eq!(
        stream_to_s(expected),
        stream_to_s(impl_regex(input).unwrap())
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
