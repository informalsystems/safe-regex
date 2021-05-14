#![forbid(unsafe_code)]
use safe_proc_macro2::TokenStream;
use safe_quote::quote;
use safe_regex_compiler::impl_regex;

#[test]
fn syntax_errors() {
    #[allow(clippy::needless_pass_by_value)]
    fn to_s(s: TokenStream) -> String {
        format!("{}", s)
    }
    let err = Err("expected a raw byte string, like br\"abc\"".to_string());
    assert_eq!(err, impl_regex(quote! {"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {r"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {b"a"}).map(to_s));
    assert_eq!(err, impl_regex(quote! {'a}).map(to_s));
    assert_eq!(err, impl_regex(quote! {b'b'}).map(to_s));
    assert_eq!(err, impl_regex(quote! {1}).map(to_s));
    assert_eq!(err, impl_regex(quote! {(br"a")}).map(to_s));
    assert_eq!(err, impl_regex(quote! {br"a";}).map(to_s));
    assert_eq!(err, impl_regex(quote! {br"a" br"b"}).map(to_s));
}

#[test]
fn empty() {
    let expected = quote! {
        safe_regex::Matcher0::new(|data: &[u8]| {
            if data.is_empty() {
                Some(())
            } else {
                None
            }
        })
    };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"" }).unwrap())
    );
}

#[test]
fn byte() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b0.is_none() {
                    return None;
                }
            } else {
                return b0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a" }).unwrap())
    );
}

#[test]
fn any_byte() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone();
                start = None;
                if b0.is_none() {
                    return None;
                }
            } else {
                return b0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"." }).unwrap())
    );
}

#[test]
fn class_inclusive() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone().filter(|_| {
                    *b == 97u8 || *b == 98u8 || *b == 99u8 || (50u8..=52u8).contains(b)
                });
                start = None;
                if b0.is_none() {
                    return None;
                }
            } else {
                return b0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[abc2-4]" }).unwrap())
    );
}

#[test]
fn class_exclusive() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut data_iter = data.iter();
        loop {
            if let Some(b) = data_iter.next() {
                b0 = start.clone().filter(|_| {
                    *b != 97u8 && *b != 98u8 && *b != 99u8 && !(50u8..=52u8).contains(b)
                });
                start = None;
                if b0.is_none() {
                    return None;
                }
            } else {
                return b0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[^abc2-4]" }).unwrap())
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn seq() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b0.is_none() && b1.is_none() && b2.is_none() {
                    return None;
                }
            } else {
                return b2;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"aab" }).unwrap())
    );
}

#[test]
fn alt() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b0.is_none() && b1.is_none() {
                    return None;
                }
            } else {
                return None.or_else(|| b0.clone()).or_else(|| b1.clone());
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a|b" }).unwrap())
    );
}

#[test]
fn group() {
    let expected = quote! { safe_regex::Matcher1::new(|data: &[u8]| {
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
                if b0.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a)" }).unwrap())
    );
}

#[test]
fn groups_nested() {
    let expected = quote! { safe_regex::Matcher2::new(|data: &[u8]| {
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
                if b0.is_none() && b1.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0, r1)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
                if r1.start == usize::MAX || r1.end == usize::MAX || r1.is_empty() {
                    0..0usize
                } else {
                    r1
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a(b))" }).unwrap())
    );
}

#[test]
fn optional() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b1.is_none() {
                    return None;
                }
            } else {
                return opt0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a?" }).unwrap())
    );
}

#[test]
fn optional_at_start() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b1.is_none() && b2.is_none() {
                    return None;
                }
            } else {
                return b2;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a?a" }).unwrap())
    );
}

#[test]
fn optional_at_end() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b0.is_none() && b2.is_none() {
                    return None;
                }
            } else {
                return opt1;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"aa?" }).unwrap())
    );
}

#[test]
fn optionals_in_seq() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b1.is_none() && b3.is_none() && b5.is_none() {
                    return None;
                }
            } else {
                return opt4;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a?a?a?" }).unwrap())
    );
}

#[test]
fn optionals_in_groups() {
    let expected = quote! { safe_regex::Matcher2::new(|data: &[u8]| {
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
                if b1.is_none() && b3.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0, r1)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
                if r1.start == usize::MAX || r1.end == usize::MAX || r1.is_empty() {
                    0..0usize
                } else {
                    r1
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a?)(a?)" }).unwrap())
    );
}

#[test]
fn star() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b1.is_none() {
                    return None;
                }
            } else {
                return star0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a*" }).unwrap())
    );
}

#[test]
fn group_star1() {
    let expected = quote! { safe_regex::Matcher1::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b2: Option<(core::ops::Range<usize>,)> = None;
        let mut accept: Option<(core::ops::Range<usize>,)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            let star0 = start.clone().or_else(|| opt1.clone());
            let opt1 = star0
                .clone()
                .map(|(r0,)| (n..n,))
                .clone()
                .or_else(|| b2.clone());
            accept = star0.clone();
            if let Some(b) = data_iter.next() {
                b2 = star0
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                start = None;
                if b2.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a?)*" }).unwrap())
    );
}

#[test]
fn group_star2() {
    let expected = quote! { safe_regex::Matcher1::new(|data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b1: Option<(core::ops::Range<usize>,)> = None;
        let mut b3: Option<(core::ops::Range<usize>,)> = None;
        let mut accept: Option<(core::ops::Range<usize>,)> = None;
        let mut data_iter = data.iter();
        let mut n = 0;
        loop {
            let star0 = start.clone().or_else(|| opt2.clone());
            let opt2 = b1.clone().or_else(|| b3.clone());
            accept = star0.clone();
            if let Some(b) = data_iter.next() {
                b3 = b1
                    .clone()
                    .filter(|_| {
                        //
                        *b == 97u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                b1 = star0
                    .clone()
                    .map(|(r0,)| (n..n,))
                    .clone()
                    .filter(|_| {
                        //
                        *b == 98u8
                    })
                    .map(|(r0,)| (r0.start..n + 1,));
                start = None;
                if b1.is_none() && b3.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(ba?)*" }).unwrap())
    );
}

#[test]
fn seq_in_star() {
    let expected = quote! { safe_regex::Matcher0::new(|data: &[u8]| {
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
                if b1.is_none() && b2.is_none() && b3.is_none() {
                    return None;
                }
            } else {
                return star0;
            }
        }
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(?:abc)*" }).unwrap())
    );
}

#[test]
fn seq_in_group() {
    let expected = quote! { safe_regex::Matcher1::new(|data: &[u8]| {
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
                if b0.is_none() && b1.is_none() && b2.is_none() && b3.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(abc)d" }).unwrap())
    );
}

#[test]
fn alt_in_group() {
    let expected = quote! { safe_regex::Matcher1::new(|data: &[u8]| {
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
                if b0.is_none() && b1.is_none() {
                    return None;
                }
            } else {
                break;
            }
            n = n + 1;
        }
        accept.map(|(r0,)| {
            [
                if r0.start == usize::MAX || r0.end == usize::MAX || r0.is_empty() {
                    0..0usize
                } else {
                    r0
                },
            ]
        })
    }) };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a|b)" }).unwrap())
    );
}
