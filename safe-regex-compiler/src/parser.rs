//! A parser for regular expressions.
//!
//! # Features
//! - Provides a [`parse`](fn.parse.html) function that converts a regular
//!   expression string into a [`FinalNode`](struct.FinalNode.html)
//!   struct which is the root of an abstract syntax tree
//! - Implements a straightforward
//!   [contex-free grammar parser](https://www.cs.umd.edu/class/summer2015/cmsc330/parsing/)
//! - Parses in a single pass
//! - No recursion, no risk of stack overflow
//! - `forbid(unsafe)`
//! - Depends only on `std`
//! - Good test coverage (92%)
//!
//! # Limitations
//! - Parses only raw byte strings, `br"..."`.
//! - Allocates.  Uses `Vec` and `String`.
//!
//! # Alternatives
//! - [`regex-syntax`](https://crates.io/crates/regex-syntax)
//!   - Mature
//!   - Popular
//!   - Maintained by the core Rust language developers
//!   - Full of features
//! - [`regular-expression`](https://crates.io/crates/regular-expression)
//!   - No documentation
#![forbid(unsafe_code)]
use crate::escape_ascii;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

/// An AST node used during parsing.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Node {
    Final(FinalNode),
    NonFinal(NonFinalNode),
}
impl Node {
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_final(self) -> FinalNode {
        match self {
            Node::Final(node) => node,
            other => panic!("unwrap_final() called on value: {:?}", other),
        }
    }

    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_non_final(self) -> NonFinalNode {
        match self {
            Node::NonFinal(node) => node,
            other => panic!("unwrap_non_final() called on value: {:?}", other),
        }
    }
}

/// An element of a regular expression character class.
///
/// [`FinalNode::Class`](enum.FinalNode.html#variant.Class) uses this.
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum ClassItem {
    Byte(u8),
    ByteRange(u8, u8),
}
impl core::fmt::Debug for ClassItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            ClassItem::Byte(b) => write!(f, "Byte({})", escape_ascii([*b])),
            ClassItem::ByteRange(a, b) => {
                write!(
                    f,
                    "ByteRange({}-{})",
                    escape_ascii([*a]),
                    escape_ascii([*b])
                )
            }
        }
    }
}

/// AST nodes used internally during parsing.
#[derive(Clone, PartialOrd, PartialEq)]
pub enum NonFinalNode {
    Escape,
    HexEscape0,
    HexEscape1(u8),
    OpenClass0,
    OpenClassNeg,
    OpenClass(/* inclusive */ bool, Vec<ClassItem>),
    OpenByteRange(u8),
    ByteRange(u8, u8),
    OpenGroup,
    OpenExtendedGroup,
    OpenNonCapturingGroup,
    OpenAlt(Vec<FinalNode>),
    RepeatMin(String),
    RepeatMax(String, String),
    RepeatToken(String, usize, Option<usize>),
}
impl core::fmt::Debug for NonFinalNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            NonFinalNode::Escape => write!(f, "Escape"),
            NonFinalNode::HexEscape0 => write!(f, "HexEscape0"),
            NonFinalNode::HexEscape1(b) => write!(f, "HexEscape1({})", escape_ascii([*b])),
            NonFinalNode::OpenClass0 => write!(f, "OpenClass0"),
            NonFinalNode::OpenClassNeg => write!(f, "OpenClassNeg"),
            NonFinalNode::OpenClass(true, items) => {
                write!(f, "OpenClass{:?}", items)
            }
            NonFinalNode::OpenClass(false, items) => {
                write!(f, "OpenClass^{:?}", items)
            }
            NonFinalNode::OpenByteRange(b) => write!(f, "OpenByteRange({})", escape_ascii([*b])),
            NonFinalNode::ByteRange(a, b) => write!(
                f,
                "ByteRange({}-{})",
                escape_ascii([*a]),
                escape_ascii([*b])
            ),
            NonFinalNode::OpenGroup => write!(f, "OpenGroup"),
            NonFinalNode::OpenExtendedGroup => write!(f, "OpenExtendedGroup"),
            NonFinalNode::OpenNonCapturingGroup => write!(f, "OpenNonCapturingGroup"),
            NonFinalNode::OpenAlt(nodes) => write!(f, "OpenAlt{:?}", nodes),
            NonFinalNode::RepeatMin(min) => write!(f, "RepeatMin({})", min),
            NonFinalNode::RepeatMax(min, max) => write!(f, "RepeatMax({},{})", min, max),
            NonFinalNode::RepeatToken(printable, min, opt_max) => {
                write!(f, "RepeatToken({:?},{},{:?})", printable, min, opt_max)
            }
        }
    }
}
impl NonFinalNode {
    /// Parsing can fail when a `NonFinalNode` is not converted into a
    /// `FinalNode`.  This function returns an explanation to show to the user.
    #[must_use]
    pub fn reason(&self) -> String {
        match self {
            NonFinalNode::Escape => "incomplete escape sequence: `\\`".to_string(),
            NonFinalNode::HexEscape0 => "incomplete escape sequence: `\\x`".to_string(),
            NonFinalNode::HexEscape1(d) => {
                format!("incomplete escape sequence: `\\x{}`", escape_ascii([*d]))
            }
            NonFinalNode::OpenClass0
            | NonFinalNode::OpenClassNeg
            | NonFinalNode::OpenClass(..)
            | NonFinalNode::ByteRange(..) => "missing closing `]`".to_string(),
            NonFinalNode::OpenByteRange(b) => {
                format!("missing byte to close range: `{}-`", escape_ascii([*b]))
            }
            NonFinalNode::OpenGroup
            | NonFinalNode::OpenExtendedGroup
            | NonFinalNode::OpenNonCapturingGroup => "missing closing `)`".to_string(),
            NonFinalNode::OpenAlt(_) => "missing element after bar `|`".to_string(),
            NonFinalNode::RepeatMin(min) => {
                format!("missing closing `}}` symbol: `{{{}`", min)
            }
            NonFinalNode::RepeatMax(min, max) => {
                format!("missing closing `}}` symbol: `{{{},{}`", min, max)
            }
            NonFinalNode::RepeatToken(printable, _, _) => {
                format!("missing element before repeat element: `{}`", printable)
            }
        }
    }

    /// Returns the contents of this `NonFinalNode::OpenClass(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_open_class(self) -> (bool, Vec<ClassItem>) {
        match self {
            NonFinalNode::OpenClass(incl, items) => (incl, items),
            other => panic!("unwrap_open_class() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::OpenAlt(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_open_alt(self) -> Vec<FinalNode> {
        match self {
            NonFinalNode::OpenAlt(nodes) => nodes,
            other => panic!("unwrap_open_alt() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::RepeatMin(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_repeat_min(self) -> String {
        match self {
            NonFinalNode::RepeatMin(min) => min,
            other => panic!("unwrap_repeat_min() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::RepeatMax(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_repeat_max(self) -> (String, String) {
        match self {
            NonFinalNode::RepeatMax(min, max) => (min, max),
            other => panic!("unwrap_repeat_max() called on value: {:?}", other),
        }
    }
}

/// A completely parsed element of a regular expression
/// [abstract syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
/// (AST).
///
/// A `FinalNode` could represent an entire regular expression (it is the root)
/// or a small part of one.
///
/// Since a regular expression ultimately processes bytes, the leaves of the
/// AST are nodes with bytes:
/// - [`Byte`](#variant.Byte)
/// - [`AnyByte`](#variant.AnyByte)
///
/// All other variants are edges of the AST:
/// - [`Seq`](#variant.Seq)
/// - [`Class`](#variant.Class)
/// - [`Group`](#variant.Group)
/// - [`Alt`](#variant.Alt)
/// - [`Repeat`](#variant.Repeat)
#[derive(Clone, PartialOrd, PartialEq)]
pub enum FinalNode {
    /// `Byte(u8)`
    ///
    /// A byte of input.
    /// This may be a non-printable byte that was escaped
    /// in the source regular expression.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::Byte(b'a')),
    ///     parse(br"a"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Byte(b'\n')),
    ///     parse(br"\n"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Byte(b'\\')),
    ///     parse(br"\\"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Byte(0x12)),
    ///     parse(br"\x12"),
    /// );
    /// ```
    Byte(u8),

    /// `AnyByte`
    ///
    /// Matches any byte of input.  This is the `.` operator.
    ///
    /// # Example
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::AnyByte),
    ///     parse(br"."),
    /// );
    /// ```
    AnyByte,

    /// `Seq(Vec<FinalNode>)`
    ///
    /// A sequence of nodes.
    ///
    /// # Example
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::Seq(vec![
    ///         FinalNode::Byte(b'a'),
    ///         FinalNode::Byte(b'b')],
    ///     )),
    ///     parse(br"ab"),
    /// );
    /// ```
    Seq(Vec<FinalNode>),

    /// `Class(inclusive: bool, Vec<ClassItem>)`
    ///
    /// A character class.  Matches any byte in the class.
    ///
    /// See [`ClassItem`](enum.ClassItem.html)
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// use safe_regex_compiler::parser::ClassItem;
    /// assert_eq!(
    ///     Ok(FinalNode::Class(true, vec![
    ///         ClassItem::Byte(b'a'),
    ///         ClassItem::Byte(b'b'),
    ///     ])),
    ///     parse(br"[ab]"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Class(false, vec![
    ///         ClassItem::Byte(b'a'),
    ///         ClassItem::Byte(b'b'),
    ///     ])),
    ///     parse(br"[^ab]"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Class(true, vec![
    ///         ClassItem::ByteRange(b'0', b'9'),
    ///     ])),
    ///     parse(br"[0-9]"),
    /// );
    /// ```
    Class(/* inclusive */ bool, Vec<ClassItem>),

    /// `Group(Box<FinalNode>)`
    ///
    /// A capturing group of nodes.
    /// Regular expression authors use it to override operator precedence rules
    /// and to capture sub-slices of input.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///    Ok(FinalNode::Seq(vec![
    ///       FinalNode::Byte(b'a'),
    ///       FinalNode::Group(Box::new(
    ///          FinalNode::Alt(vec![
    ///             FinalNode::Byte(b'b'),
    ///             FinalNode::Byte(b'c'),
    ///          ])
    ///       )),
    ///    ])),
    ///    parse(br"a(b|c)"),
    /// );
    /// assert_eq!(
    ///    Ok(FinalNode::Repeat(
    ///       Box::new(FinalNode::Group(Box::new(
    ///           FinalNode::Seq(vec![
    ///              FinalNode::Byte(b'a'),
    ///              FinalNode::Byte(b'b'),
    ///           ])
    ///       ))),
    ///       0,    // min
    ///       None, // max
    ///    )),
    ///    parse(br"(ab)*"),
    /// );
    /// ```
    Group(Box<FinalNode>),

    /// `NonCapturingGroup(Box<FinalNode>)`
    ///
    /// A non-capturing group of nodes.
    /// Regular expression authors use it to override operator precedence rules.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///    Ok(FinalNode::Seq(vec![
    ///       FinalNode::Byte(b'a'),
    ///       FinalNode::NonCapturingGroup(Box::new(
    ///          FinalNode::Alt(vec![
    ///             FinalNode::Byte(b'b'),
    ///             FinalNode::Byte(b'c'),
    ///          ])
    ///       )),
    ///    ])),
    ///    parse(br"a(?:b|c)"),
    /// );
    /// assert_eq!(
    ///    Ok(FinalNode::Repeat(
    ///       Box::new(FinalNode::NonCapturingGroup(Box::new(
    ///           FinalNode::Seq(vec![
    ///              FinalNode::Byte(b'a'),
    ///              FinalNode::Byte(b'b'),
    ///           ])
    ///       ))),
    ///       0,    // min
    ///       None, // max
    ///    )),
    ///    parse(br"(?:ab)*"),
    /// );
    /// ```
    NonCapturingGroup(Box<FinalNode>),

    /// `Alt(Vec<FinalNode>)`
    ///
    /// A list of alternate nodes.  The input can match any of them.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::Alt(vec![
    ///         FinalNode::Byte(b'a'),
    ///         FinalNode::Byte(b'b'),
    ///     ])),
    ///     parse(br"a|b"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Alt(vec![
    ///         FinalNode::Byte(b'a'),
    ///         FinalNode::Byte(b'b'),
    ///         FinalNode::Seq(vec![
    ///             FinalNode::AnyByte,
    ///             FinalNode::Byte(b'c'),
    ///         ]),
    ///     ])),
    ///     parse(br"a|b|.c"),
    /// );
    /// ```
    Alt(Vec<FinalNode>),

    /// `Repeat(Box<FinalNode>, min: usize, max: Option<usize>)`
    ///
    /// A repetition of a node.  It contains the minimum number of repetitions
    /// and an optional inclusive maximum number.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_compiler::parser::parse;
    /// use safe_regex_compiler::parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         0,       // min
    ///         Some(1), // max
    ///     )),
    ///     parse(br"a?"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         0,    // min
    ///         None, // max
    ///     )),
    ///     parse(br"a*"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         1,    // min
    ///         None, // max
    ///     )),
    ///     parse(br"a+"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         5,       // min
    ///         Some(5), // max
    ///     )),
    ///     parse(br"a{5}"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         5,    // min
    ///         None, // max
    ///     )),
    ///     parse(br"a{5,}"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         0,       // min
    ///         Some(7), // max
    ///     )),
    ///     parse(br"a{,7}"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         5,       // min
    ///         Some(7), // max
    ///     )),
    ///     parse(br"a{5,7}"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Repeat(
    ///         Box::new(FinalNode::Byte(b'a')),
    ///         0,    // min
    ///         None, // max
    ///     )),
    ///     parse(br"a{,}"),
    /// );
    /// ```
    Repeat(Box<FinalNode>, usize, Option<usize>),
}
impl FinalNode {
    /// Assumes this is a `FinalNode::Alt(_)` and returns its contents.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::missing_panics_doc)]
    pub fn unwrap_alt(self) -> Vec<FinalNode> {
        match self {
            FinalNode::Alt(nodes) => nodes,
            other => panic!("unwrap_alt() called on value: {:?}", other),
        }
    }
}
impl core::fmt::Debug for FinalNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            FinalNode::Byte(b) => write!(f, "Byte({})", escape_ascii([*b])),
            FinalNode::AnyByte => write!(f, "AnyByte"),
            FinalNode::Seq(nodes) => write!(f, "Seq{:?}", nodes),
            FinalNode::Class(true, items) => write!(f, "Class{:?}", items),
            FinalNode::Class(false, items) => write!(f, "Class^{:?}", items),
            FinalNode::Group(nodes) => write!(f, "Group({:?})", nodes),
            FinalNode::NonCapturingGroup(nodes) => write!(f, "NonCapturingGroup({:?})", nodes),
            FinalNode::Alt(nodes) => write!(f, "Alt{:?}", nodes),
            FinalNode::Repeat(node, min, opt_max) => {
                write!(f, "Repeat({:?},{}-{:?})", node, min, opt_max)
            }
        }
    }
}

/// Applies one parser rule.  Each rule may do any of:
/// - Match against the next byte of input, `byte`
/// - Consume the byte by calling `byte.take()`
/// - Match against the node at the top of the stack, `last`
/// - Consume the node by calling `last.take()`
/// - Match against the second-from-top node on the stack, `prev`
/// - Consume the node by calling `prev.take()`
/// - Return a new node to put on the stack above `prev` and `last`
/// - Return an error string with an explanation for the regex author
///
/// There are two kinds of parser rule:
/// - A byte consumer rule consumes a byte of input and creates
///   a new node or modifies an existing node.
/// - A node combiner rule merges two nodes into one node.
///   It may modify one node and discard the other, or replace both with
///   one new node.
///
/// The parser must check all combiner rules before byte consumer
/// rules.  This is necessary for operator precedence.
#[allow(clippy::too_many_lines)]
fn apply_rule_once(
    mut prev: &mut Option<Node>,
    mut last: &mut Option<Node>,
    byte: &mut Option<u8>,
) -> Result<Option<Node>, String> {
    use FinalNode::{Alt, AnyByte, Byte, Class, Group, NonCapturingGroup, Repeat, Seq};
    use Node::{Final, NonFinal};
    use NonFinalNode::{
        ByteRange, Escape, HexEscape0, HexEscape1, OpenAlt, OpenByteRange, OpenClass, OpenClass0,
        OpenClassNeg, OpenExtendedGroup, OpenGroup, OpenNonCapturingGroup, RepeatMax, RepeatMin,
        RepeatToken,
    };
    #[allow(clippy::match_same_arms, clippy::unnested_or_patterns)]
    match (&mut prev, &mut last, byte.map(|b| b)) {
        (Some(_), None, _) => unreachable!(),
        // Combine class nodes
        (Some(NonFinal(OpenByteRange(a))), Some(Final(Byte(b))), _) => {
            let node = NonFinal(ByteRange(*a, *b));
            prev.take();
            last.take();
            Ok(Some(node))
        }
        (Some(NonFinal(OpenClass0)), Some(Final(Byte(b))), _) => {
            let node = NonFinal(OpenClass(true, vec![ClassItem::Byte(*b)]));
            prev.take();
            last.take();
            Ok(Some(node))
        }
        (Some(NonFinal(OpenClassNeg)), Some(Final(Byte(b))), _) => {
            let node = NonFinal(OpenClass(false, vec![ClassItem::Byte(*b)]));
            prev.take();
            last.take();
            Ok(Some(node))
        }
        (Some(NonFinal(OpenClass(_, items))), Some(Final(Byte(b))), _) => {
            let item = ClassItem::Byte(*b);
            last.take();
            items.push(item);
            Ok(None)
        }
        (Some(NonFinal(OpenClass(_, items))), Some(NonFinal(ByteRange(a, b))), _) => {
            let item = ClassItem::ByteRange(*a, *b);
            last.take();
            items.push(item);
            Ok(None)
        }

        // Combine repeat tokens
        (None, Some(NonFinal(RepeatToken(printable, _, _))), _)
        | (Some(NonFinal(_)), Some(NonFinal(RepeatToken(printable, _, _))), _) => Err(format!(
            "missing element before repeat element: `{}`",
            printable
        )),
        (Some(Final(Seq(items))), Some(NonFinal(RepeatToken(_, min, opt_max))), _) => {
            let min_copy = *min;
            let opt_max_copy = *opt_max;
            last.take();
            let last_item = items.pop().unwrap();
            items.push(Repeat(Box::new(last_item), min_copy, opt_max_copy));
            Ok(None)
        }
        (Some(Final(_)), Some(NonFinal(RepeatToken(_, min, opt_max))), _) => {
            let inner = prev.take().unwrap().unwrap_final();
            let node = Final(Repeat(Box::new(inner), *min, *opt_max));
            last.take();
            Ok(Some(node))
        }

        // Combine group tokens
        (Some(NonFinal(OpenGroup)), Some(Final(_)), None) => Err(OpenGroup.reason()),
        (Some(NonFinal(OpenExtendedGroup)), Some(Final(_)), None) => {
            Err(OpenExtendedGroup.reason())
        }
        (Some(NonFinal(OpenNonCapturingGroup)), Some(Final(_)), None) => {
            Err(OpenNonCapturingGroup.reason())
        }
        // Combine Seq tokens
        (Some(Final(Seq(nodes))), Some(Final(_)), _) => {
            let node = last.take().unwrap().unwrap_final();
            nodes.push(node);
            Ok(None)
        }

        // Combine alternation/or nodes
        (Some(NonFinal(OpenAlt(_))), Some(Final(_)), b)
            if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
        {
            let node = last.take().unwrap().unwrap_final();
            let mut nodes = prev.take().unwrap().unwrap_non_final().unwrap_open_alt();
            nodes.push(node);
            Ok(Some(Final(Alt(nodes))))
        }
        (Some(Final(Alt(nodes))), Some(Final(_)), b)
            if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
        {
            let node = last.take().unwrap().unwrap_final();
            if let Seq(ref mut seq_nodes) = nodes.last_mut().unwrap() {
                seq_nodes.push(node);
            } else {
                let prev_node = nodes.pop().unwrap();
                nodes.push(Seq(vec![prev_node, node]));
            }
            Ok(None)
        }

        // Create Seq token
        // For proper precedence, these rules must appear after all the
        // specialized `Some(Final(something))` rules above.
        (Some(Final(_)), Some(Final(_)), b)
            if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
        {
            Ok(Some(Final(Seq(vec![
                prev.take().unwrap().unwrap_final(),
                last.take().unwrap().unwrap_final(),
            ]))))
        }

        // Escape `\n`
        (_, Some(NonFinal(Escape)), Some(b'\\')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Byte(b'\\'))))
        }
        (_, _, Some(b'\\')) => {
            byte.take();
            Ok(Some(NonFinal(Escape)))
        }
        (_, Some(NonFinal(Escape)), Some(b'n')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Byte(b'\n'))))
        }
        (_, Some(NonFinal(Escape)), Some(b'r')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Byte(b'\r'))))
        }
        (_, Some(NonFinal(Escape)), Some(b't')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Byte(b'\t'))))
        }
        (_, Some(NonFinal(Escape)), Some(b'0')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Byte(0))))
        }
        (_, Some(NonFinal(Escape)), Some(b)) if b"'\"?+.*^$|(){}[]".contains(&b) => {
            let node = Final(Byte(b));
            last.take();
            byte.take();
            Ok(Some(node))
        }

        // Hex escape `\x20`
        (_, Some(NonFinal(Escape)), Some(b'x')) => {
            last.take();
            byte.take();
            Ok(Some(NonFinal(HexEscape0)))
        }
        (_, Some(NonFinal(Escape)), Some(d)) => {
            Err(format!("invalid escape sequence `\\{}`", escape_ascii([d])))
        }
        (_, Some(NonFinal(HexEscape0)), Some(d)) => {
            last.take();
            byte.take();
            Ok(Some(NonFinal(HexEscape1(d))))
        }
        (_, Some(NonFinal(HexEscape1(d1))), Some(d0))
            if d1.is_ascii_hexdigit() && d0.is_ascii_hexdigit() =>
        {
            let string = String::from_utf8(vec![*d1, d0]).unwrap();
            let b = u8::from_str_radix(&string, 16).unwrap();
            last.take();
            byte.take();
            Ok(Some(Final(Byte(b))))
        }
        (_, Some(NonFinal(HexEscape1(d1))), Some(d0)) => Err(format!(
            "invalid escape sequence `\\x{}`",
            escape_ascii([*d1, d0])
        )),

        // Class `[ab0-9]`, `[^-ab0-9]`
        (_, Some(NonFinal(OpenClass0)), Some(b'['))
        | (_, Some(NonFinal(OpenClassNeg)), Some(b'['))
        | (_, Some(NonFinal(OpenClass(..))), Some(b'[')) => {
            byte.take();
            Ok(Some(Final(Byte(b'['))))
        }
        (_, _, Some(b'[')) => {
            byte.take();
            Ok(Some(NonFinal(OpenClass0)))
        }
        (_, Some(NonFinal(OpenClass0)), Some(b'^')) => {
            last.take();
            byte.take();
            Ok(Some(NonFinal(OpenClassNeg)))
        }
        (_, Some(NonFinal(OpenClass(_, ref mut items))), Some(b'-')) => {
            byte.take();
            match items.pop().unwrap() {
                // "[a-"
                ClassItem::Byte(b) => Ok(Some(NonFinal(OpenByteRange(b)))),
                // "[a-b-"
                ClassItem::ByteRange(a, b) => Err(format!(
                    "expected byte before '-' symbol, not range: `{}-{}-`",
                    escape_ascii([a]),
                    escape_ascii([b])
                )),
            }
        }
        (_, Some(NonFinal(OpenClass0)), Some(b']')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Class(true, Vec::new()))))
        }
        (_, Some(NonFinal(OpenClassNeg)), Some(b']')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Class(false, Vec::new()))))
        }
        (_, Some(NonFinal(OpenClass(..))), Some(b']')) => {
            byte.take();
            let (incl, items) = last.take().unwrap().unwrap_non_final().unwrap_open_class();
            Ok(Some(Final(Class(incl, items))))
        }
        (Some(NonFinal(OpenClass(..))), Some(NonFinal(non_final)), Some(b']')) => {
            Err(non_final.reason())
        }

        // Bytes inside classes.
        // These must come before all of the generic `(_, _, b'X')` rules below.
        (_, Some(NonFinal(OpenClass0)), Some(b))
        | (_, Some(NonFinal(OpenClassNeg)), Some(b))
        | (_, Some(NonFinal(OpenClass(..))), Some(b))
            if b != b']' =>
        {
            byte.take();
            Ok(Some(Final(Byte(b))))
        }

        // Group `(ab)` and NonCapturingGroup `(?:ab)`
        (_, _, Some(b'(')) => {
            byte.take();
            Ok(Some(NonFinal(OpenGroup)))
        }
        (_, Some(NonFinal(OpenGroup)), Some(b')')) => {
            last.take();
            byte.take();
            Ok(Some(Final(Group(Box::new(Seq(vec![]))))))
        }
        (_, Some(NonFinal(OpenGroup)), Some(b'?')) => {
            last.take();
            byte.take();
            Ok(Some(NonFinal(OpenExtendedGroup)))
        }
        (_, Some(NonFinal(OpenExtendedGroup)), Some(b':')) => {
            last.take();
            byte.take();
            Ok(Some(NonFinal(OpenNonCapturingGroup)))
        }
        (_, Some(NonFinal(OpenExtendedGroup)), Some(_)) => {
            Err("unexpected symbol after `(?`".to_string())
        }
        (_, Some(NonFinal(OpenNonCapturingGroup)), Some(b')')) => {
            last.take();
            byte.take();
            Ok(Some(Final(NonCapturingGroup(Box::new(Seq(vec![]))))))
        }
        (Some(NonFinal(OpenGroup)), Some(NonFinal(non_final)), Some(b')')) => {
            Err(non_final.reason())
        }
        (Some(NonFinal(OpenNonCapturingGroup)), Some(NonFinal(non_final)), Some(b')')) => {
            Err(non_final.reason())
        }
        (Some(NonFinal(OpenGroup)), Some(Final(_)), Some(b')')) => {
            byte.take();
            let node = last.take().unwrap().unwrap_final();
            prev.take();
            Ok(Some(Final(Group(Box::new(node)))))
        }
        (Some(NonFinal(OpenNonCapturingGroup)), Some(Final(_)), Some(b')')) => {
            byte.take();
            let node = last.take().unwrap().unwrap_final();
            prev.take();
            Ok(Some(Final(NonCapturingGroup(Box::new(node)))))
        }

        // Repeat, postfix operators `?` `+` `*` `{n}` `{n,}` `{,m}` `{n,m}`
        (_, _, Some(b'?')) => {
            byte.take();
            Ok(Some(NonFinal(RepeatToken("?".to_string(), 0, Some(1)))))
        }
        (_, _, Some(b'*')) => {
            byte.take();
            Ok(Some(NonFinal(RepeatToken("*".to_string(), 0, None))))
        }
        (_, _, Some(b'+')) => {
            byte.take();
            Ok(Some(NonFinal(RepeatToken("+".to_string(), 1, None))))
        }
        (_, _, Some(b'{')) => {
            byte.take();
            Ok(Some(NonFinal(RepeatMin(String::new()))))
        }
        (_, Some(NonFinal(RepeatMin(_))), Some(b',')) => {
            byte.take();
            let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
            Ok(Some(NonFinal(RepeatMax(min, String::new()))))
        }
        (_, Some(NonFinal(RepeatMin(_))), Some(b'}')) => {
            byte.take();
            let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
            let min_usize = min
                .parse::<usize>()
                .map_err(|e| format!("invalid repetition value `{{{}}}`: {}", min, e))?;
            Ok(Some(NonFinal(RepeatToken(
                format!("{{{}}}", min),
                min_usize,
                Some(min_usize),
            ))))
        }
        (_, Some(NonFinal(RepeatMin(min))), Some(b)) => {
            byte.take();
            min.push(char::from(b));
            Ok(None)
        }
        (_, Some(NonFinal(RepeatMax(..))), Some(b'}')) => {
            byte.take();
            let (min, max) = last.take().unwrap().unwrap_non_final().unwrap_repeat_max();
            let min_usize = if min.is_empty() {
                0
            } else {
                min.parse::<usize>()
                    .map_err(|e| format!("invalid repetition value `{{{},{}}}`: {}", min, max, e))?
            };
            let max_opt_usize = if max.is_empty() {
                None
            } else {
                let max_usize = max.parse::<usize>().map_err(|e| {
                    format!("invalid repetition value `{{{},{}}}`: {}", min, max, e)
                })?;
                if max_usize < min_usize {
                    return Err(format!(
                        "repeating element has max that is smaller than min: `{{{},{}}}`",
                        min, max
                    ));
                }
                Some(max_usize)
            };
            Ok(Some(NonFinal(RepeatToken(
                format!("{{{},{}}}", min, max),
                min_usize,
                max_opt_usize,
            ))))
        }
        (_, Some(NonFinal(RepeatMax(_, ref mut max))), Some(b)) => {
            max.push(char::from(b));
            byte.take();
            Ok(None)
        }

        // Any byte `.`
        (_, _, Some(b'.')) => {
            byte.take();
            Ok(Some(Final(AnyByte)))
        }

        // Alternate `a|b|c`
        (_, Some(Final(Alt(_))), Some(b'|')) => {
            byte.take();
            let nodes = last.take().unwrap().unwrap_final().unwrap_alt();
            Ok(Some(NonFinal(OpenAlt(nodes))))
        }
        (_, Some(Final(_)), Some(b'|')) => {
            byte.take();
            let node = last.take().unwrap().unwrap_final();
            Ok(Some(NonFinal(OpenAlt(vec![node]))))
        }
        (_, None, Some(b'|')) => Err("missing element before bar `|`".to_string()),

        // Other bytes
        (_, _, Some(b)) => {
            byte.take();
            Ok(Some(Final(Byte(b))))
        }

        // No more bytes.
        (_, Some(NonFinal(node)), None) => Err(node.reason()),
        (None, None, None) => unreachable!(),
        (None, Some(Final(_)), None) => unreachable!(),

        // These cases should be unreachable.
        (Some(NonFinal(Escape)), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(HexEscape0)), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(HexEscape1(_))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(RepeatMin(..))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(RepeatMax(..))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(OpenClass0)), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(OpenClassNeg)), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(OpenClass(..))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(OpenAlt(_))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(OpenByteRange(_))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(ByteRange(..))), Some(Final(_)), _) => unreachable!(),
        (Some(NonFinal(RepeatToken(..))), Some(Final(_)), _) => unreachable!(),
        (Some(Final(_)), Some(Final(_)), _) => unreachable!(),
    }
}

/// Parses `regex` as a regular expression.
///
/// Returns a [`FinalNode`](enum.FinalNode.htmls) which is the root of the
/// [abstract syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
/// (AST) of the expression.
///
/// # Errors
/// On error, returns a string explaining the problem.
///
/// # Examples
/// ```
/// use safe_regex_compiler::parser::parse;
/// use safe_regex_compiler::parser::FinalNode;
/// assert_eq!(
///     Ok(FinalNode::Byte(b'a')), parse(br"a")
/// );
/// assert_eq!(
///     Ok(FinalNode::Alt(vec![
///         FinalNode::Byte(b'a'),
///         FinalNode::Byte(b'b'),
///         FinalNode::Byte(b'c'),
///     ])),
///     parse(br"a|b|c"),
/// );
/// assert_eq!(
///     Err("missing closing `)`".to_string()),
///     parse(br"(a"),
/// );
/// ```
/// See [`FinalNode`](enum.FinalNode.html) variants for more examples.
#[allow(clippy::missing_panics_doc)]
pub fn parse(regex: &[u8]) -> Result<FinalNode, String> {
    if regex.is_empty() {
        return Ok(FinalNode::Seq(Vec::new()));
    }
    let mut data_iter = regex.iter().copied().peekable();
    let mut stack: Vec<Node> = Vec::new();
    while data_iter.peek().is_some() || stack.len() > 1 {
        crate::dprintln!(
            "process {:?} next={:?}",
            stack,
            data_iter.peek().map(|b| escape_ascii([*b]))
        );
        let mut byte = data_iter.peek().copied();
        let byte_was_some = byte.is_some();
        // Pull the top two items from the stack, so we can work with them and
        // keep the borrow checker happy.
        let mut last = stack.pop();
        let mut prev = stack.pop();
        // Anything put in here becomes the new top of the stack in the next loop.
        let mut to_push = apply_rule_once(&mut prev, &mut last, &mut byte)?;
        // Put items back in `stack`.
        if let Some(node) = prev.take() {
            stack.push(node);
        }
        if let Some(node) = last.take() {
            stack.push(node);
        }
        if let Some(node) = to_push.take() {
            stack.push(node);
        }
        if byte_was_some && byte.is_none() {
            data_iter.next().unwrap();
        }
    }
    crate::dprintln!("stack {:?}", stack);
    // Check for incomplete elements.  Example: br"(ab"
    for node in stack.iter().rev() {
        if let Node::NonFinal(non_final) = node {
            return Err(non_final.reason());
        }
    }
    assert_eq!(1, stack.len());
    Ok(stack.pop().unwrap().unwrap_final())
}
