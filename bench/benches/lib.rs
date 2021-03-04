// $ cargo +nightly bench --package bench
//     Finished bench [optimized] target(s) in 9.11s
//      Running target/release/deps/lib-9f5f0d3bacb64d1e
// running 28 tests
// test datetime_capture_100_regex       ... bench:         192 ns/iter (+/- 26)
// test datetime_capture_100_safe_regex  ... bench:       2,322 ns/iter (+/- 476)
// test datetime_capture_10kb_regex      ... bench:      12,840 ns/iter (+/- 1,351)
// test datetime_capture_10kb_safe_regex ... bench:     243,715 ns/iter (+/- 21,231)
// test datetime_capture_1kb_regex       ... bench:       1,033 ns/iter (+/- 244)
// test datetime_capture_1kb_safe_regex  ... bench:      24,471 ns/iter (+/- 1,837)
// test datetime_parse_regex             ... bench:         394 ns/iter (+/- 88)
// test datetime_parse_safe_regex        ... bench:         426 ns/iter (+/- 37)
// test pem_base64_regex                 ... bench:         129 ns/iter (+/- 38)
// test pem_base64_safe_regex            ... bench:       3,668 ns/iter (+/- 876)
// test phone_capture_100kb_regex        ... bench:     172,749 ns/iter (+/- 18,956)
// test phone_capture_100kb_safe_regex   ... bench:   1,339,288 ns/iter (+/- 64,526)
// test phone_capture_10kb_regex         ... bench:      18,135 ns/iter (+/- 2,541)
// test phone_capture_10kb_safe_regex    ... bench:     132,114 ns/iter (+/- 14,371)
// test phone_capture_1kb_regex          ... bench:       1,814 ns/iter (+/- 221)
// test phone_capture_1kb_safe_regex     ... bench:      12,782 ns/iter (+/- 2,120)
// test phone_capture_1mb_regex          ... bench:   1,793,311 ns/iter (+/- 84,392)
// test phone_capture_1mb_safe_regex     ... bench:  13,305,021 ns/iter (+/- 389,683)
// test repeat10_regex                   ... bench:          69 ns/iter (+/- 15)
// test repeat10_safe_regex              ... bench:          99 ns/iter (+/- 15)
// test string_search_100_regex          ... bench:          22 ns/iter (+/- 5)
// test string_search_100_safe_regex     ... bench:         410 ns/iter (+/- 44)
// test string_search_100kb_regex        ... bench:       1,014 ns/iter (+/- 174)
// test string_search_100kb_safe_regex   ... bench:     419,041 ns/iter (+/- 46,730)
// test string_search_10kb_regex         ... bench:          82 ns/iter (+/- 7)
// test string_search_10kb_safe_regex    ... bench:      41,883 ns/iter (+/- 3,790)
// test string_search_1kb_regex          ... bench:          22 ns/iter (+/- 3)
// test string_search_1kb_safe_regex     ... bench:       4,395 ns/iter (+/- 206)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 28 measured; 0 filtered out; finished in 41.40s
#![allow(soft_unstable)]
#![feature(test)]
#![forbid(unsafe_code)]
extern crate test;
use regex::bytes::Regex;
use safe_regex::{regex, Matcher0, Matcher1, Matcher10, Matcher3, Matcher5};
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
    let re: Matcher1<_> = regex!(br".*(2G8H81RFNZ).*");
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
    let re: Matcher1<_> = regex!(br".*(2G8H81RFNZ).*");
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
    let re: Matcher1<_> = regex!(br".*(2G8H81RFNZ).*");
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
    let re: Matcher1<_> = regex!(br".*(2G8H81RFNZ).*");
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
    let re: Matcher0<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa");
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
//     let re: Matcher0<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat30_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa$").unwrap();
//     b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat30_safe_regex(b: &mut Bencher) {
//     let re: Matcher0<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
// }

//////////////

// #[bench]
// fn repeat_capture10_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat_capture10_safe_regex(b: &mut Bencher) {
//     let re: Matcher1<_> = regex!(br"(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)");
//     b.iter(|| re.match_all(b"aaaaaaaaaa"));
// }

// #[bench]
// fn repeat_capture20_regex(b: &mut Bencher) {
//     let re =
//         Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
// }
//
// #[bench]
// fn repeat_capture20_safe_regex(b: &mut Bencher) {
//     let re: Matcher1<_> =
//         regex!(br"(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)");
//     b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
// }

//////////////

// #[bench]
// fn capture10_regex(b: &mut Bencher) {
//     let re = Regex::new(r"^(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa$").unwrap();
//     b.iter(|| re.captures(b"aaaaaaaaaa"));
// }
//
// #[bench]
// fn capture10_safe_regex(b: &mut Bencher) {
//     let re: Matcher10<_> = regex!(br"(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa");
//     b.iter(|| re.match_all(b"aaaaaaaaaa"));
// }

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
    let re: Matcher5<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
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
    let re: Matcher5<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
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
    let re: Matcher5<_> = regex!(br".*([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+).*");
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
    let re: Matcher5<_> = regex!(br"([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)");
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
    let re: Matcher0<_> = regex!(br"[a-zA-Z0-9+/=]{0,64}=*");
    b.iter(|| re.match_all(PEM_BASE64_LINE));
}

//////////////

#[bench]
fn phone_capture_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher3<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
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
    let re: Matcher3<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
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
    let re: Matcher3<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.match_all(&data));
}

#[bench]
fn phone_capture_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn phone_capture_1mb_regex(b: &mut Bencher) {
    let re = Regex::new(r"([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4})").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024 * 1024);
    b.iter(|| re.captures(&data));
}

#[bench]
fn phone_capture_1mb_safe_regex(b: &mut Bencher) {
    let re: Matcher3<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(1024 * 1024);
    b.iter(|| re.match_all(&data));
}
