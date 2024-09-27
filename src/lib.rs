mod adapter;
mod blanket;

pub use adapter::*;
pub use blanket::*;

/// A trait that can be implemented for iterators that allow accessing the underlying data
/// without calling next(). There are blanket implementations for all iterators that implement
/// AsRef<[Self::Item]>
pub trait Peek<'a, T>: Iterator {
    type PeekItem;
    fn peek(&'a self) -> Option<Self::PeekItem>;
}

#[cfg(test)]
mod test {
    use crate::Peek;
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
