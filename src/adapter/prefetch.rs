use crate::Peek;

/// Provide [Peek] by using a similar strategy as [std::iter::Peekable]. Since self is not mutable
/// in [Peek::peek], this implementation eagerly fetches the value of next(). While this adapter
/// defeats the main purpose of this crate, it may be useful in edge cases where you want to compose
/// on [Peek] but have no other way of providing it.
///
/// # Examples
/// ```
/// use fastpeek::{Peek, PeekAdapters};
///
/// let mut i = [1,2,3].into_iter().prefetch_peekable();
/// assert_eq!(i.peek().cloned(), i.next());
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PrefetchPeekableIter<I: Iterator> {
    inner: I,
    peeked: Option<I::Item>,
}

impl<I: Iterator> PrefetchPeekableIter<I> {
    pub fn new(mut inner: I) -> Self {
        let peeked = inner.next();
        Self { inner, peeked }
    }
}

impl<I: Iterator> Iterator for PrefetchPeekableIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_none() {
            None
        } else {
            let mut result = self.inner.next();
            core::mem::swap(&mut self.peeked, &mut result);
            result
        }
    }
}

impl<'a, I> Peek<'a, I> for PrefetchPeekableIter<I>
where
    I: Iterator,
    I::Item: 'a,
{
    type PeekItem = &'a I::Item;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.peeked.as_ref()
    }
}

impl<I> Clone for PrefetchPeekableIter<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            peeked: self.peeked.clone(),
        }
    }
}
