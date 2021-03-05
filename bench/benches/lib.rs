// $ cargo +nightly bench --package bench
//    Compiling safe-regex-compiler v0.1.1 (safe-regex-rs/safe-regex-compiler)
//    Compiling safe-regex-macro v0.1.1 (safe-regex-rs/safe-regex-macro)
//    Compiling safe-regex v0.1.1 (safe-regex-rs/safe-regex)
//    Compiling bench v0.0.0 (safe-regex-rs/bench)
//     Finished bench [optimized] target(s) in 10.22s
//      Running safe-regex-rs/target/release/deps/lib-9f5f0d3bacb64d1e
//
// running 40 tests
// test capture10_regex                  ... bench:       2,267 ns/iter (+/- 370)
// test capture10_safe_regex             ... bench:       1,777 ns/iter (+/- 696)
// test datetime_capture_100_regex       ... bench:         220 ns/iter (+/- 15)
// test datetime_capture_100_safe_regex  ... bench:       1,966 ns/iter (+/- 516)
// test datetime_capture_10kb_regex      ... bench:      11,765 ns/iter (+/- 2,426)
// test datetime_capture_10kb_safe_regex ... bench:     199,057 ns/iter (+/- 22,629)
// test datetime_capture_1kb_regex       ... bench:       1,004 ns/iter (+/- 236)
// test datetime_capture_1kb_safe_regex  ... bench:      19,776 ns/iter (+/- 4,234)
// test datetime_parse_regex             ... bench:         373 ns/iter (+/- 41)
// test datetime_parse_safe_regex        ... bench:         322 ns/iter (+/- 56)
// test pem_base64_regex                 ... bench:         119 ns/iter (+/- 23)
// test pem_base64_safe_regex            ... bench:       3,344 ns/iter (+/- 659)
// test phone_capture_100kb_regex        ... bench:     182,787 ns/iter (+/- 13,992)
// test phone_capture_100kb_safe_regex   ... bench:   1,198,306 ns/iter (+/- 151,207)
// test phone_capture_10kb_regex         ... bench:      18,319 ns/iter (+/- 3,423)
// test phone_capture_10kb_safe_regex    ... bench:     106,017 ns/iter (+/- 16,037)
// test phone_capture_1kb_regex          ... bench:       1,744 ns/iter (+/- 254)
// test phone_capture_1kb_safe_regex     ... bench:      10,180 ns/iter (+/- 2,121)
// test phone_capture_1mb_regex          ... bench:   1,904,455 ns/iter (+/- 166,638)
// test phone_capture_1mb_safe_regex     ... bench:  11,500,098 ns/iter (+/- 1,071,888)
// test repeat10_regex                   ... bench:          32 ns/iter (+/- 2)
// test repeat10_safe_regex              ... bench:         103 ns/iter (+/- 2)
// test repeat20_regex                   ... bench:         178 ns/iter (+/- 30)
// test repeat20_safe_regex              ... bench:         447 ns/iter (+/- 79)
// test repeat30_regex                   ... bench:          63 ns/iter (+/- 19)
// test repeat30_safe_regex              ... bench:       1,190 ns/iter (+/- 91)
// test repeat_capture10_regex           ... bench:         234 ns/iter (+/- 58)
// test repeat_capture10_safe_regex      ... bench:         138 ns/iter (+/- 9)
// test repeat_capture20_regex           ... bench:         312 ns/iter (+/- 30)
// test repeat_capture20_safe_regex      ... bench:         616 ns/iter (+/- 137)
// test repeat_capture30_regex           ... bench:         368 ns/iter (+/- 69)
// test repeat_capture30_safe_regex      ... bench:       1,555 ns/iter (+/- 241)
// test string_search_100_regex          ... bench:          22 ns/iter (+/- 1)
// test string_search_100_safe_regex     ... bench:         473 ns/iter (+/- 47)
// test string_search_100kb_regex        ... bench:       1,058 ns/iter (+/- 63)
// test string_search_100kb_safe_regex   ... bench:     512,418 ns/iter (+/- 187,392)
// test string_search_10kb_regex         ... bench:          82 ns/iter (+/- 1)
// test string_search_10kb_safe_regex    ... bench:      44,745 ns/iter (+/- 6,804)
// test string_search_1kb_regex          ... bench:          21 ns/iter (+/- 3)
// test string_search_1kb_safe_regex     ... bench:       4,486 ns/iter (+/- 571)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 61.07s
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
    let re = Regex::new(r"^a{10,20}$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaa"));
}

#[bench]
fn repeat10_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a{10,20}");
    b.iter(|| re.match_all(b"aaaaaaaaaa"));
}

#[bench]
fn repeat20_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a{20,40}$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
}

// Very very slow.  May never complete.
#[bench]
fn repeat20_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a{20,40}");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a{30,60}$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a{30,60}");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

//////////////

#[bench]
fn repeat_capture10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a{10,20})$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture10_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a{10,20})");
    b.iter(|| re.match_all(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a{20,40})$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a{20,40})");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a{30,60})$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture30_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a{30,60})");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

//////////////

#[bench]
fn capture10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaa"));
}

#[bench]
fn capture10_safe_regex(b: &mut Bencher) {
    let re: Matcher10<_> = regex!(br"(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)(a?)aaaaaaaaaa");
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

// TODO(mleonhard) Add tests for star: (a*)(a*)(a*), (a*)(b*)(c*), etc.

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
