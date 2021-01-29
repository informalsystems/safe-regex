use safe_regex::Regex;
use safe_regex::{any_byte, bytes, group, or, or3, or4, or5, seq, seq3, seq4, seq5};
use std::cell::Cell;

#[test]
fn test_match_all() {
    assert_eq!(Some(()), "".match_all(b""));
    assert_eq!(None, "".match_all(b"a"));
    assert_eq!(Some(()), "b".match_all(b"b"));
    assert_eq!(None, "b".match_all(b"a"));
    assert_eq!(None, "b".match_all(b"ab"));
    assert_eq!(None, "b".match_all(b"bc"));
    assert_eq!(None, "b".match_all(b"abc"));
    assert_eq!(Some(()), "abc".match_all(b"abc"));
    assert_eq!(None, "abc".match_all(b"Xabc"));
    assert_eq!(None, "abc".match_all(b"abcY"));
    assert_eq!(None, "abc".match_all(b"XabcY"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"ac"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abc"));
    assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abbbc"));
}

#[test]
fn test_match_prefix() {
    assert_eq!(Some(0), "".match_prefix(b""));
    assert_eq!(Some(0), "".match_prefix(b"a"));
    assert_eq!(Some(1), "b".match_prefix(b"b"));
    assert_eq!(None, "b".match_prefix(b"a"));
    assert_eq!(None, "b".match_prefix(b"ab"));
    assert_eq!(Some(1), "b".match_prefix(b"bc"));
    assert_eq!(None, "b".match_prefix(b"abc"));
    assert_eq!(Some(3), "abc".match_prefix(b"abc"));
    assert_eq!(None, "abc".match_prefix(b"Xabc"));
    assert_eq!(Some(3), "abc".match_prefix(b"abcY"));
    assert_eq!(None, "abc".match_prefix(b"XabcY"));

    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"a"));
    assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"b"));
    assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"bY"));
    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xb"));
    assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"ab"));
    assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"abY"));
    assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xab"));

    assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"a"));
    assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"ad"));
    assert_eq!(Some(2), seq("a", ("b", 0..)).match_prefix(b"ab"));
    assert_eq!(Some(3), seq("a", ("b", 0..)).match_prefix(b"abb"));
    assert_eq!(Some(4), seq("a", ("b", 0..)).match_prefix(b"abbb"));

    assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"ac"));
    assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"acd"));
    assert_eq!(Some(3), seq("a", seq(("b", 0..), "c")).match_prefix(b"abc"));
    assert_eq!(
        Some(3),
        seq("a", seq(("b", 0..), "c")).match_prefix(b"abcd")
    );
    assert_eq!(
        Some(4),
        seq("a", seq(("b", 0..), "c")).match_prefix(b"abbcd")
    );
}

#[test]
fn test_match_suffix() {
    assert_eq!(Some(0..0), "".match_suffix(b""));
    assert_eq!(Some(1..1), "".match_suffix(b"a"));
    assert_eq!(Some(0..1), "b".match_suffix(b"b"));
    assert_eq!(None, "b".match_suffix(b"a"));
    assert_eq!(Some(1..2), "b".match_suffix(b"ab"));
    assert_eq!(None, "b".match_suffix(b"bc"));
    assert_eq!(None, "b".match_suffix(b"abc"));
    assert_eq!(Some(0..3), "abc".match_suffix(b"abc"));
    assert_eq!(Some(1..4), "abc".match_suffix(b"Xabc"));
    assert_eq!(None, "abc".match_suffix(b"abcY"));
    assert_eq!(None, "abc".match_suffix(b"XabcY"));

    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"a"));
    assert_eq!(Some(0..1), seq(("a", 0..), "b").match_suffix(b"b"));
    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"bY"));
    assert_eq!(Some(1..2), seq(("a", 0..), "b").match_suffix(b"Xb"));
    assert_eq!(Some(0..2), seq(("a", 0..), "b").match_suffix(b"ab"));
    assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"abY"));
    assert_eq!(Some(1..3), seq(("a", 0..), "b").match_suffix(b"Xab"));

    assert_eq!(Some(0..1), seq("a", ("b", 0..)).match_suffix(b"a"));
    assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"aY"));
    assert_eq!(Some(1..2), seq("a", ("b", 0..)).match_suffix(b"Xa"));
    assert_eq!(Some(0..2), seq("a", ("b", 0..)).match_suffix(b"ab"));
    assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"abY"));
    assert_eq!(Some(1..3), seq("a", ("b", 0..)).match_suffix(b"Xab"));
    assert_eq!(Some(0..3), seq("a", ("b", 0..)).match_suffix(b"abb"));
    assert_eq!(Some(0..4), seq("a", ("b", 0..)).match_suffix(b"abbb"));

    assert_eq!(
        Some(0..2),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"ac")
    );
    assert_eq!(None, seq("a", seq(("b", 0..), "c")).match_suffix(b"acY"));
    assert_eq!(
        Some(1..3),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xac")
    );
    assert_eq!(
        Some(0..3),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"abc")
    );
    assert_eq!(
        Some(1..4),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabc")
    );
    assert_eq!(
        Some(1..5),
        seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabbc")
    );
}

#[test]
fn test_search() {}

#[test]
fn test_impl_for_str() {
    assert_eq!(Some(0..0), "".search(b""));
    assert_eq!(Some(1..3), "bb".search(b"abbc"));
}

#[test]
fn test_impl_for_string() {
    assert_eq!(Some(0..0), String::new().search(b""));
    assert_eq!(Some(1..3), String::from("bb").search(b"abbc"));
}

#[test]
fn test_impl_for_u8_slice() {
    // Empty pattern
    assert_eq!(Some(0..0), b"".as_ref().search(b""));
    assert_eq!(Some(0..0), b"".as_ref().search(b"a"));
    // // Whole string matches
    assert_eq!(Some(0..2), b"bb".as_ref().search(b"bb"));
    // Matches at beginning
    assert_eq!(Some(0..2), b"bb".as_ref().search(b"bbc"));
    // Matches at end
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abb"));
    // Matches in middle
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbb"));
    // Does not accept partial matches
    assert_eq!(None, b"abc".as_ref().search(b"ab"));
    assert_eq!(None, b"abc".as_ref().search(b"abd"));
    assert_eq!(None, b"abc".as_ref().search(b"bc"));
    assert_eq!(None, b"abc".as_ref().search(b"bcd"));
    // Check returned range
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbbc"));
    assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbcbbc"));
}

#[test]
fn test_bytes() {
    assert_eq!(Some(0..0), bytes(b"").search(b""));
    assert_eq!(Some(1..3), bytes(b"bb").search(b"abbc"));

    let value = bytes(b"abc");
    let value_copy = value; // Copy
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Bytes { bytes: [97, 98, 99], phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < bytes(b"def")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_seq() {
    assert_eq!(Some(0..4), seq("ab", "cd").search(b"abcdX"));
    assert_eq!(Some(1..5), seq("ab", "cd").search(b"Xabcd"));
    assert_eq!(None, seq("ab", "cd").search(b"abXcd"));
    assert_eq!(Some(1..4), seq("a", seq("b", "c")).search(b"XabcY"));

    let value = seq("a", "b");
    let value_copy = value; // Copy
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Seq { a: \"a\", b: \"b\", phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < seq("d", "d")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_seq3() {
    assert_eq!(Some(0..6), seq3("ab", "cd", "ef").search(b"abcdefX"));
    assert_eq!(Some(1..7), seq3("ab", "cd", "ef").search(b"Xabcdef"));
    assert_eq!(None, seq3("ab", "cd", "ef").search(b"abXcdef"));
    assert_eq!(Some(1..4), seq3("a", "b", "c").search(b"XabcY"));
}

#[test]
fn test_seq4() {
    assert_eq!(
        Some(0..8),
        seq4("ab", "cd", "ef", "gh").search(b"abcdefghX")
    );
    assert_eq!(
        Some(1..9),
        seq4("ab", "cd", "ef", "gh").search(b"Xabcdefgh")
    );
    assert_eq!(None, seq4("ab", "cd", "ef", "gh").search(b"abXcdefgh"));
    assert_eq!(Some(1..5), seq4("a", "b", "c", "d").search(b"XabcdY"));
}

#[test]
fn test_seq5() {
    assert_eq!(
        Some(0..10),
        seq5("ab", "cd", "ef", "gh", "ij").search(b"abcdefghijX")
    );
    assert_eq!(
        Some(1..11),
        seq5("ab", "cd", "ef", "gh", "ij").search(b"Xabcdefghij")
    );
    assert_eq!(
        None,
        seq5("ab", "cd", "ef", "gh", "ij").search(b"abXcdefghij")
    );
    assert_eq!(Some(1..6), seq5("a", "b", "c", "d", "e").search(b"XabcdeY"));
}

#[test]
fn test_range() {
    assert_eq!(None, (..).search(b""));
    assert_eq!(Some(0..1), (..).search(b"a"));

    assert_eq!(None, (b'b'..).search(b""));
    assert_eq!(None, (b'b'..).search(b"a"));
    assert_eq!(Some(0..1), (b'b'..).search(b"b"));
    assert_eq!(Some(0..1), (b'b'..).search(b"c"));

    assert_eq!(None, (..b'c').search(b""));
    assert_eq!(Some(0..1), (..b'c').search(b"a"));
    assert_eq!(Some(0..1), (..b'c').search(b"b"));
    assert_eq!(None, (..b'c').search(b"c"));

    assert_eq!(None, (..=b'b').search(b""));
    assert_eq!(Some(0..1), (..=b'b').search(b"a"));
    assert_eq!(Some(0..1), (..=b'b').search(b"b"));
    assert_eq!(None, (..=b'b').search(b"c"));

    assert_eq!(None, (b'b'..b'd').search(b""));
    assert_eq!(None, (b'b'..b'd').search(b"a"));
    assert_eq!(Some(0..1), (b'b'..b'd').search(b"b"));
    assert_eq!(Some(0..1), (b'b'..b'd').search(b"c"));
    assert_eq!(None, (b'b'..b'd').search(b"d"));

    assert_eq!(None, (b'b'..=b'c').search(b""));
    assert_eq!(None, (b'b'..=b'c').search(b"a"));
    assert_eq!(Some(0..1), (b'b'..=b'c').search(b"b"));
    assert_eq!(Some(0..1), (b'b'..=b'c').search(b"c"));
    assert_eq!(None, (b'b'..=b'c').search(b"d"));
}

#[test]
fn test_repeat_range() {
    // zero of, '{0}'
    assert_eq!(Some(0..0), ("b", ..=0).search(b""));
    assert_eq!(Some(0..0), (("b", ..=1), ..=1).search(b"a"));

    // zero or one, '?', '{0,1}'
    assert_eq!(Some(0..0), ("b", ..=1).search(b""));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"a"));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"ab"));
    assert_eq!(Some(0..1), ("b", ..=1).search(b"b"));
    assert_eq!(Some(0..1), ("b", ..=1).search(b"bb"));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b""));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b"a"));
    assert_eq!(Some(0..0), ("bc", ..=1).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", ..=1).search(b"bc"));
    assert_eq!(Some(0..2), ("bc", ..=1).search(b"bcbc"));

    // zero or more, '*', '{0,}'
    assert_eq!(Some(0..0), ("b", ..).search(b""));
    assert_eq!(Some(0..0), ("b", ..).search(b"a"));
    assert_eq!(Some(0..0), ("b", ..).search(b"ab"));
    assert_eq!(Some(0..1), ("b", ..).search(b"b"));
    assert_eq!(Some(0..4), ("b", ..).search(b"bbbb"));
    assert_eq!(Some(0..0), ("bc", ..).search(b""));
    assert_eq!(Some(0..0), ("bc", ..).search(b"a"));
    assert_eq!(Some(0..0), ("bc", ..).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", ..).search(b"bc"));
    assert_eq!(Some(0..4), ("bc", ..).search(b"bcbc"));

    // one or more, '+', '{1,}'
    assert_eq!(None, ("b", 1..).search(b""));
    assert_eq!(None, ("b", 1..).search(b"a"));
    assert_eq!(Some(1..2), ("b", 1..).search(b"ab"));
    assert_eq!(Some(0..1), ("b", 1..).search(b"b"));
    assert_eq!(Some(0..4), ("b", 1..).search(b"bbbb"));
    assert_eq!(None, ("bc", 1..).search(b""));
    assert_eq!(None, ("bc", 1..).search(b"a"));
    assert_eq!(Some(1..3), ("bc", 1..).search(b"abc"));
    assert_eq!(Some(0..2), ("bc", 1..).search(b"bc"));
    assert_eq!(Some(1..3), ("bc", 1..).search(b"bbc"));
    assert_eq!(Some(0..4), ("bc", 1..).search(b"bcbc"));

    // n of, '{n}'
    assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));
    assert_eq!(None, ("b", 1..=1).search(b""));
    assert_eq!(Some(0..1), ("b", 1..=1).search(b"b"));

    assert_eq!(None, ("b", 2..=2).search(b""));
    assert_eq!(None, ("b", 2..=2).search(b"aaa"));
    assert_eq!(None, ("b", 2..=2).search(b"abaa"));
    assert_eq!(Some(1..3), ("b", 2..=2).search(b"abb"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bb"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbc"));
    assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbbb"));

    assert_eq!(None, ("bc", 2..=2).search(b""));
    assert_eq!(None, ("bc", 2..=2).search(b"aa"));
    assert_eq!(None, ("bc", 2..=2).search(b"abb"));
    assert_eq!(None, ("bc", 2..=2).search(b"ccd"));
    assert_eq!(Some(1..5), ("bc", 2..=2).search(b"abcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbc"));
    assert_eq!(Some(1..5), ("bc", 2..=2).search(b"bbcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbcbcbc"));

    // m to n of, '{m,n}'
    assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));

    assert_eq!(Some(0..0), ("b", 0..=1).search(b""));
    assert_eq!(Some(0..0), ("b", 0..=1).search(b"a"));
    assert_eq!(Some(0..0), ("b", 0..=1).search(b"ab"));

    assert_eq!(None, ("b", 1..=2).search(b""));
    assert_eq!(None, ("b", 1..=2).search(b"a"));
    assert_eq!(Some(0..1), ("b", 1..=2).search(b"b"));
    assert_eq!(Some(0..1), ("b", 1..=2).search(b"bc"));
    assert_eq!(Some(1..2), ("b", 1..=2).search(b"ab"));
    assert_eq!(Some(1..2), ("b", 1..=2).search(b"abc"));
    assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbc"));
    assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbbbc"));

    assert_eq!(None, ("b", 2..=4).search(b""));
    assert_eq!(None, ("b", 2..=4).search(b"aa"));
    assert_eq!(None, ("b", 2..=4).search(b"ab"));
    assert_eq!(None, ("b", 2..=4).search(b"abc"));
    assert_eq!(Some(0..2), ("b", 2..=4).search(b"bb"));
    assert_eq!(Some(0..2), ("b", 2..=4).search(b"bbcc"));
    assert_eq!(Some(2..4), ("b", 2..=4).search(b"aabb"));
    assert_eq!(Some(1..3), ("b", 2..=4).search(b"abbc"));
    assert_eq!(Some(1..4), ("b", 2..=4).search(b"abbbc"));
    assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbc"));
    assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbbbbbc"));

    assert_eq!(None, ("bc", 2..=4).search(b""));
    assert_eq!(None, ("bc", 2..=4).search(b"aaaa"));
    assert_eq!(None, ("bc", 2..=4).search(b"abc"));
    assert_eq!(None, ("bc", 2..=4).search(b"abcb"));
    assert_eq!(None, ("bc", 2..=4).search(b"abcd"));
    assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbc"));
    assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbcdddd"));
    assert_eq!(Some(4..8), ("bc", 2..=4).search(b"aaaabcbc"));
    assert_eq!(Some(1..5), ("bc", 2..=4).search(b"abcbcd"));
    assert_eq!(Some(2..6), ("bc", 2..=4).search(b"abbcbc"));
    assert_eq!(Some(1..7), ("bc", 2..=4).search(b"abcbcbc"));
    assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbc"));
    assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbcbc"));

    assert_eq!(Some(0..0), ("b", ..).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..).search(b"abc"));
    assert_eq!(Some(0..0), ("b", ..2).search(b"abc"));
    assert_eq!(Some(0..0), ("b", ..=1).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..2).search(b"abc"));
    assert_eq!(Some(1..2), ("b", 1..=1).search(b"abc"));
}

#[test]
fn test_any_byte() {
    assert_eq!(None, any_byte().search(b""));
    assert_eq!(Some(0..1), any_byte().search(b"a"));
    assert_eq!(Some(0..1), any_byte().search(b"ab"));

    let value = any_byte();
    let value_copy = value; // Copy
    let _value_clone = value.clone(); // Clone
    assert_eq!("AnyByte", format!("{:?}", value)); // Debug
    assert!(!(value < any_byte())); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_or() {
    assert_eq!(None, or("a", "b").search(b""));
    assert_eq!(Some(1..2), or("a", "b").search(b"XaY"));
    assert_eq!(Some(1..2), or("a", "b").search(b"XbY"));
    assert_eq!(Some(0..1), or("a", "b").search(b"ab"));
    assert_eq!(None, or("a", "b").search(b"XY"));

    let value = or("a", "b");
    let value_copy = value; // Copy
    let _value_clone = value.clone(); // Clone
    assert_eq!(
        "Or { a: \"a\", b: \"b\", phantom: PhantomData }",
        format!("{:?}", value)
    ); // Debug
    assert!(value < or("d", "d")); // PartialOrd
    assert_eq!(value, value_copy); // PartialEq
}

#[test]
fn test_or3() {
    assert_eq!(None, or3("a", "b", "c").search(b""));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XaY"));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XbY"));
    assert_eq!(Some(1..2), or3("a", "b", "c").search(b"XcY"));
    assert_eq!(Some(0..1), or3("a", "b", "c").search(b"abc"));
    assert_eq!(None, or3("a", "b", "c").search(b"XY"));
}

#[test]
fn test_or4() {
    assert_eq!(None, or4("a", "b", "c", "d").search(b""));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XaY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XbY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XcY"));
    assert_eq!(Some(1..2), or4("a", "b", "c", "d").search(b"XdY"));
    assert_eq!(Some(0..1), or4("a", "b", "c", "d").search(b"abcd"));
    assert_eq!(None, or4("a", "b", "c", "d").search(b"XY"));
}

#[test]
fn test_or5() {
    assert_eq!(None, or5("a", "b", "c", "d", "e").search(b""));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XaY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XbY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XcY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XdY"));
    assert_eq!(Some(1..2), or5("a", "b", "c", "d", "e").search(b"XeY"));
    assert_eq!(Some(0..1), or5("a", "b", "c", "d", "e").search(b"abcde"));
    assert_eq!(None, or5("a", "b", "c", "d", "e").search(b"XY"));
}

#[test]
fn test_group() {
    let cell: Cell<Option<&[u8]>> = Cell::new(None);
    assert_eq!(Some(0..0), group(&cell, "").search(b""));
    assert_eq!(b"", cell.get().unwrap());
    cell.take();
    assert_eq!(None, group(&cell, "b").search(b"a"));
    assert_eq!(None, cell.get());
    cell.take();
    assert_eq!(Some(1..3), group(&cell, "bb").search(b"abb"));
    assert_eq!(b"bb", cell.get().unwrap());
    cell.take();
    assert_eq!(Some(0..2), group(&cell, "bb").search(b"bbc"));
    assert_eq!(b"bb", cell.get().unwrap());
    cell.take();
    assert_eq!(Some(1..3), group(&cell, "bb").search(b"abbc"));
    assert_eq!(b"bb", cell.get().unwrap());
    {
        let cell_b: Cell<Option<&[u8]>> = Cell::new(None);
        let cell_d: Cell<Option<&[u8]>> = Cell::new(None);
        assert_eq!(
            Some(1..8),
            seq5(
                "a",
                group(&cell_b, ("b", 1..)),
                ("c", 1..),
                group(&cell_d, ("d", 1..)),
                ("e", 1..),
            )
            .search(b"XabcdddeY")
        );
        assert_eq!(b"b", cell_b.get().unwrap());
        assert_eq!(b"ddd", cell_d.get().unwrap());
    }
    {
        let cell_b: Cell<Option<&[u8]>> = Cell::new(None);
        let cell_d: Cell<Option<&[u8]>> = Cell::new(None);
        assert_eq!(
            None,
            seq5(
                "a",
                group(&cell_b, ("b", 1..)),
                ("c", 1..),
                group(&cell_d, ("d", 1..)),
                ("e", 1..),
            )
            .search(b"abbde")
        );
        assert_eq!(b"bb", cell_b.get().unwrap());
        assert_eq!(None, cell_d.get());
    }

    {
        let cell: Cell<Option<&[u8]>> = Cell::new(None);
        let value = group(&cell, "a");
        let value_copy = value; // Copy
        let _value_clone = value.clone(); // Clone
        assert_eq!("Group(Cell { value: None }, \"a\")", format!("{:?}", value)); // Debug
        assert!(value < group(&cell, "b")); // PartialOrd
        assert_eq!(value, value_copy); // PartialEq
    }
}
