use crate::{Peek, PeekBack, PeekIter};
pub struct Owned;
pub struct Borrowed;
impl<'a, T, I> Peek<'a, Owned> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

impl<'a, T, I> PeekBack<'a, Owned> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().last()
    }
}

impl<'a, I, T> PeekIter<'a, Owned> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type Iter = std::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_ref().iter()
    }
}

impl<'a, T, I> Peek<'a, Borrowed> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

impl<'a, T, I> PeekBack<'a, Borrowed> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().last()
    }
}

impl<'a, I, T> PeekIter<'a, Borrowed> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type Iter = std::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_ref().iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{Peek, PeekBack, PeekIter};
    #[test]
    fn test_vec_into_iter() {
        let vec = vec![1, 2, 3];
        let mut i = vec.into_iter();
        assert_eq!(i.peek().cloned(), i.next());
        assert_eq!(i.peek_back().cloned(), i.next_back());
    }

    #[test]
    fn test_vec_iter() {
        let vec = vec![1, 2, 3];
        let mut i = vec.iter();
        assert_eq!(i.peek().cloned(), i.next().cloned());
        assert_eq!(i.peek_back().cloned(), i.next_back().cloned());
    }

    #[test]
    fn test_peek_iter() {
        let vec = vec![1, 2, 3];
        let i = vec.iter();
        let peeked = i.peek_iter().map(|o| *o).collect::<Vec<_>>();

        assert!(peeked.iter().zip(i).all(|(a, b)| *a == *b))
    }

    #[test]
    fn test_peek_into_iter() {
        let i = vec![1, 2, 3].into_iter();
        let peeked = i.peek_iter().map(|o| *o).collect::<Vec<_>>();

        assert!(peeked.iter().zip(i).all(|(a, b)| *a == b))
    }
}
