// $ cargo +nightly bench --package bench
//     Finished bench [optimized] target(s) in 9.11s
//      Running target/release/deps/lib-9f5f0d3bacb64d1e
//
// running 30 tests
// test capture10_regex                  ... bench:       2,324 ns/iter (+/- 356)
// test capture10_safe_regex             ... bench:   1,829,491 ns/iter (+/- 183,937)
// test datetime_capture_100_regex       ... bench:         217 ns/iter (+/- 20)
// test datetime_capture_100_safe_regex  ... bench:      23,064 ns/iter (+/- 3,036)
// test datetime_capture_10kb_regex      ... bench:      12,152 ns/iter (+/- 2,820)
// test datetime_capture_10kb_safe_regex ... bench:   2,212,062 ns/iter (+/- 201,341)
// test datetime_capture_1kb_regex       ... bench:       1,145 ns/iter (+/- 102)
// test datetime_capture_1kb_safe_regex  ... bench:     233,475 ns/iter (+/- 27,248)
// test datetime_parse_regex             ... bench:         401 ns/iter (+/- 82)
// test datetime_parse_safe_regex        ... bench:      11,441 ns/iter (+/- 15,730)
// test pem_base64_regex                 ... bench:         116 ns/iter (+/- 7)
// test pem_base64_safe_regex            ... bench:   1,022,333 ns/iter (+/- 79,905)
// test phone_capture_100kb_regex        ... bench:     185,383 ns/iter (+/- 12,965)
// test phone_capture_100kb_safe_regex   ... bench:  17,014,889 ns/iter (+/- 1,540,627)
// test phone_capture_10kb_regex         ... bench:      17,555 ns/iter (+/- 2,999)
// test phone_capture_10kb_safe_regex    ... bench:   1,712,844 ns/iter (+/- 299,877)
// test phone_capture_1kb_regex          ... bench:       2,058 ns/iter (+/- 446)
// test phone_capture_1kb_safe_regex     ... bench:     163,661 ns/iter (+/- 11,667)
// test repeat10_regex                   ... bench:          72 ns/iter (+/- 7)
// test repeat10_safe_regex              ... bench:       9,898 ns/iter (+/- 810)
// test repeat_capture10_regex           ... bench:       1,413 ns/iter (+/- 415)
// test repeat_capture10_safe_regex      ... bench:      15,337 ns/iter (+/- 1,822)
// test string_search_100_regex          ... bench:          22 ns/iter (+/- 3)
// test string_search_100_safe_regex     ... bench:      11,415 ns/iter (+/- 1,552)
// test string_search_100kb_regex        ... bench:       1,243 ns/iter (+/- 148)
// test string_search_100kb_safe_regex   ... bench:  12,103,077 ns/iter (+/- 993,985)
// test string_search_10kb_regex         ... bench:          84 ns/iter (+/- 8)
// test string_search_10kb_safe_regex    ... bench:   1,182,229 ns/iter (+/- 116,326)
// test string_search_1kb_regex          ... bench:          22 ns/iter (+/- 5)
// test string_search_1kb_safe_regex     ... bench:     116,343 ns/iter (+/- 12,469)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 30 measured; 0 filtered out; finished in 77.27s
#![allow(soft_unstable)]
#![feature(test)]
#![forbid(unsafe_code)]
extern crate test;
use regex::bytes::Regex;
use safe_regex::{regex, Matcher};
use test::Bencher;

fn rand_bytes_without_z(n: usize) -> Vec<u8> {
    core::iter::from_fn(|| Some(rand::random()))
        .filter(|b| *b != b'Z' && *b != b'z')
        .take(n)
        .collect()
}

#[bench]
fn string_search_100_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_100_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes_without_z(100);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn string_search_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn string_search_10kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_10kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn string_search_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_100kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*(2G8H81RFNZ).*");
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.match_all(&data));
}

//////////////

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

// #[bench]
// fn repeat20_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// // Very very slow.  May never complete.
// #[bench]
// fn repeat20_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat30_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat30_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa$").unwrap();
//     b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
// }

//////////////

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

// #[bench]
// fn repeat_capture20_regex(b: &mut Bencher) {
//     let re =
//         Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat_capture20_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br"(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
// }

//////////////

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

// #[bench]
// fn capture20_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaaaaaaaaaaaa$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn capture20_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br"(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
// }

//////////////

#[bench]
fn datetime_capture_100_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(100);
    b.iter(|| re.captures(&data));
}

#[bench]
fn datetime_capture_100_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
    let data: Vec<u8> = rand_bytes_without_z(100);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn datetime_capture_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn datetime_capture_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn datetime_capture_10kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn datetime_capture_10kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.match_all(&data));
}

//////////////

const DATE_TIME: &[u8] = b"1999-12-32 23:59";
#[bench]
fn datetime_parse_regex(b: &mut Bencher) {
    let re = Regex::new(r"^([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)$").unwrap();
    b.iter(|| re.captures(DATE_TIME));
}

#[bench]
fn datetime_parse_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)");
    b.iter(|| re.match_all(DATE_TIME));
}

//////////////

const PEM_BASE64_LINE: &[u8] = b"psGUNwWXrARgiInCeQkvN3toQrXOyQ5Df3MwrTAUIy0Nec7MrUEcdjrE0Mks3HhH";

#[bench]
fn pem_base64_regex(b: &mut Bencher) {
    let re = Regex::new(r"^[a-zA-Z0-9+/]{0,64}=*$").unwrap();
    b.iter(|| re.is_match(PEM_BASE64_LINE));
}

#[bench]
fn pem_base64_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br"[a-zA-Z0-9+/=]{0,64}=*");
    b.iter(|| re.match_all(PEM_BASE64_LINE));
}

//////////////

#[bench]
fn phone_capture_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn phone_capture_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn phone_capture_10kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn phone_capture_10kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn phone_capture_100kb_safe_regex(b: &mut Bencher) {
    let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn phone_capture_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.captures(&data));
}

// #[bench]
// fn phone_capture_1mb_safe_regex(b: &mut Bencher) {
//     let re: Matcher<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
//     let data: Vec<u8> = rand_bytes_without_z(1024 * 1024);
//     b.iter(|| re.match_all(&data));
// }
//
// #[bench]
// fn phone_capture_1mb_regex(b: &mut Bencher) {
//     let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
//     let data: Vec<u8> = rand_bytes_without_z(1024 * 1024);
//     b.iter(|| re.captures(&data));
// }
