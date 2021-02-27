#![forbid(unsafe_code)]
#![allow(clippy::too_many_lines)]
use safe_regex_compiler::parser::FinalNode::{Alt, AnyByte, Byte, Class, Group, Repeat, Seq};
use safe_regex_compiler::parser::{parse, ClassItem};

#[test]
fn test() {
    assert_eq!(Ok(Seq(Vec::new())), parse(br""));
    assert_eq!(Ok(Byte(b'a')), parse(br"a"));
    assert_eq!(
        Ok(Seq(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')])),
        parse(br"abc")
    );
    assert_eq!(Ok(AnyByte), parse(br"."));
}

#[test]
fn escapes() {
    assert_eq!(
        Err(r"incomplete escape sequence: `\`".to_string()),
        parse(br"\")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\e`".to_string()),
        parse(br"\e")
    );
    // Rust byte escapes
    // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
    assert_eq!(Ok(Byte(b'\n')), parse(br"\n"));
    assert_eq!(Ok(Byte(b'\r')), parse(br"\r"));
    assert_eq!(Ok(Byte(b'\t')), parse(br"\t"));
    assert_eq!(Ok(Byte(b'\\')), parse(br"\\"));
    assert_eq!(Ok(Byte(0)), parse(br"\0"));
    assert_eq!(
        Err(r"incomplete escape sequence: `\x`".to_string()),
        parse(br"\x")
    );
    assert_eq!(
        Err(r"incomplete escape sequence: `\x0`".to_string()),
        parse(br"\x0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\xg0`".to_string()),
        parse(br"\xg0")
    );
    assert_eq!(
        Err(r"invalid escape sequence `\x0g`".to_string()),
        parse(br"\x0g")
    );
    assert_eq!(Ok(Byte(0)), parse(br"\x00"));
    assert_eq!(Ok(Byte(0x12)), parse(br"\x12"));
    assert_eq!(Ok(Byte(0x34)), parse(br"\x34"));
    assert_eq!(Ok(Byte(0x56)), parse(br"\x56"));
    assert_eq!(Ok(Byte(0x78)), parse(br"\x78"));
    assert_eq!(Ok(Byte(0x90)), parse(br"\x90"));
    assert_eq!(Ok(Byte(0xAB)), parse(br"\xab"));
    assert_eq!(Ok(Byte(0xAB)), parse(br"\xAB"));
    assert_eq!(Ok(Byte(0xCD)), parse(br"\xcd"));
    assert_eq!(Ok(Byte(0xCD)), parse(br"\xCD"));
    assert_eq!(Ok(Byte(0xEF)), parse(br"\xef"));
    assert_eq!(Ok(Byte(0xEF)), parse(br"\xEF"));
    assert_eq!(Ok(Byte(0xFF)), parse(br"\xFF"));
    // Rust quote escapes
    // https://doc.rust-lang.org/reference/tokens.html#quote-escapes
    assert_eq!(Ok(Byte(b'\'')), parse(br"\'"));
    assert_eq!(Ok(Byte(b'"')), parse(br#"\""#));
    // Regex escapes
    assert_eq!(Ok(Byte(b'?')), parse(br"\?"));
    assert_eq!(Ok(Byte(b'+')), parse(br"\+"));
    assert_eq!(Ok(Byte(b'.')), parse(br"\."));
    assert_eq!(Ok(Byte(b'*')), parse(br"\*"));
    assert_eq!(Ok(Byte(b'^')), parse(br"\^"));
    assert_eq!(Ok(Byte(b'$')), parse(br"\$"));
    assert_eq!(Ok(Byte(b'|')), parse(br"\|"));
    assert_eq!(Ok(Byte(b'(')), parse(br"\("));
    assert_eq!(Ok(Byte(b')')), parse(br"\)"));
    assert_eq!(Ok(Byte(b'{')), parse(br"\{"));
    assert_eq!(Ok(Byte(b'}')), parse(br"\}"));
    assert_eq!(Ok(Byte(b'[')), parse(br"\["));
    assert_eq!(Ok(Byte(b']')), parse(br"\]"));
}

#[test]
fn or() {
    assert_eq!(
        Err(r"missing element before bar `|`".to_string()),
        parse(br"|")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
        parse(br"a|")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
        parse(br"(a|)")
    );
    assert_eq!(
        Err(r"missing element after bar `|`".to_string()),
        parse(br"(a|bc|)d")
    );
    assert_eq!(Ok(Alt(vec![Byte(b'a'), Byte(b'b')])), parse(br"a|b"));
    assert_eq!(
        Ok(Alt(vec![Byte(b'a'), Byte(b'b'), Byte(b'c')])),
        parse(br"a|b|c")
    );
    assert_eq!(
        Ok(Alt(vec![
            Seq(vec![Byte(b'a'), Byte(b'b')]),
            Seq(vec![Byte(b'c'), Byte(b'd'), Byte(b'e')]),
            Seq(vec![Byte(b'f'), Byte(b'g')])
        ])),
        parse(br"ab|cde|fg")
    );
}

#[test]
fn class() {
    assert_eq!(Err("missing closing `]`".to_string()), parse(br"[a"));
    assert_eq!(Err("missing closing `]`".to_string()), parse(br"[^a"));
    assert_eq!(Ok(Class(true, vec![])), parse(br"[]"));
    assert_eq!(Ok(Class(false, vec![])), parse(br"[^]"));
    assert_eq!(Ok(Class(true, vec![ClassItem::Byte(b'a')])), parse(br"[a]"));
    assert_eq!(
        Ok(Class(false, vec![ClassItem::Byte(b'a')])),
        parse(br"[^a]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![ClassItem::Byte(b'^'), ClassItem::Byte(b'a')]
        )),
        parse(br"[^^a]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b'),
                ClassItem::Byte(b'c')
            ]
        )),
        parse(br"[abc]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b'),
                ClassItem::Byte(b'c')
            ]
        )),
        parse(br"[^abc]")
    );
    // ?+*.^$|(){}[]
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'?'),
                ClassItem::Byte(b'+'),
                ClassItem::Byte(b'*'),
                ClassItem::Byte(b'.'),
                ClassItem::Byte(b'^'),
                ClassItem::Byte(b'$'),
                ClassItem::Byte(b'|'),
                ClassItem::Byte(b'('),
                ClassItem::Byte(b')'),
                ClassItem::Byte(b'{'),
                ClassItem::Byte(b'}'),
                ClassItem::Byte(b'['),
                ClassItem::Byte(b']'),
            ]
        )),
        parse(br"[?+*.^$|(){}[\]]")
    );

    assert_eq!(
        Err("missing byte to close range: `b-`".to_string()),
        parse(br"[ab-]")
    );
    assert_eq!(
        Err("missing byte to close range: `a-`".to_string()),
        parse(br"[^a-]")
    );
    assert_eq!(
        Err("expected byte before '-' symbol, not range: `a-b-`".to_string()),
        parse(br"[a-b-]")
    );
    assert_eq!(
        Ok(Class(
            false,
            vec![ClassItem::Byte(b'-'), ClassItem::Byte(b'a')]
        )),
        parse(br"[^-a]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::ByteRange(b'^', b'a')])),
        parse(br"[^^-a]")
    );
    assert_eq!(
        Ok(Class(true, vec![ClassItem::ByteRange(b'a', b'c')])),
        parse(br"[a-c]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::ByteRange(b'a', b'c')])),
        parse(br"[^a-c]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::ByteRange(b'a', b'c'),
                ClassItem::ByteRange(b'g', b'h')
            ]
        )),
        parse(br"[a-cg-h]")
    );
    assert_eq!(
        Ok(Class(
            true,
            vec![
                ClassItem::Byte(b'-'),
                ClassItem::Byte(b'a'),
                ClassItem::Byte(b'b')
            ]
        )),
        parse(br"[-ab]")
    );
    assert_eq!(
        Ok(Class(false, vec![ClassItem::Byte(b'-'),])),
        parse(br"[^-]")
    );
}

#[test]
fn group() {
    assert_eq!(Err("missing closing `)`".to_string()), parse(br"(."));
    assert_eq!(Ok(Group(Box::new(Seq(vec![])))), parse(br"()"));
    assert_eq!(Ok(Group(Box::new(AnyByte))), parse(br"(.)"));
    assert_eq!(
        Ok(Group(Box::new(Group(Box::new(AnyByte))))),
        parse(br"((.))")
    );
    assert_eq!(
        Ok(Group(Box::new(Seq(vec![
            AnyByte,
            Group(Box::new(AnyByte))
        ])))),
        parse(br"(.(.))")
    );
}

#[test]
fn question_mark() {
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"?")
    );
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"b|?")
    );
    assert_eq!(
        Err("missing element before repeat element: `?`".to_string()),
        parse(br"(?)")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(1))), parse(br".?"));
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 0, Some(1)),
        ])),
        parse(br"...?")
    );
    assert_eq!(
        Ok(Alt(vec![
            Byte(b'a'),
            Repeat(Box::new(Byte(b'b')), 0, Some(1)),
        ])),
        parse(br"a|b?")
    );
    assert_eq!(
        Ok(Alt(vec![
            Byte(b'a'),
            Seq(vec![Repeat(Box::new(Byte(b'b')), 0, Some(1)), Byte(b'c')]),
        ])),
        parse(br"a|b?c")
    );
    assert_eq!(
        Ok(Alt(vec![
            Byte(b'a'),
            Seq(vec![Byte(b'b'), Repeat(Box::new(Byte(b'c')), 0, Some(1))]),
        ])),
        parse(br"a|bc?")
    );
}

#[test]
fn star() {
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"*")
    );
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"b|*")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".*"));
    assert_eq!(
        Err("missing element before repeat element: `*`".to_string()),
        parse(br"(*)")
    );
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 0, None),
        ])),
        parse(br"...*")
    );
}

#[test]
fn plus() {
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"+")
    );
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"b|+")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, None)), parse(br".+"));
    assert_eq!(
        Err("missing element before repeat element: `+`".to_string()),
        parse(br"(+)")
    );
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 1, None),
        ])),
        parse(br"...+")
    );
}

#[test]
fn repeat_single_num() {
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"{1}")
    );
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"(ab|{1})")
    );
    assert_eq!(
        Err("missing element before repeat element: `{1}`".to_string()),
        parse(br"({1})")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1`".to_string()),
        parse(br".{1")
    );
    assert_eq!(
        Err("invalid repetition value `{}`: cannot parse integer from empty string".to_string()),
        parse(br".{}")
    );
    assert_eq!(
        Err("invalid repetition value `{a}`: invalid digit found in string".to_string()),
        parse(br".{a}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, Some(1))), parse(br".{1}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 99, Some(99))),
        parse(br".{99}")
    );
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 1, Some(1)),
        ])),
        parse(br"...{1}")
    );
}

#[test]
fn repeat() {
    assert_eq!(
        Err("missing element before repeat element: `{,}`".to_string()),
        parse(br"{,}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{,`".to_string()),
        parse(br".{,")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".{,}"));
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 0, None),
        ])),
        parse(br"...{,}")
    );
}

#[test]
fn repeat_min() {
    assert_eq!(
        Err("missing element before repeat element: `{1,}`".to_string()),
        parse(br"{1,}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1,`".to_string()),
        parse(br".{1,")
    );
    assert_eq!(
        Err("invalid repetition value `{a,}`: invalid digit found in string".to_string()),
        parse(br".{a,}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, None)), parse(br".{0,}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, None)), parse(br".{1,}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 99, None)), parse(br".{99,}"));
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 1, None),
        ])),
        parse(br"...{1,}")
    );
}

#[test]
fn repeat_max() {
    assert_eq!(
        Err("missing element before repeat element: `{,1}`".to_string()),
        parse(br"{,1}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{,1`".to_string()),
        parse(br".{,1")
    );
    assert_eq!(
        Err("invalid repetition value `{,a}`: invalid digit found in string".to_string()),
        parse(br".{,a}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{,0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(1))), parse(br".{,1}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 0, Some(99))),
        parse(br".{,99}")
    );
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 0, Some(1)),
        ])),
        parse(br"...{,1}")
    );
}

#[test]
fn repeat_min_and_max() {
    assert_eq!(
        Err("missing element before repeat element: `{1,2}`".to_string()),
        parse(br"{1,2}")
    );
    assert_eq!(
        Err("missing closing `}` symbol: `{1,2`".to_string()),
        parse(br".{1,2")
    );
    assert_eq!(
        Err("invalid repetition value `{0,b}`: invalid digit found in string".to_string()),
        parse(br".{0,b}")
    );
    assert_eq!(
        Err("invalid repetition value `{a,1}`: invalid digit found in string".to_string()),
        parse(br".{a,1}")
    );
    assert_eq!(
        Err("invalid repetition value `{a,b}`: invalid digit found in string".to_string()),
        parse(br".{a,b}")
    );
    assert_eq!(
        Err("repeating element has max that is smaller than min: `{2,1}`".to_string()),
        parse(br".{2,1}")
    );
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 0, Some(0))), parse(br".{0,0}"));
    assert_eq!(Ok(Repeat(Box::new(AnyByte), 1, Some(2))), parse(br".{1,2}"));
    assert_eq!(
        Ok(Repeat(Box::new(AnyByte), 10, Some(99))),
        parse(br".{10,99}")
    );
    assert_eq!(
        Ok(Seq(vec![
            AnyByte,
            AnyByte,
            Repeat(Box::new(AnyByte), 1, Some(2)),
        ])),
        parse(br"...{1,2}")
    );
}

#[test]
fn precedence() {
    // Regular expressions have four types of syntax:
    // - Discrete tokens: . a [abc]
    // - Postfix operators: a? a* a+ a{n}.  These are unambiguous.
    // - Concatenation: ab
    // - Alternation/Or: a|b
    // We will test all combinations of these types to confirm correct parsing.
    // For example, we want to make sure that `a|bc` gets parsed as `a|(bc)` and
    // and not `(a|b)c`.
    // Postfix & Concatenation
    assert_eq!(
        Ok(Seq(vec![
            Byte(b'a'),
            Repeat(Box::new(Byte(b'b')), 0, Some(1))
        ])),
        parse(br"ab?")
    );
    // Postfix & Alternation
    assert_eq!(
        Ok(Alt(vec![
            Repeat(Box::new(Byte(b'a')), 0, None),
            Repeat(Box::new(Byte(b'b')), 0, None),
            Repeat(Box::new(Byte(b'c')), 0, None),
        ])),
        parse(br"a*|b*|c*")
    );
    // Concatenation & Alternation
    assert_eq!(
        Ok(Alt(vec![
            Seq(vec![Byte(b'a'), Byte(b'b')]),
            Seq(vec![Byte(b'c'), Byte(b'd')]),
            Seq(vec![Byte(b'e'), Byte(b'f')]),
        ])),
        parse(br"ab|cd|ef")
    );
}
