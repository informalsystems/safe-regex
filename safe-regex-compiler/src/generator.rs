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
use crate::parser::{ClassItem, FinalNode};
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

fn make_variant_and_fn_names(
    variant_names: &mut Vec<Ident>,
    prefix: &'static str,
) -> (Ident, Ident) {
    let variant_name = format_ident!("{}{}", prefix, variant_names.len());
    let fn_name = format_ident!("{}{}", prefix.to_ascii_lowercase(), variant_names.len());
    variant_names.push(variant_name.clone());
    (variant_name, fn_name)
}

fn build(
    enclosing_group_num: Option<usize>,
    mut variant_names: &mut Vec<Ident>,
    functions: &mut Vec<TokenStream>,
    next_fn_name: &Ident,
    node: &FinalNode,
) -> Ident {
    println!("build {:?}", node);
    let result = match node {
        FinalNode::Byte(b) => {
            let (variant_name, fn_name) = make_variant_and_fn_names(&mut variant_names, "Byte");
            let format_string = format!("{} {}", fn_name, "opt_b={:?} n={} ranges={:?}");
            let clone_ranges_and_skip_past_n = if let Some(group_num) = enclosing_group_num {
                quote! { &ranges.clone().skip_past(#group_num, n) }
            } else {
                quote! { &ranges.clone() }
            };
            functions.push(quote! {
                fn #fn_name(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                    println!(#format_string, opt_b, n, ranges);
                    match opt_b {
                        Some(#b) => Self::#next_fn_name(
                            #clone_ranges_and_skip_past_n,
                            None,
                            n + 1,
                            next_states,
                        ),
                        Some(_) => {}
                        None => {
                            next_states.insert(Self::#variant_name(ranges.clone()));
                        }
                    }
                }
            });
            fn_name
        }
        FinalNode::AnyByte => {
            let (variant_name, fn_name) = make_variant_and_fn_names(&mut variant_names, "AnyByte");
            let format_string = format!("{} {}", fn_name, "opt_b={:?} n={} ranges={:?}");
            let clone_ranges_and_skip_past_n = if let Some(group_num) = enclosing_group_num {
                quote! { &ranges.clone().skip_past(#group_num, n) }
            } else {
                quote! { &ranges.clone() }
            };
            functions.push(quote! {
                fn #fn_name(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                    println!(#format_string, opt_b, n, ranges);
                    match opt_b {
                        Some(_) => Self::#next_fn_name(
                            #clone_ranges_and_skip_past_n,
                            None,
                            n + 1,
                            next_states,
                        ),
                        None => {
                            next_states.insert(Self::#variant_name(ranges.clone()));
                        }
                    }
                }
            });
            fn_name
        }
        FinalNode::Class(incl, items) => {
            let (variant_name, fn_name) = make_variant_and_fn_names(&mut variant_names, "Class");
            let format_string = format!("{} {}", fn_name, "opt_b={:?} n={} ranges={:?}");
            let comparisons = items.iter().map(|item| match (incl, item) {
                (true, ClassItem::Byte(b)) => quote! {b == #b},
                (false, ClassItem::Byte(b)) => quote! {b != #b},
                (true, ClassItem::ByteRange(x, y)) => quote! {(#x ..= #y).contains(&b)},
                (false, ClassItem::ByteRange(x, y)) => quote! {!(#x ..= #y).contains(&b)},
            });
            let comparison_expr = if *incl {
                quote! { #( #comparisons )||* }
            } else {
                quote! { #( #comparisons )&&* }
            };

            let clone_ranges_and_skip_past_n = if let Some(group_num) = enclosing_group_num {
                quote! { &ranges.clone().skip_past(#group_num, n) }
            } else {
                quote! { &ranges.clone() }
            };
            functions.push(quote! {
                fn #fn_name(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                    println!(#format_string, opt_b, n, ranges);
                    match opt_b {
                        Some(b) if #comparison_expr => {
                            Self::#next_fn_name(
                                #clone_ranges_and_skip_past_n,
                                None,
                                n + 1,
                                next_states,
                            )
                        }
                        Some(_) => {}
                        None => {
                            next_states.insert(Self::#variant_name(ranges.clone()));
                        }
                    }
                }
            });
            fn_name
        }
        //         FinalNode::Seq(nodes) => {
        //             if nodes.is_empty() {
        //                 panic!("unimplemented {:?}", node)
        //             }
        //             let mut child_name = format_ident!("unreachable");
        //             let mut seq_next_stmt = next_state_stmt.clone();
        //             for node in nodes.iter().rev() {
        //                 child_name = build(enclosing_group_num, names, clauses, &seq_next_stmt, node);
        //                 seq_next_stmt = quote! {
        //                     Self::#child_name(ranges_clone).make_next_states(None, n, next_states)
        //                 };
        //             }
        //             child_name
        //         }
        //         FinalNode::Or(_nodes) => {
        //             panic!("unimplemented {:?}", node)
        //         }
        //         FinalNode::Repeat(_node, _, _) => {
        //             panic!("unimplemented {:?}", node)
        //         }
        //         FinalNode::Group(node) => {
        //             let name = make_name(&mut names, "Group");
        //             let matched_name = make_name(&mut names, "GroupMatched");
        //             let group_next_stmt = quote! {
        //                 Self::#matched_name(ranges_clone).make_next_states(None, n, next_states)
        //             };
        //             let group_number = enclosing_group_num + 1;
        //             let child_name = build(group_number, names, clauses, &group_next_stmt, node);
        //             clauses.push(quote! {
        //                 (Self::#name(ranges), Some(b)) => {
        //                     let mut ranges_clone = ranges.clone();
        //                     ranges_clone[#group_number] = n..n;
        //                     Self::#child_name(ranges_clone).make_next_states(Some(b), n, next_states);
        //                 }
        //                 (Self::#matched_name(ranges), None) => {
        //                     let mut ranges_clone = ranges.clone();
        //                     ranges_clone[#enclosing_group_num].end = ranges_clone[#group_number].end;
        //                     #next_state_stmt
        //                 }
        //             });
        //             name
        //         }
        other => panic!("unimplemented {:?}", other),
    };
    println!("build returning {:?}", result);
    result
}

/// Generates an enum that implements `parsed_re` and implements the
/// [`safe_regex::internal::Machine`](https://docs.rs/safe-regex/latest/safe_regex/internal/trait.Machine.html)
/// trait.
pub fn generate(literal_re: String, parsed_re: FinalNode) -> safe_proc_macro2::TokenStream {
    let num_groups: usize = count_groups(&parsed_re);
    let ranges_inner = quote!([core::ops::Range<u32>; #num_groups]);
    let ranges_struct = if num_groups == 0 {
        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            struct Ranges_;
            impl Ranges_ {
                pub fn new() -> Self {
                    Self
                }
                pub fn skip_past(self, _group: usize, _n: u32) -> Self {
                    self
                }
                pub fn into_inner(self) -> [core::ops::Range<u32>; 0usize] {
                    []
                }
            }
        }
    } else {
        let default_ranges = core::iter::repeat(quote!(u32::MAX..u32::MAX)).take(num_groups);
        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            struct Ranges_(#ranges_inner);
            impl Ranges_ {
                pub fn new() -> Self {
                    Self([ #( #default_ranges ),* ])
                }
                pub fn enter(mut self, group: usize, n: u32) -> Self {
                    self.0[group].start = n;
                    self.0[group].end = n;
                    self
                }
                pub fn skip_past(mut self, group: usize, n: u32) -> Self {
                    self.0[group].end = n + 1;
                    self
                }
                pub fn inner(&self) -> &#ranges_inner {
                    &self.0
                }
            }
        }
    };
    let mut variant_names: Vec<Ident> = Vec::new();
    let mut functions: Vec<TokenStream> = Vec::new();
    // Perform a depth-first walk of the AST and make trait names and clauses.
    let initial_fn_name = build(
        None,
        &mut variant_names,
        &mut functions,
        &format_ident!("accept"),
        &parsed_re,
    );
    let clauses: Vec<TokenStream> = variant_names
        .iter()
        .map(|ident| {
            (
                ident,
                format_ident!("{}", ident.to_string().to_ascii_lowercase()),
            )
        })
        .map(|(variant_name, fn_name)| {
            quote! {
                Self::#variant_name(ranges) => Self::#fn_name(ranges, Some(b), n, next_states)
            }
        })
        .collect();
    let result = quote! { {
        #ranges_struct
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = #literal_re]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            #( #variant_names(Ranges_) ),* ,
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            #( #functions )*
            fn accept(ranges: &Ranges_, opt_b: Option<u8>, n: u32, next_states: &mut States_) {
                println!("accept opt_b={:?} n={} ranges={:?}", opt_b, n, ranges);
                match opt_b {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = #ranges_inner;
            fn start(next_states: &mut States_) {
                Self::#initial_fn_name(&Ranges_::new(), None, 0, next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.clone().into_inner()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                println!("make_next_states b={:?} n={} {:?}", b, n, self);
                match self {
                    #( #clauses ),* ,
                    Self::Accept(ranges) => Self::accept(ranges, Some(b), n, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    println!("result={}", result);
    result
}
