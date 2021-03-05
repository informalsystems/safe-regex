// $ cargo +nightly bench --package bench
//     Finished bench [optimized] target(s) in 7.28s
//      Running safe-regex-rs/target/release/deps/lib-9f5f0d3bacb64d1e
//
// running 38 tests
// test capture10_regex                  ... bench:       2,228 ns/iter (+/- 357)
// test capture10_safe_regex             ... bench:       1,739 ns/iter (+/- 239)
// test datetime_capture_100_regex       ... bench:         191 ns/iter (+/- 29)
// test datetime_capture_100_safe_regex  ... bench:       1,961 ns/iter (+/- 159)
// test datetime_capture_10kb_regex      ... bench:      11,508 ns/iter (+/- 2,883)
// test datetime_capture_10kb_safe_regex ... bench:     209,243 ns/iter (+/- 14,722)
// test datetime_capture_1kb_regex       ... bench:       1,178 ns/iter (+/- 305)
// test datetime_capture_1kb_safe_regex  ... bench:      20,785 ns/iter (+/- 1,039)
// test datetime_parse_regex             ... bench:         374 ns/iter (+/- 31)
// test datetime_parse_safe_regex        ... bench:         328 ns/iter (+/- 42)
// test pem_base64_regex                 ... bench:         115 ns/iter (+/- 17)
// test pem_base64_safe_regex            ... bench:       3,542 ns/iter (+/- 846)
// test phone_capture_100kb_regex        ... bench:     182,595 ns/iter (+/- 15,503)
// test phone_capture_100kb_safe_regex   ... bench:   1,084,106 ns/iter (+/- 89,293)
// test phone_capture_10kb_regex         ... bench:      17,562 ns/iter (+/- 752)
// test phone_capture_10kb_safe_regex    ... bench:     100,845 ns/iter (+/- 3,527)
// test phone_capture_1kb_regex          ... bench:       1,899 ns/iter (+/- 252)
// test phone_capture_1kb_safe_regex     ... bench:      10,059 ns/iter (+/- 1,585)
// test phone_capture_1mb_regex          ... bench:   1,801,609 ns/iter (+/- 70,862)
// test phone_capture_1mb_safe_regex     ... bench:  11,776,301 ns/iter (+/- 387,629)
// test repeat10_regex                   ... bench:          68 ns/iter (+/- 3)
// test repeat10_safe_regex              ... bench:         120 ns/iter (+/- 11)
// test repeat20_regex                   ... bench:         257 ns/iter (+/- 60)
// test repeat20_safe_regex              ... bench:         517 ns/iter (+/- 42)
// test repeat30_regex                   ... bench:         139 ns/iter (+/- 29)
// test repeat30_safe_regex              ... bench:       1,157 ns/iter (+/- 74)
// test repeat_capture10_regex           ... bench:       1,344 ns/iter (+/- 221)
// test repeat_capture10_safe_regex      ... bench:         155 ns/iter (+/- 22)
// test repeat_capture20_regex           ... bench:       4,228 ns/iter (+/- 455)
// test repeat_capture20_safe_regex      ... bench:         620 ns/iter (+/- 97)
// test string_search_100_regex          ... bench:          22 ns/iter (+/- 2)
// test string_search_100_safe_regex     ... bench:         498 ns/iter (+/- 60)
// test string_search_100kb_regex        ... bench:       1,048 ns/iter (+/- 61)
// test string_search_100kb_safe_regex   ... bench:     527,838 ns/iter (+/- 102,330)
// test string_search_10kb_regex         ... bench:          82 ns/iter (+/- 14)
// test string_search_10kb_safe_regex    ... bench:      50,741 ns/iter (+/- 10,356)
// test string_search_1kb_regex          ... bench:          22 ns/iter (+/- 3)
// test string_search_1kb_safe_regex     ... bench:       5,171 ns/iter (+/- 377)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 38 measured; 0 filtered out; finished in 72.29s
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

#[bench]
fn repeat20_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
}

// Very very slow.  May never complete.
#[bench]
fn repeat20_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

//////////////

#[bench]
fn repeat_capture10_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture10_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa)");
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
    let re: Matcher1<_> =
        regex!(br"(a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa)");
    b.iter(|| re.match_all(b"aaaaaaaaaaaaaaaaaaaa"));
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
