struct Owned;
struct Borrowed;

/// A trait that can be implemented for iterators that allow accessing the underlying data
/// without calling next(). There are blanket implementations for all iterators that implement
/// AsRef<[Self::Item]>
pub trait PeekableIter<'a, T>: Iterator {
    type PeekItem;
    fn peek(&'a self) -> Option<Self::PeekItem>;
}

impl<'a, T, I> PeekableIter<'a, Owned> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

impl<'a, T, I> PeekableIter<'a, Borrowed> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

#[cfg(test)]
mod test {
    use crate::PeekableIter;
    #[test]
    fn test_vec_into_iter() {
        let vec = vec![1, 2, 3];
        let mut i = vec.into_iter();
        assert_eq!(i.peek().cloned(), i.next());
    }

    #[test]
    fn test_vec_iter() {
        let vec = vec![1, 2, 3];
        let mut i = vec.iter();
        assert_eq!(i.peek().cloned(), i.next().cloned());
    }
}
