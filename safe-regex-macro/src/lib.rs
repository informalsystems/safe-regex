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
    Byte(u8),
    QMark,
    Plus,
    Dot,
    Star,
    Caret,
    Dollar,
    Bar,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

fn tokenize_raw_byte_string(data: &[u8]) -> Result<Vec<Token>, String> {
    println!("tokenize {:?} {}", data, escape_ascii(data));
    let mut result = Vec::new();
    let mut iter = data.iter().map(|b| *b);
    let mut windows = Vec::new();
    let mut item0 = iter.next();
    let mut item1 = iter.next();
    let mut item2 = iter.next();
    let mut item3 = iter.next();
    while item0.is_some() {
        windows.push((item0.unwrap(), item1, item2, item3));
        item0 = item1;
        item1 = item2;
        item2 = item3;
        item3 = iter.next();
    }
    let mut windows_iter = windows.iter().map(|v| *v);
    while let Some(window) = windows_iter.next() {
        println!("tokenize process {:?}", window);
        let token = match window {
            (b'\\', None, _, _) => return Err("incomplete escape sequence `\\`".to_string()),
            (b'\\', Some(b'x'), None, _) => {
                return Err("incomplete escape sequence `\\x`".to_string())
            }
            (b'\\', Some(b'x'), Some(b), None) => {
                return Err(format!(
                    "incomplete escape sequence `\\x{}`",
                    escape_ascii([b])
                ))
            }
            (b'\\', Some(b'x'), Some(digit1), Some(digit0))
                if digit1.is_ascii_hexdigit() && digit0.is_ascii_hexdigit() =>
            {
                windows_iter.next().unwrap();
                windows_iter.next().unwrap();
                windows_iter.next().unwrap();
                let string = String::from_utf8(vec![digit1, digit0]).unwrap();
                let byte = u8::from_str_radix(&string, 16).unwrap();
                Token::Byte(byte)
            }
            (b'\\', Some(b'x'), Some(digit1), Some(digit0)) => {
                return Err(format!(
                    "invalid escape sequence `\\x{}`",
                    escape_ascii([digit1, digit0])
                ))
            }
            (b'\\', Some(b'n'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'\n')
            }
            (b'\\', Some(b'r'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'\r')
            }
            (b'\\', Some(b't'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'\t')
            }
            (b'\\', Some(b'\\'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'\\')
            }
            (b'\\', Some(b'0'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(0)
            }
            (b'\\', Some(b'\''), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'\'')
            }
            (b'\\', Some(b'"'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'"')
            }
            (b'\\', Some(b'?'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'?')
            }
            (b'\\', Some(b'+'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'+')
            }
            (b'\\', Some(b'.'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'.')
            }
            (b'\\', Some(b'*'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'*')
            }
            (b'\\', Some(b'^'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'^')
            }
            (b'\\', Some(b'$'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'$')
            }
            (b'\\', Some(b'|'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'|')
            }
            (b'\\', Some(b'('), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'(')
            }
            (b'\\', Some(b')'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b')')
            }
            (b'\\', Some(b'{'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'{')
            }
            (b'\\', Some(b'}'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'}')
            }
            (b'\\', Some(b'['), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b'[')
            }
            (b'\\', Some(b']'), _, _) => {
                windows_iter.next().unwrap();
                Token::Byte(b']')
            }
            (b'\\', Some(b), _, _) => {
                return Err(format!("invalid escape sequence `\\{}`", escape_ascii([b])))
            }
            (b'?', _, _, _) => Token::QMark,
            (b'+', _, _, _) => Token::Plus,
            (b'.', _, _, _) => Token::Dot,
            (b'*', _, _, _) => Token::Star,
            (b'^', _, _, _) => Token::Caret,
            (b'$', _, _, _) => Token::Dollar,
            (b'|', _, _, _) => Token::Bar,
            (b'(', _, _, _) => Token::OpenParen,
            (b')', _, _, _) => Token::CloseParen,
            (b'{', _, _, _) => Token::OpenBrace,
            (b'}', _, _, _) => Token::CloseBrace,
            (b'[', _, _, _) => Token::OpenBracket,
            (b']', _, _, _) => Token::CloseBracket,
            (b, _, _, _) => Token::Byte(b),
        };
        println!("tokenize push {:?}", token);
        result.push(token);
    }
    println!("tokenize result {:?}", result);
    Ok(result)
}

#[cfg(test)]
#[test]
fn test_tokenize() {
    use Token::{
        Bar, Byte, Caret, CloseBrace, CloseBracket, CloseParen, Dollar, Dot, OpenBrace,
        OpenBracket, OpenParen, Plus, QMark, Star,
    };
    let empty: Vec<Token> = Vec::new();
    assert_eq!(Ok(empty), tokenize_raw_byte_string(br""));
    assert_eq!(Ok(vec![Byte(b'a')]), tokenize_raw_byte_string(br"a"));
    assert_eq!(
        Ok(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')]),
        tokenize_raw_byte_string(br"abc")
    );
    assert_eq!(
        Err(r"incomplete escape sequence `\`".to_string()),
        tokenize_raw_byte_string(br"\")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\e`".to_string()),
        tokenize_raw_byte_string(br"\e")
    );
    // Rust byte escapes
    // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
    assert_eq!(
        Err(r"incomplete escape sequence `\x`".to_string()),
        tokenize_raw_byte_string(br"\x")
    );
    assert_eq!(
        Err(r"incomplete escape sequence `\x0`".to_string()),
        tokenize_raw_byte_string(br"\x0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\xg0`".to_string()),
        tokenize_raw_byte_string(br"\xg0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\x0g`".to_string()),
        tokenize_raw_byte_string(br"\x0g")
    );
    assert_eq!(Ok(vec![Byte(0)]), tokenize_raw_byte_string(br"\x00"));
    assert_eq!(Ok(vec![Byte(0x12)]), tokenize_raw_byte_string(br"\x12"));
    assert_eq!(Ok(vec![Byte(0x34)]), tokenize_raw_byte_string(br"\x34"));
    assert_eq!(Ok(vec![Byte(0x56)]), tokenize_raw_byte_string(br"\x56"));
    assert_eq!(Ok(vec![Byte(0x78)]), tokenize_raw_byte_string(br"\x78"));
    assert_eq!(Ok(vec![Byte(0x90)]), tokenize_raw_byte_string(br"\x90"));
    assert_eq!(Ok(vec![Byte(0xAB)]), tokenize_raw_byte_string(br"\xab"));
    assert_eq!(Ok(vec![Byte(0xAB)]), tokenize_raw_byte_string(br"\xAB"));
    assert_eq!(Ok(vec![Byte(0xCD)]), tokenize_raw_byte_string(br"\xcd"));
    assert_eq!(Ok(vec![Byte(0xCD)]), tokenize_raw_byte_string(br"\xCD"));
    assert_eq!(Ok(vec![Byte(0xEF)]), tokenize_raw_byte_string(br"\xef"));
    assert_eq!(Ok(vec![Byte(0xEF)]), tokenize_raw_byte_string(br"\xEF"));
    assert_eq!(Ok(vec![Byte(0xFF)]), tokenize_raw_byte_string(br"\xFF"));
    assert_eq!(
        Ok(vec![Byte(b'a'), Byte(0x00), Byte(b'b')]),
        tokenize_raw_byte_string(br"a\x00b")
    );
    assert_eq!(
        Ok(vec![
            Byte(b'\n'),
            Byte(b'\r'),
            Byte(b'\t'),
            Byte(b'\\'),
            Byte(0),
        ]),
        tokenize_raw_byte_string(br"\n\r\t\\\0")
    );
    // Rust quote escapes
    //
    assert_eq!(
        Ok(vec![Byte(b'\''), Byte(b'"'),]),
        tokenize_raw_byte_string(br#"\'\""#)
    );
    // Regex escapes
    assert_eq!(Ok(vec![Byte(b'?')]), tokenize_raw_byte_string(br"\?"));
    assert_eq!(Ok(vec![Byte(b'+')]), tokenize_raw_byte_string(br"\+"));
    assert_eq!(Ok(vec![Byte(b'.')]), tokenize_raw_byte_string(br"\."));
    assert_eq!(Ok(vec![Byte(b'*')]), tokenize_raw_byte_string(br"\*"));
    assert_eq!(Ok(vec![Byte(b'^')]), tokenize_raw_byte_string(br"\^"));
    assert_eq!(Ok(vec![Byte(b'$')]), tokenize_raw_byte_string(br"\$"));
    assert_eq!(Ok(vec![Byte(b'|')]), tokenize_raw_byte_string(br"\|"));
    assert_eq!(Ok(vec![Byte(b'(')]), tokenize_raw_byte_string(br"\("));
    assert_eq!(Ok(vec![Byte(b')')]), tokenize_raw_byte_string(br"\)"));
    assert_eq!(Ok(vec![Byte(b'{')]), tokenize_raw_byte_string(br"\{"));
    assert_eq!(Ok(vec![Byte(b'}')]), tokenize_raw_byte_string(br"\}"));
    assert_eq!(Ok(vec![Byte(b'[')]), tokenize_raw_byte_string(br"\["));
    assert_eq!(Ok(vec![Byte(b']')]), tokenize_raw_byte_string(br"\]"));
    // Regex tokens
    assert_eq!(Ok(vec![QMark]), tokenize_raw_byte_string(br"?"));
    assert_eq!(Ok(vec![Plus]), tokenize_raw_byte_string(br"+"));
    assert_eq!(Ok(vec![Dot]), tokenize_raw_byte_string(br"."));
    assert_eq!(Ok(vec![Star]), tokenize_raw_byte_string(br"*"));
    assert_eq!(Ok(vec![Caret]), tokenize_raw_byte_string(br"^"));
    assert_eq!(Ok(vec![Dollar]), tokenize_raw_byte_string(br"$"));
    assert_eq!(Ok(vec![Bar]), tokenize_raw_byte_string(br"|"));
    assert_eq!(Ok(vec![OpenParen]), tokenize_raw_byte_string(br"("));
    assert_eq!(Ok(vec![CloseParen]), tokenize_raw_byte_string(br")"));
    assert_eq!(Ok(vec![OpenBrace]), tokenize_raw_byte_string(br"{"));
    assert_eq!(Ok(vec![CloseBrace]), tokenize_raw_byte_string(br"}"));
    assert_eq!(Ok(vec![OpenBracket]), tokenize_raw_byte_string(br"["));
    assert_eq!(Ok(vec![CloseBracket]), tokenize_raw_byte_string(br"]"));
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
    let tokens = tokenize_raw_byte_string(raw_byte_string.as_bytes())?;

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
