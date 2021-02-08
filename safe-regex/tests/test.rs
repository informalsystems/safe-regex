use safe_regex::{match_all, Byte, CapturingGroup, DiscardingRange, Either, Seq};

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

/// A function that implements regex `(b*)b`
// fn match1(data: &[u8]) -> Option<&[u8]> {
//     // let mut repeat1 = Repeat(&mut byte1, 0..);
//     // let mut group1 = Group(&mut repeat1);
//     // let mut byte2 = Byte(b'b');
//     // let mut seq1 = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
//     // seq1.check(data)
//     //     .map(|_| &data[group1.matching_range()])
//     //     .or(&[])
//     if seq1.match_all(data) {
//         Some(data)
//     } else {
//         None
//     }
// }

#[test]
fn byte() {
    let mut re = Byte::new(b'a');
    assert_eq!("Byte(a)", format!("{:?}", re));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"aa"));
}

#[test]
fn matcher_fn() {
    // assert_eq!(None, match1(b""));
    // assert_eq!(None, match1(b"a"));
    // assert_eq!(None, match1(b"b"));
    // assert_eq!(None, match1(b"c"));
    // assert_eq!(None, match1(b"X"));
    // assert_eq!(None, match1(b"a"));
    // assert_eq!(None, match1(b"ab"));
    // assert_eq!(None, match1(b"aba"));
    // assert_eq!(None, match1(b"abab"));
    // assert_eq!("abc", escape_ascii(match1(b"abc").unwrap()));
    // assert_eq!(None, match1(b"aabc"));
    // assert_eq!(None, match1(b"abca"));
    // assert_eq!(None, match1(b"YYYabc"));
    // assert_eq!(None, match1(b""));
    // assert_eq!(None, match1(b"ab"));
    // assert_eq!(None, match1(b"ba"));
    // assert_eq!("", escape_ascii(match1(b"b").unwrap()));
    // assert_eq!("b", escape_ascii(match1(b"bb").unwrap()));
    // assert_eq!("bb", escape_ascii(match1(b"bbb").unwrap()));
    // assert_eq!("bbb", escape_ascii(match1(b"bbbb").unwrap()));
    //
    // let input = Vec::from(&b"bbb"[..]);
    // let mut result = Vec::new();
    // if let Some(matching_part) = match1(&input) {
    //     result.extend_from_slice(matching_part);
    //     result.push(b'.');
    // }
    // assert_eq!("bb.", escape_ascii(&result));
}

// #[test]
// fn temporary() {
//     let class1 = safe_regex::RangeBytes(b'a'..=b'z');
//     let class2 = safe_regex::RangeBytes(b'0'..=b'9');
//     let seq1 = safe_regex::Seq(&mut class1, &mut class2);
//     if let Some(matching_part) = seq1.check(input) {}
// }

// #[test]
// fn test_match_all() {
//     assert_eq!(Some(()), "".match_all(b""));
//     assert_eq!(None, "".match_all(b"a"));
//     assert_eq!(Some(()), "b".match_all(b"b"));
//     assert_eq!(None, "b".match_all(b"a"));
//     assert_eq!(None, "b".match_all(b"ab"));
//     assert_eq!(None, "b".match_all(b"bc"));
//     assert_eq!(None, "b".match_all(b"abc"));
//     assert_eq!(Some(()), "abc".match_all(b"abc"));
//     assert_eq!(None, "abc".match_all(b"Xabc"));
//     assert_eq!(None, "abc".match_all(b"abcY"));
//     assert_eq!(None, "abc".match_all(b"XabcY"));
//     assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"ac"));
//     assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abc"));
//     assert_eq!(Some(()), seq("a", seq(("b", 0..), "c")).match_all(b"abbbc"));
// }
//
// #[test]
// fn test_match_prefix() {
//     assert_eq!(Some(0), "".match_prefix(b""));
//     assert_eq!(Some(0), "".match_prefix(b"a"));
//     assert_eq!(Some(1), "b".match_prefix(b"b"));
//     assert_eq!(None, "b".match_prefix(b"a"));
//     assert_eq!(None, "b".match_prefix(b"ab"));
//     assert_eq!(Some(1), "b".match_prefix(b"bc"));
//     assert_eq!(None, "b".match_prefix(b"abc"));
//     assert_eq!(Some(3), "abc".match_prefix(b"abc"));
//     assert_eq!(None, "abc".match_prefix(b"Xabc"));
//     assert_eq!(Some(3), "abc".match_prefix(b"abcY"));
//     assert_eq!(None, "abc".match_prefix(b"XabcY"));
//
//     assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"a"));
//     assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"b"));
//     assert_eq!(Some(1), seq(("a", 0..), "b").match_prefix(b"bY"));
//     assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xb"));
//     assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"ab"));
//     assert_eq!(Some(2), seq(("a", 0..), "b").match_prefix(b"abY"));
//     assert_eq!(None, seq(("a", 0..), "b").match_prefix(b"Xab"));
//
//     assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"a"));
//     assert_eq!(Some(1), seq("a", ("b", 0..)).match_prefix(b"ad"));
//     assert_eq!(Some(2), seq("a", ("b", 0..)).match_prefix(b"ab"));
//     assert_eq!(Some(3), seq("a", ("b", 0..)).match_prefix(b"abb"));
//     assert_eq!(Some(4), seq("a", ("b", 0..)).match_prefix(b"abbb"));
//
//     assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"ac"));
//     assert_eq!(Some(2), seq("a", seq(("b", 0..), "c")).match_prefix(b"acd"));
//     assert_eq!(Some(3), seq("a", seq(("b", 0..), "c")).match_prefix(b"abc"));
//     assert_eq!(
//         Some(3),
//         seq("a", seq(("b", 0..), "c")).match_prefix(b"abcd")
//     );
//     assert_eq!(
//         Some(4),
//         seq("a", seq(("b", 0..), "c")).match_prefix(b"abbcd")
//     );
// }
//
// #[test]
// fn test_match_suffix() {
//     assert_eq!(Some(0..0), "".match_suffix(b""));
//     assert_eq!(Some(1..1), "".match_suffix(b"a"));
//     assert_eq!(Some(0..1), "b".match_suffix(b"b"));
//     assert_eq!(None, "b".match_suffix(b"a"));
//     assert_eq!(Some(1..2), "b".match_suffix(b"ab"));
//     assert_eq!(None, "b".match_suffix(b"bc"));
//     assert_eq!(None, "b".match_suffix(b"abc"));
//     assert_eq!(Some(0..3), "abc".match_suffix(b"abc"));
//     assert_eq!(Some(1..4), "abc".match_suffix(b"Xabc"));
//     assert_eq!(None, "abc".match_suffix(b"abcY"));
//     assert_eq!(None, "abc".match_suffix(b"XabcY"));
//
//     assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"a"));
//     assert_eq!(Some(0..1), seq(("a", 0..), "b").match_suffix(b"b"));
//     assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"bY"));
//     assert_eq!(Some(1..2), seq(("a", 0..), "b").match_suffix(b"Xb"));
//     assert_eq!(Some(0..2), seq(("a", 0..), "b").match_suffix(b"ab"));
//     assert_eq!(None, seq(("a", 0..), "b").match_suffix(b"abY"));
//     assert_eq!(Some(1..3), seq(("a", 0..), "b").match_suffix(b"Xab"));
//
//     assert_eq!(Some(0..1), seq("a", ("b", 0..)).match_suffix(b"a"));
//     assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"aY"));
//     assert_eq!(Some(1..2), seq("a", ("b", 0..)).match_suffix(b"Xa"));
//     assert_eq!(Some(0..2), seq("a", ("b", 0..)).match_suffix(b"ab"));
//     assert_eq!(None, seq("a", ("b", 0..)).match_suffix(b"abY"));
//     assert_eq!(Some(1..3), seq("a", ("b", 0..)).match_suffix(b"Xab"));
//     assert_eq!(Some(0..3), seq("a", ("b", 0..)).match_suffix(b"abb"));
//     assert_eq!(Some(0..4), seq("a", ("b", 0..)).match_suffix(b"abbb"));
//
//     assert_eq!(
//         Some(0..2),
//         seq("a", seq(("b", 0..), "c")).match_suffix(b"ac")
//     );
//     assert_eq!(None, seq("a", seq(("b", 0..), "c")).match_suffix(b"acY"));
//     assert_eq!(
//         Some(1..3),
//         seq("a", seq(("b", 0..), "c")).match_suffix(b"Xac")
//     );
//     assert_eq!(
//         Some(0..3),
//         seq("a", seq(("b", 0..), "c")).match_suffix(b"abc")
//     );
//     assert_eq!(
//         Some(1..4),
//         seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabc")
//     );
//     assert_eq!(
//         Some(1..5),
//         seq("a", seq(("b", 0..), "c")).match_suffix(b"Xabbc")
//     );
// }
//
// #[test]
// fn test_impl_for_u8_slice() {
//     // Empty pattern
//     assert_eq!(Some(0..0), b"".as_ref().search(b""));
//     assert_eq!(Some(0..0), b"".as_ref().search(b"a"));
//     // // Whole string matches
//     assert_eq!(Some(0..2), b"bb".as_ref().search(b"bb"));
//     // Matches at beginning
//     assert_eq!(Some(0..2), b"bb".as_ref().search(b"bbc"));
//     // Matches at end
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abb"));
//     // Matches in middle
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbb"));
//     // Does not accept partial matches
//     assert_eq!(None, b"abc".as_ref().search(b"ab"));
//     assert_eq!(None, b"abc".as_ref().search(b"abd"));
//     assert_eq!(None, b"abc".as_ref().search(b"bc"));
//     assert_eq!(None, b"abc".as_ref().search(b"bcd"));
//     // Check returned range
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbc"));
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbbc"));
//     assert_eq!(Some(1..3), b"bb".as_ref().search(b"abbcbbc"));
// }

// #[test]
// fn test_bytes() {
//     assert_eq!(Some(0..0), bytes(b"").search(b""));
//     assert_eq!(Some(1..3), bytes(b"bb").search(b"abbc"));
//
//     let value = bytes(b"abc");
//     let value_copy = value; // Copy
//     #[allow(clippy::clone_on_copy)]
//     let _value_clone = value.clone(); // Clone
//     assert_eq!(
//         "Bytes { bytes: [97, 98, 99], phantom: PhantomData }",
//         format!("{:?}", value)
//     ); // Debug
//     assert!(value < bytes(b"def")); // PartialOrd
//     assert_eq!(value, value_copy); // PartialEq
// }

#[test]
fn seq() {
    let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"a"));
    assert!(match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"aab"));
    assert!(!match_all(&mut re, b"aba"));
    assert!(!match_all(&mut re, b"abab"));
}

#[test]
fn seq_reset() {
    let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"X"));
}

#[test]
fn seq_nested() {
    let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"bc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(match_all(&mut re, b"abc"));
    assert!(!match_all(&mut re, b"Xabc"));
    assert!(!match_all(&mut re, b"abcX"));
    assert!(!match_all(&mut re, b"aabc"));
    assert!(!match_all(&mut re, b"abcc"));
    assert!(!match_all(&mut re, b"abca"));
    assert!(!match_all(&mut re, b"abcabc"));
}

#[test]
fn seq_deeply_nested() {
    let mut re = Seq::new(
        Byte::new(b'a'),
        Seq::new(Seq::new(Byte::new(b'b'), Byte::new(b'c')), Byte::new(b'd')),
    );
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"d"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"bc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(!match_all(&mut re, b"abc"));
    assert!(!match_all(&mut re, b"bcd"));
    assert!(match_all(&mut re, b"abcd"));
    assert!(!match_all(&mut re, b"Xabcd"));
    assert!(!match_all(&mut re, b"abcdX"));
    assert!(!match_all(&mut re, b"aabcd"));
    assert!(!match_all(&mut re, b"abcda"));
    assert!(!match_all(&mut re, b"abcdabcd"));
}

#[test]
fn seq_debug() {
    let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
    assert_eq!("Seq(Byte(a),None,Byte(b))", format!("{:?}", re));
    assert!(!match_all(&mut re, b"a"));
    assert_eq!(
        "Seq(Byte(a),Some(DiscardingRange),Byte(b))",
        format!("{:?}", re)
    );
}

// #[test]
// fn test_range() {
//     assert_eq!(None, (..).search(b""));
//     assert_eq!(Some(0..1), (..).search(b"a"));
//
//     assert_eq!(None, (b'b'..).search(b""));
//     assert_eq!(None, (b'b'..).search(b"a"));
//     assert_eq!(Some(0..1), (b'b'..).search(b"b"));
//     assert_eq!(Some(0..1), (b'b'..).search(b"c"));
//
//     assert_eq!(None, (..b'c').search(b""));
//     assert_eq!(Some(0..1), (..b'c').search(b"a"));
//     assert_eq!(Some(0..1), (..b'c').search(b"b"));
//     assert_eq!(None, (..b'c').search(b"c"));
//
//     assert_eq!(None, (..=b'b').search(b""));
//     assert_eq!(Some(0..1), (..=b'b').search(b"a"));
//     assert_eq!(Some(0..1), (..=b'b').search(b"b"));
//     assert_eq!(None, (..=b'b').search(b"c"));
//
//     assert_eq!(None, (b'b'..b'd').search(b""));
//     assert_eq!(None, (b'b'..b'd').search(b"a"));
//     assert_eq!(Some(0..1), (b'b'..b'd').search(b"b"));
//     assert_eq!(Some(0..1), (b'b'..b'd').search(b"c"));
//     assert_eq!(None, (b'b'..b'd').search(b"d"));
//
//     assert_eq!(None, (b'b'..=b'c').search(b""));
//     assert_eq!(None, (b'b'..=b'c').search(b"a"));
//     assert_eq!(Some(0..1), (b'b'..=b'c').search(b"b"));
//     assert_eq!(Some(0..1), (b'b'..=b'c').search(b"c"));
//     assert_eq!(None, (b'b'..=b'c').search(b"d"));
// }
//
// #[test]
// fn test_repeat_range() {
//     // zero of, '{0}'
//     assert_eq!(Some(0..0), ("b", ..=0).search(b""));
//     assert_eq!(Some(0..0), (("b", ..=1), ..=1).search(b"a"));
//
//     // zero or one, '?', '{0,1}'
//     assert_eq!(Some(0..0), ("b", ..=1).search(b""));
//     assert_eq!(Some(0..0), ("b", ..=1).search(b"a"));
//     assert_eq!(Some(0..0), ("b", ..=1).search(b"ab"));
//     assert_eq!(Some(0..1), ("b", ..=1).search(b"b"));
//     assert_eq!(Some(0..1), ("b", ..=1).search(b"bb"));
//     assert_eq!(Some(0..0), ("bc", ..=1).search(b""));
//     assert_eq!(Some(0..0), ("bc", ..=1).search(b"a"));
//     assert_eq!(Some(0..0), ("bc", ..=1).search(b"abc"));
//     assert_eq!(Some(0..2), ("bc", ..=1).search(b"bc"));
//     assert_eq!(Some(0..2), ("bc", ..=1).search(b"bcbc"));
//
//     // zero or more, '*', '{0,}'
//     assert_eq!(Some(0..0), ("b", ..).search(b""));
//     assert_eq!(Some(0..0), ("b", ..).search(b"a"));
//     assert_eq!(Some(0..0), ("b", ..).search(b"ab"));
//     assert_eq!(Some(0..1), ("b", ..).search(b"b"));
//     assert_eq!(Some(0..4), ("b", ..).search(b"bbbb"));
//     assert_eq!(Some(0..0), ("bc", ..).search(b""));
//     assert_eq!(Some(0..0), ("bc", ..).search(b"a"));
//     assert_eq!(Some(0..0), ("bc", ..).search(b"abc"));
//     assert_eq!(Some(0..2), ("bc", ..).search(b"bc"));
//     assert_eq!(Some(0..4), ("bc", ..).search(b"bcbc"));
//
//     // one or more, '+', '{1,}'
//     assert_eq!(None, ("b", 1..).search(b""));
//     assert_eq!(None, ("b", 1..).search(b"a"));
//     assert_eq!(Some(1..2), ("b", 1..).search(b"ab"));
//     assert_eq!(Some(0..1), ("b", 1..).search(b"b"));
//     assert_eq!(Some(0..4), ("b", 1..).search(b"bbbb"));
//     assert_eq!(None, ("bc", 1..).search(b""));
//     assert_eq!(None, ("bc", 1..).search(b"a"));
//     assert_eq!(Some(1..3), ("bc", 1..).search(b"abc"));
//     assert_eq!(Some(0..2), ("bc", 1..).search(b"bc"));
//     assert_eq!(Some(1..3), ("bc", 1..).search(b"bbc"));
//     assert_eq!(Some(0..4), ("bc", 1..).search(b"bcbc"));
//
//     // n of, '{n}'
//     assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
//     assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));
//     assert_eq!(None, ("b", 1..=1).search(b""));
//     assert_eq!(Some(0..1), ("b", 1..=1).search(b"b"));
//
//     assert_eq!(None, ("b", 2..=2).search(b""));
//     assert_eq!(None, ("b", 2..=2).search(b"aaa"));
//     assert_eq!(None, ("b", 2..=2).search(b"abaa"));
//     assert_eq!(Some(1..3), ("b", 2..=2).search(b"abb"));
//     assert_eq!(Some(0..2), ("b", 2..=2).search(b"bb"));
//     assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbc"));
//     assert_eq!(Some(0..2), ("b", 2..=2).search(b"bbbb"));
//
//     assert_eq!(None, ("bc", 2..=2).search(b""));
//     assert_eq!(None, ("bc", 2..=2).search(b"aa"));
//     assert_eq!(None, ("bc", 2..=2).search(b"abb"));
//     assert_eq!(None, ("bc", 2..=2).search(b"ccd"));
//     assert_eq!(Some(1..5), ("bc", 2..=2).search(b"abcbc"));
//     assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbc"));
//     assert_eq!(Some(1..5), ("bc", 2..=2).search(b"bbcbc"));
//     assert_eq!(Some(0..4), ("bc", 2..=2).search(b"bcbcbcbc"));
//
//     // m to n of, '{m,n}'
//     assert_eq!(Some(0..0), ("b", 0..=0).search(b""));
//     assert_eq!(Some(0..0), ("b", 0..=0).search(b"a"));
//
//     assert_eq!(Some(0..0), ("b", 0..=1).search(b""));
//     assert_eq!(Some(0..0), ("b", 0..=1).search(b"a"));
//     assert_eq!(Some(0..0), ("b", 0..=1).search(b"ab"));
//
//     assert_eq!(None, ("b", 1..=2).search(b""));
//     assert_eq!(None, ("b", 1..=2).search(b"a"));
//     assert_eq!(Some(0..1), ("b", 1..=2).search(b"b"));
//     assert_eq!(Some(0..1), ("b", 1..=2).search(b"bc"));
//     assert_eq!(Some(1..2), ("b", 1..=2).search(b"ab"));
//     assert_eq!(Some(1..2), ("b", 1..=2).search(b"abc"));
//     assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbc"));
//     assert_eq!(Some(1..3), ("b", 1..=2).search(b"abbbbc"));
//
//     assert_eq!(None, ("b", 2..=4).search(b""));
//     assert_eq!(None, ("b", 2..=4).search(b"aa"));
//     assert_eq!(None, ("b", 2..=4).search(b"ab"));
//     assert_eq!(None, ("b", 2..=4).search(b"abc"));
//     assert_eq!(Some(0..2), ("b", 2..=4).search(b"bb"));
//     assert_eq!(Some(0..2), ("b", 2..=4).search(b"bbcc"));
//     assert_eq!(Some(2..4), ("b", 2..=4).search(b"aabb"));
//     assert_eq!(Some(1..3), ("b", 2..=4).search(b"abbc"));
//     assert_eq!(Some(1..4), ("b", 2..=4).search(b"abbbc"));
//     assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbc"));
//     assert_eq!(Some(1..5), ("b", 2..=4).search(b"abbbbbbbbc"));
//
//     assert_eq!(None, ("bc", 2..=4).search(b""));
//     assert_eq!(None, ("bc", 2..=4).search(b"aaaa"));
//     assert_eq!(None, ("bc", 2..=4).search(b"abc"));
//     assert_eq!(None, ("bc", 2..=4).search(b"abcb"));
//     assert_eq!(None, ("bc", 2..=4).search(b"abcd"));
//     assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbc"));
//     assert_eq!(Some(0..4), ("bc", 2..=4).search(b"bcbcdddd"));
//     assert_eq!(Some(4..8), ("bc", 2..=4).search(b"aaaabcbc"));
//     assert_eq!(Some(1..5), ("bc", 2..=4).search(b"abcbcd"));
//     assert_eq!(Some(2..6), ("bc", 2..=4).search(b"abbcbc"));
//     assert_eq!(Some(1..7), ("bc", 2..=4).search(b"abcbcbc"));
//     assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbc"));
//     assert_eq!(Some(1..9), ("bc", 2..=4).search(b"abcbcbcbcbc"));
//
//     assert_eq!(Some(0..0), ("b", ..).search(b"abc"));
//     assert_eq!(Some(1..2), ("b", 1..).search(b"abc"));
//     assert_eq!(Some(0..0), ("b", ..2).search(b"abc"));
//     assert_eq!(Some(0..0), ("b", ..=1).search(b"abc"));
//     assert_eq!(Some(1..2), ("b", 1..2).search(b"abc"));
//     assert_eq!(Some(1..2), ("b", 1..=1).search(b"abc"));
// }
//
// #[test]
// fn test_any_byte() {
//     assert_eq!(None, any_byte().search(b""));
//     assert_eq!(Some(0..1), any_byte().search(b"a"));
//     assert_eq!(Some(0..1), any_byte().search(b"ab"));
//
//     let value = any_byte();
//     let value_copy = value; // Copy
//     #[allow(clippy::clone_on_copy)]
//     let _value_clone = value.clone(); // Clone
//     assert_eq!("AnyByte", format!("{:?}", value)); // Debug
//     assert_eq!(
//         Some(core::cmp::Ordering::Equal),
//         value.partial_cmp(&any_byte()) // PartialOrd
//     );
//     assert_eq!(value, value_copy); // PartialEq
// }
//
// #[test]
// fn test_not() {
//     assert_eq!(None, not("X").search(b""));
//     assert_eq!(None, not("X").search(b"X"));
//     assert_eq!(None, not("X").search(b"XX"));
//     assert_eq!(Some(0..1), not("X").search(b"ab"));
//     assert_eq!(Some(1..2), not("X").search(b"Xab"));
//     assert_eq!(Some(2..3), not("X").search(b"XXab"));
//     assert_eq!(Some(0..1), not(seq(any_byte(), "X")).search(b"aX"));
//
//     let value = not("X");
//     let value_copy = value; // Copy
//     #[allow(clippy::clone_on_copy)]
//     let _value_clone = value.clone(); // Clone
//     assert_eq!(
//         "Not { re: \"X\", phantom: PhantomData }",
//         format!("{:?}", value)
//     ); // Debug
//     assert!(value < not("Y")); // PartialOrd
//     assert_eq!(value, value_copy); // PartialEq
// }

#[test]
fn either() {
    let mut re = Either::new(Byte::new(b'a'), Byte::new(b'b'));
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"Xb"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"bX"));
    assert!(!match_all(&mut re, b"XaY"));
    assert!(!match_all(&mut re, b"XbY"));
    assert!(!match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"ba"));
    assert!(!match_all(&mut re, b"bb"));
    assert!(match_all(&mut re, b"a"));
    assert!(match_all(&mut re, b"b"));
}

#[test]
fn either_group() {
    let mut group = CapturingGroup::new(Either::new(Byte::new(b'a'), Byte::new(b'b')));
    assert!(!match_all(&mut group, b""));
    assert!(!match_all(&mut group, b"X"));
    assert!(!match_all(&mut group, b"Xa"));
    assert!(!match_all(&mut group, b"Xb"));
    assert!(!match_all(&mut group, b"aX"));
    assert!(!match_all(&mut group, b"bX"));
    assert!(!match_all(&mut group, b"XaY"));
    assert!(!match_all(&mut group, b"XbY"));
    assert!(!match_all(&mut group, b"aa"));
    assert!(!match_all(&mut group, b"ab"));
    assert!(!match_all(&mut group, b"ba"));
    assert!(!match_all(&mut group, b"bb"));
    assert!(match_all(&mut group, b"a"));
    assert_eq!(Some(0..1), group.range());
    assert!(match_all(&mut group, b"b"));
    assert_eq!(Some(0..1), group.range());
}

#[test]
fn either_seq() {
    let mut re = Seq::new(
        Either::new(Byte::new(b'a'), Byte::new(b'b')),
        Either::new(Byte::new(b'c'), Byte::new(b'd')),
    );
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"Xac"));
    assert!(!match_all(&mut re, b"Xad"));
    assert!(!match_all(&mut re, b"Xbc"));
    assert!(!match_all(&mut re, b"Xbd"));
    assert!(!match_all(&mut re, b"acX"));
    assert!(!match_all(&mut re, b"adX"));
    assert!(!match_all(&mut re, b"bcX"));
    assert!(!match_all(&mut re, b"bdX"));
    assert!(!match_all(&mut re, b"XacY"));
    assert!(!match_all(&mut re, b"XadY"));
    assert!(!match_all(&mut re, b"XbcY"));
    assert!(!match_all(&mut re, b"XbdY"));
    assert!(!match_all(&mut re, b"aac"));
    assert!(!match_all(&mut re, b"add"));
    assert!(!match_all(&mut re, b"acac"));
    assert!(!match_all(&mut re, b"acbd"));
    assert!(match_all(&mut re, b"ac"));
    assert!(match_all(&mut re, b"ad"));
    assert!(!match_all(&mut re, b"ba"));
    assert!(!match_all(&mut re, b"bb"));
    assert!(match_all(&mut re, b"bc"));
    assert!(match_all(&mut re, b"bd"));
    assert!(!match_all(&mut re, b"ca"));
    assert!(!match_all(&mut re, b"cb"));
    assert!(!match_all(&mut re, b"cc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(!match_all(&mut re, b"da"));
    assert!(!match_all(&mut re, b"db"));
    assert!(!match_all(&mut re, b"dc"));
    assert!(!match_all(&mut re, b"dd"));
}

#[test]
fn either_debug() {
    let re: Either<DiscardingRange, _, _> = Either::new(Byte::new(b'a'), Byte::new(b'b'));
    assert_eq!("Either(Byte(a),Byte(b))", format!("{:?}", re));
}

#[test]
fn group() {
    let mut group = CapturingGroup::new(Byte::new(b'a'));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group),
        &group
    );
    assert!(!match_all(&mut group, b""));
    assert!(match_all(&mut group, b"a"));
    assert_eq!(Some(0..1), group.range());
    assert!(!match_all(&mut group, b"Xa"));
    assert!(!match_all(&mut group, b"ab"));
    assert!(!match_all(&mut group, b"aa"));
}

#[test]
fn group_nested1() {
    // ((a))
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_outer = CapturingGroup::new(&mut group_a);
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_outer),
        &group_outer
    );
    assert!(!match_all(&mut group_outer, b""));
    assert!(!match_all(&mut group_outer, b"Xa"));
    assert!(!match_all(&mut group_outer, b"ab"));
    assert!(!match_all(&mut group_outer, b"aa"));
    assert!(match_all(&mut group_outer, b"a"));
    assert_eq!(Some(0..1), group_outer.range());
    assert_eq!(Some(0..1), group_a.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested2() {
    // (a(b))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested3() {
    // ((a)b)
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, Byte::new(b'b')));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(0..1), group_a.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested4() {
    // ((a)(b))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, &mut group_b));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(0..1), group_a.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested5() {
    // ((a(b)) (c))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut group_abc = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_c));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abc),
        group_abc
    );
    assert!(!match_all(&mut group_abc, b"Xabc"));
    assert!(!match_all(&mut group_abc, b"abcX"));
    assert!(!match_all(&mut group_abc, b"aabc"));
    assert!(!match_all(&mut group_abc, b"abca"));
    assert!(!match_all(&mut group_abc, b"abcc"));
    assert!(!match_all(&mut group_abc, b"abcabc"));
    assert!(!match_all(&mut group_abc, b""));
    assert!(!match_all(&mut group_abc, b"a"));
    assert!(!match_all(&mut group_abc, b"ab"));
    assert!(match_all(&mut group_abc, b"abc"));
    assert_eq!(Some(0..3), group_abc.range());
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested6() {
    // ((a) ((b)c))
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_bc = CapturingGroup::new(Seq::new(&mut group_b, Byte::new(b'c')));
    let mut group_abc = CapturingGroup::new(Seq::new(&mut group_a, &mut group_bc));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abc),
        group_abc
    );
    assert!(!match_all(&mut group_abc, b"Xabc"));
    assert!(!match_all(&mut group_abc, b"abcX"));
    assert!(!match_all(&mut group_abc, b"aabc"));
    assert!(!match_all(&mut group_abc, b"abca"));
    assert!(!match_all(&mut group_abc, b"abcc"));
    assert!(!match_all(&mut group_abc, b"abcabc"));
    assert!(!match_all(&mut group_abc, b""));
    assert!(!match_all(&mut group_abc, b"a"));
    assert!(!match_all(&mut group_abc, b"ab"));
    assert!(match_all(&mut group_abc, b"abc"));
    assert_eq!(Some(0..3), group_abc.range());
    assert_eq!(Some(1..3), group_bc.range());
    assert_eq!(Some(0..1), group_a.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested7() {
    // ((a(b)) ((c)d))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
    let mut group_abcd = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_cd));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abcd),
        group_abcd
    );
    assert!(!match_all(&mut group_abcd, b"Xabcd"));
    assert!(!match_all(&mut group_abcd, b"abcdX"));
    assert!(!match_all(&mut group_abcd, b"aabcd"));
    assert!(!match_all(&mut group_abcd, b"abcda"));
    assert!(!match_all(&mut group_abcd, b"abcdd"));
    assert!(!match_all(&mut group_abcd, b"abcdabcd"));
    assert!(!match_all(&mut group_abcd, b""));
    assert!(!match_all(&mut group_abcd, b"a"));
    assert!(!match_all(&mut group_abcd, b"ab"));
    assert!(!match_all(&mut group_abcd, b"abc"));
    assert!(match_all(&mut group_abcd, b"abcd"));
    assert_eq!(Some(0..4), group_abcd.range());
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(2..4), group_cd.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested8() {
    // (a (b (c)))
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_bc = CapturingGroup::new(Seq::new(Byte::new(b'b'), &mut group_c));
    let mut seq_abc = Seq::new(Byte::new(b'a'), &mut group_bc);
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&seq_abc),
        seq_abc
    );
    assert!(!match_all(&mut seq_abc, b"Xabc"));
    assert!(!match_all(&mut seq_abc, b"abcX"));
    assert!(!match_all(&mut seq_abc, b"aabc"));
    assert!(!match_all(&mut seq_abc, b"abca"));
    assert!(!match_all(&mut seq_abc, b"abcc"));
    assert!(!match_all(&mut seq_abc, b"abcabc"));
    assert!(!match_all(&mut seq_abc, b""));
    assert!(!match_all(&mut seq_abc, b"a"));
    assert!(!match_all(&mut seq_abc, b"ab"));
    assert!(match_all(&mut seq_abc, b"abc"));
    assert_eq!(Some(1..3), group_bc.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested9() {
    // (a(b)) ((c)d) ((e)(f))
    let mut group_f = CapturingGroup::new(Byte::new(b'f'));
    let mut group_e = CapturingGroup::new(Byte::new(b'e'));
    let mut group_ef = CapturingGroup::new(Seq::new(&mut group_e, &mut group_f));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut seq_abcdef = Seq::new(&mut group_ab, Seq::new(&mut group_cd, &mut group_ef));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&seq_abcdef),
        seq_abcdef
    );
    assert!(!match_all(&mut seq_abcdef, b""));
    assert!(!match_all(&mut seq_abcdef, b"a"));
    assert!(!match_all(&mut seq_abcdef, b"ab"));
    assert!(!match_all(&mut seq_abcdef, b"abc"));
    assert!(!match_all(&mut seq_abcdef, b"abd"));
    assert!(!match_all(&mut seq_abcdef, b"abe"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefa"));
    assert!(!match_all(&mut seq_abcdef, b"aabcdef"));
    assert!(!match_all(&mut seq_abcdef, b"Xabcdef"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefX"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefabcdef"));
    assert!(match_all(&mut seq_abcdef, b"abcdef"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(2..4), group_cd.range());
    assert_eq!(Some(4..6), group_ef.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
    assert_eq!(Some(4..5), group_e.range());
    assert_eq!(Some(5..6), group_f.range());
}

#[test]
fn group_debug() {
    let group: CapturingGroup<DiscardingRange, _> = CapturingGroup::new(Byte::new(b'a'));
    assert_eq!("CapturingGroup(Byte(a))", format!("{:?}", group));
}
