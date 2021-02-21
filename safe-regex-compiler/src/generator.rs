//! Provides a [`generate`](fn.generate.html) function used by the `regex!`
//! proc macro.
//!
//! How-to develop proc macros: <https://github.com/dtolnay/proc-macro-workshop>

// // Each variant is 8 bytes.
// enum E {
//     A([Range<usize>; 0]),
//     B([Range<usize>; 0]),
//     C([Range<usize>; 0]),
//     D([Range<usize>; 0]),
// }
// // Each variant is 24 bytes.
// enum E {
//     A([Range<usize>; 1]),
//     B([Range<usize>; 1]),
//     C([Range<usize>; 1]),
//     D([Range<usize>; 1]),
// }
// Each variant is 40 bytes.
// enum E {
//     A([Range<usize>; 2]),
//     B([Range<usize>; 2]),
//     C([Range<usize>; 2]),
//     D([Range<usize>; 2]),
// }
// println!(
//     "size is {} bytes",
//     std::mem::size_of_val(&E::A([0..0, 0..0]))
// );

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
    enclosing_group_num: usize,
    mut names: &mut Vec<Ident>,
    clauses: &mut Vec<TokenStream>,
    next_state_stmt: &TokenStream,
    node: &FinalNode,
) -> Ident {
    match node {
        FinalNode::Byte(b) => {
            let name = make_name(&mut names, "Byte");
            clauses.push(quote! {
                (Self::#name(ranges), Some(#b)) => {
                    let mut ranges_clone = ranges.clone();
                    ranges_clone[#enclosing_group_num].end = n + 1;
                    #next_state_stmt
                }
                (Self::#name(_), Some(_)) => {}
            });
            name
        }
        FinalNode::AnyByte => {
            let name = make_name(&mut names, "AnyByte");
            clauses.push(quote! {
                (Self::#name(ranges), Some(_)) => {
                    let mut ranges_clone = ranges.clone();
                    ranges_clone[#enclosing_group_num].end = n + 1;
                    #next_state_stmt
                }
            });
            name
        }
        FinalNode::Class(_incl, _items) => {
            unimplemented!()
        }
        FinalNode::Or(_nodes) => {
            unimplemented!()
        }
        FinalNode::Seq(_nodes) => {
            unimplemented!()
        }
        FinalNode::Repeat(_node, _, _) => {
            unimplemented!()
        }
        FinalNode::Group(node) => {
            let name = make_name(&mut names, "Group");
            let matched_name = make_name(&mut names, "GroupMatched");
            let group_next_stmt = quote! {
                Self::#matched_name(ranges_clone).make_next_states(None, n, next_states)
            };
            let group_number = enclosing_group_num + 1;
            let child_name = build(group_number, names, clauses, &group_next_stmt, node);
            clauses.push(quote! {
                (Self::#name(ranges), Some(b)) => {
                    let mut ranges_clone = ranges.clone();
                    ranges_clone[#group_number] = n..n;
                    Self::#child_name(ranges_clone).make_next_states(Some(b), n, next_states);
                }
                (Self::#matched_name(ranges), None) => {
                    let mut ranges_clone = ranges.clone();
                    ranges_clone[#enclosing_group_num].end = ranges_clone[#group_number].end;
                    #next_state_stmt
                }
            });
            name
        }
    }
}

/// Generates an enum that implements `parsed_re` and implements the
/// [`safe_regex::internal::Machine`](https://docs.rs/safe-regex/latest/safe_regex/internal/trait.Machine.html)
/// trait.
pub fn generate(literal_re: String, parsed_re: FinalNode) -> safe_proc_macro2::TokenStream {
    let num_groups = count_groups(&parsed_re) + 1;
    let state_type = quote!([core::ops::Range<u32>; #num_groups]);
    let mut names: Vec<Ident> = Vec::new();
    let mut clauses: Vec<TokenStream> = Vec::new();
    // Perform a depth-first walk of the AST and make trait names and clauses.
    let initial_state_name = build(
        0,
        &mut names,
        &mut clauses,
        &quote! { next_states.insert(Self::Accept(ranges_clone)); },
        &parsed_re,
    );
    let variants = names.iter().map(|name| quote!(#name(#state_type)));
    let default_ranges = core::iter::once(quote!(0..0))
        .chain(core::iter::repeat(quote!(u32::MAX..u32::MAX)))
        .take(num_groups);
    let result = quote! {
    {
        #[doc = #literal_re]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            #( #variants ),* ,
            Accept(#state_type),
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type State = #state_type;
            fn start() -> Self { Self::#initial_state_name([#(#default_ranges),*]) }
            fn accept(&self) -> Option<Self::State> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone()),
                    _ => None,
                }
            }
            fn make_next_states(
                &self,
                opt_b: Option<u8>,
                n: u32,
                next_states: &mut std::collections::HashSet<
                    Self,
                    std::collections::hash_map::RandomState,
                >,
            ) {
                safe_regex::internal::println_make_next_states(&opt_b, &n, &self);
                match (self, opt_b) {
                    #(#clauses)*
                    (Self::Accept(_), _) => {}
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
