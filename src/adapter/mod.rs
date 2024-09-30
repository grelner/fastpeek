mod cloning;
mod func;
mod prefetch;

pub use cloning::CloningPeekableIter;
pub use func::FnPeekableIter;
pub use prefetch::PrefetchPeekableIter;

/// Convenience trait for creating a Peek adapter. It is implemented for all iterators.
pub trait PeekAdapters: Iterator + Sized {
    fn cloning_peekable(self) -> CloningPeekableIter<Self> {
        CloningPeekableIter::new(self)
    }
    fn prefetch_peekable(self) -> PrefetchPeekableIter<Self> {
        PrefetchPeekableIter::new(self)
    }

    fn fn_peekable<'a, F>(self, func: F) -> FnPeekableIter<'a, Self, F, (), ()>
    where
        F: Fn(&Self) -> Option<&Self::Item>,
    {
        FnPeekableIter::new(self, func)
    }
}

impl<I: Iterator> PeekAdapters for I {}

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
    fn test_cloned_peek_iter() {
        let i = [1, 2, 3].into_iter().cloning_peekable();
        let i2 = i.clone();

        assert!(i.zip(i2.peek_iter()).all(|(a, b)| a == b));
    }

    #[test]
    fn test_prefetch() {
        let vec = [1, 2, 3];
        let mut i = vec.into_iter().prefetch_peekable();
        assert_eq!(i.peek().cloned(), i.next());
    }
}
