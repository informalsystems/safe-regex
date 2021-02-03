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

    pub fn unwrap_final_mut(&mut self) -> &mut FinalNode {
        match self {
            Node::Final(node) => node,
            other => panic!("unwrap_final_mut() called on value: {:?}", other),
        }
    }

    pub fn unwrap_non_final(self) -> NonFinalNode {
        match self {
            Node::NonFinal(node) => node,
            other => panic!("unwrap_non_final() called on value: {:?}", other),
        }
    }

    pub fn unwrap_non_final_mut(&mut self) -> &mut NonFinalNode {
        match self {
            Node::NonFinal(node) => node,
            other => panic!("unwrap_non_final_mut() called on value: {:?}", other),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum ClassItem {
    Byte(u8),
    ByteRange(u8, u8),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
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
                format!("missing element before `{}` symbol", printable)
            }
        }
    }

    pub fn unwrap_open_class(self) -> (bool, Vec<ClassItem>) {
        match self {
            NonFinalNode::OpenClass(incl, items) => (incl, items),
            other => panic!("unwrap_open_class() called on value: {:?}", other),
        }
    }

    pub fn unwrap_open_class_mut(&mut self) -> (&mut bool, &mut Vec<ClassItem>) {
        match self {
            NonFinalNode::OpenClass(incl, items) => (incl, items),
            other => panic!("unwrap_open_class_mut() called on value: {:?}", other),
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

    pub fn unwrap_repeat_min_mut(&mut self) -> &mut String {
        match self {
            NonFinalNode::RepeatMin(min) => min,
            other => panic!("unwrap_repeat_min_mut() called on value: {:?}", other),
        }
    }

    pub fn unwrap_repeat_max(self) -> (String, String) {
        match self {
            NonFinalNode::RepeatMax(min, max) => (min, max),
            other => panic!("unwrap_repeat_max() called on value: {:?}", other),
        }
    }

    pub fn unwrap_repeat_max_mut(&mut self) -> (&mut String, &mut String) {
        match self {
            NonFinalNode::RepeatMax(min, max) => (min, max),
            other => panic!("unwrap_repeat_max_mut() called on value: {:?}", other),
        }
    }

    pub fn unwrap_repeat_token(self) -> (String, usize, Option<usize>) {
        match self {
            NonFinalNode::RepeatToken(printable, min, opt_max) => (printable, min, opt_max),
            other => panic!("unwrap_repeat_token() called on value: {:?}", other),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
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

    pub fn unwrap_or_mut(&mut self) -> &mut Vec<FinalNode> {
        match self {
            FinalNode::Or(nodes) => nodes,
            other => panic!("unwrap_or_mut() called on value: {:?}", other),
        }
    }

    pub fn unwrap_seq_mut(&mut self) -> &mut Vec<FinalNode> {
        match self {
            FinalNode::Seq(nodes) => nodes,
            other => panic!("unwrap_seq_mut() called on value: {:?}", other),
        }
    }
}

fn invalid_escape(bytes: impl AsRef<[u8]>) -> String {
    format!("invalid escape sequence `\\{}`", escape_ascii(bytes))
}

fn replace_top(stack: &mut Vec<Node>, node: Node) {
    stack.pop().unwrap();
    stack.push(node);
}

fn parse(data: &[u8]) -> Result<FinalNode, String> {
    // This parser works, but it is very hard to understand.
    // We should separate the parser and the grammar declarations.
    //
    // Working with the stack is ugly because of Rust's borrowing rules.
    // Idea for making it cleaner: In each loop iteration, keep the top and
    // second-from-top token in a `mut Option<Node>`.  Then code can call `take`
    // to discard a token and assign new values to replace.
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
    while let Some(b) = iter.next() {
        //println!("stack {:?}", stack);
        //println!("process {}", escape_ascii([b]));
        // Process the new byte.
        let prev = if stack.len() < 2 {
            None
        } else {
            stack.get(stack.len() - 2)
        };
        match (prev, stack.last(), b) {
            // Escaped characters `\n`
            (_, Some(NonFinal(Escape)), b'\\') => replace_top(&mut stack, Final(Byte(b'\\'))),
            (_, _, b'\\') => stack.push(NonFinal(Escape)),
            (_, Some(NonFinal(Escape)), b'n') => replace_top(&mut stack, Final(Byte(b'\n'))),
            (_, Some(NonFinal(Escape)), b'r') => replace_top(&mut stack, Final(Byte(b'\r'))),
            (_, Some(NonFinal(Escape)), b't') => replace_top(&mut stack, Final(Byte(b'\t'))),
            (_, Some(NonFinal(Escape)), b'0') => replace_top(&mut stack, Final(Byte(0))),
            (_, Some(NonFinal(Escape)), b) if b"'\"?+.*^$|(){}[]".contains(&b) => {
                replace_top(&mut stack, Final(Byte(b)))
            }
            // Hex characters `\x20`
            (_, Some(NonFinal(Escape)), b'x') => replace_top(&mut stack, NonFinal(HexEscape0)),
            (_, Some(NonFinal(Escape)), d) => return Err(invalid_escape([d])),
            (_, Some(NonFinal(HexEscape0)), d) => replace_top(&mut stack, NonFinal(HexEscape1(d))),
            (_, Some(NonFinal(HexEscape1(d1))), d0)
                if d1.is_ascii_hexdigit() && d0.is_ascii_hexdigit() =>
            {
                let string = String::from_utf8(vec![*d1, d0]).unwrap();
                let byte = u8::from_str_radix(&string, 16).unwrap();
                replace_top(&mut stack, Final(Byte(byte)))
            }
            (_, Some(NonFinal(HexEscape1(d1))), d0) => return Err(invalid_escape([b'x', *d1, d0])),
            // Class `[ab0-9]`, `[^-ab0-9]`
            (_, Some(NonFinal(OpenClass0)), b'[')
            | (_, Some(NonFinal(OpenClassNeg)), b'[')
            | (_, Some(NonFinal(OpenClass(..))), b'[') => stack.push(Final(Byte(b'['))),
            (_, _, b'[') => stack.push(NonFinal(OpenClass0)),
            (_, Some(NonFinal(OpenClass0)), b'^') => {
                replace_top(&mut stack, NonFinal(OpenClassNeg))
            }
            (_, Some(NonFinal(OpenClass(..))), b'-') => {
                let (_, items) = stack
                    .last_mut()
                    .unwrap()
                    .unwrap_non_final_mut()
                    .unwrap_open_class_mut();
                match items.pop().unwrap() {
                    // "[a-"
                    ClassItem::Byte(b) => {
                        stack.push(NonFinal(OpenByteRange(b)));
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
            (_, Some(NonFinal(OpenClass0)), b']') => {
                replace_top(&mut stack, Final(Class(true, Vec::new())))
            }
            (_, Some(NonFinal(OpenClassNeg)), b']') => {
                replace_top(&mut stack, Final(Class(false, Vec::new())))
            }
            (_, Some(NonFinal(OpenClass(..))), b']') => {
                let (incl, items) = stack.pop().unwrap().unwrap_non_final().unwrap_open_class();
                stack.push(Final(Class(incl, items)))
            }
            (Some(NonFinal(OpenClass(..))), Some(NonFinal(non_final)), b']') => {
                return Err(non_final.reason())
            }
            // Other characters inside character classes
            (_, Some(NonFinal(OpenClass0)), b)
            | (_, Some(NonFinal(OpenClassNeg)), b)
            | (_, Some(NonFinal(OpenClass(..))), b)
                if b != b']' =>
            {
                stack.push(Final(Byte(b)))
            }
            // Any byte `.`
            (_, _, b'.') => stack.push(Final(AnyByte)),
            // Single-character postfix operators `?` `+` `*`
            (_, None, b'?') => return Err("missing element before `?` symbol".to_string()),
            (_, None, b'+') => return Err("missing element before `+` symbol".to_string()),
            (_, None, b'*') => return Err("missing element before `*` symbol".to_string()),
            (_, Some(NonFinal(non_final)), b'?') => return Err(non_final.reason()),
            (_, Some(NonFinal(non_final)), b'+') => return Err(non_final.reason()),
            (_, Some(NonFinal(non_final)), b'*') => return Err(non_final.reason()),
            (_, Some(Final(_)), b'?') => {
                stack.push(NonFinal(RepeatToken("?".to_string(), 0, Some(1))))
            }
            (_, Some(Final(_)), b'*') => {
                stack.push(NonFinal(RepeatToken("*".to_string(), 0, None)))
            }
            (_, Some(Final(_)), b'+') => {
                stack.push(NonFinal(RepeatToken("+".to_string(), 1, None)))
            }
            // Repeat postfix operators `{n}` `{n,}` `{,m}` `{n,m}`
            (_, None, b'{') => return Err("missing element before `{` symbol".to_string()),
            (_, Some(NonFinal(non_final)), b'{') => return Err(non_final.reason()),
            (_, Some(Final(_)), b'{') => stack.push(NonFinal(RepeatMin(String::new()))),
            (_, Some(NonFinal(RepeatMin(_))), b',') => {
                let min = stack.pop().unwrap().unwrap_non_final().unwrap_repeat_min();
                stack.push(NonFinal(RepeatMax(min, String::new())))
            }
            (_, Some(NonFinal(RepeatMin(_))), b'}') => {
                let min = stack.pop().unwrap().unwrap_non_final().unwrap_repeat_min();
                let min_usize = usize::from_str_radix(&min, 10)
                    .map_err(|_| format!("invalid repetition value: `{{{}}}`", min))?;
                stack.push(NonFinal(RepeatToken(
                    format!("{{{}}}", min),
                    min_usize,
                    Some(min_usize),
                )))
            }
            (_, Some(NonFinal(RepeatMin(_))), b) => {
                let min = stack
                    .last_mut()
                    .unwrap()
                    .unwrap_non_final_mut()
                    .unwrap_repeat_min_mut();
                min.push(char::from(b))
            }
            (_, Some(NonFinal(RepeatMax(..))), b'}') => {
                let (min, max) = stack.pop().unwrap().unwrap_non_final().unwrap_repeat_max();
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
                stack.push(NonFinal(RepeatToken(
                    format!("{{{},{}}}", min, max),
                    min_usize,
                    max_opt_usize,
                )))
            }
            (_, Some(NonFinal(RepeatMax(..))), b) => {
                let (_, max) = stack
                    .last_mut()
                    .unwrap()
                    .unwrap_non_final_mut()
                    .unwrap_repeat_max_mut();
                max.push(char::from(b));
            }
            // Alternation (Or) `a|b|c`
            (_, Some(Final(Or(_))), b'|') => {
                let nodes = stack.pop().unwrap().unwrap_final().unwrap_or();
                stack.push(NonFinal(OpenOr(nodes)))
            }
            (_, Some(Final(_)), b'|') => {
                let node = stack.pop().unwrap().unwrap_final();
                stack.push(NonFinal(OpenOr(vec![node])))
            }
            (_, None, b'|') => return Err("missing element before bar `|`".to_string()),
            // Group `(ab)`
            (_, _, b'(') => stack.push(NonFinal(OpenGroup)),
            (_, Some(NonFinal(OpenGroup)), b')') => {
                replace_top(&mut stack, Final(Group(Box::new(Seq(vec![])))))
            }
            (Some(NonFinal(OpenGroup)), Some(NonFinal(non_final)), b')') => {
                return Err(non_final.reason())
            }
            (Some(NonFinal(OpenGroup)), Some(Final(_)), b')') => {
                let node = stack.pop().unwrap().unwrap_final();
                replace_top(&mut stack, Final(Group(Box::new(node))));
            }
            // Other bytes
            (_, _, byte) => stack.push(Final(Byte(byte))),
        };
        //println!("stack {:?}", stack);

        // Combine and reduce tokens.
        while stack.len() >= 2 {
            match (
                stack.get(stack.len() - 2).unwrap(),
                stack.last().unwrap(),
                iter.peek(),
            ) {
                (Final(_), Final(_), Some(b)) | (NonFinal(OpenOr(_)), Final(_), Some(b))
                    if *b == b'?' || *b == b'*' || *b == b'+' || *b == b'{' =>
                {
                    break
                }
                (Final(_), NonFinal(RepeatToken(..)), _) => {
                    let (_, min, opt_max) = stack
                        .pop()
                        .unwrap()
                        .unwrap_non_final()
                        .unwrap_repeat_token();
                    let node = stack.pop().unwrap().unwrap_final();
                    stack.push(Final(Repeat(Box::new(node), min, opt_max)))
                }
                // Do not transform other non-final nodes.
                (_, NonFinal(_), _) => break,
                // Alternation (Or) `a|b|c`
                (NonFinal(OpenOr(_)), Final(_), _) => {
                    let node = stack.pop().unwrap().unwrap_final();
                    let mut nodes = stack.pop().unwrap().unwrap_non_final().unwrap_open_or();
                    nodes.push(node);
                    stack.push(Final(Or(nodes)));
                }
                (Final(Or(_)), Final(_), _) => {
                    let node = stack.pop().unwrap().unwrap_final();
                    let nodes = stack.last_mut().unwrap().unwrap_final_mut().unwrap_or_mut();
                    match nodes.last_mut().unwrap() {
                        Seq(ref mut seq_nodes) => seq_nodes.push(node),
                        _ => {
                            let prev_node = nodes.pop().unwrap();
                            nodes.push(Seq(vec![prev_node, node]))
                        }
                    }
                }
                // Class `[ab0-9]`, `[^-ab0-9]`
                (NonFinal(OpenByteRange(a)), Final(Byte(b)), _) => {
                    let node = Final(ByteRange(*a, *b));
                    stack.pop().unwrap();
                    stack.pop().unwrap();
                    stack.push(node);
                }
                (NonFinal(OpenClass0), Final(Byte(b)), _) => {
                    let node = NonFinal(OpenClass(true, vec![ClassItem::Byte(*b)]));
                    stack.pop().unwrap();
                    stack.pop().unwrap();
                    stack.push(node);
                }
                (NonFinal(OpenClassNeg), Final(Byte(b)), _) => {
                    let node = NonFinal(OpenClass(false, vec![ClassItem::Byte(*b)]));
                    stack.pop().unwrap();
                    stack.pop().unwrap();
                    stack.push(node);
                }
                (NonFinal(OpenClass(..)), Final(Byte(b)), _) => {
                    let item = ClassItem::Byte(*b);
                    stack.pop().unwrap();
                    let (_, ref mut items) = stack
                        .last_mut()
                        .unwrap()
                        .unwrap_non_final_mut()
                        .unwrap_open_class_mut();
                    items.push(item);
                }
                (NonFinal(OpenClass(..)), Final(ByteRange(a, b)), _) => {
                    let item = ClassItem::ByteRange(*a, *b);
                    stack.pop().unwrap();
                    let (_, ref mut items) = stack
                        .last_mut()
                        .unwrap()
                        .unwrap_non_final_mut()
                        .unwrap_open_class_mut();
                    items.push(item);
                }
                // Group `(ab)`
                (NonFinal(OpenGroup), Final(_), _) => break,
                (Final(Seq(_)), Final(_), _) => {
                    let node = stack.pop().unwrap().unwrap_final();
                    let nodes = stack
                        .last_mut()
                        .unwrap()
                        .unwrap_final_mut()
                        .unwrap_seq_mut();
                    nodes.push(node)
                }
                (Final(_), Final(_), _) => {
                    let node = stack.pop().unwrap().unwrap_final();
                    let prev = stack.pop().unwrap().unwrap_final();
                    stack.push(Final(Seq(vec![prev, node])))
                }
                (NonFinal(Escape), Final(_), _)
                | (NonFinal(HexEscape0), Final(_), _)
                | (NonFinal(HexEscape1(_)), Final(_), _)
                | (NonFinal(RepeatMin(..)), Final(_), _)
                | (NonFinal(RepeatMax(..)), Final(_), _)
                | (NonFinal(OpenClass0), Final(_), _)
                | (NonFinal(OpenClassNeg), Final(_), _)
                | (NonFinal(OpenClass(..)), Final(_), _)
                | (NonFinal(OpenByteRange(_)), Final(_), _)
                | (NonFinal(RepeatToken(..)), Final(_), _) => unreachable!(),
            }
        }
    }
    //println!("stack {:?}", stack);
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
        Err("missing element before `?` symbol".to_string()),
        parse(br"?")
    );
    // TODO(mleonhard) Return "missing element before `?` symbol" error.
    assert_eq!(
        Err("missing element after bar `|`".to_string()),
        parse(br"b|?")
    );
    // TODO(mleonhard) Return "missing element before `{` symbol" error.
    assert_eq!(Err("missing closing `)`".to_string()), parse(br"(?)"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(1))), parse(br".?"));

    // *
    assert_eq!(
        Err("missing element before `*` symbol".to_string()),
        parse(br"*")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".*"));

    // +
    assert_eq!(
        Err("missing element before `+` symbol".to_string()),
        parse(br"+")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, None)), parse(br".+"));

    // {1}
    assert_eq!(
        Err("missing element before `{` symbol".to_string()),
        parse(br"{1}")
    );
    // TODO(mleonhard) Return "missing element before `{` symbol" error.
    assert_eq!(
        Err("missing element after bar `|`".to_string()),
        parse(br"(ab|{1})")
    );
    // TODO(mleonhard) Return "missing element before `{` symbol" error.
    assert_eq!(Err("missing closing `)`".to_string()), parse(br"({1})"));
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
        Err("missing element before `{` symbol".to_string()),
        parse(br"{,}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{,`".to_string()),
        parse(br".{,")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".{,}"));

    // {1,}
    assert_eq!(
        Err("missing element before `{` symbol".to_string()),
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
        Err("missing element before `{` symbol".to_string()),
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
        Err("missing element before `{` symbol".to_string()),
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
