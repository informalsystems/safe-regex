//! This is a failed attempt to add capturing to the NFA regex matcher.
//! The algorithm does not allow for `Optional` (`?` operator) to generate
//! multiple matching states.  See the commented-out tests below.

#![forbid(unsafe_code)]
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Range;

/// Converts the bytes into an ASCII string.
pub fn escape_ascii(input: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for byte in input.as_ref() {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

pub trait RangeTrait {
    fn is_discarding_range(&self) -> bool;
    fn extend(self, r: &Range<usize>) -> Option<Self>
    where
        Self: Sized;
    fn end(&self) -> usize;
    fn range(&self) -> Range<usize>;
}

#[derive(Clone, PartialEq)]
pub struct DiscardingRange;
impl RangeTrait for DiscardingRange {
    fn is_discarding_range(&self) -> bool {
        true
    }

    fn extend(self, _r: &Range<usize>) -> Option<Self> {
        Some(self)
    }

    fn end(&self) -> usize {
        0
    }
    fn range(&self) -> Range<usize> {
        0..0
    }
}
impl Debug for DiscardingRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "DiscardingRange")
    }
}

#[derive(Clone)]
pub struct MatchRange<R> {
    start: usize,
    end: usize,
    outer: R,
}
impl<R> MatchRange<R> {
    #[must_use]
    pub fn into_outer(self) -> R {
        self.outer
    }
}
impl MatchRange<()> {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            start: 0,
            end: 0,
            outer: (),
        }
    }
}
impl<R: RangeTrait + Clone + Debug> MatchRange<R> {
    #[must_use]
    pub fn new(outer: R, n: usize) -> Self {
        if outer.is_discarding_range() {
            Self {
                start: n,
                end: n,
                outer,
            }
        } else {
            let end = outer.range().end;
            if end != n {
                panic!("{} is not immediately after {:?}", n, outer);
            }
            Self {
                start: end,
                end,
                outer,
            }
        }
    }
}
impl<R: Clone + Debug> RangeTrait for MatchRange<R> {
    fn is_discarding_range(&self) -> bool {
        false
    }

    fn extend(mut self, r: &Range<usize>) -> Option<Self> {
        println!("extend {:?} + {:?}", &self, &r);
        if self.end != r.start {
            println!("{:?} is not immediately after {:?}", &r, &self);
            return None;
        }
        if r.end < r.start {
            panic!("bad range: {:?}", &r);
        }
        self.end = r.end;
        Some(self)
    }

    fn end(&self) -> usize {
        self.end
    }
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}
impl<R: Clone + Debug> Debug for MatchRange<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "MatchRange({}..{},{:?})",
            self.start, self.end, self.outer
        )
    }
}

pub trait Matcher {
    type RangeType;
    fn reset(&mut self);
    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType>;
    // This method takes `&mut self` so we can eliminate the `AsRef<[T]>` type
    // param on `Repeat`.
    fn matches_empty(&mut self) -> bool;
}

pub fn match_all<T: Matcher<RangeType = MatchRange<()>> + Debug>(
    matcher: &mut T,
    data: &[u8],
) -> bool {
    if data.is_empty() {
        return matcher.matches_empty();
    }
    matcher.reset();
    println!("{:?}", &matcher);
    println!("process_byte {}", escape_ascii([data[0]]));
    let mut final_state = matcher.process_byte(Some(MatchRange::zero()), data[0], 0);
    println!("{:?}", &matcher);
    println!("final_state = {:?}", final_state);
    for (n, b) in data.iter().copied().enumerate().skip(1) {
        println!("process_byte {}", escape_ascii([b]));
        final_state = matcher.process_byte(None, b, n);
        println!("{:?}", &matcher);
        println!("final_state = {:?}", final_state);
    }
    if let Some(matching_range) = final_state {
        matching_range.range() == (0..data.len())
    } else {
        false
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Byte<R: RangeTrait + Debug> {
    value: u8,
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> Byte<R> {
    #[must_use]
    pub fn new(b: u8) -> Self {
        Self {
            value: b,
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for Byte<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        if b == self.value {
            println!(
                "{:?} extend {:?} + {}..{}",
                self,
                prev_matching_range,
                n,
                n + 1
            );
            #[allow(clippy::range_plus_one)]
            prev_matching_range.extend(&(n..n + 1))
        } else {
            None
        }
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for Byte<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Byte({})", escape_ascii([self.value]))
    }
}

pub struct AnyByte<R: RangeTrait + Debug> {
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> AnyByte<R> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for AnyByte<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        _b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        println!(
            "{:?} extend {:?} + {}..{}",
            self,
            prev_matching_range,
            n,
            n + 1
        );
        #[allow(clippy::range_plus_one)]
        prev_matching_range.extend(&(n..n + 1))
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for AnyByte<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "AnyByte")
    }
}

pub struct Class<R: RangeTrait + Debug> {
    incl: bool,
    bytes: &'static [u8],
    phantom: PhantomData<R>,
}
impl<R: RangeTrait + Debug> Class<R> {
    #[must_use]
    pub fn new(incl: bool, bytes: &'static [u8]) -> Self {
        Self {
            incl,
            bytes,
            phantom: PhantomData,
        }
    }
}
impl<R: RangeTrait + Debug> Matcher for Class<R> {
    type RangeType = R;

    fn reset(&mut self) {}

    fn process_byte(
        &mut self,
        prev_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        let prev_matching_range = prev_state?;
        let contains = self.bytes.contains(&b);
        if (self.incl && contains) || (!self.incl && !contains) {
            println!(
                "{:?} extend {:?} + {}..{}",
                self,
                prev_matching_range,
                n,
                n + 1
            );
            #[allow(clippy::range_plus_one)]
            prev_matching_range.extend(&(n..n + 1))
        } else {
            None
        }
    }

    fn matches_empty(&mut self) -> bool {
        false
    }
}
impl<R: RangeTrait + Debug> Debug for Class<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Class{}({})",
            if self.incl { "" } else { "^" },
            escape_ascii(self.bytes)
        )
    }
}

pub struct Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    a: A,
    b: B,
    prev_a_state: Option<R>,
}
impl<R, A, B> Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            prev_a_state: None,
        }
    }

    fn reset_impl(&mut self) {
        self.a.reset();
        self.b.reset();
        self.prev_a_state = None;
    }

    fn process_byte_impl(&mut self, prev_state: Option<R>, b: u8, n: usize) -> Option<R> {
        let prev_a_state_clone = self.prev_a_state.clone();
        let result = self.b.process_byte(self.prev_a_state.take(), b, n);
        println!(
            "Seq.process_byte({},{}) {:?} --{:?}-> {:?}",
            escape_ascii([b]),
            n,
            prev_a_state_clone,
            self.b,
            result
        );

        let prev_state_clone = prev_state.clone();
        self.prev_a_state = self.a.process_byte(prev_state, b, n);
        println!(
            "Seq.process_byte({},{}) {:?} --{:?}-> {:?}",
            escape_ascii([b]),
            n,
            prev_state_clone,
            self.a,
            self.prev_a_state
        );
        result
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.a.matches_empty() && self.b.matches_empty()
    }
}
impl<R, A, B> Matcher for Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Matcher for &mut Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Debug for Seq<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Seq({:?},{:?},{:?})", self.a, self.prev_a_state, self.b)
    }
}

pub struct Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    a: A,
    b: B,
    phantom: PhantomData<R>,
}
impl<R, A, B> Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }

    fn reset_impl(&mut self) {
        self.a.reset();
        self.b.reset();
    }

    fn process_byte_impl(&mut self, prev_state: Option<R>, b: u8, n: usize) -> Option<R> {
        let prev_state_clone = prev_state.clone();
        if let Some(match_range) = self.a.process_byte(prev_state, b, n) {
            return Some(match_range);
        }
        self.b.process_byte(prev_state_clone, b, n)
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.a.matches_empty() || self.b.matches_empty()
    }
}
impl<R, A, B> Matcher for Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Matcher for &mut Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, A, B> Debug for Either<R, A, B>
where
    R: RangeTrait + Clone + Debug,
    A: Matcher<RangeType = R> + Debug,
    B: Matcher<RangeType = R> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Either({:?},{:?})", self.a, self.b)
    }
}

pub struct Optional<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = R> + Debug,
{
    inner: T,
    phantom: PhantomData<R>,
}
impl<R, T> Optional<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = R> + Debug,
{
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    fn reset_impl(&mut self) {
        self.inner.reset();
    }

    fn process_byte_impl(&mut self, prev_state: Option<R>, b: u8, n: usize) -> Option<R> {
        let prev_state_clone = prev_state.clone();
        if let Some(match_range) = self.inner.process_byte(prev_state, b, n) {
            Some(match_range)
        } else {
            prev_state_clone
        }
    }

    fn matches_empty_impl(&mut self) -> bool {
        true
    }
}
impl<R, T> Matcher for Optional<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Matcher for &mut Optional<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = R> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Debug for Optional<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = R> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Optional({:?})", self.inner)
    }
}

#[derive(Clone)]
pub struct CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    range: Option<Range<usize>>,
    inner: T,
    phantom: PhantomData<R>,
}
impl<R, T> CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            range: None,
            inner,
            phantom: PhantomData,
        }
    }

    pub fn range(&self) -> Option<Range<usize>> {
        self.range.clone()
    }

    fn reset_impl(&mut self) {
        self.inner.reset();
        self.range = None;
    }

    fn process_byte_impl(
        &mut self,
        prev_outer_state: Option<<CapturingGroup<R, T> as Matcher>::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<<CapturingGroup<R, T> as Matcher>::RangeType> {
        let prev_state = prev_outer_state.map(|p| MatchRange::new(p, n));
        let state = self.inner.process_byte(prev_state, b, n);
        let match_range = state?;
        let range = match_range.range();
        let outer_range = match_range.into_outer().extend(&range);
        self.range = Some(range);
        outer_range
    }

    fn matches_empty_impl(&mut self) -> bool {
        self.inner.matches_empty()
    }
}
impl<R, T> Matcher for CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Matcher for &mut CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    type RangeType = R;

    fn reset(&mut self) {
        self.reset_impl()
    }

    fn process_byte(
        &mut self,
        prev_outer_state: Option<Self::RangeType>,
        b: u8,
        n: usize,
    ) -> Option<Self::RangeType> {
        self.process_byte_impl(prev_outer_state, b, n)
    }

    fn matches_empty(&mut self) -> bool {
        self.matches_empty_impl()
    }
}
impl<R, T> Debug for CapturingGroup<R, T>
where
    R: RangeTrait + Clone + Debug,
    T: Matcher<RangeType = MatchRange<R>> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "CapturingGroup({:?})", self.inner)
    }
}

#[test]
fn byte() {
    let mut re = Byte::new(b'a');
    assert_eq!("Byte(a)", format!("{:?}", re));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"aa"));
}

#[test]
fn seq() {
    let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"a"));
    assert!(match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"aab"));
    assert!(!match_all(&mut re, b"aba"));
    assert!(!match_all(&mut re, b"abab"));
}

#[test]
fn seq_reset() {
    let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"X"));
}

#[test]
fn seq_nested() {
    let mut re = Seq::new(Byte::new(b'a'), Seq::new(Byte::new(b'b'), Byte::new(b'c')));
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"bc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(match_all(&mut re, b"abc"));
    assert!(!match_all(&mut re, b"Xabc"));
    assert!(!match_all(&mut re, b"abcX"));
    assert!(!match_all(&mut re, b"aabc"));
    assert!(!match_all(&mut re, b"abcc"));
    assert!(!match_all(&mut re, b"abca"));
    assert!(!match_all(&mut re, b"abcabc"));
}

#[test]
fn seq_deeply_nested() {
    let mut re = Seq::new(
        Byte::new(b'a'),
        Seq::new(Seq::new(Byte::new(b'b'), Byte::new(b'c')), Byte::new(b'd')),
    );
    println!("size {} bytes: {:?}", core::mem::size_of_val(&re), &re);
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    assert!(!match_all(&mut re, b"d"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"bc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(!match_all(&mut re, b"abc"));
    assert!(!match_all(&mut re, b"bcd"));
    assert!(match_all(&mut re, b"abcd"));
    assert!(!match_all(&mut re, b"Xabcd"));
    assert!(!match_all(&mut re, b"abcdX"));
    assert!(!match_all(&mut re, b"aabcd"));
    assert!(!match_all(&mut re, b"abcda"));
    assert!(!match_all(&mut re, b"abcdabcd"));
}

#[test]
fn seq_debug() {
    let mut re = Seq::new(Byte::new(b'a'), Byte::new(b'b'));
    assert_eq!("Seq(Byte(a),None,Byte(b))", format!("{:?}", re));
    assert!(!match_all(&mut re, b"a"));
    assert_eq!(
        "Seq(Byte(a),Some(MatchRange(0..1,())),Byte(b))",
        format!("{:?}", re)
    );
}

#[test]
fn optional() {
    let mut re = Optional::new(Byte::new(b'a'));
    assert_eq!("Optional(Byte(a))", format!("{:?}", re));
    assert!(match_all(&mut re, b""));
    assert!(match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"Xa"));
}

#[test]
fn optional_at_start() {
    let mut re = Seq::new(Optional::new(Byte::new(b'a')), Byte::new(b'a'));
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"Xa"));
    // assert!(match_all(&mut re, b"a"));
    // assert!(match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"aaa"));
    assert!(!match_all(&mut re, b"Xaa"));
    assert!(!match_all(&mut re, b"aaX"));
}

#[test]
fn optional_at_end() {
    let mut re = Seq::new(Byte::new(b'a'), Optional::new(Byte::new(b'a')));
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"Xa"));
    // assert!(match_all(&mut re, b"a"));
    // assert!(match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"aaa"));
    assert!(!match_all(&mut re, b"Xaa"));
    assert!(!match_all(&mut re, b"aaX"));
}

#[test]
fn optional_in_middle() {
    let mut re = Seq::new(
        Byte::new(b'a'),
        Seq::new(Optional::new(Byte::new(b'a')), Byte::new(b'a')),
    );
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"Xaa"));
    assert!(!match_all(&mut re, b"aaX"));
    assert!(!match_all(&mut re, b"aXa"));
    // assert!(match_all(&mut re, b"aa"));
    // assert!(match_all(&mut re, b"aaa"));
    assert!(!match_all(&mut re, b"aaaa"));
    assert!(!match_all(&mut re, b"aaaaa"));
    assert!(!match_all(&mut re, b"aaaaaa"));
    assert!(!match_all(&mut re, b"Xaaa"));
    assert!(!match_all(&mut re, b"aaaX"));
    assert!(!match_all(&mut re, b"XaaaX"));
}

#[test]
fn optional_in_group() {
    let matcher = |data| {
        let mut group = CapturingGroup::new(Optional::new(Byte::new(b'a')));
        if match_all(
            &mut Seq::new(Byte::new(b'a'), Seq::new(&mut group, Byte::new(b'a'))),
            data,
        ) {
            Some(group.range())
        } else {
            None
        }
    };
    assert_eq!(None, matcher(b""));
    assert_eq!(None, matcher(b"X"));
    assert_eq!(None, matcher(b"a"));
    assert_eq!(None, matcher(b"aX"));
    assert_eq!(None, matcher(b"Xa"));
    // assert_eq!(Some(None), matcher(b"aa"));
    assert_eq!(None, matcher(b"Xaa"));
    assert_eq!(None, matcher(b"aXa"));
    assert_eq!(None, matcher(b"aaX"));
    assert_eq!(None, matcher(b"XaaX"));
    assert_eq!(None, matcher(b"Xaaa"));
    assert_eq!(None, matcher(b"aXaa"));
    assert_eq!(None, matcher(b"aaXa"));
    assert_eq!(None, matcher(b"aaaX"));
    assert_eq!(None, matcher(b"XaaaX"));
    // assert_eq!(Some(Some(1..2)), matcher(b"aaa"));
    assert_eq!(None, matcher(b"aaaa"));
    assert_eq!(None, matcher(b"aaaaa"));
    assert_eq!(None, matcher(b"aaaaaa"));
}

#[test]
fn any_byte() {
    let mut any_byte = AnyByte::new();
    assert!(!match_all(&mut any_byte, b""));
    assert!(match_all(&mut any_byte, b"X"));
    assert!(!match_all(&mut any_byte, b"XY"));
    // Debug
    assert_eq!("AnyByte", format!("{:?}", any_byte));
    // AnyByte should match only one byte.
    let mut group = CapturingGroup::new(AnyByte::new());
    assert!(match_all(&mut Seq::new(&mut group, AnyByte::new()), b"XY"));
    assert_eq!(Some(0..1), group.range());
}

#[test]
fn class_inclusive() {
    let mut re = Class::new(true, b"abc");
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"abc"));
    assert!(match_all(&mut re, b"a"));
    assert!(match_all(&mut re, b"b"));
    assert!(match_all(&mut re, b"c"));
    // Debug
    assert_eq!("Class(abc)", format!("{:?}", re));
    // Class should match only one byte.
    let mut group = CapturingGroup::new(Class::new(true, b"abc"));
    assert!(match_all(&mut Seq::new(&mut group, AnyByte::new()), b"aa"));
    assert_eq!(Some(0..1), group.range());
}

#[test]
fn class_exclusive() {
    let mut re = Class::new(false, b"abc");
    assert!(!match_all(&mut re, b""));
    assert!(match_all(&mut re, b"X"));
    assert!(match_all(&mut re, b"Y"));
    assert!(!match_all(&mut re, b"XY"));
    assert!(!match_all(&mut re, b"a"));
    assert!(!match_all(&mut re, b"b"));
    assert!(!match_all(&mut re, b"c"));
    // Debug
    assert_eq!("Class^(abc)", format!("{:?}", re));
    // Class should match only one byte.
    let mut group = CapturingGroup::new(Class::new(false, b"abc"));
    assert!(match_all(&mut Seq::new(&mut group, AnyByte::new()), b"XX"));
    assert_eq!(Some(0..1), group.range());
}

#[test]
fn either() {
    let mut re = Either::new(Byte::new(b'a'), Byte::new(b'b'));
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"Xa"));
    assert!(!match_all(&mut re, b"Xb"));
    assert!(!match_all(&mut re, b"aX"));
    assert!(!match_all(&mut re, b"bX"));
    assert!(!match_all(&mut re, b"XaY"));
    assert!(!match_all(&mut re, b"XbY"));
    assert!(!match_all(&mut re, b"aa"));
    assert!(!match_all(&mut re, b"ab"));
    assert!(!match_all(&mut re, b"ba"));
    assert!(!match_all(&mut re, b"bb"));
    assert!(match_all(&mut re, b"a"));
    assert!(match_all(&mut re, b"b"));
}

#[test]
fn either_group() {
    let mut group = CapturingGroup::new(Either::new(Byte::new(b'a'), Byte::new(b'b')));
    assert!(!match_all(&mut group, b""));
    assert!(!match_all(&mut group, b"X"));
    assert!(!match_all(&mut group, b"Xa"));
    assert!(!match_all(&mut group, b"Xb"));
    assert!(!match_all(&mut group, b"aX"));
    assert!(!match_all(&mut group, b"bX"));
    assert!(!match_all(&mut group, b"XaY"));
    assert!(!match_all(&mut group, b"XbY"));
    assert!(!match_all(&mut group, b"aa"));
    assert!(!match_all(&mut group, b"ab"));
    assert!(!match_all(&mut group, b"ba"));
    assert!(!match_all(&mut group, b"bb"));
    assert!(match_all(&mut group, b"a"));
    assert_eq!(Some(0..1), group.range());
    assert!(match_all(&mut group, b"b"));
    assert_eq!(Some(0..1), group.range());
}

#[test]
fn either_seq() {
    let mut re = Seq::new(
        Either::new(Byte::new(b'a'), Byte::new(b'b')),
        Either::new(Byte::new(b'c'), Byte::new(b'd')),
    );
    assert!(!match_all(&mut re, b""));
    assert!(!match_all(&mut re, b"X"));
    assert!(!match_all(&mut re, b"Xac"));
    assert!(!match_all(&mut re, b"Xad"));
    assert!(!match_all(&mut re, b"Xbc"));
    assert!(!match_all(&mut re, b"Xbd"));
    assert!(!match_all(&mut re, b"acX"));
    assert!(!match_all(&mut re, b"adX"));
    assert!(!match_all(&mut re, b"bcX"));
    assert!(!match_all(&mut re, b"bdX"));
    assert!(!match_all(&mut re, b"XacY"));
    assert!(!match_all(&mut re, b"XadY"));
    assert!(!match_all(&mut re, b"XbcY"));
    assert!(!match_all(&mut re, b"XbdY"));
    assert!(!match_all(&mut re, b"aac"));
    assert!(!match_all(&mut re, b"add"));
    assert!(!match_all(&mut re, b"acac"));
    assert!(!match_all(&mut re, b"acbd"));
    assert!(match_all(&mut re, b"ac"));
    assert!(match_all(&mut re, b"ad"));
    assert!(!match_all(&mut re, b"ba"));
    assert!(!match_all(&mut re, b"bb"));
    assert!(match_all(&mut re, b"bc"));
    assert!(match_all(&mut re, b"bd"));
    assert!(!match_all(&mut re, b"ca"));
    assert!(!match_all(&mut re, b"cb"));
    assert!(!match_all(&mut re, b"cc"));
    assert!(!match_all(&mut re, b"cd"));
    assert!(!match_all(&mut re, b"da"));
    assert!(!match_all(&mut re, b"db"));
    assert!(!match_all(&mut re, b"dc"));
    assert!(!match_all(&mut re, b"dd"));
}

#[test]
fn either_debug() {
    let re: Either<DiscardingRange, _, _> = Either::new(Byte::new(b'a'), Byte::new(b'b'));
    assert_eq!("Either(Byte(a),Byte(b))", format!("{:?}", re));
}

#[test]
fn group() {
    let mut group = CapturingGroup::new(Byte::new(b'a'));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group),
        &group
    );
    assert!(!match_all(&mut group, b""));
    assert!(match_all(&mut group, b"a"));
    assert_eq!(Some(0..1), group.range());
    assert!(!match_all(&mut group, b"Xa"));
    assert!(!match_all(&mut group, b"ab"));
    assert!(!match_all(&mut group, b"aa"));
}

#[test]
fn group_nested1() {
    // ((a))
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_outer = CapturingGroup::new(&mut group_a);
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_outer),
        &group_outer
    );
    assert!(!match_all(&mut group_outer, b""));
    assert!(!match_all(&mut group_outer, b"Xa"));
    assert!(!match_all(&mut group_outer, b"ab"));
    assert!(!match_all(&mut group_outer, b"aa"));
    assert!(match_all(&mut group_outer, b"a"));
    assert_eq!(Some(0..1), group_outer.range());
    assert_eq!(Some(0..1), group_a.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested2() {
    // (a(b))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested3() {
    // ((a)b)
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, Byte::new(b'b')));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(0..1), group_a.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested4() {
    // ((a)(b))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_ab = CapturingGroup::new(Seq::new(&mut group_a, &mut group_b));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_ab),
        group_ab
    );
    assert!(!match_all(&mut group_ab, b"Xab"));
    assert!(!match_all(&mut group_ab, b"abX"));
    assert!(!match_all(&mut group_ab, b"aab"));
    assert!(!match_all(&mut group_ab, b"aba"));
    assert!(!match_all(&mut group_ab, b"abb"));
    assert!(!match_all(&mut group_ab, b"abab"));
    assert!(!match_all(&mut group_ab, b""));
    assert!(!match_all(&mut group_ab, b"a"));
    assert!(match_all(&mut group_ab, b"ab"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(0..1), group_a.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested5() {
    // ((a(b)) (c))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut group_abc = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_c));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abc),
        group_abc
    );
    assert!(!match_all(&mut group_abc, b"Xabc"));
    assert!(!match_all(&mut group_abc, b"abcX"));
    assert!(!match_all(&mut group_abc, b"aabc"));
    assert!(!match_all(&mut group_abc, b"abca"));
    assert!(!match_all(&mut group_abc, b"abcc"));
    assert!(!match_all(&mut group_abc, b"abcabc"));
    assert!(!match_all(&mut group_abc, b""));
    assert!(!match_all(&mut group_abc, b"a"));
    assert!(!match_all(&mut group_abc, b"ab"));
    assert!(match_all(&mut group_abc, b"abc"));
    assert_eq!(Some(0..3), group_abc.range());
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested6() {
    // ((a) ((b)c))
    let mut group_a = CapturingGroup::new(Byte::new(b'a'));
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_bc = CapturingGroup::new(Seq::new(&mut group_b, Byte::new(b'c')));
    let mut group_abc = CapturingGroup::new(Seq::new(&mut group_a, &mut group_bc));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abc),
        group_abc
    );
    assert!(!match_all(&mut group_abc, b"Xabc"));
    assert!(!match_all(&mut group_abc, b"abcX"));
    assert!(!match_all(&mut group_abc, b"aabc"));
    assert!(!match_all(&mut group_abc, b"abca"));
    assert!(!match_all(&mut group_abc, b"abcc"));
    assert!(!match_all(&mut group_abc, b"abcabc"));
    assert!(!match_all(&mut group_abc, b""));
    assert!(!match_all(&mut group_abc, b"a"));
    assert!(!match_all(&mut group_abc, b"ab"));
    assert!(match_all(&mut group_abc, b"abc"));
    assert_eq!(Some(0..3), group_abc.range());
    assert_eq!(Some(1..3), group_bc.range());
    assert_eq!(Some(0..1), group_a.range());
    assert_eq!(Some(1..2), group_b.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested7() {
    // ((a(b)) ((c)d))
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
    let mut group_abcd = CapturingGroup::new(Seq::new(&mut group_ab, &mut group_cd));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&group_abcd),
        group_abcd
    );
    assert!(!match_all(&mut group_abcd, b"Xabcd"));
    assert!(!match_all(&mut group_abcd, b"abcdX"));
    assert!(!match_all(&mut group_abcd, b"aabcd"));
    assert!(!match_all(&mut group_abcd, b"abcda"));
    assert!(!match_all(&mut group_abcd, b"abcdd"));
    assert!(!match_all(&mut group_abcd, b"abcdabcd"));
    assert!(!match_all(&mut group_abcd, b""));
    assert!(!match_all(&mut group_abcd, b"a"));
    assert!(!match_all(&mut group_abcd, b"ab"));
    assert!(!match_all(&mut group_abcd, b"abc"));
    assert!(match_all(&mut group_abcd, b"abcd"));
    assert_eq!(Some(0..4), group_abcd.range());
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(2..4), group_cd.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested8() {
    // (a (b (c)))
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_bc = CapturingGroup::new(Seq::new(Byte::new(b'b'), &mut group_c));
    let mut seq_abc = Seq::new(Byte::new(b'a'), &mut group_bc);
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&seq_abc),
        seq_abc
    );
    assert!(!match_all(&mut seq_abc, b"Xabc"));
    assert!(!match_all(&mut seq_abc, b"abcX"));
    assert!(!match_all(&mut seq_abc, b"aabc"));
    assert!(!match_all(&mut seq_abc, b"abca"));
    assert!(!match_all(&mut seq_abc, b"abcc"));
    assert!(!match_all(&mut seq_abc, b"abcabc"));
    assert!(!match_all(&mut seq_abc, b""));
    assert!(!match_all(&mut seq_abc, b"a"));
    assert!(!match_all(&mut seq_abc, b"ab"));
    assert!(match_all(&mut seq_abc, b"abc"));
    assert_eq!(Some(1..3), group_bc.range());
    assert_eq!(Some(2..3), group_c.range());
}

#[test]
#[allow(clippy::similar_names)]
fn group_nested9() {
    // (a(b)) ((c)d) ((e)(f))
    let mut group_f = CapturingGroup::new(Byte::new(b'f'));
    let mut group_e = CapturingGroup::new(Byte::new(b'e'));
    let mut group_ef = CapturingGroup::new(Seq::new(&mut group_e, &mut group_f));
    let mut group_c = CapturingGroup::new(Byte::new(b'c'));
    let mut group_cd = CapturingGroup::new(Seq::new(&mut group_c, Byte::new(b'd')));
    let mut group_b = CapturingGroup::new(Byte::new(b'b'));
    let mut group_ab = CapturingGroup::new(Seq::new(Byte::new(b'a'), &mut group_b));
    let mut seq_abcdef = Seq::new(&mut group_ab, Seq::new(&mut group_cd, &mut group_ef));
    println!(
        "size {} bytes: {:?}",
        core::mem::size_of_val(&seq_abcdef),
        seq_abcdef
    );
    assert!(!match_all(&mut seq_abcdef, b""));
    assert!(!match_all(&mut seq_abcdef, b"a"));
    assert!(!match_all(&mut seq_abcdef, b"ab"));
    assert!(!match_all(&mut seq_abcdef, b"abc"));
    assert!(!match_all(&mut seq_abcdef, b"abd"));
    assert!(!match_all(&mut seq_abcdef, b"abe"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefa"));
    assert!(!match_all(&mut seq_abcdef, b"aabcdef"));
    assert!(!match_all(&mut seq_abcdef, b"Xabcdef"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefX"));
    assert!(!match_all(&mut seq_abcdef, b"abcdefabcdef"));
    assert!(match_all(&mut seq_abcdef, b"abcdef"));
    assert_eq!(Some(0..2), group_ab.range());
    assert_eq!(Some(2..4), group_cd.range());
    assert_eq!(Some(4..6), group_ef.range());
    assert_eq!(Some(1..2), group_b.range());
    assert_eq!(Some(2..3), group_c.range());
    assert_eq!(Some(4..5), group_e.range());
    assert_eq!(Some(5..6), group_f.range());
}

#[test]
fn group_debug() {
    let group: CapturingGroup<DiscardingRange, _> = CapturingGroup::new(Byte::new(b'a'));
    assert_eq!("CapturingGroup(Byte(a))", format!("{:?}", group));
}
