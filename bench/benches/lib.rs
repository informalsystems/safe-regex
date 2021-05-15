// $ cargo +nightly bench --package bench
//    Compiling bench v0.0.0 (safe-regex-rs/bench)
//     Finished bench [optimized] target(s) in 7.89s
//      Running safe-regex-rs/target/release/deps/lib-c7a79fd1a7d4629d
//
// running 40 tests
// test capture10_regex                  ... bench:       2,720 ns/iter (+/- 1,105)
// test capture10_safe_regex             ... bench:       1,518 ns/iter (+/- 465)
// test datetime_capture_100_regex       ... bench:         203 ns/iter (+/- 56)
// test datetime_capture_100_safe_regex  ... bench:       1,979 ns/iter (+/- 228)
// test datetime_capture_10kb_regex      ... bench:      12,562 ns/iter (+/- 1,496)
// test datetime_capture_10kb_safe_regex ... bench:     204,491 ns/iter (+/- 18,534)
// test datetime_capture_1kb_regex       ... bench:       1,147 ns/iter (+/- 127)
// test datetime_capture_1kb_safe_regex  ... bench:      20,047 ns/iter (+/- 2,411)
// test datetime_parse_regex             ... bench:         394 ns/iter (+/- 161)
// test datetime_parse_safe_regex        ... bench:         299 ns/iter (+/- 33)
// test pem_base64_regex                 ... bench:         129 ns/iter (+/- 15)
// test pem_base64_safe_regex            ... bench:       5,768 ns/iter (+/- 657)
// test phone_capture_100kb_regex        ... bench:     189,552 ns/iter (+/- 18,942)
// test phone_capture_100kb_safe_regex   ... bench:     995,685 ns/iter (+/- 145,972)
// test phone_capture_10kb_regex         ... bench:      19,032 ns/iter (+/- 2,239)
// test phone_capture_10kb_safe_regex    ... bench:     104,156 ns/iter (+/- 11,140)
// test phone_capture_1kb_regex          ... bench:       1,892 ns/iter (+/- 344)
// test phone_capture_1kb_safe_regex     ... bench:       9,927 ns/iter (+/- 1,115)
// test phone_capture_1mb_regex          ... bench:   1,945,744 ns/iter (+/- 225,035)
// test phone_capture_1mb_safe_regex     ... bench:  10,556,256 ns/iter (+/- 945,806)
// test repeat10_regex                   ... bench:          37 ns/iter (+/- 4)
// test repeat10_safe_regex              ... bench:         109 ns/iter (+/- 35)
// test repeat20_regex                   ... bench:         209 ns/iter (+/- 32)
// test repeat20_safe_regex              ... bench:         602 ns/iter (+/- 65)
// test repeat30_regex                   ... bench:          70 ns/iter (+/- 7)
// test repeat30_safe_regex              ... bench:       1,322 ns/iter (+/- 211)
// test repeat_capture10_regex           ... bench:         346 ns/iter (+/- 379)
// test repeat_capture10_safe_regex      ... bench:         242 ns/iter (+/- 37)
// test repeat_capture20_regex           ... bench:         329 ns/iter (+/- 39)
// test repeat_capture20_safe_regex      ... bench:       1,016 ns/iter (+/- 95)
// test repeat_capture30_regex           ... bench:         372 ns/iter (+/- 74)
// test repeat_capture30_safe_regex      ... bench:       2,372 ns/iter (+/- 224)
// test string_search_100_regex          ... bench:          30 ns/iter (+/- 3)
// test string_search_100_safe_regex     ... bench:         546 ns/iter (+/- 59)
// test string_search_100kb_regex        ... bench:       1,239 ns/iter (+/- 233)
// test string_search_100kb_safe_regex   ... bench:     544,677 ns/iter (+/- 66,326)
// test string_search_10kb_regex         ... bench:         114 ns/iter (+/- 13)
// test string_search_10kb_safe_regex    ... bench:      54,988 ns/iter (+/- 4,956)
// test string_search_1kb_regex          ... bench:          29 ns/iter (+/- 2)
// test string_search_1kb_safe_regex     ... bench:       5,484 ns/iter (+/- 485)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 146.77s
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
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
    let data: Vec<u8> = rand_bytes_without_z(100);
    b.iter(|| re.match_slices(&data));
}

#[bench]
fn string_search_1kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.match_slices(&data));
}

#[bench]
fn string_search_10kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_10kb_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
    let data: Vec<u8> = rand_bytes_without_z(10 * 1024);
    b.iter(|| re.match_slices(&data));
}

#[bench]
fn string_search_100kb_regex(b: &mut Bencher) {
    let re = Regex::new(r"2G8H81RFNZ").unwrap();
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.find(&data));
}

#[bench]
fn string_search_100kb_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
    let data: Vec<u8> = rand_bytes_without_z(100 * 1024);
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(b"aaaaaaaaaa"));
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
    b.iter(|| re.match_slices(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^a{30,60}$").unwrap();
    b.iter(|| re.is_match(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat30_safe_regex(b: &mut Bencher) {
    let re: Matcher0<_> = regex!(br"a{30,60}");
    b.iter(|| re.match_slices(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
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
    b.iter(|| re.match_slices(b"aaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a{20,40})$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture20_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a{20,40})");
    b.iter(|| re.match_slices(b"aaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture30_regex(b: &mut Bencher) {
    let re = Regex::new(r"^(a{30,60})$").unwrap();
    b.iter(|| re.captures(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn repeat_capture30_safe_regex(b: &mut Bencher) {
    let re: Matcher1<_> = regex!(br"(a{30,60})");
    b.iter(|| re.match_slices(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
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
    b.iter(|| re.match_slices(b"aaaaaaaaaa"));
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
//     b.iter(|| re.match_slices(b"aaaaaaaaaaaaaaaaaaaa"));
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
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(DATE_TIME));
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
    let re: Matcher0<_> = regex!(br"[a-zA-Z0-9+/]{0,64}=*");
    b.iter(|| re.match_slices(PEM_BASE64_LINE));
}

//////////////

#[bench]
fn phone_capture_1kb_safe_regex(b: &mut Bencher) {
    let re: Matcher3<_> = regex!(br".*([0-9]{3})[-. ]?([0-9]{3})[-. ]?([0-9]{4}).*");
    let data: Vec<u8> = rand_bytes_without_z(1024);
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(&data));
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
    b.iter(|| re.match_slices(&data));
}
