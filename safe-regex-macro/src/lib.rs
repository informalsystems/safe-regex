//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-macro.svg)](https://crates.io/crates/safe-regex-macro)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! This crate provides the `regex!` macro used by the
//! [`safe_regex`](https://crates.io/crates/safe-regex) crate.
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - Implement `regex!` macro.
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote_spanned;

fn escape_ascii(input: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for byte in input.as_ref() {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum Node {
    FinalNode(Final),
    NonFinalNode(NonFinal),
}
impl Node {
    pub fn is_final_node(&self) -> bool {
        match self {
            Node::FinalNode(_) => true,
            Node::NonFinalNode(_) => false,
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum NonFinal {
    Escape,
    HexEscape0,
    HexEscape1(u8),
    OpenGroup,
    OpenOr(Vec<Final>),
    RepeatMin(Box<Final>, String),
    RepeatMax(Box<Final>, String, String),
}
impl NonFinal {
    pub fn reason(&self) -> String {
        match self {
            NonFinal::Escape => "incomplete escape sequence `\\`".to_string(),
            NonFinal::HexEscape0 => "incomplete escape sequence `\\x`".to_string(),
            NonFinal::HexEscape1(d) => {
                format!("incomplete escape sequence `\\x{}`", escape_ascii([*d]))
            }
            NonFinal::OpenGroup => "group missing closing `)`".to_string(),
            NonFinal::OpenOr(_) => "missing item after bar `|`".to_string(),
            NonFinal::RepeatMin(_, s) => format!("incomplete repeat mark `{{{}`", s),
            NonFinal::RepeatMax(_, min, max) => {
                format!("incomplete repeat mark `{{{},{}`", min, max)
            }
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum Final {
    Byte(u8),
    AnyByte,
    Seq(Vec<Final>),
    Group(Box<Final>),
    Or(Vec<Final>),
    Repeat(Box<Final>, usize, Option<usize>),
}

fn invalid_escape(bytes: impl AsRef<[u8]>) -> String {
    format!("invalid escape sequence `\\{}`", escape_ascii(bytes))
}

fn parse(mut data: &[u8]) -> Result<Final, String> {
    use Final::{AnyByte, Byte, Group, Or, Repeat, Seq};
    use Node::{FinalNode, NonFinalNode};
    use NonFinal::{Escape, HexEscape0, HexEscape1, OpenGroup, OpenOr, RepeatMax, RepeatMin};
    if data.is_empty() {
        return Ok(Seq(Vec::new()));
    }
    let mut iter = data.iter().copied();
    let mut stack: Vec<Node> = Vec::new();
    while let Some(b) = iter.next() {
        let top = stack.pop();
        match (stack.last_mut(), top, b) {
            (_, Some(NonFinalNode(Escape)), b'\\') => stack.push(FinalNode(Byte(b'\\'))),
            (_, opt_top, b'\\') => {
                opt_top.map(|top| stack.push(top));
                stack.push(NonFinalNode(Escape))
            }
            (_, Some(NonFinalNode(Escape)), b'n') => stack.push(FinalNode(Byte(b'\n'))),
            (_, Some(NonFinalNode(Escape)), b'r') => stack.push(FinalNode(Byte(b'\r'))),
            (_, Some(NonFinalNode(Escape)), b't') => stack.push(FinalNode(Byte(b'\t'))),
            (_, Some(NonFinalNode(Escape)), b'0') => stack.push(FinalNode(Byte(0))),
            (_, Some(NonFinalNode(Escape)), b'\'') => stack.push(FinalNode(Byte(b'\''))),
            (_, Some(NonFinalNode(Escape)), b'"') => stack.push(FinalNode(Byte(b'"'))),
            (_, Some(NonFinalNode(Escape)), b'?') => stack.push(FinalNode(Byte(b'?'))),
            (_, Some(NonFinalNode(Escape)), b'+') => stack.push(FinalNode(Byte(b'+'))),
            (_, Some(NonFinalNode(Escape)), b'.') => stack.push(FinalNode(Byte(b'.'))),
            (_, Some(NonFinalNode(Escape)), b'*') => stack.push(FinalNode(Byte(b'*'))),
            (_, Some(NonFinalNode(Escape)), b'^') => stack.push(FinalNode(Byte(b'^'))),
            (_, Some(NonFinalNode(Escape)), b'$') => stack.push(FinalNode(Byte(b'$'))),
            (_, Some(NonFinalNode(Escape)), b'|') => stack.push(FinalNode(Byte(b'|'))),
            (_, Some(NonFinalNode(Escape)), b'(') => stack.push(FinalNode(Byte(b'('))),
            (_, Some(NonFinalNode(Escape)), b')') => stack.push(FinalNode(Byte(b')'))),
            (_, Some(NonFinalNode(Escape)), b'{') => stack.push(FinalNode(Byte(b'{'))),
            (_, Some(NonFinalNode(Escape)), b'}') => stack.push(FinalNode(Byte(b'}'))),
            (_, Some(NonFinalNode(Escape)), b'[') => stack.push(FinalNode(Byte(b'['))),
            (_, Some(NonFinalNode(Escape)), b']') => stack.push(FinalNode(Byte(b']'))),
            (_, Some(NonFinalNode(Escape)), b'x') => stack.push(NonFinalNode(HexEscape0)),
            (_, Some(NonFinalNode(Escape)), d) => return Err(invalid_escape([d])),
            (_, Some(NonFinalNode(HexEscape0)), d) => stack.push(NonFinalNode(HexEscape1(d))),
            (_, Some(NonFinalNode(HexEscape1(d1))), d0)
                if d1.is_ascii_hexdigit() && d0.is_ascii_hexdigit() =>
            {
                let string = String::from_utf8(vec![d1, d0]).unwrap();
                let byte = u8::from_str_radix(&string, 16).unwrap();
                stack.push(FinalNode(Byte(byte)))
            }
            (_, Some(NonFinalNode(HexEscape1(d1))), d0) => {
                return Err(invalid_escape([b'x', d1, d0]))
            }
            (_, Some(FinalNode(item)), b'?') => {
                stack.push(FinalNode(Repeat(Box::new(item), 0, Some(1))))
            }
            (_, Some(FinalNode(item)), b'+') => {
                stack.push(FinalNode(Repeat(Box::new(item), 1, None)))
            }
            (_, Some(FinalNode(item)), b'*') => {
                stack.push(FinalNode(Repeat(Box::new(item), 0, None)))
            }
            (_, opt_top, b'.') => {
                opt_top.map(|top| stack.push(top));
                stack.push(FinalNode(AnyByte))
            }
            (_, Some(FinalNode(Or(items))), b'|') => stack.push(NonFinalNode(OpenOr(items))),
            (_, Some(FinalNode(item)), b'|') => stack.push(NonFinalNode(OpenOr(vec![item]))),
            (_, None, b'|') => return Err("missing item before bar `|`".to_string()),
            (_, opt_top, b'(') => {
                opt_top.map(|top| stack.push(top));
                stack.push(NonFinalNode(OpenGroup));
            }
            (Some(NonFinalNode(OpenGroup)), Some(NonFinalNode(non_final)), b')') => {
                return Err(non_final.reason())
            }
            (Some(NonFinalNode(OpenGroup)), Some(FinalNode(item)), b')') => {
                stack.pop();
                stack.push(FinalNode(Group(Box::new(item))));
            }
            (_, opt_top, byte) => {
                opt_top.map(|top| stack.push(top));
                stack.push(FinalNode(Byte(byte)));
            }
            _ => unimplemented!(),
            // ?+.*^$|(){}[]
        };
        while stack.len() >= 2 && stack.last().unwrap().is_final_node() {
            let top = stack.pop().unwrap();
            match (stack.last_mut(), top) {
                (_, NonFinalNode(non_final)) => stack.push(NonFinalNode(non_final)),
                (Some(NonFinalNode(OpenOr(_))), FinalNode(item)) => {
                    if let Some(NonFinalNode(OpenOr(mut items))) = stack.pop() {
                        items.push(item);
                        stack.push(FinalNode(Or(items)))
                    } else {
                        unreachable!()
                    }
                }
                (Some(FinalNode(Or(items))), FinalNode(item)) => match items.last_mut().unwrap() {
                    Seq(seq_items) => seq_items.push(item),
                    _ => {
                        let prev_item = items.pop().unwrap();
                        items.push(Seq(vec![prev_item, item]))
                    }
                },
                (Some(NonFinalNode(OpenGroup)), FinalNode(item)) => {
                    stack.push(FinalNode(item));
                    break;
                }
                (Some(FinalNode(Seq(ref mut items))), FinalNode(item)) => items.push(item),
                (Some(FinalNode(_)), FinalNode(top)) => {
                    if let Some(FinalNode(prev)) = stack.pop() {
                        stack.push(FinalNode(Seq(vec![prev, top])))
                    } else {
                        unreachable!()
                    }
                }
                (prev, top) => panic!("{:?} {:?}", prev, top),
            }
        }
    }
    println!("stack {:?}", stack);
    for node in stack.iter().rev() {
        if let NonFinalNode(non_final) = node {
            return Err(non_final.reason());
        }
    }
    assert_eq!(1, stack.len());
    if let FinalNode(node) = stack.pop().unwrap() {
        Ok(node)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
#[test]
fn test_parse() {
    use Final::{AnyByte, Byte, Group, Or, Repeat, Seq};
    assert_eq!(Ok(Seq(Vec::new())), parse(br""));
    assert_eq!(Ok(Byte(b'a')), parse(br"a"));
    assert_eq!(
        Ok(Seq(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')])),
        parse(br"abc")
    );
    assert_eq!(Ok(AnyByte), parse(br"."));
}

#[cfg(test)]
#[test]
fn test_parse_escapes() {
    use Final::Byte;
    assert_eq!(
        Err(r"incomplete escape sequence `\`".to_string()),
        parse(br"\")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\e`".to_string()),
        parse(br"\e")
    );
    // Rust byte escapes
    // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
    assert_eq!(Ok(Byte(b'\n')), parse(br"\n"));
    assert_eq!(Ok(Byte(b'\r')), parse(br"\r"));
    assert_eq!(Ok(Byte(b'\t')), parse(br"\t"));
    assert_eq!(Ok(Byte(b'\\')), parse(br"\\"));
    assert_eq!(Ok(Byte(0)), parse(br"\0"));
    assert_eq!(
        Err(r"incomplete escape sequence `\x`".to_string()),
        parse(br"\x")
    );
    assert_eq!(
        Err(r"incomplete escape sequence `\x0`".to_string()),
        parse(br"\x0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\xg0`".to_string()),
        parse(br"\xg0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\x0g`".to_string()),
        parse(br"\x0g")
    );
    assert_eq!(Ok(Byte(0)), parse(br"\x00"));
    assert_eq!(Ok(Byte(0x12)), parse(br"\x12"));
    assert_eq!(Ok(Byte(0x34)), parse(br"\x34"));
    assert_eq!(Ok(Byte(0x56)), parse(br"\x56"));
    assert_eq!(Ok(Byte(0x78)), parse(br"\x78"));
    assert_eq!(Ok(Byte(0x90)), parse(br"\x90"));
    assert_eq!(Ok(Byte(0xAB)), parse(br"\xab"));
    assert_eq!(Ok(Byte(0xAB)), parse(br"\xAB"));
    assert_eq!(Ok(Byte(0xCD)), parse(br"\xcd"));
    assert_eq!(Ok(Byte(0xCD)), parse(br"\xCD"));
    assert_eq!(Ok(Byte(0xEF)), parse(br"\xef"));
    assert_eq!(Ok(Byte(0xEF)), parse(br"\xEF"));
    assert_eq!(Ok(Byte(0xFF)), parse(br"\xFF"));
    // Rust quote escapes
    // https://doc.rust-lang.org/reference/tokens.html#quote-escapes
    assert_eq!(Ok(Byte(b'\'')), parse(br"\'"));
    assert_eq!(Ok(Byte(b'"')), parse(br#"\""#));
    // Regex escapes
    assert_eq!(Ok(Byte(b'?')), parse(br"\?"));
    assert_eq!(Ok(Byte(b'+')), parse(br"\+"));
    assert_eq!(Ok(Byte(b'.')), parse(br"\."));
    assert_eq!(Ok(Byte(b'*')), parse(br"\*"));
    assert_eq!(Ok(Byte(b'^')), parse(br"\^"));
    assert_eq!(Ok(Byte(b'$')), parse(br"\$"));
    assert_eq!(Ok(Byte(b'|')), parse(br"\|"));
    assert_eq!(Ok(Byte(b'(')), parse(br"\("));
    assert_eq!(Ok(Byte(b')')), parse(br"\)"));
    assert_eq!(Ok(Byte(b'{')), parse(br"\{"));
    assert_eq!(Ok(Byte(b'}')), parse(br"\}"));
    assert_eq!(Ok(Byte(b'[')), parse(br"\["));
    assert_eq!(Ok(Byte(b']')), parse(br"\]"));
}

#[cfg(test)]
#[test]
fn test_parse_or() {
    use Final::{AnyByte, Byte, Group, Or, Repeat, Seq};
    assert_eq!(
        Err(r"missing item before bar `|`".to_string()),
        parse(br"|")
    );
    assert_eq!(
        Err(r"missing item after bar `|`".to_string()),
        parse(br"a|")
    );
    assert_eq!(
        Err(r"missing item after bar `|`".to_string()),
        parse(br"(a|)")
    );
    assert_eq!(
        Err(r"missing item after bar `|`".to_string()),
        parse(br"(a|bc|)d")
    );
    assert_eq!(Ok(Or(vec![Byte(b'a'), Byte(b'b')])), parse(br"a|b"));
    assert_eq!(
        Ok(Or(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')])),
        parse(br"a|b|c")
    );
    assert_eq!(
        Ok(Or(vec![
            Seq(vec![Byte(b'a'), Byte(b'b')]),
            Seq(vec![Byte(b'c'), Byte(b'd'), Byte(b'e')]),
            Seq(vec![Byte(b'f'), Byte(b'g')])
        ])),
        parse(br"ab|cde|fg")
    );
}

// #[cfg(test)]
// #[test]
// fn test_parse_or() {
//     use Final::{AnyByte, Byte, Group, Or, Repeat, Seq};
//     use Node::Seq;
//     assert_eq!(
//         Ok(Seq(vec![Literal(b'a'), Literal(b'b')])),
//         parse(vec![Byte(b'a'), Byte(b'b')])
//     );
//     use Final::{AnyByte, Byte, Group, Or, Repeat, Seq};
//     // TODO(mleonhard) Test precedence.
//     // Regex tokens
//     assert_eq!(Ok(Repeat(Box::new(Byte(b'a')), 0, Some(1))), parse(br"a?"));
//     assert_eq!(Ok(Repeat(Box::new(Byte(b'a')), 1, None)), parse(br"a+"));
//     assert_eq!(Ok(Repeat(Box::new(Byte(b'a')), 0, None)), parse(br"a*"));
//     assert_eq!(Ok(AnyByte), parse(br"."));
//     assert_eq!(
//         Ok(Group(Box::new(Seq(vec![Byte(b'a'), Byte(b'b')])))),
//         parse(br"(ab)")
//     );
//
// }
//
// #[cfg(test)]
// #[test]
// fn test_parse_class() {
//     use Node::Class;
//     use Token::{
//         Bar, Byte, Caret, CloseCurly, CloseRound, CloseSquare, Dollar, Dot, OpenCurly, OpenRound,
//         OpenSquare, Plus, QMark, Star,
//     };
//     assert_eq!(
//         Ok(Class(vec![b'a'])),
//         parse(vec![OpenSquare, Byte(b'a'), CloseSquare])
//     );
//     assert_eq!(
//         Ok(Class(vec![b'a', b'b', b'c'])),
//         parse(vec![
//             OpenSquare,
//             Byte(b'a'),
//             Byte(b'b'),
//             Byte(b'c'),
//             CloseSquare
//         ])
//     );
//     // ?+.*^$|(){}[]
//     assert_eq!(
//         Ok(Class(vec![
//             b'?', b'+', b'.', b'*', b'^', b'$', b'|', b'(', b')', b'{', b'}', b'[', b']'
//         ])),
//         parse(vec![
//             OpenSquare,
//             QMark,
//             Plus,
//             Dot,
//             Star,
//             Caret,
//             Dollar,
//             Bar,
//             OpenRound,
//             CloseRound,
//             OpenCurly,
//             CloseCurly,
//             OpenSquare,
//             Byte(b']'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Err("character class ends with `-`".to_string()),
//         parse(vec![OpenSquare, Byte(b'a'), Byte(b'-'), CloseSquare])
//     );
//     assert_eq!(
//         Ok(Class(vec![b'a', b'b', b'c'])),
//         parse(vec![
//             OpenSquare,
//             Byte(b'a'),
//             Byte(b'-'),
//             Byte(b'c'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Ok(Class(vec![b'a', b'b', b'c', b'g', b'g', b'h'])),
//         parse(vec![
//             OpenSquare,
//             Byte(b'a'),
//             Byte(b'-'),
//             Byte(b'c'),
//             Byte(b'g'),
//             Byte(b'-'),
//             Byte(b'h'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Ok(Class(vec![b'-', b'a', b'b'])),
//         parse(vec![
//             OpenSquare,
//             Byte(b'-'),
//             Byte(b'a'),
//             Byte(b'b'),
//             CloseSquare
//         ])
//     );
// }
//
// #[cfg(test)]
// #[test]
// fn test_parse_negative_class() {
//     use Node::NegativeClass;
//     use Token::{
//         Bar, Byte, Caret, CloseCurly, CloseRound, CloseSquare, Dollar, Dot, OpenCurly, OpenRound,
//         OpenSquare, Plus, QMark, Star,
//     };
//     assert_eq!(
//         Ok(NegativeClass(vec![b'a'])),
//         parse(vec![OpenSquare, Caret, Byte(b'a'), CloseSquare])
//     );
//     assert_eq!(
//         Ok(NegativeClass(vec![b'a', b'b', b'c'])),
//         parse(vec![
//             OpenSquare,
//             Caret,
//             Byte(b'a'),
//             Byte(b'b'),
//             Byte(b'c'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Ok(NegativeClass(vec![
//             b'?', b'+', b'.', b'*', b'^', b'$', b'|', b'(', b')', b'{', b'}', b'[', b']'
//         ])),
//         parse(vec![
//             OpenSquare,
//             Caret,
//             QMark,
//             Plus,
//             Dot,
//             Star,
//             Caret,
//             Dollar,
//             Bar,
//             OpenRound,
//             CloseRound,
//             OpenCurly,
//             CloseCurly,
//             OpenSquare,
//             Byte(b']'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Err("character class ends with `-`".to_string()),
//         parse(vec![OpenSquare, Caret, Byte(b'a'), Byte(b'-'), CloseSquare])
//     );
//     assert_eq!(
//         Ok(NegativeClass(vec![b'a', b'b', b'c'])),
//         parse(vec![
//             OpenSquare,
//             Caret,
//             Byte(b'a'),
//             Byte(b'-'),
//             Byte(b'c'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Ok(NegativeClass(vec![b'a', b'b', b'c', b'g', b'g', b'h'])),
//         parse(vec![
//             OpenSquare,
//             Caret,
//             Byte(b'a'),
//             Byte(b'-'),
//             Byte(b'c'),
//             Byte(b'g'),
//             Byte(b'-'),
//             Byte(b'h'),
//             CloseSquare
//         ])
//     );
//     assert_eq!(
//         Ok(NegativeClass(vec![b'-', b'a', b'b'])),
//         parse(vec![
//             OpenSquare,
//             Caret,
//             Byte(b'-'),
//             Byte(b'a'),
//             Byte(b'b'),
//             CloseSquare
//         ])
//     );
// }
//
// #[cfg(test)]
// #[test]
// fn test_parse_group() {
//     use Node::{AnyByte, Empty, Group, Seq};
//     use Token::{CloseRound, Dot, OpenRound};
//     assert_eq!(
//         Err("group is missing closing `)` symbol".to_string()),
//         parse(vec![OpenRound, Dot])
//     );
//     assert_eq!(
//         Ok(Group(Box::new(Empty))),
//         parse(vec![OpenRound, CloseRound])
//     );
//     assert_eq!(
//         Ok(Group(Box::new(AnyByte))),
//         parse(vec![OpenRound, Dot, CloseRound])
//     );
//     assert_eq!(
//         Ok(Group(Box::new(Group(Box::new(AnyByte))))),
//         parse(vec![OpenRound, OpenRound, Dot, CloseRound, CloseRound])
//     );
//     assert_eq!(
//         Ok(Group(Box::new(Seq(vec![
//             AnyByte,
//             Group(Box::new(AnyByte))
//         ])))),
//         parse(vec![OpenRound, Dot, OpenRound, Dot, CloseRound, CloseRound])
//     );
// }
//
// #[cfg(test)]
// #[test]
// fn test_parse_repeat() {
//     use Node::{AnyByte, Repeat};
//     use Token::{Byte, CloseCurly, Dot, OpenCurly, Plus, QMark, Star};
//     // ?
//     assert_eq!(
//         Err("missing element before `?` symbol".to_string()),
//         parse(vec![QMark])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(1))),
//         parse(vec![Dot, QMark])
//     );
//
//     // *
//     assert_eq!(
//         Err("missing element before `*` symbol".to_string()),
//         parse(vec![Star])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, None)),
//         parse(vec![Dot, Star])
//     );
//
//     // +
//     assert_eq!(
//         Err("missing element before `+` symbol".to_string()),
//         parse(vec![Plus])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 1, None)),
//         parse(vec![Dot, Plus])
//     );
//
//     // {1}
//     assert_eq!(
//         Err("missing element before `{` symbol".to_string()),
//         parse(vec![OpenCurly, Byte(b'1'), CloseCurly])
//     );
//     assert_eq!(
//         Err("missing `}` symbol".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b'1')])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(0))),
//         parse(vec![Dot, OpenCurly, Byte(b'0'), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 1, Some(1))),
//         parse(vec![Dot, OpenCurly, Byte(b'1'), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 99, Some(99))),
//         parse(vec![Dot, OpenCurly, Byte(b'9'), Byte(b'9'), CloseCurly])
//     );
//
//     // {,}
//     assert_eq!(
//         Err("missing element before `{` symbol".to_string()),
//         parse(vec![OpenCurly, Byte(b','), CloseCurly])
//     );
//     assert_eq!(
//         Err("missing `}` symbol".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b',')])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, None)),
//         parse(vec![Dot, OpenCurly, Byte(b','), CloseCurly])
//     );
//
//     // {1,}
//     assert_eq!(
//         Err("missing element before `{` symbol".to_string()),
//         parse(vec![OpenCurly, Byte(b'1'), Byte(b','), CloseCurly])
//     );
//     assert_eq!(
//         Err("missing `}` symbol".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b'1'), Byte(b',')])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, None)),
//         parse(vec![Dot, OpenCurly, Byte(b'0'), Byte(b','), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 1, None)),
//         parse(vec![Dot, OpenCurly, Byte(b'1'), Byte(b','), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 99, None)),
//         parse(vec![
//             Dot,
//             OpenCurly,
//             Byte(b'9'),
//             Byte(b'9'),
//             Byte(b','),
//             CloseCurly
//         ])
//     );
//
//     // {,1}
//     assert_eq!(
//         Err("missing element before `{` symbol".to_string()),
//         parse(vec![OpenCurly, Byte(b','), Byte(b'1'), CloseCurly])
//     );
//     assert_eq!(
//         Err("missing `}` symbol".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b','), Byte(b'1')])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(0))),
//         parse(vec![Dot, OpenCurly, Byte(b','), Byte(b'0'), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(1))),
//         parse(vec![Dot, OpenCurly, Byte(b','), Byte(b'1'), CloseCurly])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(99))),
//         parse(vec![
//             Dot,
//             OpenCurly,
//             Byte(b','),
//             Byte(b'9'),
//             Byte(b'9'),
//             CloseCurly
//         ])
//     );
//
//     // {1,2}
//     assert_eq!(
//         Err("missing element before `{` symbol".to_string()),
//         parse(vec![
//             OpenCurly,
//             Byte(b'1'),
//             Byte(b','),
//             Byte(b'2'),
//             CloseCurly
//         ])
//     );
//     assert_eq!(
//         Err("missing `}` symbol".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b'1'), Byte(b','), Byte(b'2')])
//     );
//     assert_eq!(
//         Err("repeating element has max that is smaller than min: {2,1}".to_string()),
//         parse(vec![Dot, OpenCurly, Byte(b'2'), Byte(b','), Byte(b'1')])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 0, Some(0))),
//         parse(vec![
//             Dot,
//             OpenCurly,
//             Byte(b'0'),
//             Byte(b','),
//             Byte(b'0'),
//             CloseCurly
//         ])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 1, Some(2))),
//         parse(vec![
//             Dot,
//             OpenCurly,
//             Byte(b'1'),
//             Byte(b','),
//             Byte(b'2'),
//             CloseCurly
//         ])
//     );
//     assert_eq!(
//         Ok(Repeat(Box::new(AnyByte), 10, Some(99))),
//         parse(vec![
//             Dot,
//             OpenCurly,
//             Byte(b'1'),
//             Byte(b'0'),
//             Byte(b','),
//             Byte(b'9'),
//             Byte(b'9'),
//             CloseCurly
//         ])
//     );
// }

fn impl_regex(stream: TokenStream) -> Result<TokenStream, String> {
    // Literal { kind: ByteStrRaw(0), symbol: "abc\\x20", suffix: None, span: #0 bytes(741..752) }
    // regex!(br"abc\x20");
    let mut tokens = stream.clone().into_iter();
    let literal = tokens
        .next()
        .ok_or("expected literal byte string".to_string())?;
    if tokens.next().is_some() {
        return Err("expected one literal byte string".to_string());
    }
    // The compiler already parsed the literal, but does not expose the fields
    // that it shows in Debug formatting.
    // So we convert the literal to a string and parse it ourselves.
    // https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-rust-proc-macro
    let literal_string = literal.to_string();
    let raw_byte_string = literal_string
        .strip_prefix("br")
        .ok_or("expected a raw byte string, like br\"abc\"".to_string())?
        // Compiler guarantees that strings are closed.
        .trim_start_matches('#')
        .trim_start_matches('"')
        .trim_end_matches('#')
        .trim_end_matches('"');
    // The compiler guarantees that a literal byte string contains only ASCII.
    // > regex!(br"€"); // error: raw byte string must be ASCII
    // Therefore, we can slice the string at any byte offset.
    let ast = parse(raw_byte_string.as_bytes())?;

    // panic!("literal: {:?} str={:?}", literal, literal.to_string());
    // if let Some(tree) = attr.into_iter().next() {
    //     return quote_spanned!(tree.span()=>compile_error!("parameters not allowed"););
    // }
    Ok(stream)

    // // Ident { ident: "async", span: #0 bytes(50..55) }
    // // Ident { ident: "fn", span: #0 bytes(56..58) }
    // // Ident { ident: "should_run_async_fn", span: #0 bytes(59..78) }
    // // Group { delimiter: Parenthesis, stream: TokenStream [], span: ...
    // // Group { delimiter: Brace, stream: TokenStream [Ident { ident: "println",...
    // let mut trees = item.into_iter();
    // let first = trees.next();
    // if let (
    //     Some(TokenTree::Ident(ref ident_async)),
    //     Some(TokenTree::Ident(ref ident_fn)),
    //     Some(TokenTree::Ident(ref ident_name)),
    // ) = (&first, trees.next(), trees.next())
    // {
    //     if *ident_async == "async" && *ident_fn == "fn" {
    //         let async_name = ident_name.to_string() + "_";
    //         let ident_async_name = Ident::new(&async_name, ident_name.span());
    //         return quote_spanned!(ident_name.span()=>
    //             #[test]
    //             pub fn #ident_async_name () {
    //                 safina_timer::start_timer_thread();
    //                 safina_executor::Executor::new(2, 1).block_on( #ident_name ())
    //             }
    //         );
    //     }
    // }
    // if let Some(tree) = first {
    //     return quote_spanned!(tree.span()=>compile_error!("expected async fn"););
    // }
    // panic!("expected async fn");
}

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = match impl_regex(input2) {
        Ok(output2) => output2,
        Err(reason) => panic!("{}", reason),
    };
    panic!(
        "output2: {}",
        output2
            .into_iter()
            .map(|tree| format!("tree: {:?}\n", tree))
            .collect::<String>()
    );
    proc_macro::TokenStream::from(output2)
}
