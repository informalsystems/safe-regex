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
//! - DONE - Implement parser
//! - DONE - Add tests for parser
//! - DONE - Implement macro generator
//! - DONE - Add tests for macro generator
//! - Add fuzzing tests
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `../release.sh`
#![forbid(unsafe_code)]
use crate::generator::generate;
use safe_proc_macro2::{TokenStream, TokenTree};

pub mod generator;
pub mod parser;

#[macro_export]
macro_rules! dprintln {
    // ($($args:tt)+) => { println!( $($args)+ ) };
    ($($args:tt)+) => {};
}

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

/// Implements the `regex!` macro.
///
/// # Errors
/// Returns `Err(String)` with a human-readable description of the problem.
pub fn impl_regex(stream: TokenStream) -> Result<TokenStream, String> {
    // Ident { sym: regex }
    // Punct { char: '!', spacing: Alone }
    // Group {
    //   delimiter: Parenthesis,
    //   stream: TokenStream [
    //     Ident { sym: enum },
    //     Ident { sym: Re },
    //     Punct { char: '=', spacing: Alone },
    //     Literal { lit: br"a" }
    //   ]
    // }
    const ERR: &str = "expected a raw byte string, like br\"abc\"";
    dprintln!(
        "impl_regex {:?}",
        stream
            .clone()
            .into_iter()
            .map(|tree| format!("{:?} ", tree))
            .collect::<String>()
    );
    let mut stream_iter = stream.into_iter();
    let literal = match stream_iter.next() {
        Some(TokenTree::Literal(literal)) => literal,
        _ => return Err(ERR.to_string()),
    };
    if stream_iter.next().is_some() {
        return Err(ERR.to_string());
    }

    // The compiler already parsed the literal, but does not expose its fields.
    // So we convert the literal to a string and parse it ourselves.
    // https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-rust-proc-macro
    let literal_string = literal.to_string();
    let raw_byte_string = literal_string
        .strip_prefix("br")
        .ok_or_else(|| ERR.to_string())?
        // Compiler guarantees that strings are closed.
        .trim_start_matches('#')
        .trim_start_matches('"')
        .trim_end_matches('#')
        .trim_end_matches('"');
    // The compiler guarantees that a literal byte string contains only ASCII.
    // > regex!(br"€"); // error: raw byte string must be ASCII
    // Therefore, we can slice the string at any byte offset.
    let parsed_re = crate::parser::parse(raw_byte_string.as_bytes())?;

    // panic!("literal: {:?} str={:?}", literal, literal.to_string());
    // if let Some(tree) = attr.into_iter().next() {
    //     return quote_spanned!(tree.span()=>compile_error!("parameters not allowed"););
    // }
    Ok(generate(&literal_string, &parsed_re))
}
