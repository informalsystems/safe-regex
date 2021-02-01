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

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Token {
    EscapedByte(u8),
    Byte(u8),
    QMark,
    Plus,
    Dot,
    Star,
    Caret,
    Dollar,
    Bar,
    OpenRound,
    CloseRound,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
}

fn invalid_escape(bytes: impl AsRef<[u8]>) -> String {
    format!("invalid escape sequence `\\{}`", escape_ascii(bytes))
}

fn tokenize(data: &[u8]) -> Result<Vec<Token>, String> {
    // println!("tokenize {:?} {}", data, escape_ascii(data));
    let mut result = Vec::new();
    let mut iter = data.iter().copied();
    while let Some(b0) = iter.next() {
        // println!("tokenize b0 {:?} {}", b0, escape_ascii([b0]));
        let token = match b0 {
            b'\\' => {
                let b1 = iter.next().ok_or_else(|| invalid_escape([]))?;
                // println!("tokenize b1 {:?} {}", b1, escape_ascii([b1]));
                match b1 {
                    b'x' => {
                        let b2 = iter.next().ok_or_else(|| invalid_escape([b1]))?;
                        // println!("tokenize b2 {:?} {}", b2, escape_ascii([b2]));
                        let b3 = iter.next().ok_or_else(|| invalid_escape([b1, b2]))?;
                        // println!("tokenize b3 {:?} {}", b3, escape_ascii([b3]));
                        if !b2.is_ascii_hexdigit() || !b3.is_ascii_hexdigit() {
                            return Err(invalid_escape([b1, b2, b3]));
                        }
                        let string = String::from_utf8(vec![b2, b3]).unwrap();
                        let byte = u8::from_str_radix(&string, 16).unwrap();
                        Token::EscapedByte(byte)
                    }
                    b'n' => Token::EscapedByte(b'\n'),
                    b'r' => Token::EscapedByte(b'\r'),
                    b't' => Token::EscapedByte(b'\t'),
                    b'\\' => Token::EscapedByte(b'\\'),
                    b'0' => Token::EscapedByte(0),
                    b'\'' => Token::EscapedByte(b'\''),
                    b'"' => Token::EscapedByte(b'"'),
                    b'?' => Token::EscapedByte(b'?'),
                    b'+' => Token::EscapedByte(b'+'),
                    b'.' => Token::EscapedByte(b'.'),
                    b'*' => Token::EscapedByte(b'*'),
                    b'^' => Token::EscapedByte(b'^'),
                    b'$' => Token::EscapedByte(b'$'),
                    b'|' => Token::EscapedByte(b'|'),
                    b'(' => Token::EscapedByte(b'('),
                    b')' => Token::EscapedByte(b')'),
                    b'{' => Token::EscapedByte(b'{'),
                    b'}' => Token::EscapedByte(b'}'),
                    b'[' => Token::EscapedByte(b'['),
                    b']' => Token::EscapedByte(b']'),
                    _ => return Err(invalid_escape([b1])),
                }
            }
            b'?' => Token::QMark,
            b'+' => Token::Plus,
            b'.' => Token::Dot,
            b'*' => Token::Star,
            b'^' => Token::Caret,
            b'$' => Token::Dollar,
            b'|' => Token::Bar,
            b'(' => Token::OpenRound,
            b')' => Token::CloseRound,
            b'{' => Token::OpenCurly,
            b'}' => Token::CloseCurly,
            b'[' => Token::OpenSquare,
            b']' => Token::CloseSquare,
            b => Token::Byte(b),
        };
        // println!("tokenize push {:?}", token);
        result.push(token);
    }
    // println!("tokenize result {:?}", result);
    Ok(result)
}

#[cfg(test)]
#[test]
fn test_tokenize() {
    use Token::{
        Bar, Byte, Caret, CloseCurly, CloseRound, CloseSquare, Dollar, Dot, EscapedByte, OpenCurly,
        OpenRound, OpenSquare, Plus, QMark, Star,
    };
    let empty: Vec<Token> = Vec::new();
    assert_eq!(Ok(empty), tokenize(br""));
    assert_eq!(Ok(vec![Byte(b'a')]), tokenize(br"a"));
    assert_eq!(
        Ok(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')]),
        tokenize(br"abc")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\`".to_string()),
        tokenize(br"\")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\e`".to_string()),
        tokenize(br"\e")
    );
    // Rust byte escapes
    // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
    assert_eq!(
        Err(r"invalid escape sequence `\x`".to_string()),
        tokenize(br"\x")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\x0`".to_string()),
        tokenize(br"\x0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\xg0`".to_string()),
        tokenize(br"\xg0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\x0g`".to_string()),
        tokenize(br"\x0g")
    );
    assert_eq!(Ok(vec![EscapedByte(0)]), tokenize(br"\x00"));
    assert_eq!(Ok(vec![EscapedByte(0x12)]), tokenize(br"\x12"));
    assert_eq!(Ok(vec![EscapedByte(0x34)]), tokenize(br"\x34"));
    assert_eq!(Ok(vec![EscapedByte(0x56)]), tokenize(br"\x56"));
    assert_eq!(Ok(vec![EscapedByte(0x78)]), tokenize(br"\x78"));
    assert_eq!(Ok(vec![EscapedByte(0x90)]), tokenize(br"\x90"));
    assert_eq!(Ok(vec![EscapedByte(0xAB)]), tokenize(br"\xab"));
    assert_eq!(Ok(vec![EscapedByte(0xAB)]), tokenize(br"\xAB"));
    assert_eq!(Ok(vec![EscapedByte(0xCD)]), tokenize(br"\xcd"));
    assert_eq!(Ok(vec![EscapedByte(0xCD)]), tokenize(br"\xCD"));
    assert_eq!(Ok(vec![EscapedByte(0xEF)]), tokenize(br"\xef"));
    assert_eq!(Ok(vec![EscapedByte(0xEF)]), tokenize(br"\xEF"));
    assert_eq!(Ok(vec![EscapedByte(0xFF)]), tokenize(br"\xFF"));
    assert_eq!(
        Ok(vec![Byte(b'a'), EscapedByte(0x00), Byte(b'b')]),
        tokenize(br"a\x00b")
    );
    assert_eq!(
        Ok(vec![
            EscapedByte(b'\n'),
            EscapedByte(b'\r'),
            EscapedByte(b'\t'),
            EscapedByte(b'\\'),
            EscapedByte(0),
        ]),
        tokenize(br"\n\r\t\\\0")
    );
    // Rust quote escapes
    //
    assert_eq!(
        Ok(vec![EscapedByte(b'\''), EscapedByte(b'"'),]),
        tokenize(br#"\'\""#)
    );
    // Regex escapes
    assert_eq!(Ok(vec![EscapedByte(b'?')]), tokenize(br"\?"));
    assert_eq!(Ok(vec![EscapedByte(b'+')]), tokenize(br"\+"));
    assert_eq!(Ok(vec![EscapedByte(b'.')]), tokenize(br"\."));
    assert_eq!(Ok(vec![EscapedByte(b'*')]), tokenize(br"\*"));
    assert_eq!(Ok(vec![EscapedByte(b'^')]), tokenize(br"\^"));
    assert_eq!(Ok(vec![EscapedByte(b'$')]), tokenize(br"\$"));
    assert_eq!(Ok(vec![EscapedByte(b'|')]), tokenize(br"\|"));
    assert_eq!(Ok(vec![EscapedByte(b'(')]), tokenize(br"\("));
    assert_eq!(Ok(vec![EscapedByte(b')')]), tokenize(br"\)"));
    assert_eq!(Ok(vec![EscapedByte(b'{')]), tokenize(br"\{"));
    assert_eq!(Ok(vec![EscapedByte(b'}')]), tokenize(br"\}"));
    assert_eq!(Ok(vec![EscapedByte(b'[')]), tokenize(br"\["));
    assert_eq!(Ok(vec![EscapedByte(b']')]), tokenize(br"\]"));
    // Regex tokens
    assert_eq!(Ok(vec![QMark]), tokenize(br"?"));
    assert_eq!(Ok(vec![Plus]), tokenize(br"+"));
    assert_eq!(Ok(vec![Dot]), tokenize(br"."));
    assert_eq!(Ok(vec![Star]), tokenize(br"*"));
    assert_eq!(Ok(vec![Caret]), tokenize(br"^"));
    assert_eq!(Ok(vec![Dollar]), tokenize(br"$"));
    assert_eq!(Ok(vec![Bar]), tokenize(br"|"));
    assert_eq!(Ok(vec![OpenRound]), tokenize(br"("));
    assert_eq!(Ok(vec![CloseRound]), tokenize(br")"));
    assert_eq!(Ok(vec![OpenCurly]), tokenize(br"{"));
    assert_eq!(Ok(vec![CloseCurly]), tokenize(br"}"));
    assert_eq!(Ok(vec![OpenSquare]), tokenize(br"["));
    assert_eq!(Ok(vec![CloseSquare]), tokenize(br"]"));
}

enum Node {
    Literal(u8),
    Class(Vec<u8>),
    NegativeClass(Vec<u8>),
    AnyByte,
    Group(Box<Node>),
    Or(Vec<Node>),
    Seq(Vec<Node>),
    Repeat(usize, Option<usize>),
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
    let tokens = tokenize(raw_byte_string.as_bytes())?;

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
