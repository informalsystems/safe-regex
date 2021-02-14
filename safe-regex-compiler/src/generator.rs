//! Provides a [`generate`](fn.generate.html) function used by the `regex!`
//! proc macro.
//!
//! How-to develop proc macros: <https://github.com/dtolnay/proc-macro-workshop>
#![forbid(unsafe_code)]
use crate::parser::FinalNode;
use safe_proc_macro2::{Ident, TokenStream};
use safe_quote::{format_ident, quote};

fn count_groups(node: &FinalNode) -> usize {
    match node {
        FinalNode::Byte(_) => 0,
        FinalNode::AnyByte => 0,
        FinalNode::Class(_, _) => 0,
        FinalNode::Or(nodes) | FinalNode::Seq(nodes) => {
            nodes.iter().map(|node| count_groups(node)).sum()
        }
        FinalNode::Repeat(node, _, _) => count_groups(node),
        FinalNode::Group(node) => 1 + count_groups(node),
    }
}

fn make_name(names: &mut Vec<Ident>, prefix: &'static str) -> Ident {
    let name = format_ident!("{}{}", prefix, names.len());
    names.push(name.clone());
    name
}

fn build(
    mut names: &mut Vec<Ident>,
    clauses: &mut Vec<TokenStream>,
    next_state: &Ident,
    node: &FinalNode,
) {
    match node {
        FinalNode::Byte(b) => {
            let name = make_name(&mut names, "Byte");
            clauses.push(quote! {
            (Self::#name, Some(#b)) => { next_states.insert(Self::#next_state); }
            });
            clauses.push(quote! { (Self::#name, Some(_)) => {} });
        }
        FinalNode::AnyByte => {}
        FinalNode::Class(incl, items) => {}
        FinalNode::Or(nodes) => {
            for node in nodes {
                build(names, clauses, next_state, node);
            }
        }
        FinalNode::Seq(nodes) => {
            for node in nodes {
                build(names, clauses, next_state, node);
            }
        }
        FinalNode::Repeat(node, _, _) => {}
        FinalNode::Group(node) => {
            build(names, clauses, next_state, node);
        }
    }
}

/// Generates an enum that implements `parsed_re` and implements the
/// [`safe_regex::internal::Machine`](https://docs.rs/safe-regex/latest/safe_regex/internal/trait.Machine.html)
/// trait.
pub fn generate(literal_re: String, parsed_re: FinalNode) -> safe_proc_macro2::TokenStream {
    let num_groups = count_groups(&parsed_re);
    let state_type = quote!([core::ops::Range<u32>; #num_groups]);
    let mut names: Vec<Ident> = Vec::new();
    let mut clauses: Vec<TokenStream> = Vec::new();
    // Perform a depth-first walk of the AST and make trait names and clauses.
    build(
        &mut names,
        &mut clauses,
        &format_ident!("Accept"),
        &parsed_re,
    );

    let first_variant = format_ident!("{}", names.first().unwrap());
    let variants = names.iter().map(|name| {
        if num_groups == 0 {
            quote!(#name)
        } else {
            quote!(#name(#state_type))
        }
    });

    let default_ranges = core::iter::repeat(quote!(u32::MAX..u32::MAX)).take(num_groups);
    let initial_state = if num_groups == 0 {
        quote! { Self::#first_variant }
    } else {
        // Self::Group0([u32::MAX..u32::MAX, u32::MAX..u32::MAX, ...])
        quote! { Self::#first_variant([#(#default_ranges),*]) }
    };
    let accept_clause = if num_groups == 0 {
        quote! { Self::Accept => Some([]) }
    } else {
        quote! { Self::Accept(ranges) => Some(ranges.clone()) }
    };
    let result = quote! {
    {
        #[doc = #literal_re]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            #( #variants ),* ,
            Accept,
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = #state_type;
            fn start() -> Self { #initial_state }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    #accept_clause,
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<Self, std::collections::hash_map::RandomState>,
            ) {
                println!(
                    "make_next_states {} {} {:?}",
                    opt_b.map_or(
                        String::from("None"),
                        |b| format!("Some({})", safe_regex::internal::escape_ascii(&[b]))),
                    n,
                    self,
                );
                match (self, opt_b) {
                    #(#clauses)*
                    (Self::Accept, _) => {}
                    other => panic!("invalid state transition {:?}", other),
                }
            }
        }

        <safe_regex::Matcher<CompiledRegex_>>::new()
    }
    };
    println!("result={}", result);
    result
}
