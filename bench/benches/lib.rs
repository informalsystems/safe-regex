// $ cargo +nightly bench --package bench
//    Compiling bench v0.0.0 (safe-regex-rs/bench)
//     Finished bench [optimized] target(s) in 7.89s
//      Running safe-regex-rs/target/release/deps/lib-c7a79fd1a7d4629d
//
// running 40 tests
// test capture10_regex                  ... bench:       2,628 ns/iter (+/- 379)
// test capture10_safe_regex             ... bench:       2,095 ns/iter (+/- 830)
// test datetime_capture_100_regex       ... bench:         232 ns/iter (+/- 25)
// test datetime_capture_100_safe_regex  ... bench:       1,263 ns/iter (+/- 153)
// test datetime_capture_10kb_regex      ... bench:      13,067 ns/iter (+/- 1,412)
// test datetime_capture_10kb_safe_regex ... bench:     128,247 ns/iter (+/- 14,771)
// test datetime_capture_1kb_regex       ... bench:       1,170 ns/iter (+/- 164)
// test datetime_capture_1kb_safe_regex  ... bench:      12,471 ns/iter (+/- 4,343)
// test datetime_parse_regex             ... bench:         408 ns/iter (+/- 106)
// test datetime_parse_safe_regex        ... bench:         254 ns/iter (+/- 33)
// test pem_base64_regex                 ... bench:         126 ns/iter (+/- 13)
// test pem_base64_safe_regex            ... bench:       4,831 ns/iter (+/- 533)
// test phone_capture_100kb_regex        ... bench:     188,823 ns/iter (+/- 18,669)
// test phone_capture_100kb_safe_regex   ... bench:   1,150,018 ns/iter (+/- 112,655)
// test phone_capture_10kb_regex         ... bench:      19,216 ns/iter (+/- 1,958)
// test phone_capture_10kb_safe_regex    ... bench:     111,017 ns/iter (+/- 13,293)
// test phone_capture_1kb_regex          ... bench:       1,982 ns/iter (+/- 165)
// test phone_capture_1kb_safe_regex     ... bench:      11,761 ns/iter (+/- 1,245)
// test phone_capture_1mb_regex          ... bench:   1,970,457 ns/iter (+/- 214,947)
// test phone_capture_1mb_safe_regex     ... bench:  11,410,710 ns/iter (+/- 982,574)
// test repeat10_regex                   ... bench:          38 ns/iter (+/- 4)
// test repeat10_safe_regex              ... bench:         139 ns/iter (+/- 13)
// test repeat20_regex                   ... bench:         208 ns/iter (+/- 21)
// test repeat20_safe_regex              ... bench:         516 ns/iter (+/- 112)
// test repeat30_regex                   ... bench:          72 ns/iter (+/- 20)
// test repeat30_safe_regex              ... bench:       1,242 ns/iter (+/- 154)
// test repeat_capture10_regex           ... bench:         259 ns/iter (+/- 20)
// test repeat_capture10_safe_regex      ... bench:         169 ns/iter (+/- 18)
// test repeat_capture20_regex           ... bench:         320 ns/iter (+/- 33)
// test repeat_capture20_safe_regex      ... bench:         744 ns/iter (+/- 117)
// test repeat_capture30_regex           ... bench:         400 ns/iter (+/- 48)
// test repeat_capture30_safe_regex      ... bench:       1,882 ns/iter (+/- 378)
// test string_search_100_regex          ... bench:          30 ns/iter (+/- 4)
// test string_search_100_safe_regex     ... bench:         389 ns/iter (+/- 39)
// test string_search_100kb_regex        ... bench:       1,236 ns/iter (+/- 198)
// test string_search_100kb_safe_regex   ... bench:     397,882 ns/iter (+/- 39,821)
// test string_search_10kb_regex         ... bench:         116 ns/iter (+/- 11)
// test string_search_10kb_safe_regex    ... bench:      39,231 ns/iter (+/- 3,176)
// test string_search_1kb_regex          ... bench:          30 ns/iter (+/- 3)
// test string_search_1kb_safe_regex     ... bench:       4,036 ns/iter (+/- 360)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 40 measured; 0 filtered out; finished in 153.45s
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
