use safe_regex::internal::escape_ascii;
use safe_regex::{regex, Matcher};

fn match_re_fn(data: &[u8]) -> bool {
    regex!(br"a").match_all(data).is_some()
}

#[test]
fn test_re_fn() {
    assert!(!match_re_fn(b""));
    assert!(match_re_fn(b"a"));
}

#[test]
fn byte() {
    let re: Matcher<_> = regex!(br"a");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
}

#[test]
fn any_byte() {
    let re: Matcher<_> = regex!(br".");
    assert_eq!(None, re.match_all(b""));
    re.match_all(b"X").unwrap();
    assert_eq!(None, re.match_all(b"XY"));
}

// #[test]
// fn seq() {
//     let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
//     println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"a"));
//     assert!(match_all(&mut re, b"ab"));
//     assert!(!match_all(&mut re, b"aab"));
//     assert!(!match_all(&mut re, b"aba"));
//     assert!(!match_all(&mut re, b"abab"));
// }
//
// #[test]
// fn seq_reset() {
//     let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
//     println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
//     assert!(!match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"b"));
//     assert!(!match_all(&mut re, b"c"));
//     assert!(!match_all(&mut re, b"X"));
// }
//
// #[test]
// fn seq_nested() {
//     let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
//     println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"b"));
//     assert!(!match_all(&mut re, b"c"));
//     assert!(!match_all(&mut re, b"ab"));
//     assert!(!match_all(&mut re, b"bc"));
//     assert!(!match_all(&mut re, b"cd"));
//     assert!(match_all(&mut re, b"abc"));
//     assert!(!match_all(&mut re, b"Xabc"));
//     assert!(!match_all(&mut re, b"abcX"));
//     assert!(!match_all(&mut re, b"aabc"));
//     assert!(!match_all(&mut re, b"abcc"));
//     assert!(!match_all(&mut re, b"abca"));
//     assert!(!match_all(&mut re, b"abcabc"));
// }
//
// #[test]
// fn seq_deeply_nested() {
//     let mut re = Seq::new(
//         Byte::new(b'a'),
//         Seq::new(Seq::new(Byte::new(b'b'), Byte::new(b'c')), Byte::new(b'd')),
//     );
//     println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"b"));
//     assert!(!match_all(&mut re, b"c"));
//     assert!(!match_all(&mut re, b"d"));
//     assert!(!match_all(&mut re, b"ab"));
//     assert!(!match_all(&mut re, b"bc"));
//     assert!(!match_all(&mut re, b"cd"));
//     assert!(!match_all(&mut re, b"abc"));
//     assert!(!match_all(&mut re, b"bcd"));
//     assert!(match_all(&mut re, b"abcd"));
//     assert!(!match_all(&mut re, b"Xabcd"));
//     assert!(!match_all(&mut re, b"abcdX"));
//     assert!(!match_all(&mut re, b"aabcd"));
//     assert!(!match_all(&mut re, b"abcda"));
//     assert!(!match_all(&mut re, b"abcdabcd"));
// }
//
// #[test]
// fn seq_debug() {
//     let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
//     assert_eq!("Seq(Byte(a),None,Byte(b))", format!("{:?}", re));
//     assert!(!match_all(&mut re, b"a"));
//     assert_eq!(
//         "Seq(Byte(a),Some(MatchRange(0..1,())),Byte(b))",
//         format!("{:?}", re)
//     );
// }
//
// #[test]
// fn optional() {
//     let mut re = Optional::new(Byte::new(b'a'));
//     assert_eq!("Optional(Byte(a))", format!("{:?}", re));
//     assert!(match_all(&mut re, b""));
//     assert!(match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"aa"));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"aX"));
//     assert!(!match_all(&mut re, b"Xa"));
// }

#[test]
fn optional_at_start() {
    // let mut re = Seq::new(Optional::new(Byte::new(b'a')), Byte::new(b'a'));
    // assert!(!match_all([0..0_usize; 2], &mut re, b""));
    // assert!(!match_all(&mut re, b"X"));
    // assert!(!match_all(&mut re, b"aX"));
    // assert!(!match_all(&mut re, b"Xa"));
    // assert!(match_all(&mut re, b"a"));
    // assert!(match_all(&mut re, b"aa"));
    // assert!(!match_all(&mut re, b"aaa"));
    // assert!(!match_all(&mut re, b"Xaa"));
    // assert!(!match_all(&mut re, b"aaX"));
}

// #[test]
// fn optional_at_end() {
//     let mut re = Seq::new(Byte::new(b'a'), Optional::new(Byte::new(b'a')));
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"aX"));
//     assert!(!match_all(&mut re, b"Xa"));
//     assert!(match_all(&mut re, b"a"));
//     assert!(match_all(&mut re, b"aa"));
//     assert!(!match_all(&mut re, b"aaa"));
//     assert!(!match_all(&mut re, b"Xaa"));
//     assert!(!match_all(&mut re, b"aaX"));
// }
//
// #[test]
// fn optional_in_middle() {
//     let mut re = Seq::new(
//         Byte::new(b'a'),
//         Seq::new(Optional::new(Byte::new(b'a')), Byte::new(b'a')),
//     );
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"aX"));
//     assert!(!match_all(&mut re, b"Xa"));
//     assert!(!match_all(&mut re, b"Xaa"));
//     assert!(!match_all(&mut re, b"aaX"));
//     assert!(!match_all(&mut re, b"aXa"));
//     assert!(match_all(&mut re, b"aa"));
//     assert!(match_all(&mut re, b"aaa"));
//     assert!(!match_all(&mut re, b"aaaa"));
//     assert!(!match_all(&mut re, b"aaaaa"));
//     assert!(!match_all(&mut re, b"aaaaaa"));
//     assert!(!match_all(&mut re, b"Xaaa"));
//     assert!(!match_all(&mut re, b"aaaX"));
//     assert!(!match_all(&mut re, b"XaaaX"));
// }
//
// #[test]
// fn optional_in_group() {
//     let matcher = |data| {
//         let mut group = CapturingGroup::new(Optional::new(Byte::new(b'a')));
//         if match_all(
//             &mut Seq::new(Byte::new(b'a'), Seq::new(&mut group, Byte::new(b'a'))),
//             data,
//         ) {
//             Some(group.range())
//         } else {
//             None
//         }
//     };
//     assert_eq!(None, matcher(b""));
//     assert_eq!(None, matcher(b"X"));
//     assert_eq!(None, matcher(b"a"));
//     assert_eq!(None, matcher(b"aX"));
//     assert_eq!(None, matcher(b"Xa"));
//     assert_eq!(Some(None), matcher(b"aa"));
//     assert_eq!(None, matcher(b"Xaa"));
//     assert_eq!(None, matcher(b"aXa"));
//     assert_eq!(None, matcher(b"aaX"));
//     assert_eq!(None, matcher(b"XaaX"));
//     assert_eq!(None, matcher(b"Xaaa"));
//     assert_eq!(None, matcher(b"aXaa"));
//     assert_eq!(None, matcher(b"aaXa"));
//     assert_eq!(None, matcher(b"aaaX"));
//     assert_eq!(None, matcher(b"XaaaX"));
//     assert_eq!(Some(Some(1..2)), matcher(b"aaa"));
//     assert_eq!(None, matcher(b"aaaa"));
//     assert_eq!(None, matcher(b"aaaaa"));
//     assert_eq!(None, matcher(b"aaaaaa"));
// }

// #[test]
// fn class_inclusive() {
//     let mut re = Class::new(true, b"abc");
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"Xa"));
//     assert!(!match_all(&mut re, b"aX"));
//     assert!(!match_all(&mut re, b"aa"));
//     assert!(!match_all(&mut re, b"abc"));
//     assert!(match_all(&mut re, b"a"));
//     assert!(match_all(&mut re, b"b"));
//     assert!(match_all(&mut re, b"c"));
//     // Debug
//     assert_eq!("Class(abc)", format!("{:?}", re));
//     // Class should match only one byte.
//     let mut group = CapturingGroup::new(Class::new(true, b"abc"));
//     assert!(match_all(&mut Seq::new(&mut group, AnyByte::new()), b"aa"));
//     assert_eq!(Some(0..1), group.range());
// }
//
// #[test]
// fn class_exclusive() {
//     let mut re = Class::new(false, b"abc");
//     assert!(!match_all(&mut re, b""));
//     assert!(match_all(&mut re, b"X"));
//     assert!(match_all(&mut re, b"Y"));
//     assert!(!match_all(&mut re, b"XY"));
//     assert!(!match_all(&mut re, b"a"));
//     assert!(!match_all(&mut re, b"b"));
//     assert!(!match_all(&mut re, b"c"));
//     // Debug
//     assert_eq!("Class^(abc)", format!("{:?}", re));
//     // Class should match only one byte.
//     let mut group = CapturingGroup::new(Class::new(false, b"abc"));
//     assert!(match_all(&mut Seq::new(&mut group, AnyByte::new()), b"XX"));
//     assert_eq!(Some(0..1), group.range());
// }
//
// #[test]
// fn either() {
//     let mut re = Either::new(Byte::new(b'a'), Byte::new(b'b'));
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"Xa"));
//     assert!(!match_all(&mut re, b"Xb"));
//     assert!(!match_all(&mut re, b"aX"));
//     assert!(!match_all(&mut re, b"bX"));
//     assert!(!match_all(&mut re, b"XaY"));
//     assert!(!match_all(&mut re, b"XbY"));
//     assert!(!match_all(&mut re, b"aa"));
//     assert!(!match_all(&mut re, b"ab"));
//     assert!(!match_all(&mut re, b"ba"));
//     assert!(!match_all(&mut re, b"bb"));
//     assert!(match_all(&mut re, b"a"));
//     assert!(match_all(&mut re, b"b"));
// }
//
// #[test]
// fn either_group() {
//     let mut group = CapturingGroup::new(Either::new(Byte::new(b'a'), Byte::new(b'b')));
//     assert!(!match_all(&mut group, b""));
//     assert!(!match_all(&mut group, b"X"));
//     assert!(!match_all(&mut group, b"Xa"));
//     assert!(!match_all(&mut group, b"Xb"));
//     assert!(!match_all(&mut group, b"aX"));
//     assert!(!match_all(&mut group, b"bX"));
//     assert!(!match_all(&mut group, b"XaY"));
//     assert!(!match_all(&mut group, b"XbY"));
//     assert!(!match_all(&mut group, b"aa"));
//     assert!(!match_all(&mut group, b"ab"));
//     assert!(!match_all(&mut group, b"ba"));
//     assert!(!match_all(&mut group, b"bb"));
//     assert!(match_all(&mut group, b"a"));
//     assert_eq!(Some(0..1), group.range());
//     assert!(match_all(&mut group, b"b"));
//     assert_eq!(Some(0..1), group.range());
// }
//
// #[test]
// fn either_seq() {
//     let mut re = Seq::new(
//         Either::new(Byte::new(b'a'), Byte::new(b'b')),
//         Either::new(Byte::new(b'c'), Byte::new(b'd')),
//     );
//     assert!(!match_all(&mut re, b""));
//     assert!(!match_all(&mut re, b"X"));
//     assert!(!match_all(&mut re, b"Xac"));
//     assert!(!match_all(&mut re, b"Xad"));
//     assert!(!match_all(&mut re, b"Xbc"));
//     assert!(!match_all(&mut re, b"Xbd"));
//     assert!(!match_all(&mut re, b"acX"));
//     assert!(!match_all(&mut re, b"adX"));
//     assert!(!match_all(&mut re, b"bcX"));
//     assert!(!match_all(&mut re, b"bdX"));
//     assert!(!match_all(&mut re, b"XacY"));
//     assert!(!match_all(&mut re, b"XadY"));
//     assert!(!match_all(&mut re, b"XbcY"));
//     assert!(!match_all(&mut re, b"XbdY"));
//     assert!(!match_all(&mut re, b"aac"));
//     assert!(!match_all(&mut re, b"add"));
//     assert!(!match_all(&mut re, b"acac"));
//     assert!(!match_all(&mut re, b"acbd"));
//     assert!(match_all(&mut re, b"ac"));
//     assert!(match_all(&mut re, b"ad"));
//     assert!(!match_all(&mut re, b"ba"));
//     assert!(!match_all(&mut re, b"bb"));
//     assert!(match_all(&mut re, b"bc"));
//     assert!(match_all(&mut re, b"bd"));
//     assert!(!match_all(&mut re, b"ca"));
//     assert!(!match_all(&mut re, b"cb"));
//     assert!(!match_all(&mut re, b"cc"));
//     assert!(!match_all(&mut re, b"cd"));
//     assert!(!match_all(&mut re, b"da"));
//     assert!(!match_all(&mut re, b"db"));
//     assert!(!match_all(&mut re, b"dc"));
//     assert!(!match_all(&mut re, b"dd"));
// }
//
// #[test]
// fn either_debug() {
//     let re: Either<DiscardingRange, _, _> = Either::new(Byte::new(b'a'), Byte::new(b'b'));
//     assert_eq!("Either(Byte(a),Byte(b))", format!("{:?}", re));
// }

#[test]
fn group() {
    let re: Matcher<_> = regex!(br"(a)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"aa"));
    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
    assert_eq!(0..1, groups.group_range(1).unwrap());
}

// #[test]
// fn group_nested1() {
//     // ((a))
//     let mut group_a = CapturingGroup::new(Byte::new(b'a'));
//     let mut group_outer = CapturingGroup::new(&mut group_a);
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_outer),
//         &group_outer
//     );
//     assert!(!match_all(&mut group_outer, b""));
//     assert!(!match_all(&mut group_outer, b"Xa"));
//     assert!(!match_all(&mut group_outer, b"ab"));
//     assert!(!match_all(&mut group_outer, b"aa"));
//     assert!(match_all(&mut group_outer, b"a"));
//     assert_eq!(Some(0..1), group_outer.range());
//     assert_eq!(Some(0..1), group_a.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested2() {
//     // (a(b))
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_ab),
//         group_ab
//     );
//     assert!(!match_all(&mut group_ab, b"Xab"));
//     assert!(!match_all(&mut group_ab, b"abX"));
//     assert!(!match_all(&mut group_ab, b"aab"));
//     assert!(!match_all(&mut group_ab, b"aba"));
//     assert!(!match_all(&mut group_ab, b"abb"));
//     assert!(!match_all(&mut group_ab, b"abab"));
//     assert!(!match_all(&mut group_ab, b""));
//     assert!(!match_all(&mut group_ab, b"a"));
//     assert!(match_all(&mut group_ab, b"ab"));
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(1..2), group_b.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested3() {
//     // ((a)b)
//     let mut group_a = CapturingGroup::new(Byte::new(b'a'));
//     let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, Byte::new(b'b')));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_ab),
//         group_ab
//     );
//     assert!(!match_all(&mut group_ab, b"Xab"));
//     assert!(!match_all(&mut group_ab, b"abX"));
//     assert!(!match_all(&mut group_ab, b"aab"));
//     assert!(!match_all(&mut group_ab, b"aba"));
//     assert!(!match_all(&mut group_ab, b"abb"));
//     assert!(!match_all(&mut group_ab, b"abab"));
//     assert!(!match_all(&mut group_ab, b""));
//     assert!(!match_all(&mut group_ab, b"a"));
//     assert!(match_all(&mut group_ab, b"ab"));
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(0..1), group_a.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested4() {
//     // ((a)(b))
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_a = CapturingGroup::new(Byte::new(b'a'));
//     let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, &mut group_b));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_ab),
//         group_ab
//     );
//     assert!(!match_all(&mut group_ab, b"Xab"));
//     assert!(!match_all(&mut group_ab, b"abX"));
//     assert!(!match_all(&mut group_ab, b"aab"));
//     assert!(!match_all(&mut group_ab, b"aba"));
//     assert!(!match_all(&mut group_ab, b"abb"));
//     assert!(!match_all(&mut group_ab, b"abab"));
//     assert!(!match_all(&mut group_ab, b""));
//     assert!(!match_all(&mut group_ab, b"a"));
//     assert!(match_all(&mut group_ab, b"ab"));
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(0..1), group_a.range());
//     assert_eq!(Some(1..2), group_b.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested5() {
//     // ((a(b)) (c))
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_c = CapturingGroup::new(Byte::new(b'c'));
//     let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
//     let mut group_abc = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_c));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_abc),
//         group_abc
//     );
//     assert!(!match_all(&mut group_abc, b"Xabc"));
//     assert!(!match_all(&mut group_abc, b"abcX"));
//     assert!(!match_all(&mut group_abc, b"aabc"));
//     assert!(!match_all(&mut group_abc, b"abca"));
//     assert!(!match_all(&mut group_abc, b"abcc"));
//     assert!(!match_all(&mut group_abc, b"abcabc"));
//     assert!(!match_all(&mut group_abc, b""));
//     assert!(!match_all(&mut group_abc, b"a"));
//     assert!(!match_all(&mut group_abc, b"ab"));
//     assert!(match_all(&mut group_abc, b"abc"));
//     assert_eq!(Some(0..3), group_abc.range());
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(1..2), group_b.range());
//     assert_eq!(Some(2..3), group_c.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested6() {
//     // ((a) ((b)c))
//     let mut group_a = CapturingGroup::new(Byte::new(b'a'));
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_bc = CapturingGroup::new(Seq::new(&mut group_b, Byte::new(b'c')));
//     let mut group_abc = CapturingGroup::new(Seq::new(&mut group_a, &mut group_bc));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_abc),
//         group_abc
//     );
//     assert!(!match_all(&mut group_abc, b"Xabc"));
//     assert!(!match_all(&mut group_abc, b"abcX"));
//     assert!(!match_all(&mut group_abc, b"aabc"));
//     assert!(!match_all(&mut group_abc, b"abca"));
//     assert!(!match_all(&mut group_abc, b"abcc"));
//     assert!(!match_all(&mut group_abc, b"abcabc"));
//     assert!(!match_all(&mut group_abc, b""));
//     assert!(!match_all(&mut group_abc, b"a"));
//     assert!(!match_all(&mut group_abc, b"ab"));
//     assert!(match_all(&mut group_abc, b"abc"));
//     assert_eq!(Some(0..3), group_abc.range());
//     assert_eq!(Some(1..3), group_bc.range());
//     assert_eq!(Some(0..1), group_a.range());
//     assert_eq!(Some(1..2), group_b.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested7() {
//     // ((a(b)) ((c)d))
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_c = CapturingGroup::new(Byte::new(b'c'));
//     let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
//     let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
//     let mut group_abcd = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_cd));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&group_abcd),
//         group_abcd
//     );
//     assert!(!match_all(&mut group_abcd, b"Xabcd"));
//     assert!(!match_all(&mut group_abcd, b"abcdX"));
//     assert!(!match_all(&mut group_abcd, b"aabcd"));
//     assert!(!match_all(&mut group_abcd, b"abcda"));
//     assert!(!match_all(&mut group_abcd, b"abcdd"));
//     assert!(!match_all(&mut group_abcd, b"abcdabcd"));
//     assert!(!match_all(&mut group_abcd, b""));
//     assert!(!match_all(&mut group_abcd, b"a"));
//     assert!(!match_all(&mut group_abcd, b"ab"));
//     assert!(!match_all(&mut group_abcd, b"abc"));
//     assert!(match_all(&mut group_abcd, b"abcd"));
//     assert_eq!(Some(0..4), group_abcd.range());
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(2..4), group_cd.range());
//     assert_eq!(Some(1..2), group_b.range());
//     assert_eq!(Some(2..3), group_c.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested8() {
//     // (a (b (c)))
//     let mut group_c = CapturingGroup::new(Byte::new(b'c'));
//     let mut group_bc = CapturingGroup::new(Seq::new(Byte::new(b'b'), &mut group_c));
//     let mut seq_abc = Seq::new(Byte::new(b'a'), &mut group_bc);
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&seq_abc),
//         seq_abc
//     );
//     assert!(!match_all(&mut seq_abc, b"Xabc"));
//     assert!(!match_all(&mut seq_abc, b"abcX"));
//     assert!(!match_all(&mut seq_abc, b"aabc"));
//     assert!(!match_all(&mut seq_abc, b"abca"));
//     assert!(!match_all(&mut seq_abc, b"abcc"));
//     assert!(!match_all(&mut seq_abc, b"abcabc"));
//     assert!(!match_all(&mut seq_abc, b""));
//     assert!(!match_all(&mut seq_abc, b"a"));
//     assert!(!match_all(&mut seq_abc, b"ab"));
//     assert!(match_all(&mut seq_abc, b"abc"));
//     assert_eq!(Some(1..3), group_bc.range());
//     assert_eq!(Some(2..3), group_c.range());
// }
//
// #[test]
// #[allow(clippy::similar_names)]
// fn group_nested9() {
//     // (a(b)) ((c)d) ((e)(f))
//     let mut group_f = CapturingGroup::new(Byte::new(b'f'));
//     let mut group_e = CapturingGroup::new(Byte::new(b'e'));
//     let mut group_ef = CapturingGroup::new(Seq::new(&mut group_e, &mut group_f));
//     let mut group_c = CapturingGroup::new(Byte::new(b'c'));
//     let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
//     let mut group_b = CapturingGroup::new(Byte::new(b'b'));
//     let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
//     let mut seq_abcdef = Seq::new(&mut group_ab, Seq::new(&mut group_cd, &mut group_ef));
//     println!(
//         "size {} bytes: {:?}",
//         core::mem::size_of_val(&seq_abcdef),
//         seq_abcdef
//     );
//     assert!(!match_all(&mut seq_abcdef, b""));
//     assert!(!match_all(&mut seq_abcdef, b"a"));
//     assert!(!match_all(&mut seq_abcdef, b"ab"));
//     assert!(!match_all(&mut seq_abcdef, b"abc"));
//     assert!(!match_all(&mut seq_abcdef, b"abd"));
//     assert!(!match_all(&mut seq_abcdef, b"abe"));
//     assert!(!match_all(&mut seq_abcdef, b"abcdefa"));
//     assert!(!match_all(&mut seq_abcdef, b"aabcdef"));
//     assert!(!match_all(&mut seq_abcdef, b"Xabcdef"));
//     assert!(!match_all(&mut seq_abcdef, b"abcdefX"));
//     assert!(!match_all(&mut seq_abcdef, b"abcdefabcdef"));
//     assert!(match_all(&mut seq_abcdef, b"abcdef"));
//     assert_eq!(Some(0..2), group_ab.range());
//     assert_eq!(Some(2..4), group_cd.range());
//     assert_eq!(Some(4..6), group_ef.range());
//     assert_eq!(Some(1..2), group_b.range());
//     assert_eq!(Some(2..3), group_c.range());
//     assert_eq!(Some(4..5), group_e.range());
//     assert_eq!(Some(5..6), group_f.range());
// }
//
// #[test]
// fn group_debug() {
//     let group: CapturingGroup<DiscardingRange, _> = CapturingGroup::new(Byte::new(b'a'));
//     assert_eq!("CapturingGroup(Byte(a))", format!("{:?}", group));
// }
