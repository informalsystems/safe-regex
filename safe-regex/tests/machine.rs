#![forbid(unsafe_code)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
use safe_regex::internal::escape_ascii;

#[test]
fn byte() {
    // regex!(br"a")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b0
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
}

#[test]
fn any_byte() {
    // regex!(br".")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start.clone();
            start = None;
        }
        b0
    };
    assert_eq!(None, re(b""));
    re(b"X").unwrap();
    assert_eq!(None, re(b"XY"));
}

#[test]
fn class_inclusive() {
    // regex!(br"[abc2-4]")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start
                .clone()
                .filter(|_| (*b == 97u8 || *b == 98u8 || *b == 99u8 || (50u8..=52u8).contains(b)));
            start = None;
        }
        b0
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    re(b"b").unwrap();
    re(b"c").unwrap();
    assert_eq!(None, re(b"1"));
    re(b"2").unwrap();
    re(b"3").unwrap();
    re(b"4").unwrap();
    assert_eq!(None, re(b"5"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(None, re(b"abc"));
}

#[test]
fn class_exclusive() {
    // regex!(br"[^abc2-4]")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start
                .clone()
                .filter(|_| (*b != 97u8 && *b != 98u8 && *b != 99u8 && !(50u8..=52u8).contains(b)));
            start = None;
        }
        b0
    };
    assert_eq!(None, re(b""));
    re(b"X").unwrap();
    re(b"Y").unwrap();
    assert_eq!(None, re(b"XY"));
    assert_eq!(None, re(b"a"));
    assert_eq!(None, re(b"b"));
    assert_eq!(None, re(b"c"));
    re(b"1").unwrap();
    assert_eq!(None, re(b"2"));
    assert_eq!(None, re(b"3"));
    assert_eq!(None, re(b"4"));
    re(b"5").unwrap();
}

#[test]
fn seq() {
    // regex!(br"aab")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        let mut b1 = None;
        let mut b2 = None;
        for b in data.iter() {
            b2 = b1.clone().filter(|_| *b == b'b');
            b1 = b0.clone().filter(|_| *b == b'a');
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b2
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(None, re(b"Xaa"));
    assert_eq!(None, re(b"aXa"));
    assert_eq!(None, re(b"aaX"));
    assert_eq!(None, re(b"aaa"));
    re(b"aab").unwrap();
    assert_eq!(None, re(b"Xaab"));
    assert_eq!(None, re(b"aXab"));
    assert_eq!(None, re(b"aaXb"));
    assert_eq!(None, re(b"aabX"));
    assert_eq!(None, re(b"aaba"));
    assert_eq!(None, re(b"aabaa"));
    assert_eq!(None, re(b"aabaab"));
}

#[test]
fn alt() {
    // regex!(br"a|b")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        let mut b1 = None;
        for b in data.iter() {
            b1 = start.clone().filter(|_| *b == b'b');
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b0.or(b1)
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    re(b"b").unwrap();
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"bX"));
    assert_eq!(None, re(b"Xb"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(None, re(b"ab"));
    assert_eq!(None, re(b"ba"));
    assert_eq!(None, re(b"bb"));
}

#[test]
fn group() {
    // regex!(br"(a)")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b0 = start.clone().filter(|_| *b == b'a').map(|_| (n..n + 1,));
            start = None;
        }
        b0.map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
    assert_eq!("a", escape_ascii(re(b"a").unwrap().0.unwrap()));
}

#[test]
fn optional() {
    // regex!(br"a?")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        start.or(b0)
    };
    re(b"").unwrap();
    re(b"a").unwrap();
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
}

#[test]
fn optional_at_start() {
    // regex!(br"a?a")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        let mut b1 = None;
        for b in data.iter() {
            b1 = start.clone().or(b0.clone()).filter(|_| *b == b'a');
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b1
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    re(b"aa").unwrap();
    assert_eq!(None, re(b"aaX"));
    assert_eq!(None, re(b"Xaa"));
    assert_eq!(None, re(b"aXa"));
    assert_eq!(None, re(b"aaa"));
}

#[test]
fn optional_at_end() {
    // regex!(br"aa?")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        let mut b1 = None;
        for b in data.iter() {
            b1 = b0.clone().filter(|_| *b == b'a');
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b0.or(b1)
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    re(b"aa").unwrap();
    assert_eq!(None, re(b"aaX"));
    assert_eq!(None, re(b"Xaa"));
    assert_eq!(None, re(b"aXa"));
    assert_eq!(None, re(b"aaa"));
}

#[test]
fn optionals_in_groups() {
    // regex!(br"(a?)(a?)aa")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>, Option<&[u8]>)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX, usize::MAX..usize::MAX));
        let mut b0: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut b1: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut b2: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut b3: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        for (n, b) in data.iter().enumerate().map(|(n, b)| (n, b)) {
            b3 = b2.clone().filter(|_| *b == b'a');
            b2 = None
                .or_else(|| {
                    start
                        .clone()
                        .map(|(r0, r1)| (n..n, r1))
                        .map(|(r0, r1)| (r0.start..n, r1))
                        .map(|(r0, r1)| (r0, n..n))
                        .map(|(r0, r1)| (r0, r1.start..n))
                })
                .or_else(|| {
                    b0.clone()
                        .map(|(r0, r1)| (r0, n..n))
                        .map(|(r0, r1)| (r0, r1.start..n))
                })
                .or_else(|| b1.clone())
                .filter(|_| *b == b'a');
            b1 = None
                .or_else(|| {
                    start
                        .clone()
                        .map(|(r0, r1)| (n..n, r1))
                        .map(|(r0, r1)| (r0.start..n, r1))
                })
                .or_else(|| b0.clone())
                .filter(|_| *b == b'a')
                .map(|(r0, r1)| (r0, n..n))
                .map(|(r0, r1)| (r0, r1.start..n + 1));
            b0 = start
                .clone()
                .map(|(r0, r1)| (n..n, r1))
                .filter(|_| *b == b'a')
                .map(|(r0, r1)| (r0.start..n + 1, r1));
            start = None;
        }
        b3.map(|(r0, r1)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
                //
                if r1.start != usize::MAX && r1.end != usize::MAX {
                    Some(&data[r1])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"a"));
    assert_eq!(None, re(b"Xaa"));
    assert_eq!(None, re(b"aXa"));
    assert_eq!(None, re(b"aaX"));
    assert_eq!(None, re(b"Xaaa"));
    assert_eq!(None, re(b"aXaa"));
    assert_eq!(None, re(b"aaXa"));
    assert_eq!(None, re(b"aaaX"));

    let groups = re(b"aa").unwrap();
    assert_eq!("", escape_ascii(groups.0.unwrap()));
    assert_eq!("", escape_ascii(groups.1.unwrap()));

    let groups = re(b"aaa").unwrap();
    assert_eq!("a", escape_ascii(groups.0.unwrap()));
    assert_eq!("", escape_ascii(groups.1.unwrap()));

    let groups = re(b"aaaa").unwrap();
    assert_eq!("a", escape_ascii(groups.0.unwrap()));
    assert_eq!("a", escape_ascii(groups.1.unwrap()));

    assert_eq!(None, re(b"Xaaaa"));
    assert_eq!(None, re(b"aXaaa"));
    assert_eq!(None, re(b"aaXaa"));
    assert_eq!(None, re(b"aaaXa"));
    assert_eq!(None, re(b"aaaaX"));
    assert_eq!(None, re(b"aaaaa"));
    assert_eq!(None, re(b"aaaaaaaa"));
}

#[test]
fn star() {
    // regex!(br"a*")
    let re: fn(&[u8]) -> Option<()> = |data: &[u8]| {
        let mut start = Some(());
        let mut b0 = None;
        for b in data.iter() {
            b0 = start.clone().or(b0.clone()).filter(|_| *b == b'a');
            start = None;
        }
        start.or(b0)
    };
    re(b"").unwrap();
    assert_eq!(None, re(b"X"));
    re(b"a").unwrap();
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    re(b"aa").unwrap();
    assert_eq!(None, re(b"Xaa"));
    assert_eq!(None, re(b"aXa"));
    assert_eq!(None, re(b"aaX"));
    re(b"aaa").unwrap();
    assert_eq!(None, re(b"Xaaa"));
    assert_eq!(None, re(b"aXaa"));
    assert_eq!(None, re(b"aaXa"));
    assert_eq!(None, re(b"aaaX"));
    re(b"aaaa").unwrap();
    assert_eq!(None, re(b"Xaaaa"));
    assert_eq!(None, re(b"aXaaa"));
    assert_eq!(None, re(b"aaXaa"));
    assert_eq!(None, re(b"aaaXa"));
    assert_eq!(None, re(b"aaaaX"));
    re(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    assert_eq!(None, re(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaX"));
}

#[test]
fn empty_group_at_start() {
    // regex!(br"()a")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((0..0,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b0 = start.clone().filter(|_| *b == b'a');
            start = None;
        }
        b0.map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(b"", re(b"a").unwrap().0.unwrap());
}

#[test]
fn empty_group_at_end() {
    // regex!(br"a()")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b0 = start.clone().filter(|_| *b == b'a').map(|_| (n..n,));
            start = None;
        }
        b0.map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(b"", re(b"a").unwrap().0.unwrap());
}

#[test]
fn empty_group_in_middle() {
    // regex!(br"a()b")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b1 = b0.clone().filter(|_| *b == b'b');
            b0 = start.clone().filter(|_| *b == b'a').map(|_| (n..n,));
            start = None;
        }
        b1.map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(None, re(b"bb"));
    let groups = re(b"ab").unwrap();
    assert_eq!(b"", groups.0.unwrap());
    assert_eq!(None, re(b"Xab"));
    assert_eq!(None, re(b"aXb"));
    assert_eq!(None, re(b"abX"));
    assert_eq!(None, re(b"aba"));
    assert_eq!(None, re(b"abab"));
}

#[test]
fn seq_in_group() {
    // regex!(br"(abc)d")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        let mut b2: Option<(core::ops::Range<usize>,)> = None;
        let mut b3: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b3 = b2.clone().filter(|_| *b == b'd');
            b2 = b1
                .clone()
                .filter(|_| *b == b'c')
                .map(|(r0,)| (r0.start..n + 1,));
            b1 = b0
                .clone()
                .filter(|_| *b == b'b')
                .map(|(r0,)| (r0.start..n + 1,));
            b0 = start.clone().filter(|_| *b == b'a').map(|_| (n..n + 1,));
            start = None;
        }
        b3.map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"a"));
    assert_eq!(None, re(b"d"));
    assert_eq!(None, re(b"ab"));
    assert_eq!(None, re(b"bc"));
    assert_eq!(None, re(b"cd"));
    assert_eq!(None, re(b"abc"));
    assert_eq!(None, re(b"acd"));
    assert_eq!("abc", escape_ascii(re(b"abcd").unwrap().0.unwrap()));
    assert_eq!(None, re(b"abcda"));
    assert_eq!(None, re(b"abcdabcd"));
    assert_eq!(None, re(b"Xabcd"));
    assert_eq!(None, re(b"aXbcd"));
    assert_eq!(None, re(b"abXcd"));
    assert_eq!(None, re(b"abcXd"));
    assert_eq!(None, re(b"abcdX"));
}

#[test]
fn alt_in_group() {
    // regex!(br"(a|b)")
    let re: fn(&[u8]) -> Option<(Option<&[u8]>,)> = |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b1 = start.clone().filter(|_| *b == b'b').map(|_| (n..n + 1,));
            b0 = start.clone().filter(|_| *b == b'a').map(|_| (n..n + 1,));
            start = None;
        }
        b0.or(b1).map(|(r0,)| {
            (
                //
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
            )
        })
    };
    assert_eq!(None, re(b""));
    assert_eq!(None, re(b"X"));
    assert_eq!("a", escape_ascii(re(b"a").unwrap().0.unwrap()));
    assert_eq!("b", escape_ascii(re(b"b").unwrap().0.unwrap()));
    assert_eq!(None, re(b"aX"));
    assert_eq!(None, re(b"Xa"));
    assert_eq!(None, re(b"bX"));
    assert_eq!(None, re(b"Xb"));
    assert_eq!(None, re(b"ab"));
    assert_eq!(None, re(b"aa"));
    assert_eq!(None, re(b"ba"));
    assert_eq!(None, re(b"bb"));
}
