//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-compiler.svg)](https://crates.io/crates/safe-regex-compiler)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! A regular expression compiler.
//!
//! If you want to use regular expressions in your software, use the
//! [`safe_regex`](https://crates.io/crates/safe-regex) crate.
//!
//! # Cargo Geiger Safety Report
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about parsing
//! - DONE - Implement `parser`
//! - DONE - Add tests for `parser`
//! - Implement `macro_generator`
//! - Add tests for `macro_generator`
//! - Add unwrap functions for other `FinalNode` variants
//! - Add fuzzing tests
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]
use proc_macro2::TokenStream;

pub mod macro_generator;
pub mod parser;

/// Converts the bytes into an ASCII string.
pub fn escape_ascii(input: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for byte in input.as_ref() {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

pub fn impl_regex(stream: TokenStream) -> Result<TokenStream, String> {
    // Literal { kind: ByteStrRaw(0), symbol: "abc\\x20", suffix: None, span: #0 bytes(741..752) }
    // regex!(br"abc\x20");
    let mut tokens = stream.clone().into_iter();
    let literal = tokens
        .next()
        .ok_or("expected literal byte string".to_string())?;
    // if tokens.next().is_some() {
    //     return Err("expected one literal byte string".to_string());
    // }
    // // The compiler already parsed the literal, but does not expose the fields
    // // that it shows in Debug formatting.
    // // So we convert the literal to a string and parse it ourselves.
    // // https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-rust-proc-macro
    // let literal_string = literal.to_string();
    // let raw_byte_string = literal_string
    //     .strip_prefix("br")
    //     .ok_or("expected a raw byte string, like br\"abc\"".to_string())?
    //     // Compiler guarantees that strings are closed.
    //     .trim_start_matches('#')
    //     .trim_start_matches('"')
    //     .trim_end_matches('#')
    //     .trim_end_matches('"');
    // // The compiler guarantees that a literal byte string contains only ASCII.
    // // > regex!(br"€"); // error: raw byte string must be ASCII
    // // Therefore, we can slice the string at any byte offset.
    // let ast = crate::parser::parse(raw_byte_string.as_bytes())?;
    //
    // // panic!("literal: {:?} str={:?}", literal, literal.to_string());
    // // if let Some(tree) = attr.into_iter().next() {
    // //     return quote_spanned!(tree.span()=>compile_error!("parameters not allowed"););
    // // }
    // Ok(stream)
    Ok(TokenStream::new())

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
