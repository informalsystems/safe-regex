//! Provides a [`generate`](fn.generate.html) function used by the `regex!`
//! proc macro.
//!
//! How-to develop proc macros: <https://github.com/dtolnay/proc-macro-workshop>
#![forbid(unsafe_code)]
use crate::parser::{ClassItem, FinalNode};
use safe_proc_macro2::{Ident, TokenStream};
use safe_quote::{format_ident, quote};

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Predicate {
    Any,
    Incl(Vec<ClassItem>),
    Excl(Vec<ClassItem>),
}
impl core::fmt::Debug for Predicate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Predicate::Any => write!(f, "Empty"),
            Predicate::Incl(items) => write!(f, "Incl{:?}", items),
            Predicate::Excl(items) => write!(f, "Excl{:?}", items),
        }
    }
}

// TODO(mleonhard) Add more tree simplifications:
// - Collapse nested Seq into one
// - Collapse nested Alt into one
// - Merge peer Bytes in Alt
// - Deduplicate Empty in Alt
// - Drop Optional(x) that comes right after Star(x)
// - Reorder Optional(x),x so the optional comes later
// - Translate x{2,5} into "xx(x(x(x)?)?)?" rather than "xxx?x?x?"
#[derive(Clone, PartialOrd, PartialEq)]
enum OptimizedNode {
    Byte(Predicate),
    Seq(Vec<OptimizedNode>),
    Alt(Vec<OptimizedNode>),
    Optional(Box<OptimizedNode>),
    Star(Box<OptimizedNode>),
    Group(Box<OptimizedNode>),
}
impl OptimizedNode {
    pub fn from_final_node(final_node: &FinalNode) -> Option<Self> {
        match final_node {
            FinalNode::AnyByte => Some(OptimizedNode::Byte(Predicate::Any)),
            FinalNode::Byte(b) => {
                Some(OptimizedNode::Byte(Predicate::Incl(vec![ClassItem::Byte(
                    *b,
                )])))
            }
            FinalNode::Class(true, items) => {
                Some(OptimizedNode::Byte(Predicate::Incl(items.clone())))
            }
            FinalNode::Class(false, items) => {
                Some(OptimizedNode::Byte(Predicate::Excl(items.clone())))
            }
            FinalNode::Seq(final_nodes) => {
                let mut nodes: Vec<OptimizedNode> = final_nodes
                    .iter()
                    .filter_map(OptimizedNode::from_final_node)
                    .collect();
                if nodes.is_empty() {
                    None
                } else if nodes.len() == 1 {
                    Some(nodes.pop().unwrap())
                } else {
                    Some(OptimizedNode::Seq(nodes))
                }
            }
            FinalNode::Alt(final_nodes) => {
                let mut nodes: Vec<OptimizedNode> = final_nodes
                    .iter()
                    .filter_map(OptimizedNode::from_final_node)
                    .collect();
                if nodes.is_empty() {
                    None
                } else if nodes.len() == 1 {
                    Some(nodes.pop().unwrap())
                } else {
                    Some(OptimizedNode::Alt(nodes))
                }
            }
            FinalNode::Repeat(inner_final_node, 0, None) => Some(OptimizedNode::Star(Box::new(
                OptimizedNode::from_final_node(inner_final_node)?,
            ))),
            FinalNode::Repeat(inner_final_node, min, None) => {
                let node = OptimizedNode::from_final_node(inner_final_node)?;
                let mut nodes = Vec::with_capacity(min + 1);
                nodes.extend(core::iter::repeat(node.clone()).take(*min));
                nodes.push(OptimizedNode::Star(Box::new(node)));
                Some(OptimizedNode::Seq(nodes))
            }
            FinalNode::Repeat(_node, 0, Some(0)) => None,
            FinalNode::Repeat(node, 1, Some(1)) => OptimizedNode::from_final_node(node),
            FinalNode::Repeat(_node, min, Some(max)) if max < min => unreachable!(),
            FinalNode::Repeat(inner_final_node, min, Some(max)) => {
                let node = OptimizedNode::from_final_node(inner_final_node)?;
                let mut nodes = Vec::with_capacity(*max);
                nodes.extend(core::iter::repeat(node.clone()).take(*min));
                nodes.extend(
                    core::iter::repeat(OptimizedNode::Optional(Box::new(node))).take(max - min),
                );
                Some(OptimizedNode::Seq(nodes))
            }
            FinalNode::Group(inner_final_node) => Some(OptimizedNode::Group(Box::new(
                OptimizedNode::from_final_node(inner_final_node).expect("found empty group"),
            ))),
        }
    }
}
impl core::fmt::Debug for OptimizedNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            OptimizedNode::Byte(items) => write!(f, "OptimizedNode::Byte({:?})", items),
            OptimizedNode::Seq(nodes) => write!(f, "OptimizedNode::Seq{:?}", nodes),
            OptimizedNode::Alt(nodes) => write!(f, "OptimizedNode::Alt{:?}", nodes),
            OptimizedNode::Optional(node) => write!(f, "OptimizedNode::Optional({:?})", node),
            OptimizedNode::Star(node) => write!(f, "OptimizedNode::Star({:?})", node),
            OptimizedNode::Group(node) => write!(f, "OptimizedNode::Group({:?})", node),
        }
    }
}

struct Counter {
    n: usize,
}
impl Counter {
    pub fn new() -> Self {
        Self { n: 0 }
    }
    pub fn get(&self) -> usize {
        self.n
    }
    pub fn get_and_increment(&mut self) -> usize {
        let result = self.n;
        self.n += 1;
        result
    }
}
#[cfg(test)]
#[test]
fn test_counter() {
    let mut counter = Counter::new();
    assert_eq!(0, counter.get());
    assert_eq!(0, counter.get_and_increment());
    assert_eq!(1, counter.get());
    assert_eq!(1, counter.get_and_increment());
    assert_eq!(2, counter.get());
    assert_eq!(2, counter.get_and_increment());
    assert_eq!(3, counter.get());
}

#[derive(Clone, PartialOrd, PartialEq)]
enum TaggedNode {
    Byte(usize, Predicate),
    Seq(Vec<TaggedNode>),
    Alt(Vec<TaggedNode>),
    Optional(usize, Box<TaggedNode>),
    Star(Box<TaggedNode>),
    Group(usize, Box<TaggedNode>),
}
impl TaggedNode {
    pub fn from_optimized(
        var_counter: &mut Counter,
        group_counter: &mut Counter,
        source: &OptimizedNode,
    ) -> Self {
        match source {
            OptimizedNode::Byte(predicate) => {
                TaggedNode::Byte(var_counter.get_and_increment(), predicate.clone())
            }
            OptimizedNode::Seq(nodes) => TaggedNode::Seq(
                nodes
                    .iter()
                    .map(|node| TaggedNode::from_optimized(var_counter, group_counter, node))
                    .collect(),
            ),
            OptimizedNode::Alt(nodes) => TaggedNode::Alt(
                nodes
                    .iter()
                    .map(|node| TaggedNode::from_optimized(var_counter, group_counter, node))
                    .collect(),
            ),
            OptimizedNode::Optional(node) => TaggedNode::Optional(
                var_counter.get_and_increment(),
                Box::new(TaggedNode::from_optimized(var_counter, group_counter, node)),
            ),
            OptimizedNode::Star(node) => TaggedNode::Star(Box::new(TaggedNode::from_optimized(
                var_counter,
                group_counter,
                node,
            ))),
            OptimizedNode::Group(node) => {
                let this_group = group_counter.get_and_increment();
                TaggedNode::Group(
                    this_group,
                    Box::new(TaggedNode::from_optimized(var_counter, group_counter, node)),
                )
            }
        }
    }
    pub fn var_name(&self) -> Ident {
        match self {
            TaggedNode::Byte(var_num, ..) => format_ident!("b{}", var_num),
            TaggedNode::Optional(var_num, _) => format_ident!("alt{}", var_num),
            TaggedNode::Seq(_)
            | TaggedNode::Alt(_)
            | TaggedNode::Star(_)
            | TaggedNode::Group(..) => {
                panic!("name called on {:?}", self)
            }
        }
    }
}
impl core::fmt::Debug for TaggedNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            TaggedNode::Byte(var_num, predicate) => write!(f, "Byte({},{:?})", var_num, predicate),
            TaggedNode::Seq(nodes) => write!(f, "Seq({:?})", nodes),
            TaggedNode::Alt(nodes) => write!(f, "Alt({:?})", nodes),
            TaggedNode::Optional(var_num, node) => write!(f, "Optional({},{:?})", var_num, node),
            TaggedNode::Star(node) => write!(f, "Star({:?})", node),
            TaggedNode::Group(group_num, node) => {
                write!(f, "Group({},{:?})", group_num, node)
            }
        }
    }
}

fn collect_var_names(var_names: &mut Vec<Ident>, node: &TaggedNode) {
    match node {
        TaggedNode::Byte(..) => var_names.push(node.var_name()),
        TaggedNode::Seq(nodes) | TaggedNode::Alt(nodes) => {
            for node in nodes {
                collect_var_names(var_names, node);
            }
        }
        TaggedNode::Optional(_, node) | TaggedNode::Star(node) | TaggedNode::Group(_, node) => {
            collect_var_names(var_names, node)
        }
    }
}

#[allow(clippy::too_many_lines)]
fn build(
    num_groups: usize,
    enclosing_groups: &Vec<usize>,
    statements1: &mut Vec<TokenStream>,
    statements2_reversed: &mut Vec<TokenStream>,
    prev_state_expr: &TokenStream,
    node: &TaggedNode,
) -> TokenStream {
    crate::dprintln!("build {:?}", node);
    let result = match node {
        TaggedNode::Byte(_, predicate) => {
            let var_name = node.var_name();
            let filter = match predicate {
                Predicate::Any => quote! {},
                Predicate::Incl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {*b == #b},
                        ClassItem::ByteRange(x, y) => quote! {(#x ..= #y).contains(b)},
                    });
                    quote! { .filter(|_| { #( #comparisons )||* } )  }
                }
                Predicate::Excl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {*b != #b},
                        ClassItem::ByteRange(x, y) => quote! {!(#x ..= #y).contains(b)},
                    });
                    quote! { .filter(|_| { #( #comparisons )&&* } )  }
                }
            };
            let update_groups = if enclosing_groups.is_empty() {
                quote! {}
            } else {
                let mut range_names = Vec::new();
                let mut range_values = Vec::new();
                for r in 0..num_groups {
                    let range_name = format_ident!("r{}", r);
                    range_names.push(range_name.clone());
                    range_values.push(if enclosing_groups.contains(&r) {
                        quote! { #range_name .start .. n + 1}
                    } else {
                        quote! { #range_name }
                    });
                }
                let extra_comma = if num_groups > 1 {
                    quote! {}
                } else {
                    quote! {,}
                };
                quote! {
                    .map(
                        |( #( #range_names ),* #extra_comma )| ( #( #range_values ),* #extra_comma )
                    )
                }
            };
            statements2_reversed.push(quote! {
                #var_name = #prev_state_expr .clone() #filter #update_groups ;
            });
            quote! { #var_name }
        }
        TaggedNode::Seq(inner_nodes) => {
            assert!(!inner_nodes.is_empty());
            let mut last_state_expr = prev_state_expr.clone();
            for node in inner_nodes {
                last_state_expr = build(
                    num_groups,
                    enclosing_groups,
                    statements1,
                    statements2_reversed,
                    &last_state_expr,
                    node,
                );
            }
            last_state_expr
        }
        TaggedNode::Alt(inner_nodes) => {
            assert!(!inner_nodes.is_empty());
            let mut arm_state_exprs: Vec<TokenStream> = Vec::new();
            for node in inner_nodes {
                arm_state_exprs.push(build(
                    num_groups,
                    enclosing_groups,
                    statements1,
                    statements2_reversed,
                    prev_state_expr,
                    node,
                ));
            }
            quote! { None #( .or_else(|| #arm_state_exprs.clone()) )* }
        }
        TaggedNode::Optional(_, inner) => {
            let node_state_expr = build(
                num_groups,
                enclosing_groups,
                statements1,
                statements2_reversed,
                prev_state_expr,
                inner,
            );
            let var_name = node.var_name();
            statements1.push(quote! {
                let #var_name = #prev_state_expr .clone() .or_else(|| #node_state_expr .clone()) ;
            });
            quote! { #var_name }
        }
        TaggedNode::Star(inner) => {
            // TODO(mleonhard) Save intermediate value like we do with Optional.
            let node_expr = build(
                num_groups,
                enclosing_groups,
                &mut Vec::new(),
                &mut Vec::new(),
                prev_state_expr,
                inner,
            );
            let prev_or_node_expr =
                quote! { #prev_state_expr .clone().or_else(|| #node_expr .clone()) };
            let node_expr = build(
                num_groups,
                enclosing_groups,
                statements1,
                statements2_reversed,
                &prev_or_node_expr,
                inner,
            );
            quote! { #prev_state_expr .clone() .or_else(|| #node_expr .clone()) }
        }
        TaggedNode::Group(group_num, inner) => {
            let inner_enclosing_groups: Vec<usize> = enclosing_groups
                .iter()
                .chain(core::iter::once(group_num))
                .copied()
                .collect();
            let inner_prev_state_expr = {
                let mut range_names = Vec::new();
                let mut range_values = Vec::new();
                let extra_comma = if num_groups > 1 {
                    quote! {}
                } else {
                    quote! {,}
                };
                for r in 0..num_groups {
                    let range_name = format_ident!("r{}", r);
                    range_names.push(range_name.clone());
                    range_values.push(if &r == group_num {
                        quote! { n .. n }
                    } else {
                        quote! { #range_name }
                    });
                }
                quote! {
                    #prev_state_expr .clone().map(
                        |( #( #range_names ),* #extra_comma )| ( #( #range_values ),* #extra_comma )
                    )
                }
            };
            build(
                num_groups,
                &inner_enclosing_groups,
                statements1,
                statements2_reversed,
                &inner_prev_state_expr,
                inner,
            )
        }
    };
    crate::dprintln!("build returning {:?}", result);
    result
}

/// Generates an enum that implements `parsed_re` and implements the
/// [`safe_regex::internal::Machine`](https://docs.rs/safe-regex/latest/safe_regex/internal/trait.Machine.html)
/// trait.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate(final_node: &FinalNode) -> safe_proc_macro2::TokenStream {
    let optimized_node = if let Some(node) = OptimizedNode::from_final_node(&final_node) {
        node
    } else {
        return quote! {
            safe_regex::Matcher0::new(|data: &[u8]| {
                if data.is_empty() {
                    Some(())
                } else {
                    None
                }
            })
        };
    };
    let mut group_counter = Counter::new();
    let tagged_node =
        TaggedNode::from_optimized(&mut Counter::new(), &mut group_counter, &optimized_node);
    let num_groups = group_counter.get();
    let matcher_type_name = format_ident!("Matcher{}", num_groups);
    let mut var_names: Vec<Ident> = Vec::new();
    collect_var_names(&mut var_names, &tagged_node);
    let mut statements1: Vec<TokenStream> = Vec::new();
    let mut statements2_reversed: Vec<TokenStream> = Vec::new();
    let accept_expr = build(
        num_groups,
        &Vec::new(),
        &mut statements1,
        &mut statements2_reversed,
        &quote! { start },
        &tagged_node,
    );
    let statements2 = statements2_reversed.iter().rev();
    let result = if num_groups == 0 {
        quote! {
            safe_regex::#matcher_type_name::new(|data: &[u8]| {
                let mut start = Some(());
                #( let mut #var_names : Option<()> = None; )*
                let mut data_iter = data.iter();
                loop {
                    #( #statements1 )*
                    if let Some(b) = data_iter.next() {
                    #( #statements2 )*
                        start = None;
                    } else {
                        return #accept_expr ;
                    }
                }
            })
        }
    } else {
        let default_ranges = core::iter::repeat(quote! { usize::MAX..usize::MAX }).take(num_groups);
        let extra_comma = if num_groups > 1 {
            quote! {}
        } else {
            quote! {,}
        };
        var_names.push(format_ident!("accept"));
        let range_types = core::iter::repeat(quote! { core::ops::Range<usize> }).take(num_groups);
        let range_type = quote! { Option<( #( #range_types ),* #extra_comma )> };
        let range_names: Vec<Ident> = (0..num_groups).map(|r| format_ident!("r{}", r)).collect();
        quote! {
            safe_regex::#matcher_type_name::new(|data: &[u8]| {
                assert!(data.len() < usize::MAX - 2);
                let mut start = Some(( #( #default_ranges ),* #extra_comma ));
                #( let mut #var_names : #range_type = None; )*
                let mut data_iter = data.iter();
                let mut n = 0;
                loop {
                    #( #statements1 )*
                    accept = #accept_expr .clone() ;
                    if let Some(b) = data_iter.next() {
                        #( #statements2 )*
                        start = None;
                    } else {
                        break;
                    }
                    n = n + 1;
                }
                accept .map(|( #( #range_names ),* #extra_comma )| {
                    (
                        #(
                            if #range_names.start != usize::MAX && #range_names.end != usize::MAX {
                                Some(&data[#range_names])
                            } else {
                                None
                            },
                         )*
                    )
                })
            })
        }
    };
    crate::dprintln!("result={}", result);
    result
}
