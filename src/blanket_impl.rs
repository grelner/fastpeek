use crate::{Peek, PeekBack, PeekIter};
impl<'a, T, I> Peek<'a, I> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

impl<'a, T, I> PeekBack<'a, I> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().last()
    }
}

impl<'a, I, T> PeekIter<'a, I> for I
where
    T: 'a,
    I: Iterator<Item = T> + AsRef<[T]>,
{
    type Iter = core::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_ref().iter()
    }
}

impl<'a, T, I> Peek<'a, &I> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().first()
    }
}

impl<'a, T, I> PeekBack<'a, &I> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_ref().last()
    }
}

impl<'a, I, T> PeekIter<'a, &I> for I
where
    T: 'a,
    I: Iterator<Item = &'a T> + AsRef<[T]>,
{
    type Iter = core::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_ref().iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{Peek, PeekBack, PeekIter};
    #[test]
    #[allow(clippy::useless_vec)]
    fn test_as_ref_into_iter() {
        let vec = &[1, 2, 3];
        let mut i = vec.into_iter();
        assert_eq!(i.peek().cloned(), i.next().cloned());
        assert_eq!(i.peek_back().cloned(), i.next_back().cloned());
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_as_ref_iter() {
        let vec = &[1, 2, 3];
        let mut i = vec.iter();
        assert_eq!(i.peek().cloned(), i.next().cloned());
        assert_eq!(i.peek_back().cloned(), i.next_back().cloned());
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_as_ref_peek_iter() {
        let vec = &[1, 2, 3];
        assert!(vec
            .iter()
            .zip(vec.iter().peek_iter())
            .all(|(a, b)| *a == *b));
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_as_ref_peek_into_iter() {
        let i = (&[1, 2, 3]).into_iter();
        let peek_i = i.clone();

        assert!(i.zip(peek_i.peek_iter()).all(|(a, b)| a == b))
    }
}
