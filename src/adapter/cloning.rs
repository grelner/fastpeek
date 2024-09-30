use crate::{Peek, PeekBack, PeekIter};

/// Provide [Peek], [PeekBack] and [PeekIter] by cloning an iterator and calling next() to peek a
/// value. This is useful for cheaply cloneable iterators, such as iterators that are backed by slices.
///
/// # Examples
/// ```
/// use fastpeek::{Peek, PeekAdapters, PeekBack};
///
/// let mut i = [1,2,3].into_iter().cloning_peekable();
/// assert_eq!(i.peek(), i.next());
/// assert_eq!(i.peek_back(), i.next_back());
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CloningPeekableIter<I> {
    inner: I,
}

impl<I> CloningPeekableIter<I> {
    pub fn new(iter: I) -> Self {
        Self { inner: iter }
    }
}
impl<I: Iterator> Iterator for CloningPeekableIter<I> {
    type Item = I::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline(always)]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }
}
impl<I: DoubleEndedIterator> DoubleEndedIterator for CloningPeekableIter<I> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    #[inline(always)]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.rfold(init, f)
    }
}
impl<I: ExactSizeIterator> ExactSizeIterator for CloningPeekableIter<I> {}

impl<I: Iterator + Clone> Peek<'_, I> for CloningPeekableIter<I> {
    type PeekItem = I::Item;

    fn peek(&'_ self) -> Option<Self::PeekItem> {
        self.inner.clone().next()
    }
}

impl<I: DoubleEndedIterator + Clone> PeekBack<'_, I> for CloningPeekableIter<I> {
    type PeekItem = I::Item;

    fn peek_back(&'_ self) -> Option<Self::PeekItem> {
        self.inner.clone().next_back()
    }
}

impl<I: Iterator + Clone> PeekIter<'_, I> for CloningPeekableIter<I> {
    type Iter = I;

    fn peek_iter(&'_ self) -> Self::Iter {
        self.inner.clone()
    }
}

impl<I: Clone> Clone for CloningPeekableIter<I> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
