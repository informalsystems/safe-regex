//! [![crates.io version](https://img.shields.io/crates/v/safe-regex-parser.svg)](https://crates.io/crates/safe-regex-parser)
//! [![license: Apache 2.0](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://gitlab.com/leonhard-llc/safe-regex-rs/-/raw/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://gitlab.com/leonhard-llc/safe-regex-rs/badges/main/pipeline.svg)](https://gitlab.com/leonhard-llc/safe-regex-rs/-/pipelines)
//!
//! This crate is used by the
//! [`safe_regex`](https://crates.io/crates/safe-regex) crate.
//! If you want to use regular expressions in your software, use that crate.
//!
//! # Features
//! - Provides a `parse` function that converts a regular expression string
//!   into a `FinalNode` struct which is the root of an abstract syntax tree
//! - Implements a straightforward
//!   [contex-free grammar parser](https://www.cs.umd.edu/class/summer2015/cmsc330/parsing/)
//! - Parses in a single pass
//! - No recursion, no risk of stack overflow
//! - Depends only on `core` (it's `no_std`)
//! - Good test coverage (93%)
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
//!
//! # Cargo Geiger Safety Report
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - DONE - Read about regular expressions
//! - DONE - Read about parsing
//! - DONE - Implement `parse`
//! - DONE - Add integration tests
//! - Add unwrap functions for other `FinalNode` variants
//! - Add fuzzing tests
//!
//! # Release Process
//! 1. Edit `Cargo.toml` and bump version number.
//! 1. Run `./release.sh`
#![forbid(unsafe_code)]

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

/// An AST node used during parsing.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Node {
    Final(FinalNode),
    NonFinal(NonFinalNode),
}
impl Node {
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn unwrap_final(self) -> FinalNode {
        match self {
            Node::Final(node) => node,
            other => panic!("unwrap_final() called on value: {:?}", other),
        }
    }

    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::match_wildcard_for_single_variants)]
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
    OpenOr(Vec<FinalNode>),
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
            NonFinalNode::OpenOr(nodes) => write!(f, "OpenOr{:?}", nodes),
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
            NonFinalNode::OpenGroup => "missing closing `)`".to_string(),
            NonFinalNode::OpenOr(_) => "missing element after bar `|`".to_string(),
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
    pub fn unwrap_open_class(self) -> (bool, Vec<ClassItem>) {
        match self {
            NonFinalNode::OpenClass(incl, items) => (incl, items),
            other => panic!("unwrap_open_class() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::OpenOr(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    pub fn unwrap_open_or(self) -> Vec<FinalNode> {
        match self {
            NonFinalNode::OpenOr(nodes) => nodes,
            other => panic!("unwrap_open_or() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::RepeatMin(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    pub fn unwrap_repeat_min(self) -> String {
        match self {
            NonFinalNode::RepeatMin(min) => min,
            other => panic!("unwrap_repeat_min() called on value: {:?}", other),
        }
    }

    /// Returns the contents of this `NonFinalNode::RepeatMax(..)`.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
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
/// - [`Or`](#variant.Or)
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
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
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
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
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
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
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
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
    /// use safe_regex_parser::ClassItem;
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
    /// A group of nodes.
    /// Regular expression authors use it to override operator precedence rules
    /// and to capture sub-slices of input.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
    /// assert_eq!(
    ///    Ok(FinalNode::Seq(vec![
    ///       FinalNode::Byte(b'a'),
    ///       FinalNode::Group(Box::new(
    ///          FinalNode::Or(vec![
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

    /// `Or(Vec<FinalNode>)`
    ///
    /// A list of alternate nodes.  The input can match any of them.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
    /// assert_eq!(
    ///     Ok(FinalNode::Or(vec![
    ///         FinalNode::Byte(b'a'),
    ///         FinalNode::Byte(b'b'),
    ///     ])),
    ///     parse(br"a|b"),
    /// );
    /// assert_eq!(
    ///     Ok(FinalNode::Or(vec![
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
    Or(Vec<FinalNode>),

    /// `Repeat(Box<FinalNode>, min: usize, max: Option<usize>)`
    ///
    /// A repetition of a node.  It contains the minimum number of repetitions
    /// and an optional inclusive maximum number.
    ///
    /// # Examples
    /// ```
    /// use safe_regex_parser::parse;
    /// use safe_regex_parser::FinalNode;
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
    /// Assumes this is a `FinalNode::Or(_)` and returns its contents.
    /// Panics if this is a different enum variant.
    #[allow(clippy::must_use_candidate)]
    pub fn unwrap_or(self) -> Vec<FinalNode> {
        match self {
            FinalNode::Or(nodes) => nodes,
            other => panic!("unwrap_or() called on value: {:?}", other),
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
            FinalNode::Or(nodes) => write!(f, "Or{:?}", nodes),
            FinalNode::Repeat(node, min, opt_max) => {
                write!(f, "Repeat({:?},{}-{:?})", node, min, opt_max)
            }
        }
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
/// use safe_regex_parser::parse;
/// use safe_regex_parser::FinalNode;
/// assert_eq!(
///     Ok(FinalNode::Byte(b'a')), parse(br"a")
/// );
/// assert_eq!(
///     Ok(FinalNode::Or(vec![
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
#[allow(clippy::too_many_lines)]
pub fn parse(regex: &[u8]) -> Result<FinalNode, String> {
    // This parser works, but it is hard to understand.
    // We should separate the parser code and the grammar declarations.
    use FinalNode::{AnyByte, Byte, Class, Group, Or, Repeat, Seq};
    use Node::{Final, NonFinal};
    use NonFinalNode::{
        ByteRange, Escape, HexEscape0, HexEscape1, OpenByteRange, OpenClass, OpenClass0,
        OpenClassNeg, OpenGroup, OpenOr, RepeatMax, RepeatMin, RepeatToken,
    };
    if regex.is_empty() {
        return Ok(Seq(Vec::new()));
    }
    let mut data_iter = regex.iter().copied().peekable();
    let mut stack: Vec<Node> = Vec::new();
    while data_iter.peek().is_some() || stack.len() > 1 {
        // println!("process {:?} next={:?}", stack, iter.peek().map(|b| escape_ascii([*b])));
        // Pull the top two items from the stack, so we can work with them and
        // keep the borrow checker happy.
        let mut last = stack.pop();
        let mut prev = stack.pop();
        // Anything put in here becomes the new top of the stack in the next loop.
        let mut to_push: Option<Node> = None;
        #[allow(clippy::match_same_arms)]
        match (&mut prev, &mut last, data_iter.peek().copied()) {
            (Some(_), None, _) => unreachable!(),
            // There are two kinds of parser rules:
            // - Byte Consumer Rule: A byte consumer rule takes a byte from the
            //   input and creates a new node or modifies an existing node.
            // - Node Combiner Rule: A node combiner rule takes two nodes and
            //   merges them into one node.  The resulting node may have a
            //   different type from the starting nodes.
            //
            //   The parser must check all combiner rules before byte consumer
            //   rules.  This is necessary for operator precedence.

            // Combine class nodes
            (Some(NonFinal(OpenByteRange(a))), Some(Final(Byte(b))), _) => {
                let node = NonFinal(ByteRange(*a, *b));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClass0)), Some(Final(Byte(b))), _) => {
                let node = NonFinal(OpenClass(true, vec![ClassItem::Byte(*b)]));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClassNeg)), Some(Final(Byte(b))), _) => {
                let node = NonFinal(OpenClass(false, vec![ClassItem::Byte(*b)]));
                last.take();
                prev = Some(node);
            }
            (Some(NonFinal(OpenClass(_, items))), Some(Final(Byte(b))), _) => {
                let item = ClassItem::Byte(*b);
                last.take();
                items.push(item);
            }
            (Some(NonFinal(OpenClass(_, items))), Some(NonFinal(ByteRange(a, b))), _) => {
                let item = ClassItem::ByteRange(*a, *b);
                last.take();
                items.push(item);
            }

            // Combine repeat tokens
            (None, Some(NonFinal(RepeatToken(printable, _, _))), _)
            | (Some(NonFinal(_)), Some(NonFinal(RepeatToken(printable, _, _))), _) => {
                return Err(format!(
                    "missing element before repeat element: `{}`",
                    printable
                ))
            }
            (Some(Final(_)), Some(NonFinal(RepeatToken(_, min, opt_max))), _) => {
                let inner = prev.take().unwrap().unwrap_final();
                prev = Some(Final(Repeat(Box::new(inner), *min, *opt_max)));
                last.take();
            }

            // Combine group tokens
            (Some(NonFinal(OpenGroup)), Some(Final(_)), None) => return Err(OpenGroup.reason()),
            // Combine Seq tokens
            (Some(Final(Seq(nodes))), Some(Final(_)), _) => {
                let node = last.take().unwrap().unwrap_final();
                nodes.push(node)
            }

            // Combine alternation/or nodes
            (Some(NonFinal(OpenOr(_))), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                let node = last.take().unwrap().unwrap_final();
                let mut nodes = prev.take().unwrap().unwrap_non_final().unwrap_open_or();
                nodes.push(node);
                prev = Some(Final(Or(nodes)));
            }
            (Some(Final(Or(nodes))), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                let node = last.take().unwrap().unwrap_final();
                if let Seq(ref mut seq_nodes) = nodes.last_mut().unwrap() {
                    seq_nodes.push(node)
                } else {
                    let prev_node = nodes.pop().unwrap();
                    nodes.push(Seq(vec![prev_node, node]))
                }
            }

            // Create Seq token
            // For proper precedence, these rules must appear after all the
            // specialized `Some(Final(something))` rules above.
            (Some(Final(_)), Some(Final(_)), b)
                if b != Some(b'?') && b != Some(b'+') && b != Some(b'*') && b != Some(b'{') =>
            {
                to_push = Some(Final(Seq(vec![
                    prev.take().unwrap().unwrap_final(),
                    last.take().unwrap().unwrap_final(),
                ])))
            }

            // Escape `\n`
            (_, Some(NonFinal(Escape)), Some(b'\\')) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(b'\\')))
            }
            (_, _, Some(b'\\')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(Escape))
            }
            (_, Some(NonFinal(Escape)), Some(b'n')) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(b'\n')))
            }
            (_, Some(NonFinal(Escape)), Some(b'r')) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(b'\r')))
            }
            (_, Some(NonFinal(Escape)), Some(b't')) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(b'\t')))
            }
            (_, Some(NonFinal(Escape)), Some(b'0')) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(0)))
            }
            (_, Some(NonFinal(Escape)), Some(b)) if b"'\"?+.*^$|(){}[]".contains(&b) => {
                data_iter.next().unwrap();
                last = Some(Final(Byte(b)))
            }

            // Hex escape `\x20`
            (_, Some(NonFinal(Escape)), Some(b'x')) => {
                data_iter.next().unwrap();
                last = Some(NonFinal(HexEscape0))
            }
            (_, Some(NonFinal(Escape)), Some(d)) => {
                return Err(format!("invalid escape sequence `\\{}`", escape_ascii([d])))
            }
            (_, Some(NonFinal(HexEscape0)), Some(d)) => {
                data_iter.next().unwrap();
                last = Some(NonFinal(HexEscape1(d)))
            }
            (_, Some(NonFinal(HexEscape1(d1))), Some(d0))
                if d1.is_ascii_hexdigit() && d0.is_ascii_hexdigit() =>
            {
                data_iter.next().unwrap();
                let string = String::from_utf8(vec![*d1, d0]).unwrap();
                let byte = u8::from_str_radix(&string, 16).unwrap();
                last = Some(Final(Byte(byte)))
            }
            (_, Some(NonFinal(HexEscape1(d1))), Some(d0)) => {
                return Err(format!(
                    "invalid escape sequence `\\x{}`",
                    escape_ascii([*d1, d0])
                ))
            }

            // Class `[ab0-9]`, `[^-ab0-9]`
            (_, Some(NonFinal(OpenClass0)), Some(b'['))
            | (_, Some(NonFinal(OpenClassNeg)), Some(b'['))
            | (_, Some(NonFinal(OpenClass(..))), Some(b'[')) => {
                data_iter.next().unwrap();
                to_push = Some(Final(Byte(b'[')))
            }
            (_, _, Some(b'[')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(OpenClass0))
            }
            (_, Some(NonFinal(OpenClass0)), Some(b'^')) => {
                data_iter.next().unwrap();
                last = Some(NonFinal(OpenClassNeg))
            }
            (_, Some(NonFinal(OpenClass(_, ref mut items))), Some(b'-')) => {
                data_iter.next().unwrap();
                match items.pop().unwrap() {
                    // "[a-"
                    ClassItem::Byte(b) => {
                        to_push = Some(NonFinal(OpenByteRange(b)));
                    }
                    // "[a-b-"
                    ClassItem::ByteRange(a, b) => {
                        return Err(format!(
                            "expected byte before '-' symbol, not range: `{}-{}-`",
                            escape_ascii([a]),
                            escape_ascii([b])
                        ))
                    }
                }
            }
            (_, Some(NonFinal(OpenClass0)), Some(b']')) => {
                data_iter.next().unwrap();
                last = Some(Final(Class(true, Vec::new())))
            }
            (_, Some(NonFinal(OpenClassNeg)), Some(b']')) => {
                data_iter.next().unwrap();
                last = Some(Final(Class(false, Vec::new())))
            }
            (_, Some(NonFinal(OpenClass(..))), Some(b']')) => {
                data_iter.next().unwrap();
                let (incl, items) = last.take().unwrap().unwrap_non_final().unwrap_open_class();
                last = Some(Final(Class(incl, items)))
            }
            (Some(NonFinal(OpenClass(..))), Some(NonFinal(non_final)), Some(b']')) => {
                return Err(non_final.reason())
            }

            // Bytes inside classes.
            // These must come before all of the generic `(_, _, b'X')` rules below.
            (_, Some(NonFinal(OpenClass0)), Some(b))
            | (_, Some(NonFinal(OpenClassNeg)), Some(b))
            | (_, Some(NonFinal(OpenClass(..))), Some(b))
                if b != b']' =>
            {
                data_iter.next().unwrap();
                to_push = Some(Final(Byte(b)))
            }

            // Repeat, postfix operators `?` `+` `*` `{n}` `{n,}` `{,m}` `{n,m}`
            (_, _, Some(b'?')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("?".to_string(), 0, Some(1))))
            }
            (_, _, Some(b'*')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("*".to_string(), 0, None)))
            }
            (_, _, Some(b'+')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(RepeatToken("+".to_string(), 1, None)))
            }
            (_, _, Some(b'{')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(RepeatMin(String::new())))
            }
            (_, Some(NonFinal(RepeatMin(_))), Some(b',')) => {
                data_iter.next().unwrap();
                let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
                last = Some(NonFinal(RepeatMax(min, String::new())))
            }
            (_, Some(NonFinal(RepeatMin(_))), Some(b'}')) => {
                data_iter.next().unwrap();
                let min = last.take().unwrap().unwrap_non_final().unwrap_repeat_min();
                let min_usize = usize::from_str_radix(&min, 10)
                    .map_err(|e| format!("invalid repetition value `{{{}}}`: {}", min, e))?;
                last = Some(NonFinal(RepeatToken(
                    format!("{{{}}}", min),
                    min_usize,
                    Some(min_usize),
                )))
            }
            (_, Some(NonFinal(RepeatMin(min))), Some(b)) => {
                data_iter.next().unwrap();
                min.push(char::from(b))
            }
            (_, Some(NonFinal(RepeatMax(..))), Some(b'}')) => {
                data_iter.next().unwrap();
                let (min, max) = last.take().unwrap().unwrap_non_final().unwrap_repeat_max();
                let min_usize = if min.is_empty() {
                    0
                } else {
                    usize::from_str_radix(&min, 10).map_err(|e| {
                        format!("invalid repetition value `{{{},{}}}`: {}", min, max, e)
                    })?
                };
                let max_opt_usize = if max.is_empty() {
                    None
                } else {
                    let max_usize = usize::from_str_radix(&max, 10).map_err(|e| {
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
                last = Some(NonFinal(RepeatToken(
                    format!("{{{},{}}}", min, max),
                    min_usize,
                    max_opt_usize,
                )))
            }
            (_, Some(NonFinal(RepeatMax(_, ref mut max))), Some(b)) => {
                data_iter.next().unwrap();
                max.push(char::from(b));
            }

            // Any byte `.`
            (_, _, Some(b'.')) => {
                data_iter.next().unwrap();
                to_push = Some(Final(AnyByte))
            }

            // Alternate/Or `a|b|c`
            (_, Some(Final(Or(_))), Some(b'|')) => {
                data_iter.next().unwrap();
                let nodes = last.take().unwrap().unwrap_final().unwrap_or();
                last = Some(NonFinal(OpenOr(nodes)))
            }
            (_, Some(Final(_)), Some(b'|')) => {
                data_iter.next().unwrap();
                let node = last.take().unwrap().unwrap_final();
                last = Some(NonFinal(OpenOr(vec![node])))
            }
            (_, None, Some(b'|')) => return Err("missing element before bar `|`".to_string()),

            // Group `(ab)`
            (_, _, Some(b'(')) => {
                data_iter.next().unwrap();
                to_push = Some(NonFinal(OpenGroup))
            }
            (_, Some(NonFinal(OpenGroup)), Some(b')')) => {
                data_iter.next().unwrap();
                last = Some(Final(Group(Box::new(Seq(vec![])))))
            }
            (Some(NonFinal(OpenGroup)), Some(NonFinal(non_final)), Some(b')')) => {
                return Err(non_final.reason())
            }
            (Some(NonFinal(OpenGroup)), Some(Final(_)), Some(b')')) => {
                data_iter.next().unwrap();
                let node = last.take().unwrap().unwrap_final();
                prev = Some(Final(Group(Box::new(node))));
            }

            // Other bytes
            (_, _, Some(b)) => {
                data_iter.next().unwrap();
                to_push = Some(Final(Byte(b)))
            }

            // No more bytes.
            (_, Some(NonFinal(node)), None) => return Err(node.reason()),
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
            (Some(NonFinal(OpenOr(_))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(OpenByteRange(_))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(ByteRange(..))), Some(Final(_)), _) => unreachable!(),
            (Some(NonFinal(RepeatToken(..))), Some(Final(_)), _) => unreachable!(),
            (Some(Final(_)), Some(Final(_)), _) => unreachable!(),
        };
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
    }
    // println!("stack {:?}", stack);
    // Check for incomplete elements.  Example: br"(ab"
    for node in stack.iter().rev() {
        if let NonFinal(non_final) = node {
            return Err(non_final.reason());
        }
    }
    assert_eq!(1, stack.len());
    Ok(stack.pop().unwrap().unwrap_final())
}
