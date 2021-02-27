// $ cargo +nightly bench
// test byte_regex                      ... bench:          25 ns/iter (+/- 1)
// test byte_safe_regex                 ... bench:         141 ns/iter (+/- 7)
// test capture10_regex                 ... bench:       2,290 ns/iter (+/- 554)
// test capture10_safe_regex            ... bench:   2,020,367 ns/iter (+/- 83,701)
// test datetime_capture_1kb_regex      ... bench:       1,085 ns/iter (+/- 129)
// test datetime_capture_1kb_safe_regex ... bench:     160,590 ns/iter (+/- 8,890)
// test datetime_capture_50_regex       ... bench:         197 ns/iter (+/- 25)
// test datetime_capture_50_safe_regex  ... bench:      12,111 ns/iter (+/- 1,887)
// test pem_base64_regex                ... bench:         119 ns/iter (+/- 12)
// test pem_base64_safe_regex           ... bench:      42,833 ns/iter (+/- 5,893)
// test phone_capture_100kb_regex       ... bench:     178,807 ns/iter (+/- 9,806)
// test phone_capture_100kb_safe_regex  ... bench:  18,234,422 ns/iter (+/- 756,328)
// test repeat10_regex                  ... bench:          74 ns/iter (+/- 4)
// test repeat10_safe_regex             ... bench:       7,153 ns/iter (+/- 1,261)
// test repeat30_regex                  ... bench:         141 ns/iter (+/- 32)
// test repeat_capture10_regex          ... bench:       1,408 ns/iter (+/- 199)
// test repeat_capture10_safe_regex     ... bench:      10,900 ns/iter (+/- 1,165)
// test repeat_capture20_regex          ... bench:       4,468 ns/iter (+/- 687)
// test repeat_capture20_safe_regex     ... bench:      37,770 ns/iter (+/- 5,292)
// test substring1_regex                ... bench:          10 ns/iter (+/- 1)
// test substring1_safe_regex           ... bench:         529 ns/iter (+/- 80)
// test substring50_regex               ... bench:          28 ns/iter (+/- 2)
// test substring50_safe_regex          ... bench:       6,497 ns/iter (+/- 340)
// test substring_100kb_regex           ... bench:       1,079 ns/iter (+/- 51)
// test substring_100kb_safe_regex      ... bench:  12,230,604 ns/iter (+/- 805,527)
// test substring_1kb_regex             ... bench:          21 ns/iter (+/- 3)
// test substring_1kb_safe_regex        ... bench:     122,942 ns/iter (+/- 10,317)
#![allow(soft_unstable)]
#![feature(test)]
#![forbid(unsafe_code)]
extern crate test;
use regex::bytes::Regex;
use safe_regex::{regex, Matcher};
use test::Bencher;

const FIFTY_BYTES: &[u8] = b"H9JTLVCN8Z5FLRGH1T8JDX4QKPNP5CKN4M6PZ139W9JJVTC8K9";
const PEM_BASE64_LINE: &[u8] = b"psGUNwWXrARgiInCeQkvN3toQrXOyQ5Df3MwrTAUIy0Nec7MrUEcdjrE0Mks3HhH";

fn rand_bytes(n: usize) -> Vec<u8> {
    core::iter::from_fn(|| Some(rand::random()))
        .filter(|b| *b != b'Z')
        .take(n)
        .collect()
}

#[bench]
fn byte_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a$").unwrap();
    b.iter(|| re.is_match(b"X"));
}

#[bench]
fn byte_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"a");
    b.iter(|| re.match_all(b"X"));
}

#[bench]
fn substring1_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    b.iter(|| re.find(b"X"));
}

#[bench]
fn substring1_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    b.iter(|| re.match_all(b"X"));
}

#[bench]
fn substring50_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    b.iter(|| re.find(FIFTY_BYTES));
}

#[bench]
fn substring50_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    b.iter(|| re.match_all(FIFTY_BYTES));
}

#[bench]
fn substring_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes(1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn substring_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes(1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn substring_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes(100 * 1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn substring_100kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes(100 * 1024);
    b.iter(|| re.match_all(&data));
}

// #[bench]
// fn substring_1mb_regex(b: &mut Bencher) {
//     let re = Regex::new(r"2G8H81RFNZ").unwrap();
//     let data: Vec<u8> = rand_bytes(1024 * 1024);
//     b.iter(|| re.find(&data));
// }
//
// #[bench]
// fn substring_1mb_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
//     let data: Vec<u8> = rand_bytes(1024 * 1024);
//     b.iter(|| re.match_all(&data));
// }

// #[bench]
// fn substring_1gb_regex(b: &mut Bencher) {
//     let re = Regex::new(r"2G8H81RFNZ").unwrap();
//     let data: Vec<u8> = rand_bytes(1024 * 1024 * 1024);
//     b.iter(|| re.find(&data));
// }
//
// #[bench]
// fn substring_1gb_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
//     let data: Vec<u8> = rand_bytes(1024 * 1024 * 1024);
//     b.iter(|| re.match_all(&data));
// }

#[bench]
fn pem_base64_regex(b: &mut Bencher) {
    let re = Regex::new(r"^[a-zA-Z0-9+/=]{4}{0,16}$").unwrap();
    b.iter(|| re.is_match(PEM_BASE64_LINE));
}

#[bench]
fn pem_base64_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"[a-zA-Z0-9+/=]{4}{0,16}");
    b.iter(|| re.match_all(PEM_BASE64_LINE));
}

#[bench]
fn repeat10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaa"));
}

#[bench]
fn repeat10_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa");
    b.iter(|| re.match_all(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture10_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)");
    b.iter(|| re.match_all(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_regex(b: &mut Bencher) {
    let re =
        Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
}

// Hangs rustc.
// #[bench]
// fn repeat30_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
// }

#[bench]
fn repeat30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn capture10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaa"));
}

#[bench]
fn capture10_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa");
    b.iter(|| re.match_all(b"aaaaaaaaaa"));
}

#[bench]
fn datetime_capture_50_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]+-[0-9]+-[0-9]+) ([0-9]+:[0-9]+).*");
    b.iter(|| re.match_all(FIFTY_BYTES));
}

#[bench]
fn datetime_capture_50_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]+-[0-9]+-[0-9]+) ([0-9]+:[0-9]+)").unwrap();
    b.iter(|| re.captures(FIFTY_BYTES));
}

#[bench]
fn datetime_capture_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]+-[0-9]+-[0-9]+) ([0-9]+:[0-9]+).*");
    let data: Vec<u8> = rand_bytes(1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn datetime_capture_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]+-[0-9]+-[0-9]+) ([0-9]+:[0-9]+)").unwrap();
    let data: Vec<u8> = rand_bytes(1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn phone_capture_100kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes(100 * 1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn phone_capture_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes(100 * 1024);
    b.iter(|| re.captures(&data));
}

// #[bench]
// fn phone_capture_1mb_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
//     let data: Vec<u8> = rand_bytes(1024 * 1024);
//     b.iter(|| re.match_all(&data));
// }
//
// #[bench]
// fn phone_capture_1mb_regex(b: &mut Bencher) {
//     let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
//     let data: Vec<u8> = rand_bytes(1024 * 1024);
//     b.iter(|| re.captures(&data));
// }
