#![forbid(unsafe_code)]
/// Wraps an iterator.
/// You can call its [`peek`](#method.peek) method multiple times.
/// Calling `next` resets the peeking.
/// It keeps peeked values in a `Vec`.
///
/// # Example
/// ```
/// let inner = ['a', 'b', 'c'];
/// use safe_regex::MultiPeekable;
/// let mut mp = MultiPeekable::new(
///     inner.iter().cloned());
/// assert_eq!(Some('a'), mp.peek());
/// assert_eq!(Some('b'), mp.peek());
/// assert_eq!(Some('c'), mp.peek());
/// assert_eq!(None, mp.peek());
///
/// // Consume a value and reset the peeking.
/// assert_eq!(Some('a'), mp.next());
/// assert_eq!(Some('b'), mp.peek());
/// assert_eq!(Some('c'), mp.peek());
/// assert_eq!(None, mp.peek());
///
/// assert_eq!(Some('b'), mp.next());
/// assert_eq!(Some('c'), mp.peek());
/// assert_eq!(None, mp.peek());
///
/// assert_eq!(Some('c'), mp.next());
/// assert_eq!(None, mp.peek());
///
/// assert_eq!(None, mp.next());
/// assert_eq!(None, mp.peek());
/// ```
pub struct MultiPeekable<T: Copy, I: Iterator<Item = T>> {
    inner: I,
    buffer: Vec<T>,
    // The index of the next item to return from `peek`.
    next_peek: usize,
}

impl<T, I> MultiPeekable<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    /// Makes a MultiPeekable wrapping `inner`.
    #[must_use]
    pub fn new(inner: I) -> MultiPeekable<T, I> {
        Self {
            inner,
            buffer: Vec::new(),
            next_peek: 0,
        }
    }

    /// Returns the next value from the inner iterator, saving a copy.
    /// You can call this multiple times and will receive subsequent values.
    /// Returns None when the inner iterator is exhausted.
    ///
    /// Calls to `peek` do not affect the results of subsequent calls to `Iterator::next`.
    ///
    /// Call `Iterator::next` to reset the iterator as if `peek` had never been called.
    pub fn peek(&mut self) -> Option<T> {
        if self.next_peek < self.buffer.len() {
            // Buffer has an un-peeked item.
            let item = self.buffer[self.next_peek];
            self.next_peek += 1;
            Some(item)
        } else {
            // All buffered items have been peeked.
            // Try to read from inner.
            assert_eq!(self.next_peek, self.buffer.len());
            let item = self.inner.next()?;
            self.buffer.push(item);
            self.next_peek += 1;
            Some(item)
        }
    }
}

impl<T, I> Iterator for MultiPeekable<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_peek = 0;
        if !self.buffer.is_empty() {
            Some(self.buffer.remove(0))
        } else {
            self.inner.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut mp = MultiPeekable::new(std::iter::empty::<bool>());
        assert_eq!(None, mp.peek());
        assert_eq!(None, mp.next());
    }

    #[test]
    fn test_peek() {
        let mut mp = MultiPeekable::new([1u8, 2u8, 3u8, 4u8].iter().cloned());
        assert_eq!(Some(1u8), mp.peek());
        assert_eq!(Some(2u8), mp.peek());
        assert_eq!(Some(3u8), mp.peek()); // buffered: [], peeked [1,2,3]

        assert_eq!(Some(1u8), mp.next()); // buffered: [2,3], peeked []
        assert_eq!(Some(2u8), mp.peek()); // buffered: [3], peeked [2]

        assert_eq!(Some(2u8), mp.next()); // buffered: [2,3] then [3], peeked []
        assert_eq!(Some(3u8), mp.peek()); // buffered: [], peeked [3]
    }

    #[test]
    fn test_next_restarts() {
        let mut mp = MultiPeekable::new([1u8, 2u8].iter().cloned());
        assert_eq!(Some(1u8), mp.peek());
        assert_eq!(Some(2u8), mp.peek());
        assert_eq!(Some(1u8), mp.next());
        assert_eq!(Some(2u8), mp.peek());
    }

    #[test]
    fn test_peek_all() {
        let mut mp = MultiPeekable::new([1u8, 2u8, 3u8].iter().cloned());
        assert_eq!(Some(1u8), mp.peek());
        assert_eq!(Some(2u8), mp.peek());
        assert_eq!(Some(3u8), mp.peek());
        assert_eq!(None, mp.peek());

        assert_eq!(Some(1u8), mp.next());
        assert_eq!(Some(2u8), mp.peek());
        assert_eq!(Some(3u8), mp.peek());
        assert_eq!(None, mp.peek());

        assert_eq!(Some(2u8), mp.next());
        assert_eq!(Some(3u8), mp.peek());
        assert_eq!(None, mp.peek());

        assert_eq!(Some(3u8), mp.next());
        assert_eq!(None, mp.peek());

        assert_eq!(None, mp.next());
        assert_eq!(None, mp.peek());
    }
}
