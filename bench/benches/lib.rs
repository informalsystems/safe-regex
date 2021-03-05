// $ cargo +nightly bench --package bench
//    Compiling bench v0.0.0 (safe-regex-rs/bench)
//     Finished bench [optimized] target(s) in 7.89s
//      Running safe-regex-rs/target/release/deps/lib-c7a79fd1a7d4629d
//
// running 40 tests
// test capture10_regex                  ... bench:       2,223 ns/iter (+/- 462)
// test capture10_safe_regex             ... bench:       1,764 ns/iter (+/- 418)
// test datetime_capture_100_regex       ... bench:         190 ns/iter (+/- 22)
// test datetime_capture_100_safe_regex  ... bench:       1,966 ns/iter (+/- 149)
// test datetime_capture_10kb_regex      ... bench:      12,093 ns/iter (+/- 3,025)
// test datetime_capture_10kb_safe_regex ... bench:     203,558 ns/iter (+/- 32,367)
// test datetime_capture_1kb_regex       ... bench:       1,105 ns/iter (+/- 312)
// test datetime_capture_1kb_safe_regex  ... bench:      20,118 ns/iter (+/- 3,166)
// test datetime_parse_regex             ... bench:         376 ns/iter (+/- 101)
// test datetime_parse_safe_regex        ... bench:         325 ns/iter (+/- 42)
// test pem_base64_regex                 ... bench:         121 ns/iter (+/- 9)
// test pem_base64_safe_regex            ... bench:       3,480 ns/iter (+/- 544)
// test phone_capture_100kb_regex        ... bench:     173,566 ns/iter (+/- 22,632)
// test phone_capture_100kb_safe_regex   ... bench:   1,064,410 ns/iter (+/- 99,961)
// test phone_capture_10kb_regex         ... bench:      17,235 ns/iter (+/- 1,796)
// test phone_capture_10kb_safe_regex    ... bench:     112,972 ns/iter (+/- 20,852)
// test phone_capture_1kb_regex          ... bench:       1,772 ns/iter (+/- 276)
// test phone_capture_1kb_safe_regex     ... bench:       9,973 ns/iter (+/- 885)
// test phone_capture_1mb_regex          ... bench:   1,825,205 ns/iter (+/- 154,677)
// test phone_capture_1mb_safe_regex     ... bench:  10,350,923 ns/iter (+/- 777,875)
// test repeat10_regex                   ... bench:          33 ns/iter (+/- 2)
// test repeat10_safe_regex              ... bench:         111 ns/iter (+/- 34)
// test repeat20_regex                   ... bench:         179 ns/iter (+/- 24)
// test repeat20_safe_regex              ... bench:         463 ns/iter (+/- 146)
// test repeat30_regex                   ... bench:          65 ns/iter (+/- 4)
// test repeat30_safe_regex              ... bench:       1,177 ns/iter (+/- 104)
// test repeat_capture10_regex           ... bench:         232 ns/iter (+/- 54)
// test repeat_capture10_safe_regex      ... bench:         133 ns/iter (+/- 27)
// test repeat_capture20_regex           ... bench:         300 ns/iter (+/- 71)
// test repeat_capture20_safe_regex      ... bench:         594 ns/iter (+/- 74)
// test repeat_capture30_regex           ... bench:         351 ns/iter (+/- 68)
// test repeat_capture30_safe_regex      ... bench:       1,496 ns/iter (+/- 258)
// test string_search_100_regex          ... bench:          21 ns/iter (+/- 1)
// test string_search_100_safe_regex     ... bench:         451 ns/iter (+/- 37)
// test string_search_100kb_regex        ... bench:       1,077 ns/iter (+/- 128)
// test string_search_100kb_safe_regex   ... bench:     416,884 ns/iter (+/- 24,524)
// test string_search_10kb_regex         ... bench:          81 ns/iter (+/- 11)
// test string_search_10kb_safe_regex    ... bench:      41,808 ns/iter (+/- 4,265)
// test string_search_1kb_regex          ... bench:          21 ns/iter (+/- 1)
// test string_search_1kb_safe_regex     ... bench:       4,197 ns/iter (+/- 326)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 46.72s
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
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
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
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
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
    let re: Matcher0<_> = regex!(br".*2G8H81RFNZ.*");
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
    let re: Matcher0<_> = regex!(br"[a-zA-Z0-9+/]{0,64}=*");
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
