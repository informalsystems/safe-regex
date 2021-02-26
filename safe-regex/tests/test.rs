#![forbid(unsafe_code)]
use safe_regex::internal::escape_ascii;
use safe_regex::{regex, Matcher};

// TODO(mleonhard) Test regexes that could match the empty string:
// - ""
// - a?
// - a?b?
// - ()
// - ()?
// - (a?)
// - (a)?
// - (ab)?
// - ()a?
// - ()?a?
// - ()()
// - (a?)(a?)
// - (a)?(b)?
// - (ab)?(cd)?
// - (|a)
// - a{,1}

// TODO(mleonhard) Test greediness

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

#[test]
fn class_inclusive() {
    let re: Matcher<_> = regex!(br"[abc2-4]");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    re.match_all(b"b").unwrap();
    re.match_all(b"c").unwrap();
    assert_eq!(None, re.match_all(b"1"));
    re.match_all(b"2").unwrap();
    re.match_all(b"3").unwrap();
    re.match_all(b"4").unwrap();
    assert_eq!(None, re.match_all(b"5"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"abc"));
}

#[test]
fn class_exclusive() {
    let re: Matcher<_> = regex!(br"[^abc2-4]");
    assert_eq!(None, re.match_all(b""));
    re.match_all(b"X").unwrap();
    re.match_all(b"Y").unwrap();
    assert_eq!(None, re.match_all(b"XY"));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"b"));
    assert_eq!(None, re.match_all(b"c"));
    re.match_all(b"1").unwrap();
    assert_eq!(None, re.match_all(b"2"));
    assert_eq!(None, re.match_all(b"3"));
    assert_eq!(None, re.match_all(b"4"));
    re.match_all(b"5").unwrap();
}

#[test]
fn seq() {
    let re: Matcher<_> = regex!(br"ab");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"b"));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"ba"));
    re.match_all(b"ab").unwrap();
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abb"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b"abXab"));
}

#[test]
fn alt() {
    let re: Matcher<_> = regex!(br"a|b");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"XaY"));
    assert_eq!(None, re.match_all(b"XbY"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));
    re.match_all(b"a").unwrap();
    re.match_all(b"b").unwrap();
}

#[test]
fn group() {
    let re: Matcher<_> = regex!(br"(a)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"aa"));
    let groups = re.match_all(b"a").unwrap();
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
    assert_eq!(0..1, groups.group_range(0).unwrap());
}

#[test]
fn optional() {
    let re: Matcher<_> = regex!(br"a?");
    re.match_all(b"").unwrap();
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
}

#[test]
fn optional_at_start() {
    let re: Matcher<_> = regex!(br"a?a");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaa"));
}

#[test]
fn optional_at_end() {
    let re: Matcher<_> = regex!(br"aa?");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaa"));
}

#[test]
fn optional_in_middle() {
    let re: Matcher<_> = regex!(br"aa?a");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"aXa"));
    re.match_all(b"aa").unwrap();
    re.match_all(b"aaa").unwrap();
    assert_eq!(None, re.match_all(b"aaaa"));
    assert_eq!(None, re.match_all(b"aaaaa"));
    assert_eq!(None, re.match_all(b"aaaaaa"));
    assert_eq!(None, re.match_all(b"Xaaa"));
    assert_eq!(None, re.match_all(b"aaaX"));
    assert_eq!(None, re.match_all(b"XaaaX"));
}

#[test]
fn star() {
    let re: Matcher<_> = regex!(br"a*");
    re.match_all(b"").unwrap();
    assert_eq!(None, re.match_all(b"X"));
    re.match_all(b"a").unwrap();
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    re.match_all(b"aa").unwrap();
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaX"));
    re.match_all(b"aaa").unwrap();
    assert_eq!(None, re.match_all(b"Xaaa"));
    assert_eq!(None, re.match_all(b"aXaa"));
    assert_eq!(None, re.match_all(b"aaXa"));
    assert_eq!(None, re.match_all(b"aaaX"));
    re.match_all(b"aaaa").unwrap();
    assert_eq!(None, re.match_all(b"Xaaaa"));
    assert_eq!(None, re.match_all(b"aXaaa"));
    assert_eq!(None, re.match_all(b"aaXaa"));
    assert_eq!(None, re.match_all(b"aaaXa"));
    assert_eq!(None, re.match_all(b"aaaaX"));
    re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        .unwrap();
    assert_eq!(
        None,
        re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaX")
    );
}

#[test]
fn empty_group_in_seq() {
    let re: Matcher<_> = regex!(br"()a");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..0, groups.group_range(0).unwrap());
    assert_eq!("", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn seq_in_group() {
    let re: Matcher<_> = regex!(br"(ab)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abab"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn alt_in_group() {
    let re: Matcher<_> = regex!(br"(a|b)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"bX"));
    assert_eq!(None, re.match_all(b"Xb"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));

    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));

    let groups = re.match_all(b"b").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!("b", escape_ascii(groups.group(0).unwrap()));
}

#[test]
fn optionals_in_groups() {
    let re: Matcher<_> = regex!(br"(a?)(ab)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aabX"));
    assert_eq!(None, re.match_all(b"Xaab"));
    assert_eq!(None, re.match_all(b"aaXb"));
    assert_eq!(None, re.match_all(b"aXab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b"aaba"));
    assert_eq!(None, re.match_all(b"aabaa"));
    assert_eq!(None, re.match_all(b"aabaab"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!("", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));

    let groups = re.match_all(b"aab").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(1..3, groups.group_range(1).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));
}

#[test]
fn optional_in_group() {
    let re: Matcher<_> = regex!(br"a(a?)a");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));

    let groups = re.match_all(b"aa").unwrap();
    assert_eq!(1..1, groups.group_range(0).unwrap());
    assert_eq!("", escape_ascii(groups.group(0).unwrap()));

    assert_eq!(None, re.match_all(b"Xaa"));
    assert_eq!(None, re.match_all(b"aXa"));
    assert_eq!(None, re.match_all(b"aaX"));
    assert_eq!(None, re.match_all(b"XaaX"));
    assert_eq!(None, re.match_all(b"Xaaa"));
    assert_eq!(None, re.match_all(b"aXaa"));
    assert_eq!(None, re.match_all(b"aaXa"));
    assert_eq!(None, re.match_all(b"aaaX"));
    assert_eq!(None, re.match_all(b"XaaaX"));

    let groups = re.match_all(b"aaa").unwrap();
    assert_eq!(1..2, groups.group_range(0).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));

    assert_eq!(None, re.match_all(b"aaaa"));
    assert_eq!(None, re.match_all(b"aaaaa"));
    assert_eq!(None, re.match_all(b"aaaaaa"));
}

#[test]
fn alt_in_seq() {
    let re: Matcher<_> = regex!(br"(a|b)(c|d)");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"Xac"));
    assert_eq!(None, re.match_all(b"aXc"));
    assert_eq!(None, re.match_all(b"acX"));
    assert_eq!(None, re.match_all(b"Xad"));
    assert_eq!(None, re.match_all(b"aXd"));
    assert_eq!(None, re.match_all(b"adX"));
    assert_eq!(None, re.match_all(b"Xbc"));
    assert_eq!(None, re.match_all(b"bXc"));
    assert_eq!(None, re.match_all(b"bcX"));
    assert_eq!(None, re.match_all(b"Xbd"));
    assert_eq!(None, re.match_all(b"bXd"));
    assert_eq!(None, re.match_all(b"bdX"));
    assert_eq!(None, re.match_all(b"XacY"));
    assert_eq!(None, re.match_all(b"XadY"));
    assert_eq!(None, re.match_all(b"XbcY"));
    assert_eq!(None, re.match_all(b"XbdY"));
    assert_eq!(None, re.match_all(b"aac"));
    assert_eq!(None, re.match_all(b"add"));
    assert_eq!(None, re.match_all(b"acac"));
    assert_eq!(None, re.match_all(b"acbd"));
    re.match_all(b"ac").unwrap();
    re.match_all(b"ad").unwrap();
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));
    re.match_all(b"bc").unwrap();
    re.match_all(b"bd").unwrap();
    assert_eq!(None, re.match_all(b"ca"));
    assert_eq!(None, re.match_all(b"cb"));
    assert_eq!(None, re.match_all(b"cc"));
    assert_eq!(None, re.match_all(b"cd"));
    assert_eq!(None, re.match_all(b"da"));
    assert_eq!(None, re.match_all(b"db"));
    assert_eq!(None, re.match_all(b"dc"));
    assert_eq!(None, re.match_all(b"dd"));
}

#[test]
fn group_nested1() {
    let re: Matcher<_> = regex!(br"((a))");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"aa"));

    let groups = re.match_all(b"a").unwrap();
    assert_eq!(0..1, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!("a", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested2() {
    let re: Matcher<_> = regex!(br"(a(b))");
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abb"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!(1..2, groups.group_range(1).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(1).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested3() {
    let re: Matcher<_> = regex!(br"((a)b)");
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abb"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested4() {
    let re: Matcher<_> = regex!(br"((a)(b))");
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aab"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abb"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));

    let groups = re.match_all(b"ab").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!(1..2, groups.group_range(2).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(2).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested5() {
    let re: Matcher<_> = regex!(br"((a(b))(c))");
    assert_eq!(None, re.match_all(b"Xabc"));
    assert_eq!(None, re.match_all(b"aXbc"));
    assert_eq!(None, re.match_all(b"abXc"));
    assert_eq!(None, re.match_all(b"abcX"));
    assert_eq!(None, re.match_all(b"aabc"));
    assert_eq!(None, re.match_all(b"abca"));
    assert_eq!(None, re.match_all(b"abcc"));
    assert_eq!(None, re.match_all(b"abcabc"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"ab"));

    let groups = re.match_all(b"abc").unwrap();
    assert_eq!(0..3, groups.group_range(0).unwrap());
    assert_eq!(0..2, groups.group_range(1).unwrap());
    assert_eq!(1..2, groups.group_range(2).unwrap());
    assert_eq!(2..3, groups.group_range(3).unwrap());
    assert_eq!("abc", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(2).unwrap()));
    assert_eq!("c", escape_ascii(groups.group(3).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested6() {
    let re: Matcher<_> = regex!(br"((a)((b)c))");
    assert_eq!(None, re.match_all(b"Xabc"));
    assert_eq!(None, re.match_all(b"aXbc"));
    assert_eq!(None, re.match_all(b"abXc"));
    assert_eq!(None, re.match_all(b"abcX"));
    assert_eq!(None, re.match_all(b"aabc"));
    assert_eq!(None, re.match_all(b"abca"));
    assert_eq!(None, re.match_all(b"abcc"));
    assert_eq!(None, re.match_all(b"abcabc"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"ab"));

    let groups = re.match_all(b"abc").unwrap();
    assert_eq!(0..3, groups.group_range(0).unwrap());
    assert_eq!(0..1, groups.group_range(1).unwrap());
    assert_eq!(1..3, groups.group_range(2).unwrap());
    assert_eq!(1..2, groups.group_range(3).unwrap());
    assert_eq!("abc", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("a", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("bc", escape_ascii(groups.group(2).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(3).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested7() {
    let re: Matcher<_> = regex!(br"((a(b))((c)d))");
    assert_eq!(None, re.match_all(b"Xabcd"));
    assert_eq!(None, re.match_all(b"aXbcd"));
    assert_eq!(None, re.match_all(b"abXcd"));
    assert_eq!(None, re.match_all(b"abcXd"));
    assert_eq!(None, re.match_all(b"abcdX"));
    assert_eq!(None, re.match_all(b"aabcd"));
    assert_eq!(None, re.match_all(b"abcda"));
    assert_eq!(None, re.match_all(b"abcdd"));
    assert_eq!(None, re.match_all(b"abcdabcd"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"abc"));

    let groups = re.match_all(b"abcd").unwrap();
    assert_eq!(0..4, groups.group_range(0).unwrap());
    assert_eq!(0..2, groups.group_range(1).unwrap());
    assert_eq!(1..2, groups.group_range(2).unwrap());
    assert_eq!(2..4, groups.group_range(3).unwrap());
    assert_eq!(2..3, groups.group_range(4).unwrap());
    assert_eq!("abcd", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("ab", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(2).unwrap()));
    assert_eq!("cd", escape_ascii(groups.group(3).unwrap()));
    assert_eq!("c", escape_ascii(groups.group(4).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested8() {
    let re: Matcher<_> = regex!(br"(a(b(c)))");
    assert_eq!(None, re.match_all(b"Xabc"));
    assert_eq!(None, re.match_all(b"aXbc"));
    assert_eq!(None, re.match_all(b"abXc"));
    assert_eq!(None, re.match_all(b"abcX"));
    assert_eq!(None, re.match_all(b"aabc"));
    assert_eq!(None, re.match_all(b"abca"));
    assert_eq!(None, re.match_all(b"abcc"));
    assert_eq!(None, re.match_all(b"abcabc"));
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"ab"));

    let groups = re.match_all(b"abc").unwrap();
    assert_eq!(0..3, groups.group_range(0).unwrap());
    assert_eq!(1..3, groups.group_range(1).unwrap());
    assert_eq!(2..3, groups.group_range(2).unwrap());
    assert_eq!("abc", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("bc", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("c", escape_ascii(groups.group(2).unwrap()));
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested9() {
    let re: Matcher<_> = regex!(br"(a(b))((c)d)((e)(f))");
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"a"));
    assert_eq!(None, re.match_all(b"ab"));
    assert_eq!(None, re.match_all(b"abc"));
    assert_eq!(None, re.match_all(b"abd"));
    assert_eq!(None, re.match_all(b"abe"));
    assert_eq!(None, re.match_all(b"abcdefa"));
    assert_eq!(None, re.match_all(b"aabcdef"));
    assert_eq!(None, re.match_all(b"Xabcdef"));
    assert_eq!(None, re.match_all(b"abcdefX"));
    assert_eq!(None, re.match_all(b"abcdefabcdef"));

    let groups = re.match_all(b"abcdef").unwrap();
    assert_eq!(0..2, groups.group_range(0).unwrap());
    assert_eq!(1..2, groups.group_range(1).unwrap());
    assert_eq!(2..4, groups.group_range(2).unwrap());
    assert_eq!(2..3, groups.group_range(3).unwrap());
    assert_eq!(4..6, groups.group_range(4).unwrap());
    assert_eq!(4..5, groups.group_range(5).unwrap());
    assert_eq!(5..6, groups.group_range(6).unwrap());
    assert_eq!("ab", escape_ascii(groups.group(0).unwrap()));
    assert_eq!("b", escape_ascii(groups.group(1).unwrap()));
    assert_eq!("cd", escape_ascii(groups.group(2).unwrap()));
    assert_eq!("c", escape_ascii(groups.group(3).unwrap()));
    assert_eq!("ef", escape_ascii(groups.group(4).unwrap()));
    assert_eq!("e", escape_ascii(groups.group(5).unwrap()));
    assert_eq!("f", escape_ascii(groups.group(6).unwrap()));
}
