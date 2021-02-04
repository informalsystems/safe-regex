use core::ops::Range;
use fixed_buffer::escape_ascii;
use safe_regex;

fn matcher(data: &[u8]) -> Option<(&[u8], Option<&[u8]>, Option<&[u8]>)> {
    let mut all_group: Option<Range<usize>> = Some(1..data.len());
    let mut group1: Option<Range<usize>> = Some(1..3);
    let mut group2: Option<Range<usize>> = None;

    all_group.map(|all_range| {
        (
            &data[all_range],
            group1.map(|r| &data[r]),
            group2.map(|r| &data[r]),
        )
    })
}

#[test]
fn matcher_fn() {
    let input = Vec::from(&b"abcdef"[..]);
    let (all, opt_group1, opt_group2) = matcher(&input).unwrap();
    assert_eq!(&b"bcdef"[..], all);
    let mut result = Vec::new();
    if let Some(group1) = opt_group1 {
        result.extend_from_slice(group1);
        result.push(b',');
    }
    if let Some(group2) = opt_group2 {
        result.extend_from_slice(group2);
    }
    assert_eq!("bc,", escape_ascii(&result));
}

#[test]
fn temporary_pattern() {
    let class1 = safe_regex::RangeBytes(b'a'..=b'z');
    let class2 = safe_regex::RangeBytes(b'0'..=b'9');
    let seq1 = safe_regex::Seq(&mut class1, &mut class2);
    if let Some(matching_part) = seq1.check(input) {}
}
