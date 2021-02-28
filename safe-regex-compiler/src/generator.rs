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
            // TODO(mleonhard) Implement OptimizedNode::Repeat(node,min,max).
            //   This will require adding count values to the state parameters (`Ranges_`).
            //   When we expand a Repeat into duplicate instances, optionals, and star,
            //   we reduce runtime memory and increase code size and compilation time.
            //   Use a Repeat only when the number of duplicate instances & optionals is high.
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
    Empty(usize),
    Seq(Vec<TaggedNode>),
    Alt(usize, Vec<TaggedNode>),
    Optional(usize, Box<TaggedNode>),
    Star(usize, Box<TaggedNode>),
    Group(usize, usize, Option<usize>, Box<TaggedNode>),
}
impl TaggedNode {
    pub fn from_optimized(
        fn_counter: &mut Counter,
        group_counter: &mut Counter,
        enclosing_group: Option<usize>,
        source: &OptimizedNode,
    ) -> Self {
        match source {
            OptimizedNode::Byte(predicate) => TaggedNode::Byte(
                fn_counter.get_and_increment(),
                enclosing_group,
                predicate.clone(),
            ),
            OptimizedNode::Empty => TaggedNode::Empty(fn_counter.get_and_increment()),
            OptimizedNode::Seq(nodes) => TaggedNode::Seq(
                nodes
                    .iter()
                    .map(|node| {
                        TaggedNode::from_optimized(fn_counter, group_counter, enclosing_group, node)
                    })
                    .collect(),
            ),
            OptimizedNode::Alt(nodes) => TaggedNode::Alt(
                fn_counter.get_and_increment(),
                nodes
                    .iter()
                    .map(|node| {
                        TaggedNode::from_optimized(fn_counter, group_counter, enclosing_group, node)
                    })
                    .collect(),
            ),
            OptimizedNode::Optional(node) => TaggedNode::Optional(
                fn_counter.get_and_increment(),
                Box::new(TaggedNode::from_optimized(
                    fn_counter,
                    group_counter,
                    enclosing_group,
                    node,
                )),
            ),
            OptimizedNode::Star(node) => TaggedNode::Star(
                fn_counter.get_and_increment(),
                Box::new(TaggedNode::from_optimized(
                    fn_counter,
                    group_counter,
                    enclosing_group,
                    node,
                )),
            ),
            OptimizedNode::Group(node) => {
                let this_group = group_counter.get_and_increment();
                TaggedNode::Group(
                    fn_counter.get_and_increment(),
                    this_group,
                    enclosing_group,
                    Box::new(TaggedNode::from_optimized(
                        fn_counter,
                        group_counter,
                        Some(this_group),
                        node,
                    )),
                )
            }
        }
    }
}
impl core::fmt::Debug for TaggedNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            TaggedNode::Byte(fn_num, enclosing_group, predicate) => write!(
                f,
                "Byte({},{},{:?})",
                fn_num,
                enclosing_group.map_or("".to_string(), |g| g.to_string()),
                predicate
            ),
            TaggedNode::Empty(fn_num) => write!(f, "Empty({})", fn_num),
            TaggedNode::Seq(nodes) => write!(f, "Seq({:?})", nodes),
            TaggedNode::Alt(fn_num, nodes) => write!(f, "Alt({},{:?})", fn_num, nodes),
            TaggedNode::Optional(fn_num, node) => write!(f, "Optional({},{:?})", fn_num, node),
            TaggedNode::Star(fn_num, node) => write!(f, "Star({},{:?})", fn_num, node),
            TaggedNode::Group(fn_num, group_num, enclosing_group, node) => {
                write!(
                    f,
                    "Group({},{},{},{:?})",
                    fn_num,
                    group_num,
                    enclosing_group.map_or("".to_string(), |g| g.to_string()),
                    node
                )
            }
        }
    }
}

/// This function works around a bug in rustc's optimizer
/// that makes it take a long time (>40 min) to compile
/// `regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")`.
/// See `src/bin/uncompilable.rs`.
///
/// I believe rustc's optimizer tries to walk valid execution paths, up to a certain depth.
/// So a set of functions that branch and call each other can create a large space to search.
/// We reduce the search space by introducing two noop functions between each branching function.
/// This reduces the effectiveness of the optimizer.  It's an ugly workaround.
fn push_intermediate_fns(functions: &mut Vec<TokenStream>, fn_name: &Ident) -> Ident {
    let b_fn_name = format_ident!("{}_b", fn_name);
    let final_fn_name = format_ident!("{}_final", fn_name);
    functions.push(quote! {
        fn #fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
            Self::#b_fn_name(ranges, ib, next_states)
        }
        fn #b_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
            Self::#final_fn_name(ranges, ib, next_states)
        }
    });
    final_fn_name
}

#[allow(clippy::too_many_lines)]
fn build(
    variant_and_fn_names: &mut Vec<(Ident, Ident)>,
    functions: &mut Vec<TokenStream>,
    next_fn_name: &Ident,
    node: &TaggedNode,
) -> Ident {
    crate::dprintln!("build {:?}", node);
    let result = match node {
        TaggedNode::Byte(fn_num, enclosing_group, predicate) => {
            let fn_name = format_ident!("byte{}", fn_num);
            let variant_name = format_ident!("Byte{}", fn_num);
            variant_and_fn_names.push((variant_name.clone(), fn_name.clone()));
            let pattern = match predicate {
                Predicate::Any => quote! { Some(_) },
                Predicate::Incl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {b == #b},
                        ClassItem::ByteRange(x, y) => quote! {(#x ..= #y).contains(&b)},
                    });
                    quote! { Some(b) if #( #comparisons )||* }
                }
                Predicate::Excl(items) => {
                    let comparisons = items.iter().map(|p| match p {
                        ClassItem::Byte(b) => quote! {b != #b},
                        ClassItem::ByteRange(x, y) => quote! {!(#x ..= #y).contains(&b)},
                    });
                    quote! { Some(b) if #( #comparisons )&&* }
                }
            };
            let maybe_some_underscore = if predicate == &Predicate::Any {
                quote! {}
            } else {
                quote! { Some(_) => {} }
            };
            let clone_ranges_and_skip_past_n = if let Some(group_num) = enclosing_group {
                quote! { &ranges.clone().skip_past(#group_num, ib.index()) }
            } else {
                quote! { ranges }
            };
            functions.push(quote! {
                fn #fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#fn_name), ib, ranges);
                    match ib.byte() {
                        #pattern => {
                            Self::#next_fn_name(
                                #clone_ranges_and_skip_past_n,
                                ib.consume(),
                                next_states,
                            )
                        }
                        #maybe_some_underscore
                        None => {
                            next_states.insert(Self::#variant_name(ranges.clone()));
                        }
                    }
                }
            });
            fn_name
        }
        TaggedNode::Empty(fn_num) => {
            let fn_name = format_ident!("empty{}", fn_num);
            let variant_name = format_ident!("Empty{}", fn_num);
            variant_and_fn_names.push((variant_name, fn_name.clone()));
            functions.push(quote! {
                fn #fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#fn_name), ib, ranges);
                    Self::#next_fn_name(
                        ranges,
                        ib,
                        next_states,
                    );
                }
            });
            fn_name
        }
        TaggedNode::Seq(nodes) => {
            assert!(!nodes.is_empty());
            let mut next = next_fn_name.clone();
            for node in nodes.iter().rev() {
                next = build(variant_and_fn_names, functions, &next, node);
            }
            next
        }
        TaggedNode::Alt(fn_num, nodes) => {
            assert!(!nodes.is_empty());
            let fn_name = format_ident!("alt{}", fn_num);
            let arm_fn_names: Vec<Ident> = nodes
                .iter()
                .map(|node| build(variant_and_fn_names, functions, next_fn_name, node))
                .collect();
            let call_arm_fn_stmts: Vec<TokenStream> = arm_fn_names
                .iter()
                .map(|arm_fn_name| {
                    quote! {
                        Self::#arm_fn_name(ranges, ib, next_states);
                    }
                })
                .collect();
            let final_fn_name = push_intermediate_fns(functions, &fn_name);
            functions.push(quote! {
                fn #final_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#fn_name), ib, ranges);
                    #( #call_arm_fn_stmts )*
                }
            });
            fn_name
        }
        TaggedNode::Group(fn_num, group_num, enclosing_group, node) => {
            let start_fn_name = format_ident!("group_start{}", fn_num);
            let end_fn_name = format_ident!("group_end{}", fn_num);
            let child_fn_name = build(variant_and_fn_names, functions, &end_fn_name, node);
            let exit_range_expr = if let Some(enclosing) = enclosing_group {
                quote! { &ranges.clone().exit(#enclosing, ib.index()) }
            } else {
                quote! { ranges }
            };
            functions.push(quote! {
                fn #start_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#start_fn_name), ib, ranges);
                    Self::#child_fn_name(&ranges.clone().enter(#group_num, ib.index()), ib, next_states);
                }
                fn #end_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#end_fn_name), ib, ranges);
                    Self::#next_fn_name(#exit_range_expr, ib, next_states);
                }
            });
            start_fn_name
        }
        TaggedNode::Optional(fn_num, node) => {
            let fn_name = format_ident!("optional{}", fn_num);
            let child_fn_name = build(variant_and_fn_names, functions, &next_fn_name, node);
            let final_fn_name = push_intermediate_fns(functions, &fn_name);
            functions.push(quote! {
                fn #final_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#fn_name), ib, ranges);
                    Self::#child_fn_name(ranges, ib, next_states);
                    Self::#next_fn_name(ranges, ib, next_states);
                }
            });
            fn_name
        }
        TaggedNode::Star(fn_num, node) => {
            let fn_name = format_ident!("star{}", fn_num);
            let child_fn_name = build(variant_and_fn_names, functions, &fn_name, node);
            let final_fn_name = push_intermediate_fns(functions, &fn_name);
            functions.push(quote! {
                fn #final_fn_name(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                    // println!("{} {:?} {:?}", stringify!(#fn_name), ib, ranges);
                    Self::#child_fn_name(ranges, ib, next_states);
                    Self::#next_fn_name(ranges, ib, next_states);
                }
            });
            fn_name
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
pub fn generate(literal: &Literal, final_node: &FinalNode) -> safe_proc_macro2::TokenStream {
    let literal_string = literal.to_string();
    let optimized_node = OptimizedNode::from_final_node(&final_node);
    let mut fn_counter = Counter::new();
    let mut group_counter = Counter::new();
    let tagged_node =
        TaggedNode::from_optimized(&mut fn_counter, &mut group_counter, None, &optimized_node);
    let num_groups = group_counter.get();
    let ranges_inner = quote!([core::ops::Range<u32>; #num_groups]);
    let ranges_struct = if num_groups == 0 {
        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            struct Ranges_;
            impl Ranges_ {
                pub fn new() -> Self {
                    Self
                }
                pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                    &[]
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
                pub fn exit(mut self, group: usize, n: u32) -> Self {
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
    let mut variant_and_fn_names: Vec<(Ident, Ident)> = Vec::new();
    let mut functions: Vec<TokenStream> = Vec::new();
    // Perform a depth-first walk of the AST and make trait names and clauses.
    let initial_fn_name = build(
        &mut variant_and_fn_names,
        &mut functions,
        &format_ident!("accept"),
        &tagged_node,
    );
    let variant_names: Vec<Ident> = variant_and_fn_names
        .iter()
        .map(|(variant_name, _fn_name)| variant_name.clone())
        .collect();
    let clauses: Vec<TokenStream> = variant_and_fn_names
        .iter()
        .map(|(variant_name, fn_name)| {
            quote! {
                Self::#variant_name(ranges) => Self::#fn_name(ranges, ib, next_states)
            }
        })
        .collect();
    let result = quote! { {
        use safe_regex::internal::InputByte;
        #ranges_struct
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = #literal_string]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            #( #variant_names(Ranges_) ),* ,
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            #( #functions )*
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                // println!("accept {:?} {:?}", ib, ranges);
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = #ranges_inner;
            fn expression() -> &'static [u8] {
                #literal
            }
            fn start(next_states: &mut States_) {
                Self::#initial_fn_name(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                // println!("make_next_states {:?} {:?}", ib, self);
                match self {
                    #( #clauses ),* ,
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    } };
    crate::dprintln!("result={}", result);
    result
}
