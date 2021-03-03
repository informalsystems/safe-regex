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

// TODO(mleonhard) Test macro with comment.

#[test]
fn byte() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start.clone().filter(|_| *b == 97u8);
            start = None;
        }
        b0
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a" }).unwrap())
    );
}

#[test]
fn any_byte() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start.clone();
            start = None;
        }
        b0
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"." }).unwrap())
    );
}

#[test]
fn class_inclusive() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start
                .clone()
                .filter(|_| *b == 97u8 || *b == 98u8 || *b == 99u8 || (50u8..=52u8).contains(b));
            start = None;
        }
        b0
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[abc2-4]" }).unwrap())
    );
}

#[test]
fn class_exclusive() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start
                .clone()
                .filter(|_| *b != 97u8 && *b != 98u8 && *b != 99u8 && !(50u8..=52u8).contains(b));
            start = None;
        }
        b0
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"[^abc2-4]" }).unwrap())
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn seq() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut b1: Option<()> = None;
        let mut b2: Option<()> = None;
        for b in data.iter() {
            b2 = b1.clone().filter(|_| *b == 98u8);
            b1 = b0.clone().filter(|_| *b == 97u8);
            b0 = start.clone().filter(|_| *b == 97u8);
            start = None;
        }
        b2
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"aab" }).unwrap())
    );
}

#[test]
fn alt() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        let mut b1: Option<()> = None;
        for b in data.iter() {
            b1 = start.clone().filter(|_| *b == 98u8);
            b0 = start.clone().filter(|_| *b == 97u8);
            start = None;
        }
        None.or_else(|| b0.clone()).or_else(|| b1.clone())
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a|b" }).unwrap())
    );
}

#[test]
fn group() {
    let expected = quote! { |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((usize::MAX..usize::MAX,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b0 = start
                .clone()
                .filter(|_| *b == 97u8)
                .map(|(r0,)| (n..n + 1,));
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
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"(a)" }).unwrap())
    );
}

#[test]
fn optional() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start.clone().filter(|_| *b == 97u8);
            start = None;
        }
        start.clone().or_else(|| b0.clone())
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a?" }).unwrap())
    );
}

#[test]
fn star() {
    let expected = quote! { |data: &[u8]| {
        let mut start = Some(());
        let mut b0: Option<()> = None;
        for b in data.iter() {
            b0 = start
                .clone()
                .or_else(|| b0.clone())
                .clone()
                .filter(|_| *b == 97u8);
            start = None;
        }
        start.clone().or_else(|| b0.clone())
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"a*" }).unwrap())
    );
}

#[test]
fn empty_group_at_start() {
    let expected = quote! { |data: &[u8]| {
        assert!(data.len() < usize::MAX - 2);
        let mut start = Some((0..0,));
        let mut b0: Option<(core::ops::Range<usize>,)> = None;
        for (n, b) in data.iter().enumerate() {
            b0 = start.clone().filter(|_| *b == 97u8);
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
    } };
    assert_eq!(
        format!("{}", expected),
        format!("{}", impl_regex(quote! { br"()a" }).unwrap())
    );
}
