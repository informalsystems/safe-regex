//! This is an earlier version of the library.
//! It implements an NFA as described in <https://swtch.com/~rsc/regexp/regexp1.html>.
//! It does not support capturing groups.

#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::marker::PhantomData;

pub trait Matcher {
    fn reset(&mut self);
    fn process_byte(&mut self, prev_matched: bool, b: u8) -> bool;
    // This method takes `&mut self` so we can eliminate the `AsRef<[T]>` type
    // param on `Repeat`.
    fn matches_empty(&mut self) -> bool;
}

pub fn match_all<T: Matcher + Debug>(matcher: &mut T, data: &[u8]) -> bool {
    if data.is_empty() {
        return matcher.matches_empty();
    }
    matcher.reset();
    println!("{:?}", &matcher);
    println!("process_byte {}", data[0]);
    let mut result = matcher.process_byte(true, data[0]);
    println!("{:?}", &matcher);
    println!("result = {}", result);
    for b in &data[1..] {
        println!("process_byte {}", b);
        result = matcher.process_byte(false, *b);
        println!("{:?}", &matcher);
        println!("result = {}", result);
    }
    result
}

#[derive(Copy, Clone, Debug)]
pub struct Byte(u8);
impl Byte {
    #[must_use]
    pub fn new(b: u8) -> Self {
        Self(b)
    }
}
impl Matcher for Byte {
    fn reset(&mut self) {}

    fn process_byte(&mut self, prev_matched: bool, b: u8) -> bool {
        prev_matched && b == self.0
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Seq<A: Matcher + Debug + Copy + Clone, B: Matcher + Debug + Copy + Clone> {
    a: A,
    b: B,
    a_matched: bool,
    matched: bool,
}
impl<A: Matcher + Debug + Copy + Clone, B: Matcher + Debug + Copy + Clone> Seq<A, B> {
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            a_matched: false,
            matched: false,
        }
    }
}
impl<A: Matcher + Debug + Copy + Clone, B: Matcher + Debug + Copy + Clone> Matcher for Seq<A, B> {
    fn reset(&mut self) {
        self.a.reset();
        self.b.reset();
    }

    fn process_byte(&mut self, prev_matched: bool, b: u8) -> bool {
        self.matched = self.b.process_byte(self.a_matched, b);
        self.a_matched = self.a.process_byte(prev_matched, b);
        self.matched
    }

    fn matches_empty(&mut self) -> bool {
        self.a.matches_empty() && self.b.matches_empty()
    }
}

pub fn double<T: Matcher + Debug + Copy + Clone>(inner: T) -> Seq<T, T> {
    Seq::new(inner, inner)
}

#[derive(Copy, Clone, Debug)]
pub struct Double<T: Matcher + Debug + Copy + Clone> {
    matchers: [T; 2],
    states: [bool; 2],
}
impl<T: Matcher + Debug + Copy + Clone> Double<T> {
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            matchers: [inner; 2],
            states: [false; 2],
        }
    }
}
impl<T: Matcher + Debug + Copy + Clone> Matcher for Double<T> {
    fn reset(&mut self) {
        for state in &mut self.states {
            *state = false;
        }
    }

    fn process_byte(&mut self, prev_matched: bool, b: u8) -> bool {
        self.states[1] = self.matchers[1].process_byte(self.states[0], b);
        self.states[0] = self.matchers[0].process_byte(prev_matched, b);
        self.states[1]
    }

    fn matches_empty(&mut self) -> bool {
        self.matchers.iter_mut().any(|m| m.matches_empty())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Repeat<T, M, S>
where
    T: Matcher + Debug + Copy + Clone,
    M: AsMut<[T]> + Debug + Copy + Clone,
    S: AsMut<[bool]> + Debug + Copy + Clone,
{
    matchers: M,
    states: S,
    min: usize,
    phantom: PhantomData<T>,
}
impl<T, M, S> Repeat<T, M, S>
where
    T: Matcher + Debug + Copy + Clone,
    M: AsMut<[T]> + Debug + Copy + Clone,
    S: AsMut<[bool]> + Debug + Copy + Clone,
{
    #[must_use]
    pub fn new(mut matchers: M, min: usize, mut states: S) -> Self {
        if matchers.as_mut().len() != states.as_mut().len() {
            panic!(
                "matchers and len have different lengths: {} and {}",
                matchers.as_mut().len(),
                states.as_mut().len()
            );
        }
        Self {
            matchers,
            min,
            states,
            phantom: PhantomData,
        }
    }
}
impl<T, M, S> Matcher for Repeat<T, M, S>
where
    T: Matcher + Debug + Copy + Clone,
    M: AsMut<[T]> + Debug + Copy + Clone,
    S: AsMut<[bool]> + Debug + Copy + Clone,
{
    fn reset(&mut self) {
        for matcher in self.matchers.as_mut() {
            matcher.reset();
        }
        for state in self.states.as_mut() {
            *state = false;
        }
    }

    fn process_byte(&mut self, prev_matched: bool, b: u8) -> bool {
        let matchers = self.matchers.as_mut();
        let states = self.states.as_mut();
        for n in (0..matchers.len()).rev() {
            let prev = if n == 0 { prev_matched } else { states[n - 1] };
            states[n] = matchers[n].process_byte(prev, b);
        }
        if self.min == 0 {
            prev_matched
        } else {
            (&states[(self.min - 1)..]).iter().any(|s| *s)
        }
    }

    fn matches_empty(&mut self) -> bool {
        if self.matchers.as_mut().is_empty() {
            return true;
        }
        self.matchers.as_mut().iter_mut().any(|m| m.matches_empty())
    }
}

#[test]
fn matcher_fn() {
    {
        let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
        println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
        assert!(!match_all(&mut re, b""));
        assert!(!match_all(&mut re, b"a"));
        assert!(match_all(&mut re, b"ab"));
        assert!(!match_all(&mut re, b"aab"));
        assert!(!match_all(&mut re, b"aba"));
        assert!(!match_all(&mut re, b"abab"));
    }
    {
        let mut re = Seq::new(Double::new(Byte::new(b'a')), Byte::new(b'a'));
        println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
        assert!(!match_all(&mut re, b""));
        assert!(!match_all(&mut re, b"a"));
        assert!(!match_all(&mut re, b"aa"));
        assert!(match_all(&mut re, b"aaa"));
        assert!(!match_all(&mut re, b"aaaa"));
        assert!(!match_all(&mut re, b"aaab"));
        assert!(!match_all(&mut re, b"baaa"));
    }
    {
        let mut re = double(Seq::new(Byte::new(b'a'), Byte::new(b'b')));
        println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
        assert!(!match_all(&mut re, b""));
        assert!(!match_all(&mut re, b"a"));
        assert!(!match_all(&mut re, b"ab"));
        assert!(!match_all(&mut re, b"aba"));
        assert!(match_all(&mut re, b"abab"));
        assert!(!match_all(&mut re, b"aabab"));
        assert!(!match_all(&mut re, b"ababX"));
        assert!(!match_all(&mut re, b"ababa"));
        assert!(!match_all(&mut re, b"abababab"));
    }
    {
        let mut re = Double::new(Seq::new(Byte::new(b'a'), Byte::new(b'b')));
        println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
        assert!(!match_all(&mut re, b""));
        assert!(!match_all(&mut re, b"a"));
        assert!(!match_all(&mut re, b"ab"));
        assert!(!match_all(&mut re, b"aba"));
        assert!(match_all(&mut re, b"abab"));
        assert!(!match_all(&mut re, b"aabab"));
        assert!(!match_all(&mut re, b"ababX"));
        assert!(!match_all(&mut re, b"ababa"));
        assert!(!match_all(&mut re, b"abababab"));
    }
    {
        let mut re = Repeat::new(
            [Seq::new(Byte::new(b'a'), Byte::new(b'b')); 2],
            2,
            [false; 2],
        );
        println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
        assert!(!match_all(&mut re, b""));
        assert!(!match_all(&mut re, b"a"));
        assert!(!match_all(&mut re, b"ab"));
        assert!(!match_all(&mut re, b"aba"));
        assert!(match_all(&mut re, b"abab"));
        assert!(!match_all(&mut re, b"aabab"));
        assert!(!match_all(&mut re, b"ababX"));
        assert!(!match_all(&mut re, b"ababa"));
        assert!(!match_all(&mut re, b"abababab"));
    }
}
