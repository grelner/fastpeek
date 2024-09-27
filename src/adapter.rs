use crate::{Peek, PeekBack, PeekIter};

/// Convenience trait for creating a Peek adapter. It is implemented for all iterators.
pub trait PeekAdapters: Iterator + Sized {
    fn cloning_peekable(self) -> CloningPeekableIter<Self> {
        CloningPeekableIter::new(self)
    }
    fn prefetch_peekable(self) -> PrefetchPeekableIter<Self> {
        PrefetchPeekableIter::new(self)
    }
}

impl<I: Iterator> PeekAdapters for I {}

macro_rules! delegate_iter {
    ($t:ident) => {
        impl<I: Iterator> Iterator for $t<I> {
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
    };
}

macro_rules! delegate_double_ended_iter {
    ($t: ident) => {
        impl<I: DoubleEndedIterator> DoubleEndedIterator for $t<I> {
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
    };
}

/// Provide [Peek] by cloning an iterator and calling next() to peek a value. This is useful
/// for cheaply cloneable iterators, such as iterators that are backed by slices.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CloningPeekableIter<I> {
    inner: I,
}

impl<I> CloningPeekableIter<I> {
    pub fn new(iter: I) -> Self {
        Self { inner: iter }
    }
}
delegate_iter!(CloningPeekableIter);
delegate_double_ended_iter!(CloningPeekableIter);
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

/// Provide [Peek] by using a similar strategy as [std::iter::Peekable]. Since self is not mutable
/// in [Peek::peek], this implementation eagerly fetches the value of next(). While this adapter
/// defeats the main purpose of this crate, it may be useful in edge cases where you want to compose
/// on [Peek] but have no other way of providing it.
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
            std::mem::swap(&mut self.peeked, &mut result);
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

#[cfg(test)]
mod test {
    use crate::{Peek, PeekAdapters, PeekBack, PeekIter};

    #[test]
    fn test_cloned() {
        let vec = [1, 2, 3];
        let mut i = vec.iter().cloning_peekable();
        assert_eq!(i.peek(), i.next());
        assert_eq!(i.peek_back(), i.next_back());
    }

    #[test]
    fn test_cloned_iter() {
        let i = [1, 2, 3].into_iter().cloning_peekable();
        let peeked = i.peek_iter().collect::<Vec<_>>();

        assert!(peeked.iter().zip(i).all(|(a, b)| *a == b))
    }

    #[test]
    fn test_prefetch() {
        let vec = [1, 2, 3];
        let mut i = vec.into_iter().prefetch_peekable();
        assert_eq!(i.peek().cloned(), i.next());
    }
}
