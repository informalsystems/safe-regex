#![forbid(unsafe_code)]
use safe_regex::internal::escape_ascii;
use safe_regex::{
    regex, IsMatch, Matcher0, Matcher1, Matcher2, Matcher3, Matcher4, Matcher5, Matcher7,
};

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

// TODO(mleonhard) Test alternate with empty arm, br"a{0}|b"

#[must_use]
fn check_non_matching_permutations(
    matcher: &impl IsMatch,
    prefix: &mut Vec<u8>,
    alphabet: &[u8],
    len: usize,
    ok: &[&[u8]],
) -> Result<(), String> {
    if ok.contains(&prefix.as_slice()) {
        return Ok(());
    }
    if matcher.is_match(&prefix) {
        return Err(format!("matched {:?}", escape_ascii(prefix)));
    }
    if len == 0 {
        return Ok(());
    }
    for b in alphabet {
        prefix.push(*b);
        check_non_matching_permutations(matcher, prefix, alphabet, len - 1, ok)?;
        prefix.pop();
    }
    Ok(())
}

#[must_use]
fn check_permutations(
    matcher: &impl IsMatch,
    alphabet: &[u8],
    len: usize,
    ok: &[&[u8]],
) -> Result<(), String> {
    for s in ok {
        if !matcher.is_match(s) {
            return Err(format!("did not match {:?}", escape_ascii(s)));
        }
    }
    check_non_matching_permutations(matcher, &mut Vec::new(), alphabet, len, ok)
}

fn match_re_fn(data: &[u8]) -> bool {
    regex!(br"a").is_match(data)
}

#[test]
fn test_re_fn() {
    assert!(!match_re_fn(b""));
    assert!(match_re_fn(b"a"));
}

#[test]
fn empty() {
    let re: Matcher0<_> = regex!(br"");
    re.match_all(b"").unwrap();
    assert!(!re.is_match(b"X"));
}

#[test]
fn byte() {
    let matcher: Matcher0<_> = regex!(br"a");
    check_permutations(&matcher, b"aX", 2, &[b"a"]).unwrap();
}

#[test]
fn any_byte() {
    let re: Matcher0<_> = regex!(br".");
    assert!(!re.is_match(b""));
    re.match_all(b"X").unwrap();
    assert!(!re.is_match(b"XY"));
}

#[test]
fn class_inclusive() {
    let re: Matcher0<_> = regex!(br"[abc2-4]");
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    re.match_all(b"a").unwrap();
    re.match_all(b"b").unwrap();
    re.match_all(b"c").unwrap();
    assert!(!re.is_match(b"1"));
    re.match_all(b"2").unwrap();
    re.match_all(b"3").unwrap();
    re.match_all(b"4").unwrap();
    assert!(!re.is_match(b"5"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"aa"));
    assert!(!re.is_match(b"abc"));
}

#[test]
fn class_exclusive() {
    let re: Matcher0<_> = regex!(br"[^abc2-4]");
    assert!(!re.is_match(b""));
    re.match_all(b"X").unwrap();
    re.match_all(b"Y").unwrap();
    assert!(!re.is_match(b"XY"));
    assert!(!re.is_match(b"a"));
    assert!(!re.is_match(b"b"));
    assert!(!re.is_match(b"c"));
    re.match_all(b"1").unwrap();
    assert!(!re.is_match(b"2"));
    assert!(!re.is_match(b"3"));
    assert!(!re.is_match(b"4"));
    re.match_all(b"5").unwrap();
}

#[test]
fn seq() {
    check_permutations(&regex!(br"ab"), b"abX", 5, &[b"ab"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"(ab)");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
    }
}

#[test]
fn alt() {
    check_permutations(&regex!(br"a|b"), b"abX", 3, &[b"a", b"b"]).unwrap();
    check_permutations(
        &regex!(br"(a|b)(c|d)"),
        b"abcdX",
        4,
        &[b"ac", b"ad", b"bc", b"bd"],
    )
    .unwrap();
    {
        let re: Matcher1<_> = regex!(br"(a|b)");
        check_permutations(&re, b"abX", 3, &[b"a", b"b"]).unwrap();

        let groups = re.match_all(b"a").unwrap();
        // assert_eq!(0..1, groups.group_range(0).unwrap());
        assert_eq!("a", escape_ascii(groups.0.unwrap()));

        let groups = re.match_all(b"b").unwrap();
        // assert_eq!(0..1, groups.group_range(0).unwrap());
        assert_eq!("b", escape_ascii(groups.0.unwrap()));
    }
}

#[test]
fn optional() {
    check_permutations(&regex!(br"a?"), b"aX", 2, &[b"", b"a"]).unwrap();
    check_permutations(&regex!(br"a?a"), b"aX", 4, &[b"a", b"aa"]).unwrap();
    check_permutations(&regex!(br"aa?"), b"aX", 4, &[b"a", b"aa"]).unwrap();
    check_permutations(&regex!(br"aaa?"), b"aX", 6, &[b"aa", b"aaa"]).unwrap();
    check_permutations(&regex!(br"(a?)(a?)"), b"aX", 4, &[b"", b"a", b"aa"]).unwrap();
    check_permutations(&regex!(br"(a?)(a?)aa"), b"aX", 8, &[b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(
        &regex!(br"(a?)(a?)(a?)"),
        b"aX",
        6,
        &[b"", b"a", b"aa", b"aaa"],
    )
    .unwrap();
}

#[test]
fn star() {
    check_permutations(
        &regex!(br"a*"),
        b"aX",
        4,
        &[b"", b"a", b"aa", b"aaa", b"aaaa"],
    )
    .unwrap();
    check_permutations(&regex!(br"a*a"), b"aX", 4, &[b"a", b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"aa*"), b"aX", 4, &[b"a", b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(
        &regex!(br"aaa*"),
        b"aX",
        5,
        &[b"aa", b"aaa", b"aaaa", b"aaaaa"],
    )
    .unwrap();
}

#[test]
fn plus() {
    check_permutations(&regex!(br"a+"), b"aX", 4, &[b"a", b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"a+a"), b"aX", 4, &[b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"aa+"), b"aX", 4, &[b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"aaa+"), b"aX", 5, &[b"aaa", b"aaaa", b"aaaaa"]).unwrap();
}

#[test]
fn repeat_empty() {
    check_permutations(
        &regex!(br"a{,}"),
        b"aX",
        4,
        &[b"", b"a", b"aa", b"aaa", b"aaaa"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a{,}a"),
        b"aX",
        4,
        &[b"a", b"aa", b"aaa", b"aaaa"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"aa{,}"),
        b"aX",
        4,
        &[b"a", b"aa", b"aaa", b"aaaa"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"aaa{,}"),
        b"aX",
        5,
        &[b"aa", b"aaa", b"aaaa", b"aaaaa"],
    )
    .unwrap();
}

#[test]
fn repeat_single_num() {
    check_permutations(&regex!(br"a{3}"), b"aX", 6, &[b"aaa"]).unwrap();
    check_permutations(&regex!(br"a{3}a"), b"aX", 7, &[b"aaaa"]).unwrap();
    check_permutations(&regex!(br"aa{3}"), b"aX", 7, &[b"aaaa"]).unwrap();
    check_permutations(&regex!(br"aaa{3}"), b"aX", 8, &[b"aaaaa"]).unwrap();
}

#[test]
fn repeat_min() {
    check_permutations(&regex!(br"a{2,}"), b"aX", 4, &[b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"a{2,}a"), b"aX", 5, &[b"aaa", b"aaaa", b"aaaaa"]).unwrap();
    check_permutations(&regex!(br"aa{2,}"), b"aX", 5, &[b"aaa", b"aaaa", b"aaaaa"]).unwrap();
    check_permutations(&regex!(br"aaa{2,}"), b"aX", 5, &[b"aaaa", b"aaaaa"]).unwrap();
}

#[test]
fn repeat_max() {
    check_permutations(&regex!(br"a{,2}"), b"aX", 4, &[b"", b"a", b"aa"]).unwrap();
    check_permutations(&regex!(br"a{,2}a"), b"aX", 5, &[b"a", b"aa", b"aaa"]).unwrap();
    check_permutations(&regex!(br"aa{,2}"), b"aX", 5, &[b"a", b"aa", b"aaa"]).unwrap();
    check_permutations(&regex!(br"aaa{,2}"), b"aX", 6, &[b"aa", b"aaa", b"aaaa"]).unwrap();
}

#[test]
fn repeat_min_and_max() {
    check_permutations(&regex!(br"a{2,4}"), b"aX", 5, &[b"aa", b"aaa", b"aaaa"]).unwrap();
    check_permutations(&regex!(br"a{2,4}a"), b"aX", 6, &[b"aaa", b"aaaa", b"aaaaa"]).unwrap();
    check_permutations(&regex!(br"aa{2,4}"), b"aX", 6, &[b"aaa", b"aaaa", b"aaaaa"]).unwrap();
    check_permutations(
        &regex!(br"aaa{2,4}"),
        b"aX",
        7,
        &[b"aaaa", b"aaaaa", b"aaaaaa"],
    )
    .unwrap();
}

#[test]
fn repeat_in_seq() {
    // These tests use '*' to catch bad parses like "(aab)*".
    check_permutations(
        &regex!(br"aab*"),
        b"abX",
        6,
        &[b"aa", b"aab", b"aabb", b"aabbb", b"aabbbb"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a*b*"),
        b"abX",
        4,
        &[
            b"", b"a", b"b", b"aa", b"ab", b"aab", b"abb", b"aaab", b"aabb", b"abbb", b"bbbb",
        ],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a*b*c*"),
        b"abcX",
        6,
        &[
            b"", b"a", b"b", b"c", //
            //
            b"aa", b"ab", b"ac", //
            b"bb", b"bc", //
            b"cc", //
            //
            b"aaa", b"aab", b"aac", b"abb", b"abc", b"acc", //
            b"bbb", b"bbc", b"bcc", //
            b"ccc", //
            //
            b"aaaa", b"aaab", b"aaac", b"aabb", b"aabc", b"aacc", b"abbb", b"abbc", b"abcc",
            b"accc", //
            b"bbbb", b"bbbc", b"bbcc", b"bccc", //
            b"cccc", //
            //
            b"aaaaa", b"aaaab", b"aaaac", b"aaabb", b"aaabc", b"aaacc", b"aabbb", b"aabbc",
            b"aabcc", b"aaccc", b"abbbb", b"abbbc", b"abbcc", b"abccc", b"acccc", //
            b"bbbbb", b"bbbbc", b"bbbcc", b"bbccc", b"bcccc", //
            b"ccccc", //
            //
            b"aaaaaa", b"aaaaab", b"aaaaac", b"aaaabb", b"aaaabc", b"aaaacc", b"aaabbb", b"aaabbc",
            b"aaabcc", b"aaaccc", b"aabbbb", b"aabbbc", b"aabbcc", b"aabccc", b"aacccc", b"abbbbb",
            b"abbbbc", b"abbbcc", b"abbccc", b"abcccc", b"accccc", //
            b"bbbbbb", b"bbbbbc", b"bbbbcc", b"bbbccc", b"bbcccc", b"bccccc", //
            b"cccccc", //
        ],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a*b*c*d*"),
        b"abcdX",
        6,
        &[
            b"", b"a", b"b", b"c", b"d", //
            //
            b"aa", b"ab", b"ac", b"ad", //
            b"bb", b"bc", b"bd", //
            b"cc", b"cd", //
            b"dd", //
            //
            b"aaa", b"aab", b"aac", b"aad", b"abb", b"abc", b"abd", b"acc", b"acd", b"add", //
            b"bbb", b"bbc", b"bbd", b"bcc", b"bcd", b"bdd", //
            b"ccc", b"ccd", b"cdd", //
            b"ddd", //
            //
            b"aaaa", b"aaab", b"aaac", b"aaad", b"aabb", b"aabc", b"aabd", b"aacc", b"aacd",
            b"aadd", b"abbb", b"abbc", b"abbd", b"abcc", b"abcd", b"abdd", b"accc", b"accd",
            b"acdd", b"addd", //
            b"bbbb", b"bbbc", b"bbbd", b"bbcc", b"bbcd", b"bbdd", b"bccc", b"bccd", b"bcdd",
            b"bddd", //
            b"cccc", b"cccd", b"ccdd", b"cddd", //
            b"dddd", //
            //
            b"aaaaa", b"aaaab", b"aaaac", b"aaaad", b"aaabb", b"aaabc", b"aaabd", b"aaacc",
            b"aaacd", b"aaadd", b"aabbb", b"aabbc", b"aabbd", b"aabcc", b"aabcd", b"aabdd",
            b"aaccc", b"aaccd", b"aacdd", b"aaddd", b"abbbb", b"abbbc", b"abbbd", b"abbcc",
            b"abbcd", b"abbdd", b"abccc", b"abccd", b"abcdd", b"abddd", b"acccc", b"acccd",
            b"accdd", b"acddd", b"adddd", //
            b"bbbbb", b"bbbbc", b"bbbbd", b"bbbcc", b"bbbcd", b"bbbdd", b"bbccc", b"bbccd",
            b"bbcdd", b"bbddd", b"bcccc", b"bcccd", b"bccdd", b"bcddd", b"bdddd", //
            b"ccccc", b"ccccd", b"cccdd", b"ccddd", b"cdddd", //
            b"ddddd", //
            //
            b"aaaaaa", b"aaaaab", b"aaaaac", b"aaaaad", b"aaaabb", b"aaaabc", b"aaaabd", b"aaaacc",
            b"aaaacd", b"aaaadd", b"aaabbb", b"aaabbc", b"aaabbd", b"aaabcc", b"aaabcd", b"aaabdd",
            b"aaaccc", b"aaaccd", b"aaacdd", b"aaaddd", b"aabbbb", b"aabbbc", b"aabbbd", b"aabbcc",
            b"aabbcd", b"aabbdd", b"aabccc", b"aabccd", b"aabcdd", b"aabddd", b"aacccc", b"aacccd",
            b"aaccdd", b"aacddd", b"aadddd", b"abbbbb", b"abbbbc", b"abbbbd", b"abbbcc", b"abbbcd",
            b"abbbdd", b"abbccc", b"abbccd", b"abbcdd", b"abbddd", b"abcccc", b"abcccd", b"abccdd",
            b"abcddd", b"abdddd", b"accccc", b"accccd", b"acccdd", b"accddd", b"acdddd",
            b"addddd", //
            b"bbbbbb", b"bbbbbc", b"bbbbbd", b"bbbbcc", b"bbbbcd", b"bbbbdd", b"bbbccc", b"bbbccd",
            b"bbbcdd", b"bbbddd", b"bbcccc", b"bbcccd", b"bbccdd", b"bbcddd", b"bbdddd", b"bccccc",
            b"bccccd", b"bcccdd", b"bccddd", b"bcdddd", b"bddddd", //
            b"cccccc", b"cccccd", b"ccccdd", b"cccddd", b"ccdddd", b"cddddd", //
            b"dddddd", //
        ],
    )
    .unwrap();
}

#[test]
fn repeat_in_alt() {
    check_permutations(&regex!(br"a|b*"), b"abX", 2, &[b"", b"a", b"b", b"bb"]).unwrap();

    check_permutations(
        &regex!(br"a|b*c"),
        b"abcX",
        4,
        &[b"a", b"c", b"bc", b"bbc", b"bbbc"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bc*"),
        b"abcX",
        4,
        &[b"a", b"b", b"bc", b"bcc", b"bccc"],
    )
    .unwrap();

    check_permutations(
        &regex!(br"a|b*cd"),
        b"abcX",
        5,
        &[b"a", b"cd", b"bcd", b"bbcd", b"bbbcd"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bc*d"),
        b"abcX",
        5,
        &[b"a", b"bd", b"bcd", b"bccd", b"bcccd"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bcd*"),
        b"abcX",
        6,
        &[b"a", b"bc", b"bcd", b"bcdd", b"bcddd", b"bcdddd"],
    )
    .unwrap();

    check_permutations(
        &regex!(br"a|bb|c*"),
        b"abcX",
        4,
        &[b"", b"a", b"bb", b"c", b"cc"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bb|c*d"),
        b"abcdX",
        5,
        &[b"a", b"bb", b"d", b"cd", b"ccd", b"cccd", b"ccccd"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bb|cd*"),
        b"abcdX",
        4,
        &[b"a", b"bb", b"c", b"cd", b"cdd", b"cddd"],
    )
    .unwrap();

    check_permutations(
        &regex!(br"a|bb|c*de"),
        b"abcdeX",
        6,
        &[b"a", b"bb", b"de", b"cde", b"ccde", b"cccde", b"ccccde"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bb|cd*e"),
        b"abcdeX",
        5,
        &[b"a", b"bb", b"ce", b"cde", b"cdde", b"cddde"],
    )
    .unwrap();
    check_permutations(
        &regex!(br"a|bb|cde*"),
        b"abcdeX",
        6,
        &[b"a", b"bb", b"cd", b"cde", b"cdee", b"cdeee", b"cdeeee"],
    )
    .unwrap();
}

#[test]
fn repeat_in_group() {
    {
        let re: Matcher2<_> = regex!(br"(a?)(ab)");
        check_permutations(&re, b"abX", 6, &[b"ab", b"aab"]).unwrap();

        let groups = re.match_all(b"ab").unwrap();
        assert_eq!("", escape_ascii(groups.0.unwrap()));
        assert_eq!("ab", escape_ascii(groups.1.unwrap()));

        let groups = re.match_all(b"aab").unwrap();
        // assert_eq!(0..1, groups.group_range(0).unwrap());
        // assert_eq!(1..3, groups.group_range(1).unwrap());
        assert_eq!("a", escape_ascii(groups.0.unwrap()));
        assert_eq!("ab", escape_ascii(groups.1.unwrap()));
    }
    {
        let re: Matcher1<_> = regex!(br"a(a?)a");
        check_permutations(&re, b"aX", 5, &[b"aa", b"aaa"]).unwrap();

        let groups = re.match_all(b"aa").unwrap();
        // assert_eq!(1..1, groups.group_range(0).unwrap());
        assert_eq!("", escape_ascii(groups.0.unwrap()));

        let groups = re.match_all(b"aaa").unwrap();
        // assert_eq!(1..2, groups.group_range(0).unwrap());
        assert_eq!("a", escape_ascii(groups.0.unwrap()));
    }
}

#[test]
fn group() {
    {
        let re: Matcher1<_> = regex!(br"(a)");
        check_permutations(&re, b"aX", 2, &[b"a"]).unwrap();
        let groups = re.match_all(b"a").unwrap();
        assert_eq!("a", escape_ascii(groups.0.unwrap()));
        // assert_eq!(0..1, groups.group_range(0).unwrap());
    }
    {
        let re: Matcher2<_> = regex!(br"((a))");
        check_permutations(&re, b"aX", 3, &[b"a"]).unwrap();
        let groups = re.match_all(b"a").unwrap();
        // assert_eq!(0..1, groups.group_range(0).unwrap());
        // assert_eq!(0..1, groups.group_range(1).unwrap());
        assert_eq!("a", escape_ascii(groups.0.unwrap()));
        assert_eq!("a", escape_ascii(groups.1.unwrap()));
    }
    {
        let re: Matcher2<_> = regex!(br"(a(b))");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        // assert_eq!(0..2, groups.group_range(0).unwrap());
        // assert_eq!(1..2, groups.group_range(1).unwrap());
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
        assert_eq!("b", escape_ascii(groups.1.unwrap()));
    }
    {
        let re: Matcher2<_> = regex!(br"((a)b)");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        // assert_eq!(0..2, groups.group_range(0).unwrap());
        // assert_eq!(0..1, groups.group_range(1).unwrap());
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
        assert_eq!("a", escape_ascii(groups.1.unwrap()));
    }
    {
        let re: Matcher3<_> = regex!(br"((a)(b))");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        // assert_eq!(0..2, groups.group_range(0).unwrap());
        // assert_eq!(0..1, groups.group_range(1).unwrap());
        // assert_eq!(1..2, groups.group_range(2).unwrap());
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
        assert_eq!("a", escape_ascii(groups.1.unwrap()));
        assert_eq!("b", escape_ascii(groups.2.unwrap()));
    }
    {
        let re: Matcher4<_> = regex!(br"((a(b))(c))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        // assert_eq!(0..3, groups.group_range(0).unwrap());
        // assert_eq!(0..2, groups.group_range(1).unwrap());
        // assert_eq!(1..2, groups.group_range(2).unwrap());
        // assert_eq!(2..3, groups.group_range(3).unwrap());
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
        assert_eq!("ab", escape_ascii(groups.1.unwrap()));
        assert_eq!("b", escape_ascii(groups.2.unwrap()));
        assert_eq!("c", escape_ascii(groups.3.unwrap()));
    }
    {
        let re: Matcher4<_> = regex!(br"((a)((b)c))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        // assert_eq!(0..3, groups.group_range(0).unwrap());
        // assert_eq!(0..1, groups.group_range(1).unwrap());
        // assert_eq!(1..3, groups.group_range(2).unwrap());
        // assert_eq!(1..2, groups.group_range(3).unwrap());
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
        assert_eq!("a", escape_ascii(groups.1.unwrap()));
        assert_eq!("bc", escape_ascii(groups.2.unwrap()));
        assert_eq!("b", escape_ascii(groups.3.unwrap()));
    }
    {
        let re: Matcher5<_> = regex!(br"((a(b))((c)d))");
        check_permutations(&re, b"abcdX", 5, &[b"abcd"]).unwrap();
        assert!(!re.is_match(b"abcdabcd"));
        let groups = re.match_all(b"abcd").unwrap();
        // assert_eq!(0..4, groups.group_range(0).unwrap());
        // assert_eq!(0..2, groups.group_range(1).unwrap());
        // assert_eq!(1..2, groups.group_range(2).unwrap());
        // assert_eq!(2..4, groups.group_range(3).unwrap());
        // assert_eq!(2..3, groups.group_range(4).unwrap());
        assert_eq!("abcd", escape_ascii(groups.0.unwrap()));
        assert_eq!("ab", escape_ascii(groups.1.unwrap()));
        assert_eq!("b", escape_ascii(groups.2.unwrap()));
        assert_eq!("cd", escape_ascii(groups.3.unwrap()));
        assert_eq!("c", escape_ascii(groups.4.unwrap()));
    }
    {
        let re: Matcher3<_> = regex!(br"(a(b(c)))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        // assert_eq!(0..3, groups.group_range(0).unwrap());
        // assert_eq!(1..3, groups.group_range(1).unwrap());
        // assert_eq!(2..3, groups.group_range(2).unwrap());
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
        assert_eq!("bc", escape_ascii(groups.1.unwrap()));
        assert_eq!("c", escape_ascii(groups.2.unwrap()));
    }
    {
        let re: Matcher7<_> = regex!(br"(a(b))((c)d)((e)(f))");
        check_permutations(&re, b"abcdefX", 6, &[b"abcdef"]).unwrap();
        assert!(!re.is_match(b"Xabcdef"));
        assert!(!re.is_match(b"abcdefX"));
        assert!(!re.is_match(b"abcdefabcdef"));

        let groups = re.match_all(b"abcdef").unwrap();
        // assert_eq!(0..2, groups.group_range(0).unwrap());
        // assert_eq!(1..2, groups.group_range(1).unwrap());
        // assert_eq!(2..4, groups.group_range(2).unwrap());
        // assert_eq!(2..3, groups.group_range(3).unwrap());
        // assert_eq!(4..6, groups.group_range(4).unwrap());
        // assert_eq!(4..5, groups.group_range(5).unwrap());
        // assert_eq!(5..6, groups.group_range(6).unwrap());
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
        assert_eq!("b", escape_ascii(groups.1.unwrap()));
        assert_eq!("cd", escape_ascii(groups.2.unwrap()));
        assert_eq!("c", escape_ascii(groups.3.unwrap()));
        assert_eq!("ef", escape_ascii(groups.4.unwrap()));
        assert_eq!("e", escape_ascii(groups.5.unwrap()));
        assert_eq!("f", escape_ascii(groups.6.unwrap()));
    }
}

#[test]
fn non_capturing_group() {
    check_permutations(&regex!(br"(?:a)"), b"aX", 2, &[b"a"]).unwrap();
    check_permutations(&regex!(br"(?:(?:a))"), b"aX", 3, &[b"a"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a))");
        check_permutations(&re, b"aX", 3, &[b"a"]).unwrap();
        let groups = re.match_all(b"a").unwrap();
        assert_eq!("a", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:a(?:b))"), b"abX", 4, &[b"ab"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"(a(?:b))");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:(?:a)b)"), b"abX", 4, &[b"ab"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a)b)");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:(?:a)(?:b))"), b"abX", 4, &[b"ab"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a)(?:b))");
        check_permutations(&re, b"abX", 4, &[b"ab"]).unwrap();
        let groups = re.match_all(b"ab").unwrap();
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:(?:a(?:b))(?:c))"), b"abcX", 6, &[b"abc"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a(?:b))(?:c))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:(?:a)(?:(?:b)c))"), b"abcX", 6, &[b"abc"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a)(?:(?:b)c))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(
        &regex!(br"(?:(?:a(?:b))(?:(?:c)d))"),
        b"abcdX",
        5,
        &[b"abcd"],
    )
    .unwrap();
    {
        let re: Matcher1<_> = regex!(br"((?:a(?:b))(?:(?:c)d))");
        check_permutations(&re, b"abcdX", 5, &[b"abcd"]).unwrap();
        assert!(!re.is_match(b"abcdabcd"));
        let groups = re.match_all(b"abcd").unwrap();
        assert_eq!("abcd", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(&regex!(br"(?:a(?:b(?:c)))"), b"abcX", 6, &[b"abc"]).unwrap();
    {
        let re: Matcher1<_> = regex!(br"(a(?:b(?:c)))");
        check_permutations(&re, b"abcX", 6, &[b"abc"]).unwrap();
        let groups = re.match_all(b"abc").unwrap();
        assert_eq!("abc", escape_ascii(groups.0.unwrap()));
    }
    check_permutations(
        &regex!(br"(?:a(?:b))(?:(?:c)d)((?:e)(?:f))"),
        b"abcdefX",
        6,
        &[b"abcdef"],
    )
    .unwrap();
    {
        let re: Matcher3<_> = regex!(br"(a(?:b))((?:c)d)((?:e)(?:f))");
        check_permutations(&re, b"abcdefX", 6, &[b"abcdef"]).unwrap();
        assert!(!re.is_match(b"Xabcdef"));
        assert!(!re.is_match(b"abcdefX"));
        assert!(!re.is_match(b"abcdefabcdef"));

        let groups = re.match_all(b"abcdef").unwrap();
        assert_eq!("ab", escape_ascii(groups.0.unwrap()));
        assert_eq!("cd", escape_ascii(groups.1.unwrap()));
        assert_eq!("ef", escape_ascii(groups.2.unwrap()));
    }
}
