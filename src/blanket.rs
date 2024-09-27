use crate::{Peek, PeekBack};
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
