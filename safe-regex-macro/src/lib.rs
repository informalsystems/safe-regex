//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-macro.svg)](https://crates.io/crates/safe-regex-macro)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/safe-regex-macro/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! This crate provides the `regex!` macro used by the
//! [`safe_regex`](https://crates.io/crates/safe-regex) crate.
//!
//! It implements a
//! [contex-free grammar parser](// https://www.cs.umd.edu/class/summer2015/cmsc330/parsing/).
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about parsing
//! - Implement `regex!` macro.
//!   - Implement `parse`
//!   - Implement `generate`
//! - Add integration tests
//! - Add token tree tests
//! - Add fuzzing tests
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]

// use proc_macro2::{Ident, TokenStream, TokenTree};
use proc_macro2::TokenStream;
use std::fmt::Formatter;
// use quote::quote_spanned;

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
    Final(FinalNode),
    NonFinal(NonFinalNode),
}
impl Node {
    pub fn unwrap_final(self) -> FinalNode {
        match self {
            Node::Final(node) => node,
            other => panic!("unwrap_final() called on value: {:?}", other),
        }
    }

    pub fn unwrap_non_final(self) -> NonFinalNode {
        match self {
            Node::NonFinal(node) => node,
            other => panic!("unwrap_non_final() called on value: {:?}", other),
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum ClassItem {
    Byte(u8),
    ByteRange(u8, u8),
}
impl core::fmt::Debug for ClassItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            ClassItem::Byte(b) => write!(f, "Byte({})", escape_ascii([*b])),
            ClassItem::ByteRange(a, b) => {
                write!(
                    f,
                    "ByteRange({}-{})",
                    escape_ascii([*a]),
                    escape_ascii([*b])
                )
            }
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
enum NonFinalNode {
    Escape,
    HexEscape0,
    HexEscape1(u8),
    OpenClass0,
    OpenClassNeg,
    OpenClass(/* inclusive */ bool, Vec<ClassItem>),
    OpenByteRange(u8),
    OpenGroup,
    OpenOr(Vec<FinalNode>),
    RepeatMin(String),
    RepeatMax(String, String),
    RepeatToken(String, usize, Option<usize>),
}
impl core::fmt::Debug for NonFinalNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            NonFinalNode::Escape => write!(f, "Escape"),
            NonFinalNode::HexEscape0 => write!(f, "HexEscape0"),
            NonFinalNode::HexEscape1(b) => write!(f, "HexEscape1({})", escape_ascii([*b])),
            NonFinalNode::OpenClass0 => write!(f, "OpenClass0"),
            NonFinalNode::OpenClassNeg => write!(f, "OpenClassNeg"),
            NonFinalNode::OpenClass(true, items) => {
                write!(f, "OpenClass{:?}", items)
            }
            NonFinalNode::OpenClass(false, items) => {
                write!(f, "OpenClass^{:?}", items)
            }
            NonFinalNode::OpenByteRange(b) => write!(f, "OpenByteRange({})", escape_ascii([*b])),
            NonFinalNode::OpenGroup => write!(f, "OpenGroup"),
            NonFinalNode::OpenOr(nodes) => write!(f, "OpenOr{:?}", nodes),
            NonFinalNode::RepeatMin(min) => write!(f, "RepeatMin({})", min),
            NonFinalNode::RepeatMax(min, max) => write!(f, "RepeatMax({},{})", min, max),
            NonFinalNode::RepeatToken(printable, min, opt_max) => {
                write!(f, "RepeatToken({:?},{},{:?})", printable, min, opt_max)
            }
        }
    }
}
impl NonFinalNode {
    pub fn reason(&self) -> String {
        match self {
            NonFinalNode::Escape => "incomplete escape sequence: `\\`".to_string(),
            NonFinalNode::HexEscape0 => "incomplete escape sequence: `\\x`".to_string(),
            NonFinalNode::HexEscape1(d) => {
                format!("incomplete escape sequence: `\\x{}`", escape_ascii([*d]))
            }
            NonFinalNode::OpenClass0
            | NonFinalNode::OpenClassNeg
            | NonFinalNode::OpenClass(_, _) => "missing closing `]`".to_string(),
            NonFinalNode::OpenByteRange(b) => {
                format!("missing byte to close range: `{}-`", escape_ascii([*b]))
            }
            NonFinalNode::OpenGroup => "missing closing `)`".to_string(),
            NonFinalNode::OpenOr(_) => "missing element after bar `|`".to_string(),
            NonFinalNode::RepeatMin(min) => {
                format!("missing closing `}}` symbol: `{{{}`", min)
            }
            NonFinalNode::RepeatMax(min, max) => {
                format!("missing closing `}}` symbol: `{{{},{}`", min, max)
            }
            NonFinalNode::RepeatToken(printable, _, _) => {
                format!("missing element before repeat element: `{}`", printable)
            }
        }
    }

    pub fn unwrap_open_class(self) -> (bool, Vec<ClassItem>) {
        match self {
            NonFinalNode::OpenClass(incl, items) => (incl, items),
            other => panic!("unwrap_open_class() called on value: {:?}", other),
        }
    }

    pub fn unwrap_open_or(self) -> Vec<FinalNode> {
        match self {
            NonFinalNode::OpenOr(nodes) => nodes,
            other => panic!("unwrap_open_or() called on value: {:?}", other),
        }
    }

    pub fn unwrap_repeat_min(self) -> String {
        match self {
            NonFinalNode::RepeatMin(min) => min,
            other => panic!("unwrap_repeat_min() called on value: {:?}", other),
        }
    }

    pub fn unwrap_repeat_max(self) -> (String, String) {
        match self {
            NonFinalNode::RepeatMax(min, max) => (min, max),
            other => panic!("unwrap_repeat_max() called on value: {:?}", other),
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
enum FinalNode {
    Byte(u8),
    AnyByte,
    Seq(Vec<FinalNode>),
    Class(/* inclusive */ bool, Vec<ClassItem>),
    ByteRange(u8, u8),
    Group(Box<FinalNode>),
    Or(Vec<FinalNode>),
    Repeat(Box<FinalNode>, usize, Option<usize>),
}
impl FinalNode {
    pub fn unwrap_or(self) -> Vec<FinalNode> {
        match self {
            FinalNode::Or(nodes) => nodes,
            other => panic!("unwrap_or() called on value: {:?}", other),
        }
    }
}
impl core::fmt::Debug for FinalNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            FinalNode::Byte(b) => write!(f, "Byte({})", escape_ascii([*b])),
            FinalNode::AnyByte => write!(f, "AnyByte"),
            FinalNode::Seq(nodes) => write!(f, "Seq{:?}", nodes),
            FinalNode::Class(true, items) => write!(f, "Class{:?}", items),
            FinalNode::Class(false, items) => write!(f, "Class^{:?}", items),
            FinalNode::ByteRange(a, b) => {
                write!(
                    f,
                    "ByteRange({}-{})",
                    escape_ascii([*a]),
                    escape_ascii([*b])
                )
            }
            FinalNode::Group(nodes) => write!(f, "Group{:?}", nodes),
            FinalNode::Or(nodes) => write!(f, "Or{:?}", nodes),
            FinalNode::Repeat(node, min, opt_max) => {
                write!(f, "Repeat({:?},{}-{:?})", node, min, opt_max)
            }
        }
    }
}

fn invalid_escape(bytes: impl AsRef<[u8]>) -> String {
    format!("invalid escape sequence `\\{}`", escape_ascii(bytes))
}

fn parse(data: &[u8]) -> Result<FinalNode, String> {
    // This parser works, but it is very hard to understand.
    // We should separate the parser and the grammar declarations.
    use FinalNode::{AnyByte, Byte, ByteRange, Class, Group, Or, Repeat, Seq};
    use Node::{Final, NonFinal};
    use NonFinalNode::{
        Escape, HexEscape0, HexEscape1, OpenByteRange, OpenClass, OpenClass0, OpenClassNeg,
        OpenGroup, OpenOr, RepeatMax, RepeatMin, RepeatToken,
    };
    if data.is_empty() {
        return Ok(Seq(Vec::new()));
    }
    let mut iter = data.iter().copied().peekable();
    let mut stack: Vec<Node> = Vec::new();
    while iter.peek().is_some() || stack.len() > 1 {
        // println!("process {:?} next={:?}", stack, iter.peek().map(|b| escape_ascii([*b])));
        let mut last = stack.pop();
        let mut prev = stack.pop();
        let mut to_push: Option<Node> = None;
        match (&mut prev, &mut last, iter.peek().copied()) {
            (Some(_), None, _) => unreachable!(),
            // Combine class nodes
            (Some(NonFinal(OpenByteRange(a))), Some(Final(Byte(b))), _) => {
                let node = Final(ByteRange(*a, *b));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClass0)), Some(Final(Byte(b))), _) => {
                let node = NonFinal(OpenClass(true, vec![ClassItem::Byte(*b)]));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClassNeg)), Some(Final(Byte(b))), _) => {
                let node = NonFinal(OpenClass(false, vec![ClassItem::Byte(*b)]));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClass(_, items))), Some(Final(Byte(b))), _) => {
                let item = ClassItem::Byte(*b);
                last.take();
                items.push(item);
            }
            (Some(NonFinal(OpenClass(_, items))), Some(Final(ByteRange(a, b))), _) => {
                let item = ClassItem::ByteRange(*a, *b);
                last.take();
                items.push(item);
            }
            // Combine repeat tokens
            (None, Some(NonFinal(RepeatToken(printable, _, _))), _)
            | (Some(NonFinal(_)), Some(NonFinal(RepeatToken(printable, _, _))), _) => {
                return Err(format!(
                    "missing element before repeat element: `{}`",
                    printable
                ))
            }
            (Some(Final(_)), Some(NonFinal(RepeatToken(_, min, opt_max))), _) => {
                let inner = prev.take().unwrap().unwrap_final();
                prev = Some(Final(Repeat(Box::new(inner), *min, *opt_max)));
                last.take();
            }
            // Combine group tokens
            (Some(NonFinal(OpenGroup)), Some(Final(_)), None) => return Err(OpenGroup.reason()),
            // Combine Seq tokens
            (Some(Final(Seq(nodes))), Some(Final(_)), _) => {
                let node = last.take().unwrap().unwrap_final();
                nodes.push(node)
            }
            // Combine alternation/or nodes
            (Some(NonFinal(OpenOr(_))), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                let node = last.take().unwrap().unwrap_final();
                let mut nodes = prev.take().unwrap().unwrap_non_final().unwrap_open_or();
                nodes.push(node);
                prev = Some(Final(Or(nodes)));
            }
            (Some(Final(Or(nodes))), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                let node = last.take().unwrap().unwrap_final();
                match nodes.last_mut().unwrap() {
                    Seq(ref mut seq_nodes) => seq_nodes.push(node),
                    _ => {
                        let prev_node = nodes.pop().unwrap();
                        nodes.push(Seq(vec![prev_node, node]))
                    }
                }
            }
            // Create Seq tokens
            (Some(Final(_)), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                to_push = Some(Final(Seq(vec![
                    prev.take().unwrap().unwrap_final(),
                    last.take().unwrap().unwrap_final(),
                ])))
            }

            // Escaped characters `\n`
            (_, Some(NonFinal(Escape)), Some(b'\\')) => {
                iter.next().unwrap();
                last = Some(Final(Byte(b'\\')))
            }
            (_, _, Some(b'\\')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(Escape))
            }
            (_, Some(NonFinal(Escape)), Some(b'n')) => {
                iter.next().unwrap();
                last = Some(Final(Byte(b'\n')))
            }
            (_, Some(NonFinal(Escape)), Some(b'r')) => {
                iter.next().unwrap();
                last = Some(Final(Byte(b'\r')))
            }
            (_, Some(NonFinal(Escape)), Some(b't')) => {
                iter.next().unwrap();
                last = Some(Final(Byte(b'\t')))
            }
            (_, Some(NonFinal(Escape)), Some(b'0')) => {
                iter.next().unwrap();
                last = Some(Final(Byte(0)))
            }
            (_, Some(NonFinal(Escape)), Some(b)) if b"'\"?+.*^$|(){}[]".contains(&b) => {
                iter.next().unwrap();
                last = Some(Final(Byte(b)))
            }
            // Hex characters `\x20`
            (_, Some(NonFinal(Escape)), Some(b'x')) => {
                iter.next().unwrap();
                last = Some(NonFinal(HexEscape0))
            }
            (_, Some(NonFinal(Escape)), Some(d)) => return Err(invalid_escape([d])),
            (_, Some(NonFinal(HexEscape0)), Some(d)) => {
                iter.next().unwrap();
                last = Some(NonFinal(HexEscape1(d)))
            }
            (_, Some(NonFinal(HexEscape1(d1))), Some(d0))
                if d1.is_ascii_hexdigit() && d0.is_ascii_hexdigit() =>
            {
                iter.next().unwrap();
                let string = String::from_utf8(vec![*d1, d0]).unwrap();
                let byte = u8::from_str_radix(&string, 16).unwrap();
                last = Some(Final(Byte(byte)))
            }
            (_, Some(NonFinal(HexEscape1(d1))), Some(d0)) => {
                return Err(invalid_escape([b'x', *d1, d0]))
            }
            // Class `[ab0-9]`, `[^-ab0-9]`
            (_, Some(NonFinal(OpenClass0)), Some(b'['))
            | (_, Some(NonFinal(OpenClassNeg)), Some(b'['))
            | (_, Some(NonFinal(OpenClass(..))), Some(b'[')) => {
                iter.next().unwrap();
                to_push = Some(Final(Byte(b'[')))
            }
            (_, _, Some(b'[')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(OpenClass0))
            }
            (_, Some(NonFinal(OpenClass0)), Some(b'^')) => {
                iter.next().unwrap();
                last = Some(NonFinal(OpenClassNeg))
            }
            (_, Some(NonFinal(OpenClass(_, ref mut items))), Some(b'-')) => {
                iter.next().unwrap();
                match items.pop().unwrap() {
                    // "[a-"
                    ClassItem::Byte(b) => {
                        to_push = Some(NonFinal(OpenByteRange(b)));
                    }
                    // "[a-b-"
                    ClassItem::ByteRange(a, b) => {
                        return Err(format!(
                            "expected byte before '-' symbol, not range: `{}-{}-`",
                            escape_ascii([a]),
                            escape_ascii([b])
                        ))
                    }
                }
            }
            (_, Some(NonFinal(OpenClass0)), Some(b']')) => {
                iter.next().unwrap();
                last = Some(Final(Class(true, Vec::new())))
            }
            (_, Some(NonFinal(OpenClassNeg)), Some(b']')) => {
                iter.next().unwrap();
                last = Some(Final(Class(false, Vec::new())))
            }
            (_, Some(NonFinal(OpenClass(..))), Some(b']')) => {
                iter.next().unwrap();
                let (incl, items) = last.take().unwrap().unwrap_non_final().unwrap_open_class();
                to_push = Some(Final(Class(incl, items)))
            }
            (Some(NonFinal(OpenClass(..))), Some(NonFinal(non_final)), Some(b']')) => {
                return Err(non_final.reason())
            }
            // Other characters inside character classes
            (_, Some(NonFinal(OpenClass0)), Some(b))
            | (_, Some(NonFinal(OpenClassNeg)), Some(b))
            | (_, Some(NonFinal(OpenClass(..))), Some(b))
                if b != b']' =>
            {
                iter.next().unwrap();
                to_push = Some(Final(Byte(b)))
            }

            // Single-character postfix operators `?` `+` `*`
            (_, _, Some(b'?')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("?".to_string(), 0, Some(1))))
            }
            (_, _, Some(b'*')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("*".to_string(), 0, None)))
            }
            (_, _, Some(b'+')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("+".to_string(), 1, None)))
            }
            // Repeat postfix operators `{n}` `{n,}` `{,m}` `{n,m}`
            (_, _, Some(b'{')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(RepeatMin(String::new())))
            }
            (_, Some(NonFinal(RepeatMin(_))), Some(b',')) => {
                iter.next().unwrap();
                let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
                last = Some(NonFinal(RepeatMax(min, String::new())))
            }
            (_, Some(NonFinal(RepeatMin(_))), Some(b'}')) => {
                iter.next().unwrap();
                let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
                let min_usize = usize::from_str_radix(&min, 10)
                    .map_err(|_| format!("invalid repetition value: `{{{}}}`", min))?;
                last = Some(NonFinal(RepeatToken(
                    format!("{{{}}}", min),
                    min_usize,
                    Some(min_usize),
                )))
            }
            (_, Some(NonFinal(RepeatMin(min))), Some(b)) => {
                iter.next().unwrap();
                min.push(char::from(b))
            }
            (_, Some(NonFinal(RepeatMax(..))), Some(b'}')) => {
                iter.next().unwrap();
                let (min, max) = last.take().unwrap().unwrap_non_final().unwrap_repeat_max();
                let min_usize = if min.is_empty() {
                    0
                } else {
                    usize::from_str_radix(&min, 10)
                        .map_err(|_| format!("invalid repetition value: `{{{},{}}}`", min, max))?
                };
                let max_opt_usize = if max.is_empty() {
                    None
                } else {
                    let max_usize = usize::from_str_radix(&max, 10)
                        .map_err(|_| format!("invalid repetition value: `{{{},{}}}`", min, max))?;
                    if max_usize < min_usize {
                        return Err(format!(
                            "repeating element has max that is smaller than min: `{{{},{}}}`",
                            min, max
                        ));
                    }
                    Some(max_usize)
                };
                last = Some(NonFinal(RepeatToken(
                    format!("{{{},{}}}", min, max),
                    min_usize,
                    max_opt_usize,
                )))
            }
            (_, Some(NonFinal(RepeatMax(_, ref mut max))), Some(b)) => {
                iter.next().unwrap();
                max.push(char::from(b));
            }

            // Any byte `.`
            (_, _, Some(b'.')) => {
                iter.next().unwrap();
                to_push = Some(Final(AnyByte))
            }

            // Alternation (Or) `a|b|c`
            (_, Some(Final(Or(_))), Some(b'|')) => {
                iter.next().unwrap();
                let nodes = last.take().unwrap().unwrap_final().unwrap_or();
                last = Some(NonFinal(OpenOr(nodes)))
            }
            (_, Some(Final(_)), Some(b'|')) => {
                iter.next().unwrap();
                let node = last.take().unwrap().unwrap_final();
                last = Some(NonFinal(OpenOr(vec![node])))
            }
            (_, None, Some(b'|')) => return Err("missing element before bar `|`".to_string()),

            // Group `(ab)`
            (_, _, Some(b'(')) => {
                iter.next().unwrap();
                to_push = Some(NonFinal(OpenGroup))
            }
            (_, Some(NonFinal(OpenGroup)), Some(b')')) => {
                iter.next().unwrap();
                last = Some(Final(Group(Box::new(Seq(vec![])))))
            }
            (Some(NonFinal(OpenGroup)), Some(NonFinal(non_final)), Some(b')')) => {
                return Err(non_final.reason())
            }
            (Some(NonFinal(OpenGroup)), Some(Final(_)), Some(b')')) => {
                iter.next().unwrap();
                let node = last.take().unwrap().unwrap_final();
                prev = Some(Final(Group(Box::new(node))));
            }

            // Other bytes
            (_, _, Some(b)) => {
                iter.next().unwrap();
                to_push = Some(Final(Byte(b)))
            }

            // No more bytes.
            (_, Some(NonFinal(node)), None) => return Err(node.reason()),
            (None, None, None) => unreachable!(),
            (None, Some(Final(_)), None) => unreachable!(),

            // These cases should be unreachable.
            (Some(NonFinal(Escape)), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(HexEscape0)), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(HexEscape1(_))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(RepeatMin(..))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(RepeatMax(..))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenClass0)), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenClassNeg)), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenClass(..))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenOr(_))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenByteRange(_))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(RepeatToken(..))), Some(Final(_)), _) => unreachable!(),
            (Some(Final(_)), Some(Final(_)), _) => unreachable!(),
        };
        if let Some(node) = prev.take() {
            stack.push(node);
        }
        if let Some(node) = last.take() {
            stack.push(node);
        }
        if let Some(node) = to_push.take() {
            stack.push(node);
        }
    }
    // println!("stack {:?}", stack);
    for node in stack.iter().rev() {
        if let NonFinal(non_final) = node {
            return Err(non_final.reason());
        }
    }
    assert_eq!(1, stack.len());
    Ok(stack.pop().unwrap().unwrap_final())
}

#[cfg(test)]
#[test]
fn test_parse() {
    use FinalNode::{AnyByte, Byte, Seq};
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
    use FinalNode::Byte;
    assert_eq!(
        Err(r"incomplete escape sequence: `\`".to_string()),
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
        Err(r"incomplete escape sequence: `\x`".to_string()),
        parse(br"\x")
    );
    assert_eq!(
        Err(r"incomplete escape sequence: `\x0`".to_string()),
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
    use FinalNode::{Byte, Or, Seq};
    assert_eq!(
        Err(r"missing element before bar `|`".to_string()),
        parse(br"|")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
        parse(br"a|")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
        parse(br"(a|)")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
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

#[cfg(test)]
#[test]
fn test_parse_class() {
    use FinalNode::Class;
    assert_eq!(Err("missing closing `]`".to_string()), parse(br"[a"));
    assert_eq!(Err("missing closing `]`".to_string()), parse(br"[^a"));
    assert_eq!(Ok(Class(true, vec![])), parse(br"[]"));
    assert_eq!(Ok(Class(false, vec![])), parse(br"[^]"));
    assert_eq!(Ok(Class(true, vec![ClassItem::Byte(b'a')])), parse(br"[a]"));
    assert_eq!(
        Ok(Class(false, vec![ClassItem::Byte(b'a')])),
        parse(br"[^a]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![ClassItem::Byte(b'^'), ClassItem::Byte(b'a')]
        )),
        parse(br"[^^a]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b'),
                ClassItem::Byte(b'c')
            ]
        )),
        parse(br"[abc]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b'),
                ClassItem::Byte(b'c')
            ]
        )),
        parse(br"[^abc]")
    );
    // ?+*.^$|(){}[]
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'?'),
                ClassItem::Byte(b'+'),
                ClassItem::Byte(b'*'),
                ClassItem::Byte(b'.'),
                ClassItem::Byte(b'^'),
                ClassItem::Byte(b'$'),
                ClassItem::Byte(b'|'),
                ClassItem::Byte(b'('),
                ClassItem::Byte(b')'),
                ClassItem::Byte(b'{'),
                ClassItem::Byte(b'}'),
                ClassItem::Byte(b'['),
                ClassItem::Byte(b']'),
            ]
        )),
        parse(br"[?+*.^$|(){}[\]]")
    );

    assert_eq!(
        Err("missing byte to close range: `b-`".to_string()),
        parse(br"[ab-]")
    );
    assert_eq!(
        Err("missing byte to close range: `a-`".to_string()),
        parse(br"[^a-]")
    );
    assert_eq!(
        Err("expected byte before '-' symbol, not range: `a-b-`".to_string()),
        parse(br"[a-b-]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![ClassItem::Byte(b'-'), ClassItem::Byte(b'a')]
        )),
        parse(br"[^-a]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::ByteRange(b'^', b'a')])),
        parse(br"[^^-a]")
    );
    assert_eq!(
        Ok(Class(true, vec![ClassItem::ByteRange(b'a', b'c')])),
        parse(br"[a-c]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::ByteRange(b'a', b'c')])),
        parse(br"[^a-c]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::ByteRange(b'a', b'c'),
                ClassItem::ByteRange(b'g', b'h')
            ]
        )),
        parse(br"[a-cg-h]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'-'),
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b')
            ]
        )),
        parse(br"[-ab]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::Byte(b'-'),])),
        parse(br"[^-]")
    );
}

#[cfg(test)]
#[test]
fn test_parse_group() {
    use FinalNode::{AnyByte, Group, Seq};
    assert_eq!(Err("missing closing `)`".to_string()), parse(br"(."));
    assert_eq!(Ok(Group(Box::new(Seq(vec![])))), parse(br"()"));
    assert_eq!(Ok(Group(Box::new(AnyByte))), parse(br"(.)"));
    assert_eq!(
        Ok(Group(Box::new(Group(Box::new(AnyByte))))),
        parse(br"((.))")
    );
    assert_eq!(
        Ok(Group(Box::new(Seq(vec![
            AnyByte,
            Group(Box::new(AnyByte))
        ])))),
        parse(br"(.(.))")
    );
}

#[cfg(test)]
#[test]
fn test_parse_repeat() {
    use FinalNode::{AnyByte, Repeat};
    // ?
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"?")
    );
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"b|?")
    );
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"(?)")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(1))), parse(br".?"));

    // *
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"*")
    );
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"b|*")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".*"));
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"(*)")
    );

    // +
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"+")
    );
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"b|+")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, None)), parse(br".+"));
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"(+)")
    );

    // {1}
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"{1}")
    );
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"(ab|{1})")
    );
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"({1})")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1`".to_string()),
        parse(br".{1")
    );
    assert_eq!(
        Err("invalid repetition value: `{}`".to_string()),
        parse(br".{}")
    );
    assert_eq!(
        Err("invalid repetition value: `{a}`".to_string()),
        parse(br".{a}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, Some(1))), parse(br".{1}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 99, Some(99))),
        parse(br".{99}")
    );

    // {,}
    assert_eq!(
        Err("missing element before repeat element: `{,}`".to_string()),
        parse(br"{,}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{,`".to_string()),
        parse(br".{,")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".{,}"));

    // {1,}
    assert_eq!(
        Err("missing element before repeat element: `{1,}`".to_string()),
        parse(br"{1,}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1,`".to_string()),
        parse(br".{1,")
    );
    assert_eq!(
        Err("invalid repetition value: `{a,}`".to_string()),
        parse(br".{a,}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".{0,}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, None)), parse(br".{1,}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 99, None)), parse(br".{99,}"));

    // {,1}
    assert_eq!(
        Err("missing element before repeat element: `{,1}`".to_string()),
        parse(br"{,1}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{,1`".to_string()),
        parse(br".{,1")
    );
    assert_eq!(
        Err("invalid repetition value: `{,a}`".to_string()),
        parse(br".{,a}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{,0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(1))), parse(br".{,1}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 0, Some(99))),
        parse(br".{,99}")
    );

    // {1,2}
    assert_eq!(
        Err("missing element before repeat element: `{1,2}`".to_string()),
        parse(br"{1,2}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1,2`".to_string()),
        parse(br".{1,2")
    );
    assert_eq!(
        Err("invalid repetition value: `{0,b}`".to_string()),
        parse(br".{0,b}")
    );
    assert_eq!(
        Err("invalid repetition value: `{a,1}`".to_string()),
        parse(br".{a,1}")
    );
    assert_eq!(
        Err("invalid repetition value: `{a,b}`".to_string()),
        parse(br".{a,b}")
    );
    assert_eq!(
        Err("repeating element has max that is smaller than min: `{2,1}`".to_string()),
        parse(br".{2,1}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{0,0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, Some(2))), parse(br".{1,2}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 10, Some(99))),
        parse(br".{10,99}")
    );
}

#[cfg(test)]
#[test]
fn test_parse_precedence() {
    // Regular expressions have four types of syntax:
    // - Discrete tokens: . a [abc]
    // - Postfix operators: a? a* a+ a{n}.  These are unambiguous.
    // - Concatenation: ab
    // - Alternation/Or: a|b
    // We will test all combinations of these types to confirm correct parsing.
    // For example, we want to make sure that `a|bc` gets parsed as `a|(bc)` and
    // and not `(a|b)c`.
    use FinalNode::{Byte, Or, Repeat, Seq};
    // Postfix & Concatenation
    assert_eq!(
        Ok(Seq(vec![
            Byte(b'a'),
            Repeat(Box::new(Byte(b'b')), 0, Some(1))
        ])),
        parse(br"ab?")
    );
    // Postfix & Alternation
    assert_eq!(
        Ok(Or(vec![
            Repeat(Box::new(Byte(b'a')), 0, None),
            Repeat(Box::new(Byte(b'b')), 0, None),
            Repeat(Box::new(Byte(b'c')), 0, None),
        ])),
        parse(br"a*|b*|c*")
    );
    // Concatenation & Alternation
    assert_eq!(
        Ok(Or(vec![
            Seq(vec![Byte(b'a'), Byte(b'b')]),
            Seq(vec![Byte(b'c'), Byte(b'd')]),
            Seq(vec![Byte(b'e'), Byte(b'f')]),
        ])),
        parse(br"ab|cd|ef")
    );
}

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
