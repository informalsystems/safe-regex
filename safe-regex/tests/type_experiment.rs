use std::marker::PhantomData;
use std::ops::Range;

trait CounterTrait {
    fn add(&mut self, n: usize);
    fn end(&self) -> usize;
    fn range(&self) -> Range<usize>;
}

struct EmptyCounter;
impl CounterTrait for EmptyCounter {
    fn add(&mut self, _n: usize) {}
    fn end(&self) -> usize {
        0
    }
    fn range(&self) -> Range<usize> {
        0..0
    }
}

struct CaptureCounter<T> {
    start: usize,
    end: usize,
    outer: T,
}
impl<T> CaptureCounter<T> {
    pub fn new(start: usize, outer: T) -> Self {
        Self {
            start,
            end: start,
            outer,
        }
    }
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn into_outer(self) -> T {
        self.outer
    }
}
impl<T> CounterTrait for CaptureCounter<T> {
    fn add(&mut self, n: usize) {
        self.end += n;
    }
    fn end(&self) -> usize {
        self.end
    }
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

trait Visitor {
    type Counter;
    fn visit(&mut self, counter: Self::Counter) -> Self::Counter;
}

struct VisitOne<C: CounterTrait> {
    phantom: PhantomData<C>,
}
impl<C: CounterTrait> VisitOne<C> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<C: CounterTrait> Visitor for VisitOne<C> {
    type Counter = C;

    fn visit(&mut self, mut counter: C) -> C {
        counter.add(1);
        counter
    }
}

struct Seq<C, A: Visitor<Counter = C>, B: Visitor<Counter = C>> {
    a: A,
    b: B,
    phantom: PhantomData<C>,
}
impl<C, A: Visitor<Counter = C>, B: Visitor<Counter = C>> Seq<C, A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }
}
impl<C, A: Visitor<Counter = C>, B: Visitor<Counter = C>> Visitor for Seq<C, A, B> {
    type Counter = C;

    fn visit(&mut self, counter: Self::Counter) -> C {
        self.b.visit(self.a.visit(counter))
    }
}

struct Capture<'r, C: CounterTrait, T: Visitor<Counter = CaptureCounter<C>>> {
    output: &'r mut Option<Range<usize>>,
    inner: T,
    phantom: PhantomData<C>,
}
impl<'r, C: CounterTrait, T: Visitor<Counter = CaptureCounter<C>>> Capture<'r, C, T> {
    pub fn new(output: &'r mut Option<Range<usize>>, inner: T) -> Self {
        Self {
            output,
            inner,
            phantom: PhantomData,
        }
    }
}
impl<'r, C: CounterTrait, T: Visitor<Counter = CaptureCounter<C>>> Visitor for Capture<'r, C, T> {
    type Counter = C;

    fn visit(&mut self, outer_counter: Self::Counter) -> C {
        let counter = CaptureCounter::new(outer_counter.end(), outer_counter);
        let counter = self.inner.visit(counter);
        let range = counter.range();
        let mut outer_counter = counter.into_outer();
        outer_counter.add(range.end - range.start);
        *self.output = Some(range);
        outer_counter
    }
}

#[test]
fn visitor() {
    let mut group1: Option<Range<usize>> = None;
    let mut capture1 = Capture::new(&mut group1, VisitOne::new());
    let mut group0: Option<Range<usize>> = None;
    let mut capture0 = Capture::new(&mut group0, Seq::new(VisitOne::new(), capture1));
    capture0.visit(EmptyCounter {});
    assert_eq!(Some(0..2), group0);
    assert_eq!(Some(1..2), group1);
    // assert_eq!(1..2, capture1.range);
}
