//! Provides a [`generate`](fn.generate.html) function used by the `regex!`
//! proc macro.
//!
//! How-to develop proc macros: <https://github.com/dtolnay/proc-macro-workshop>
#![forbid(unsafe_code)]
use crate::parser::FinalNode;
use proc_macro2::TokenStream;

/// Generates an enum that implements `parsed_re` and implements the
/// [`safe_regex::Regex`](https://docs.rs/safe-regex/latest/safe_regex/trait.Regex.html)
/// trait.
pub fn generate(parsed_re: FinalNode) -> proc_macro2::TokenStream {
    // eprintln!(
    //     "regex({}) -> {}",
    //     input2,
    //     output2
    //         .clone()
    //         .into_iter()
    //         .map(|tree| format!("tree: {:?}\n", tree))
    //         .collect::<String>()
    // );

    TokenStream::new()
}
