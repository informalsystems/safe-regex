#![forbid(unsafe_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(clippy::unseparated_literal_suffix)]
use safe_regex::internal::escape_ascii;
use safe_regex::{IsMatch, Matcher0, Matcher1, Matcher2};

#[test]
fn byte() {
    // regex!(br"a")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return b0;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aa"));
}

#[test]
fn any_byte() {
    // regex!(br".")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone();
                start = None;
            } else {
                return b0;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(re.is_match(b"X"));
    assert!(!re.is_match(b"XY"));
}

#[test]
fn class_inclusive() {
    // regex!(br"[abc2-4]")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone().filter(|_| {
                    *b == 97u8 || *b == 98u8 || *b == 99u8 || (50u8..=52u8).contains(b)
                });
                start = None;
            } else {
                return b0;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(re.is_match(b"b"));
    assert!(re.is_match(b"c"));
    assert!(!re.is_match(b"1"));
    assert!(re.is_match(b"2"));
    assert!(re.is_match(b"3"));
    assert!(re.is_match(b"4"));
    assert!(!re.is_match(b"5"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"aa"));
    assert!(!re.is_match(b"abc"));
}

#[test]
fn class_exclusive() {
    // regex!(br"[^abc2-4]")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone().filter(|_| {
                    *b != 97u8 && *b != 98u8 && *b != 99u8 && !(50u8..=52u8).contains(b)
                });
                start = None;
            } else {
                return b0;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(re.is_match(b"X"));
    assert!(re.is_match(b"Y"));
    assert!(!re.is_match(b"XY"));
    assert!(!re.is_match(b"a"));
    assert!(!re.is_match(b"b"));
    assert!(!re.is_match(b"c"));
    assert!(re.is_match(b"1"));
    assert!(!re.is_match(b"2"));
    assert!(!re.is_match(b"3"));
    assert!(!re.is_match(b"4"));
    assert!(re.is_match(b"5"));
}

#[test]
fn seq() {
    // regex!(br"aab")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut b1: Option<()> = None;
        let mut b2: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b2 = b1.clone().filter(|_| {
                    //
                    *b == 98u8
                });
                b1 = b0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                b0 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return b2;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aa"));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaX"));
    assert!(!re.is_match(b"aaa"));
    assert!(re.is_match(b"aab"));
    assert!(!re.is_match(b"Xaab"));
    assert!(!re.is_match(b"aXab"));
    assert!(!re.is_match(b"aaXb"));
    assert!(!re.is_match(b"aabX"));
    assert!(!re.is_match(b"aaba"));
    assert!(!re.is_match(b"aabaa"));
    assert!(!re.is_match(b"aabaab"));
}

#[test]
fn alt() {
    // regex!(br"a|b")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut b1: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b1 = start.clone().filter(|_| {
                    //
                    *b == 98u8
                });
                b0 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return None.or_else(|| b0.clone()).or_else(|| b1.clone());
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(re.is_match(b"b"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"bX"));
    assert!(!re.is_match(b"Xb"));
    assert!(!re.is_match(b"aa"));
    assert!(!re.is_match(b"ab"));
    assert!(!re.is_match(b"ba"));
    assert!(!re.is_match(b"bb"));
}

#[test]
fn group() {
    // regex!(br"(a)")
    let re: safe_regex::Matcher1<_> = safe_regex::Matcher1::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut accept: Option<(core::ops::Range<usize>,)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            accept = b0.clone();
            if let Some(b) = data_iter.next() {
                b0 = start
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                start = None;
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            (if r0.start != usize::MAX && r0.end != usize::MAX {
                Some(&data[r0])
            } else {
                None
            },)
        })
    });
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
    assert_eq!("a", escape_ascii(re.match_all(b"a").unwrap().0.unwrap()));
}

#[test]
fn groups_nested() {
    // regex!(br"(a(b))")
    let re: safe_regex::Matcher2<_> = safe_regex::Matcher2::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX, usize::MAX..usize::MAX));
        let mut b0: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut b1: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut accept: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            accept = b1.clone();
            if let Some(b) = data_iter.next() {
                b1 = b0
                    .clone()
                    .map(|(r0, r1)| (r0, n..n))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 98u8
                    })
                    .map(|(r0, r1)| (r0.start..n + 1, r1.start..n + 1));
                b0 = start
                    .clone()
                    .map(|(r0, r1)| (n..n, r1))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0, r1)| (r0.start..n + 1, r1));
                start = None;
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0, r1)| {
            (
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
                if r1.start != usize::MAX && r1.end != usize::MAX {
                    Some(&data[r1])
                } else {
                    None
                },
            )
        })
    });
    assert_eq!(None, re.match_all(b""));
    assert_eq!(None, re.match_all(b"X"));
    assert_eq!(None, re.match_all(b"aX"));
    assert_eq!(None, re.match_all(b"Xa"));
    assert_eq!(None, re.match_all(b"aa"));
    let groups = re.match_all(b"ab").unwrap();
    assert_eq!("ab", escape_ascii(groups.0.unwrap()));
    assert_eq!("b", escape_ascii(groups.1.unwrap()));
    assert_eq!(None, re.match_all(b"ba"));
    assert_eq!(None, re.match_all(b"bb"));
    assert_eq!(None, re.match_all(b"Xab"));
    assert_eq!(None, re.match_all(b"aXb"));
    assert_eq!(None, re.match_all(b"abX"));
    assert_eq!(None, re.match_all(b"aba"));
    assert_eq!(None, re.match_all(b"abab"));
    assert_eq!(None, re.match_all(b"abXab"));
}

#[test]
fn optional() {
    // regex!(br"a?")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b1: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let opt0 = start.clone().or_else(|| b1.clone());
            if let Some(b) = data_iter.next() {
                b1 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return opt0;
            }
        }
    });
    assert!(re.is_match(b""));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"X"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aa"));
}

// TODO(mleonhard) Add these and others to compiler test.
#[test]
fn optional_at_start() {
    // regex!(br"a?a")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b1: Option<()> = None;
        let mut b2: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let opt0 = start.clone().or_else(|| b1.clone());
            if let Some(b) = data_iter.next() {
                b2 = opt0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                b1 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return b2;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(re.is_match(b"aa"));
    assert!(!re.is_match(b"aaX"));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaa"));
}

#[test]
fn optional_at_end() {
    // regex!(br"aa?")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut b2: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let opt1 = b0.clone().or_else(|| b2.clone());
            if let Some(b) = data_iter.next() {
                b2 = b0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                b0 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return opt1;
            }
        }
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(re.is_match(b"aa"));
    assert!(!re.is_match(b"aaX"));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaa"));
}

#[test]
fn optionals_in_seq() {
    // regex!(br"a?a?a?")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b1: Option<()> = None;
        let mut b3: Option<()> = None;
        let mut b5: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let opt0 = start.clone().or_else(|| b1.clone());
            let opt2 = opt0.clone().or_else(|| b3.clone());
            let opt4 = opt2.clone().or_else(|| b5.clone());
            if let Some(b) = data_iter.next() {
                b5 = opt2.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                b3 = opt0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                b1 = start.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return opt4;
            }
        }
    });
    assert!(re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aX"));
    assert!(re.is_match(b"aa"));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaX"));
    assert!(re.is_match(b"aaa"));
    assert!(!re.is_match(b"Xaaa"));
    assert!(!re.is_match(b"aXaa"));
    assert!(!re.is_match(b"aaXa"));
    assert!(!re.is_match(b"aaaX"));
    assert!(!re.is_match(b"aaaaaa"));
    assert!(!re.is_match(b"aaaXaaa"));
}

#[test]
fn optionals_in_groups() {
    // regex!(br"(a?)(a?)")
    let re: Matcher2<_> = safe_regex::Matcher2::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX, usize::MAX..usize::MAX));
        let mut b1: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut b3: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut accept: Option<(core::ops::Range<usize>, core::ops::Range<usize>)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            let opt0 = start
                .clone()
                .map(|(r0, r1)| (n..n, r1))
                .clone()
                .or_else(|| b1.clone());
            let opt2 = opt0
                .clone()
                .map(|(r0, r1)| (r0, n..n))
                .clone()
                .or_else(|| b3.clone());
            accept = opt2.clone();
            if let Some(b) = data_iter.next() {
                b3 = opt0
                    .clone()
                    .map(|(r0, r1)| (r0, n..n))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0, r1)| (r0, r1.start..n + 1));
                b1 = start
                    .clone()
                    .map(|(r0, r1)| (n..n, r1))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0, r1)| (r0.start..n + 1, r1));
                start = None;
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0, r1)| {
            (
                if r0.start != usize::MAX && r0.end != usize::MAX {
                    Some(&data[r0])
                } else {
                    None
                },
                if r1.start != usize::MAX && r1.end != usize::MAX {
                    Some(&data[r1])
                } else {
                    None
                },
            )
        })
    });
    let groups = re.match_all(b"").unwrap();
    assert_eq!("", escape_ascii(groups.0.unwrap()));
    assert_eq!("", escape_ascii(groups.1.unwrap()));
    assert!(!re.is_match(b"X"));
    let groups = re.match_all(b"a").unwrap();
    assert_eq!("a", escape_ascii(groups.0.unwrap()));
    assert_eq!("", escape_ascii(groups.1.unwrap()));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"aX"));
    let groups = re.match_all(b"aa").unwrap();
    assert_eq!("a", escape_ascii(groups.0.unwrap()));
    assert_eq!("a", escape_ascii(groups.1.unwrap()));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaX"));
    assert!(!re.is_match(b"aaaa"));
    assert!(!re.is_match(b"aaXaa"));
}

#[test]
fn star() {
    // regex!(br"a*")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b1: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let star0 = start.clone().or_else(|| b1.clone());
            if let Some(b) = data_iter.next() {
                b1 = star0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return star0;
            }
        }
    });
    assert!(re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(re.is_match(b"a"));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(re.is_match(b"aa"));
    assert!(!re.is_match(b"Xaa"));
    assert!(!re.is_match(b"aXa"));
    assert!(!re.is_match(b"aaX"));
    assert!(re.is_match(b"aaa"));
    assert!(!re.is_match(b"Xaaa"));
    assert!(!re.is_match(b"aXaa"));
    assert!(!re.is_match(b"aaXa"));
    assert!(!re.is_match(b"aaaX"));
    assert!(re.is_match(b"aaaa"));
    assert!(!re.is_match(b"Xaaaa"));
    assert!(!re.is_match(b"aXaaa"));
    assert!(!re.is_match(b"aaXaa"));
    assert!(!re.is_match(b"aaaXa"));
    assert!(!re.is_match(b"aaaaX"));
    assert!(re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    assert!(!re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaX"));
}

#[test]
fn seq_in_star() {
    // regex!(br"(?:abc)*")
    let re: Matcher0<_> = safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b1: Option<()> = None;
        let mut b2: Option<()> = None;
        let mut b3: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            let star0 = start.clone().or_else(|| b3.clone());
            if let Some(b) = data_iter.next() {
                b3 = b2.clone().filter(|_| {
                    //
                    *b == 99u8
                });
                b2 = b1.clone().filter(|_| {
                    //
                    *b == 98u8
                });
                b1 = star0.clone().filter(|_| {
                    //
                    *b == 97u8
                });
                start = None;
            } else {
                return star0;
            }
        }
    });
    assert!(re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert!(!re.is_match(b"a"));
    assert!(!re.is_match(b"ab"));
    assert!(!re.is_match(b"Xabc"));
    assert!(!re.is_match(b"aXbc"));
    assert!(!re.is_match(b"abXc"));
    assert!(!re.is_match(b"abcX"));
    assert!(!re.is_match(b"abca"));
    assert!(!re.is_match(b"abcab"));
    assert!(!re.is_match(b"abcXabc"));
    assert!(re.is_match(b"abc"));
    assert!(re.is_match(b"abcabc"));
    assert!(re.is_match(b"abcabcabc"));
    assert!(re.is_match(b"abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc"));
    assert!(!re.is_match(b"abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcX"));
}

#[test]
fn seq_in_group() {
    // regex!(br"(abc)d")
    let re: Matcher1<_> = safe_regex::Matcher1::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        let mut b2: Option<(core::ops::Range<usize>,)> = None;
        let mut b3: Option<(core::ops::Range<usize>,)> = None;
        let mut accept: Option<(core::ops::Range<usize>,)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            accept = b3.clone();
            if let Some(b) = data_iter.next() {
                b3 = b2.clone().filter(|_| {
                    //
                    *b == 100u8
                });
                b2 = b1
                    .clone()
                    .filter(|_| {
                        //
                        *b == 99u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                b1 = b0
                    .clone()
                    .filter(|_| {
                        //
                        *b == 98u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                b0 = start
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                start = None;
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            (if r0.start != usize::MAX && r0.end != usize::MAX {
                Some(&data[r0])
            } else {
                None
            },)
        })
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"a"));
    assert!(!re.is_match(b"d"));
    assert!(!re.is_match(b"ab"));
    assert!(!re.is_match(b"bc"));
    assert!(!re.is_match(b"cd"));
    assert!(!re.is_match(b"abc"));
    assert!(!re.is_match(b"acd"));
    assert_eq!(
        "abc",
        escape_ascii(re.match_all(b"abcd").unwrap().0.unwrap())
    );
    assert!(!re.is_match(b"abcda"));
    assert!(!re.is_match(b"abcdabcd"));
    assert!(!re.is_match(b"Xabcd"));
    assert!(!re.is_match(b"aXbcd"));
    assert!(!re.is_match(b"abXcd"));
    assert!(!re.is_match(b"abcXd"));
    assert!(!re.is_match(b"abcdX"));
}

#[test]
fn alt_in_group() {
    // regex!(br"(a|b)")
    let re: Matcher1<_> = safe_regex::Matcher1::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        let mut accept: Option<(core::ops::Range<usize>,)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            accept = None.or_else(|| b0.clone()).or_else(|| b1.clone()).clone();
            if let Some(b) = data_iter.next() {
                b1 = start
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 98u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                b0 = start
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                start = None;
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            (if r0.start != usize::MAX && r0.end != usize::MAX {
                Some(&data[r0])
            } else {
                None
            },)
        })
    });
    assert!(!re.is_match(b""));
    assert!(!re.is_match(b"X"));
    assert_eq!("a", escape_ascii(re.match_all(b"a").unwrap().0.unwrap()));
    assert_eq!("b", escape_ascii(re.match_all(b"b").unwrap().0.unwrap()));
    assert!(!re.is_match(b"aX"));
    assert!(!re.is_match(b"Xa"));
    assert!(!re.is_match(b"bX"));
    assert!(!re.is_match(b"Xb"));
    assert!(!re.is_match(b"ab"));
    assert!(!re.is_match(b"aa"));
    assert!(!re.is_match(b"ba"));
    assert!(!re.is_match(b"bb"));
}
