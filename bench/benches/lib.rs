// $ cargo +nightly bench --package bench
//     Finished bench [optimized] target(s) in 9.11s
//      Running target/release/deps/lib-9f5f0d3bacb64d1e
//
// running 30 tests
// test capture10_regex                  ... bench:       2,481 ns/iter (+/- 413)
// test capture10_safe_regex             ... bench:   1,948,217 ns/iter (+/- 243,264)
// test datetime_capture_100_regex       ... bench:         187 ns/iter (+/- 19)
// test datetime_capture_100_safe_regex  ... bench:      22,115 ns/iter (+/- 5,124)
// test datetime_capture_10kb_regex      ... bench:      12,294 ns/iter (+/- 2,193)
// test datetime_capture_10kb_safe_regex ... bench:   2,202,325 ns/iter (+/- 221,897)
// test datetime_capture_1kb_regex       ... bench:       1,104 ns/iter (+/- 159)
// test datetime_capture_1kb_safe_regex  ... bench:     231,559 ns/iter (+/- 19,834)
// test datetime_parse_regex             ... bench:         383 ns/iter (+/- 75)
// test datetime_parse_safe_regex        ... bench:       3,352 ns/iter (+/- 604)
// test pem_base64_regex                 ... bench:         121 ns/iter (+/- 10)
// test pem_base64_safe_regex            ... bench:   1,013,212 ns/iter (+/- 55,439)
// test phone_capture_100kb_regex        ... bench:     174,812 ns/iter (+/- 18,220)
// test phone_capture_100kb_safe_regex   ... bench:  16,739,427 ns/iter (+/- 1,197,910)
// test phone_capture_10kb_regex         ... bench:      17,594 ns/iter (+/- 2,913)
// test phone_capture_10kb_safe_regex    ... bench:   1,689,970 ns/iter (+/- 206,171)
// test phone_capture_1kb_regex          ... bench:       1,833 ns/iter (+/- 193)
// test phone_capture_1kb_safe_regex     ... bench:     172,116 ns/iter (+/- 21,268)
// test repeat10_regex                   ... bench:          68 ns/iter (+/- 6)
// test repeat10_safe_regex              ... bench:      10,080 ns/iter (+/- 2,099)
// test repeat_capture10_regex           ... bench:       1,469 ns/iter (+/- 625)
// test repeat_capture10_safe_regex      ... bench:      15,816 ns/iter (+/- 5,678)
// test string_search_100_regex          ... bench:          22 ns/iter (+/- 1)
// test string_search_100_safe_regex     ... bench:      11,603 ns/iter (+/- 1,812)
// test string_search_100kb_regex        ... bench:       1,075 ns/iter (+/- 105)
// test string_search_100kb_safe_regex   ... bench:  11,292,451 ns/iter (+/- 558,640)
// test string_search_10kb_regex         ... bench:          80 ns/iter (+/- 12)
// test string_search_10kb_safe_regex    ... bench:   1,148,750 ns/iter (+/- 122,307)
// test string_search_1kb_regex          ... bench:          22 ns/iter (+/- 2)
// test string_search_1kb_safe_regex     ... bench:     114,065 ns/iter (+/- 5,979)
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 30 measured; 0 filtered out; finished in 56.58s
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
