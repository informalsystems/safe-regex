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
use safe_proc_macro2::{Ident, Literal, TokenStream};
use safe_quote::{format_ident, quote, ToTokens};

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
// - Remove Empty from Seq
// - Deduplicate Empty in Alt
// - Remove Optional(Empty) and Star(Empty)
// - Collapse Seq/Alt with one entry
// - Drop Optional(x) that comes right after Star(x)
// - Reorder Optional(x),x so the optional comes later
// - Translate x{2,5} into "xx(x(x(x)?)?)?" rather than "xxx?x?x?"
#[derive(Clone, PartialOrd, PartialEq)]
enum OptimizedNode {
    Byte(Predicate),
    Empty,
    Seq(Vec<OptimizedNode>),
    Alt(Vec<OptimizedNode>),
    Optional(Box<OptimizedNode>),
    Star(Box<OptimizedNode>),
    Group(Box<OptimizedNode>),
}
impl OptimizedNode {
    pub fn from_final_node(final_node: &FinalNode) -> Self {
        match final_node {
            FinalNode::AnyByte => OptimizedNode::Byte(Predicate::Any),
            FinalNode::Byte(b) => OptimizedNode::Byte(Predicate::Incl(vec![ClassItem::Byte(*b)])),
            FinalNode::Class(true, items) => OptimizedNode::Byte(Predicate::Incl(items.clone())),
            FinalNode::Class(false, items) => OptimizedNode::Byte(Predicate::Excl(items.clone())),
            FinalNode::Seq(nodes) if nodes.is_empty() => OptimizedNode::Empty,
            FinalNode::Seq(nodes) if nodes.len() == 1 => {
                OptimizedNode::from_final_node(nodes.first().unwrap())
            }
            FinalNode::Seq(nodes) => OptimizedNode::Seq(
                nodes
                    .iter()
                    .map(|node| OptimizedNode::from_final_node(node))
                    .collect(),
            ),
            FinalNode::Alt(nodes) if nodes.is_empty() => OptimizedNode::Empty,
            FinalNode::Alt(nodes) if nodes.len() == 1 => {
                OptimizedNode::from_final_node(nodes.first().unwrap())
            }
            FinalNode::Alt(nodes) => OptimizedNode::Alt(
                nodes
                    .iter()
                    .map(|node| OptimizedNode::from_final_node(node))
                    .collect(),
            ),
            FinalNode::Repeat(node, 0, None) => {
                OptimizedNode::Star(Box::new(OptimizedNode::from_final_node(node)))
            }
            FinalNode::Repeat(node, min, None) => {
                let inner = OptimizedNode::from_final_node(node);
                let required_instances = core::iter::repeat(inner.clone()).take(*min);
                let star = core::iter::once(OptimizedNode::Star(Box::new(inner)));
                OptimizedNode::Seq(required_instances.chain(star).collect())
            }
            FinalNode::Repeat(_node, 0, Some(0)) => OptimizedNode::Empty,
            FinalNode::Repeat(_node, min, Some(max)) if max < min => unreachable!(),
            FinalNode::Repeat(node, min, Some(max)) => {
                let inner = OptimizedNode::from_final_node(node);
                let required_instances = core::iter::repeat(inner.clone()).take(*min);
                let optional_instances =
                    core::iter::repeat(OptimizedNode::Optional(Box::new(inner))).take(max - min);
                OptimizedNode::Seq(required_instances.chain(optional_instances).collect())
            }
            FinalNode::Group(node) => {
                OptimizedNode::Group(Box::new(OptimizedNode::from_final_node(node)))
            }
        }
    }
}
impl core::fmt::Debug for OptimizedNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            OptimizedNode::Byte(items) => write!(f, "OptimizedNode::Byte({:?})", items),
            OptimizedNode::Empty => write!(f, "OptimizedNode::Empty"),
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
    Byte(usize, Option<usize>, Predicate),
    Empty,
    Seq(Vec<TaggedNode>),
    Alt(Vec<TaggedNode>),
    Optional(Box<TaggedNode>),
    Star(Box<TaggedNode>),
    Group(usize, Option<usize>, Box<TaggedNode>),
}
impl TaggedNode {
    pub fn from_optimized(
        var_counter: &mut Counter,
        group_counter: &mut Counter,
        enclosing_group: Option<usize>,
        source: &OptimizedNode,
    ) -> Self {
        match source {
            OptimizedNode::Byte(predicate) => TaggedNode::Byte(
                var_counter.get_and_increment(),
                enclosing_group,
                predicate.clone(),
            ),
            OptimizedNode::Empty => TaggedNode::Empty,
            OptimizedNode::Seq(nodes) => TaggedNode::Seq(
                nodes
                    .iter()
                    .map(|node| {
                        TaggedNode::from_optimized(
                            var_counter,
                            group_counter,
                            enclosing_group,
                            node,
                        )
                    })
                    .collect(),
            ),
            OptimizedNode::Alt(nodes) => TaggedNode::Alt(
                nodes
                    .iter()
                    .map(|node| {
                        TaggedNode::from_optimized(
                            var_counter,
                            group_counter,
                            enclosing_group,
                            node,
                        )
                    })
                    .collect(),
            ),
            OptimizedNode::Optional(node) => TaggedNode::Optional(Box::new(
                TaggedNode::from_optimized(var_counter, group_counter, enclosing_group, node),
            )),
            OptimizedNode::Star(node) => TaggedNode::Star(Box::new(TaggedNode::from_optimized(
                var_counter,
                group_counter,
                enclosing_group,
                node,
            ))),
            OptimizedNode::Group(node) => {
                let this_group = group_counter.get_and_increment();
                TaggedNode::Group(
                    this_group,
                    enclosing_group,
                    Box::new(TaggedNode::from_optimized(
                        var_counter,
                        group_counter,
                        Some(this_group),
                        node,
                    )),
                )
            }
        }
    }
    pub fn var_name(&self) -> Ident {
        match self {
            TaggedNode::Byte(var_num, ..) => format_ident!("b{}", var_num),
            TaggedNode::Empty
            | TaggedNode::Seq(_)
            | TaggedNode::Alt(_)
            | TaggedNode::Optional(_)
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
            TaggedNode::Byte(var_num, enclosing_group, predicate) => write!(
                f,
                "Byte({},{},{:?})",
                var_num,
                enclosing_group.map_or("".to_string(), |g| g.to_string()),
                predicate
            ),
            TaggedNode::Empty => write!(f, "Empty"),
            TaggedNode::Seq(nodes) => write!(f, "Seq({:?})", nodes),
            TaggedNode::Alt(nodes) => write!(f, "Alt({:?})", nodes),
            TaggedNode::Optional(node) => write!(f, "Optional({:?})", node),
            TaggedNode::Star(node) => write!(f, "Star({:?})", node),
            TaggedNode::Group(group_num, enclosing_group, node) => {
                write!(
                    f,
                    "Group({},{},{:?})",
                    group_num,
                    enclosing_group.map_or("".to_string(), |g| g.to_string()),
                    node
                )
            }
        }
    }
}

fn collect_var_names(var_names: &mut Vec<Ident>, node: &TaggedNode) {
    match node {
        TaggedNode::Byte(..) => var_names.push(node.var_name()),
        TaggedNode::Empty => {}
        TaggedNode::Seq(nodes) | TaggedNode::Alt(nodes) => {
            for node in nodes {
                collect_var_names(var_names, node);
            }
        }
        TaggedNode::Optional(node) | TaggedNode::Star(node) | TaggedNode::Group(_, _, node) => {
            collect_var_names(var_names, node)
        }
    }
}

#[allow(clippy::too_many_lines)]
fn build(
    num_groups: usize,
    statements: &mut Vec<TokenStream>,
    prev_state_expr: &TokenStream,
    node: &TaggedNode,
) -> TokenStream {
    crate::dprintln!("build {:?}", node);
    let result = match node {
        TaggedNode::Byte(_var_num, enclosing_group, predicate) => {
            let var_name = node.var_name();
            let filter = match predicate {
                Predicate::Any => quote! {},
                Predicate::Incl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {*b == #b},
                        ClassItem::ByteRange(x, y) => quote! {(#x ..= #y).contains(b)},
                    });
                    quote! { .filter(|_| #( #comparisons )||* )  }
                }
                Predicate::Excl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {*b != #b},
                        ClassItem::ByteRange(x, y) => quote! {!(#x ..= #y).contains(b)},
                    });
                    quote! { .filter(|_| #( #comparisons )&&* )  }
                }
            };
            let update_group = if let Some(group_num) = enclosing_group {
                let range_names: Vec<Ident> =
                    (0..num_groups).map(|r| format_ident!("r{}", r)).collect();
                let mut range_values: Vec<TokenStream> =
                    range_names.iter().map(|r| r.into_token_stream()).collect();
                if let Some(group_num) = enclosing_group {
                    *(range_values.get_mut(*group_num).unwrap()) = quote! {n..n + 1};
                }
                quote! {
                    .map(
                        |( #( #range_names , )* )| ( #( #range_values , )* )
                    )
                }
            } else {
                quote! {}
            };
            statements.push(quote! {
                #var_name = #prev_state_expr .clone() #filter #update_group ;
            });
            quote! { #var_name }
        }
        TaggedNode::Empty => prev_state_expr.clone(),
        TaggedNode::Seq(nodes) => {
            assert!(!nodes.is_empty());
            let mut last_state_expr = prev_state_expr.clone();
            for node in nodes {
                last_state_expr = build(num_groups, statements, &last_state_expr, node);
            }
            last_state_expr
        }
        TaggedNode::Alt(nodes) => {
            assert!(!nodes.is_empty());
            let mut arm_state_exprs: Vec<TokenStream> = Vec::new();
            for node in nodes {
                arm_state_exprs.push(build(num_groups, statements, prev_state_expr, node));
            }
            quote! { None #( .or_else(|| #arm_state_exprs.clone()) )* }
        }
        TaggedNode::Optional(node) => {
            let node_state_expr = build(num_groups, statements, prev_state_expr, node);
            quote! { #prev_state_expr .clone() .or_else(|| #node_state_expr .clone()) }
        }
        TaggedNode::Star(node) => {
            let node_expr = build(num_groups, &mut Vec::new(), prev_state_expr, node);
            let prev_or_node_expr =
                quote! { #prev_state_expr .clone().or_else(|| #node_expr .clone()) };
            let node_expr = build(num_groups, statements, &prev_or_node_expr, node);
            quote! { #prev_state_expr .clone() .or_else(|| #node_expr .clone()) }
        }
        TaggedNode::Group(group_num, enclosing_group, node) => {
            let node_state_expr = build(num_groups, statements, prev_state_expr, node);
            quote! { #node_state_expr }
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
    let optimized_node = OptimizedNode::from_final_node(&final_node);
    let mut group_counter = Counter::new();
    let tagged_node = TaggedNode::from_optimized(
        &mut Counter::new(),
        &mut group_counter,
        None,
        &optimized_node,
    );
    let num_groups = group_counter.get();
    let mut var_names: Vec<Ident> = Vec::new();
    collect_var_names(&mut var_names, &tagged_node);
    let mut statements: Vec<TokenStream> = Vec::new();
    let accept_expr = build(num_groups, &mut statements, &quote! { start }, &tagged_node);
    let statements_reversed = statements.iter().rev();
    let result = if num_groups == 0 {
        quote! {
            |data: &[u8]| {
                let mut start = Some(());
                #( let mut #var_names : Option<()> = None; )*
                for b in data.iter() {
                    #( #statements_reversed )*
                    start = None;
                }
                #accept_expr
            }
        }
    } else {
        let default_ranges =
            core::iter::repeat(quote! { usize::MAX..usize::MAX, }).take(num_groups);
        let range_types = core::iter::repeat(quote! { core::ops::Range<usize>, }).take(num_groups);
        let range_type = quote! { Option<( #( #range_types )* )> };
        let range_names: Vec<Ident> = (0..num_groups).map(|r| format_ident!("r{}", r)).collect();
        quote! {
            |data: &[u8]| {
                assert!(data.len() < usize::MAX - 2);
                let mut start = Some(( #( #default_ranges )* ));
                #( let mut #var_names : #range_type = None; )*
                for (n, b) in data.iter().enumerate() {
                    #( #statements_reversed )*
                    start = None;
                }
                #accept_expr .map(|( #( #range_names , )* )| {
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
            }
        }
    };
    crate::dprintln!("result={}", result);
    result
}
