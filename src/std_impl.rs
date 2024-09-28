use crate::{Peek, PeekBack, PeekIter};

impl<'a, T: 'a, const N: usize> Peek<'a, ()> for core::array::IntoIter<T, N> {
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_slice().first()
    }
}

impl<'a, T: 'a, const N: usize> PeekBack<'a, ()> for core::array::IntoIter<T, N> {
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_slice().last()
    }
}

impl<'a, T: 'a, const N: usize> PeekIter<'a, ()> for core::array::IntoIter<T, N> {
    type Iter = core::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_slice().iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{Peek, PeekBack, PeekIter};
    #[test]
    fn test_array_into_iter() {
        let vec = [1, 2, 3];
        let mut i = vec.into_iter();
        assert_eq!(i.peek().cloned(), i.next());
        assert_eq!(i.peek_back().cloned(), i.next_back());
    }

    #[test]
    fn test_array_peek_into_iter() {
        let vec = [1, 2, 3];
        let i = vec.into_iter();
        let i2 = i.clone();

        assert!(i.zip(i2.peek_iter()).all(|(a, b)| a == *b));
    }
}
