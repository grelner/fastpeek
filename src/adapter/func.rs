use crate::{Peek, PeekBack, PeekIter};
use core::marker::PhantomData;

/// Provide [Peek] via a function. Additionally, [PeekBack] and [PeekIter] may be implemented
/// by chaining [FnPeekableIter::new], [FnPeekableIter::with_peek_back] and/or
/// [FnPeekableIter::with_peek_iter]
///
/// # Examples
/// ```
/// use fastpeek::{Peek, PeekAdapters};
///
/// let mut i = [1,2,3].into_iter().fn_peekable(|i| i.as_slice().first());
/// assert_eq!(i.peek().cloned(), i.next());
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct FnPeekableIter<'a, I, Peek, PeekBack, PeekIter> {
    inner: I,
    peek: Peek,
    peek_back: PeekBack,
    peek_iter: PeekIter,
    _ref: PhantomData<&'a I>,
}

impl<'a, I, P> FnPeekableIter<'a, I, P, (), ()> {
    pub fn new(inner: I, peek_func: P) -> Self
    where
        I: Iterator,
        P: Fn(&I) -> Option<&I::Item>,
    {
        Self {
            inner,
            peek: peek_func,
            peek_back: (),
            peek_iter: (),
            _ref: Default::default(),
        }
    }
}

impl<'a, I, P, PB, PI> FnPeekableIter<'a, I, P, PB, PI> {
    /// Provide [PeekBack] via a function.
    ///
    /// # Examples
    /// ```
    /// use fastpeek::{PeekAdapters, PeekBack};
    ///
    /// let mut i = [1,2,3].into_iter()
    ///     .fn_peekable(|i| i.as_slice().first())
    ///     .with_peek_back(|i| i.as_slice().last());
    /// assert_eq!(i.peek_back().cloned(), i.next_back());
    /// ```
    pub fn with_peek_back<F>(self, peek_back_func: F) -> FnPeekableIter<'a, I, P, F, PI>
    where
        I: Iterator,
        F: Fn(&I) -> Option<&I::Item>,
    {
        FnPeekableIter {
            inner: self.inner,
            peek: self.peek,
            peek_back: peek_back_func,
            peek_iter: self.peek_iter,
            _ref: Default::default(),
        }
    }

    /// Provide [PeekIter] via a function.
    ///
    /// # Examples
    /// ```
    ///  use fastpeek::{PeekAdapters, PeekIter};
    ///  let vec = [1, 2, 3];
    ///  let i = vec
    ///     .into_iter()
    ///     .fn_peekable(|o| o.as_slice().first())
    ///     .with_peek_iter(|o| o.as_slice().iter());
    ///  assert!(vec.iter().zip(i.peek_iter()).all(|(a, b)| a == b));
    /// ```
    pub fn with_peek_iter<F, RI>(self, peek_iter_func: F) -> FnPeekableIter<'a, I, P, PB, F>
    where
        I: Iterator,
        RI: Iterator,
        F: Fn(&'a I) -> RI,
    {
        FnPeekableIter {
            inner: self.inner,
            peek: self.peek,
            peek_back: self.peek_back,
            peek_iter: peek_iter_func,
            _ref: Default::default(),
        }
    }
}

impl<I: Iterator, P, PB, PI> Iterator for FnPeekableIter<'_, I, P, PB, PI> {
    type Item = I::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline(always)]
    fn fold<B, Fold>(self, init: B, f: Fold) -> B
    where
        Self: Sized,
        Fold: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }
}

impl<I: DoubleEndedIterator, P, PB, PI> DoubleEndedIterator for FnPeekableIter<'_, I, P, PB, PI> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    #[inline(always)]
    fn rfold<B, Fold>(self, init: B, f: Fold) -> B
    where
        Self: Sized,
        Fold: FnMut(B, Self::Item) -> B,
    {
        self.inner.rfold(init, f)
    }
}

impl<I: ExactSizeIterator, P, PB, PI> ExactSizeIterator for FnPeekableIter<'_, I, P, PB, PI> {}
impl<I: Clone, P: Clone, PB: Clone, PI: Clone> Clone for FnPeekableIter<'_, I, P, PB, PI> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            peek: self.peek.clone(),
            peek_back: self.peek_back.clone(),
            peek_iter: self.peek_iter.clone(),
            _ref: Default::default(),
        }
    }
}

impl<'a, I, P, PB, PI> Peek<'a, I> for FnPeekableIter<'a, I, P, PB, PI>
where
    I: Iterator,
    P: Fn(&'a I) -> Option<&'a I::Item>,
{
    type PeekItem = &'a I::Item;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        (self.peek)(&self.inner)
    }
}

impl<'a, I, P, PB, PI> PeekBack<'a, I> for FnPeekableIter<'a, I, P, PB, PI>
where
    I: Iterator,
    PB: Fn(&'a I) -> Option<&'a I::Item>,
{
    type PeekItem = &'a I::Item;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        (self.peek_back)(&self.inner)
    }
}

impl<'a, I, P, PB, PI, RI> PeekIter<'a, I> for FnPeekableIter<'a, I, P, PB, PI>
where
    I: Iterator,
    PI: Fn(&'a I) -> RI,
    RI: Iterator,
{
    type Iter = RI;

    fn peek_iter(&'a self) -> Self::Iter {
        (self.peek_iter)(&self.inner)
    }
}
