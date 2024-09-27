use crate::{Peek, PeekBack, PeekIter};

impl<'a, T: 'a, const N: usize> Peek<'a, ()> for std::array::IntoIter<T, N> {
    type PeekItem = &'a T;

    fn peek(&'a self) -> Option<Self::PeekItem> {
        self.as_slice().first()
    }
}

impl<'a, T: 'a, const N: usize> PeekBack<'a, ()> for std::array::IntoIter<T, N> {
    type PeekItem = &'a T;

    fn peek_back(&'a self) -> Option<Self::PeekItem> {
        self.as_slice().last()
    }
}

impl<'a, T: 'a, const N: usize> PeekIter<'a, ()> for std::array::IntoIter<T, N> {
    type Iter = std::slice::Iter<'a, T>;

    fn peek_iter(&'a self) -> Self::Iter {
        self.as_slice().iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{Peek, PeekBack, PeekIter};
    #[test]
    fn test_vec_into_iter() {
        let vec = [1, 2, 3];
        let mut i = vec.into_iter();
        assert_eq!(i.peek().cloned(), i.next());
        assert_eq!(i.peek_back().cloned(), i.next_back());
    }

    #[test]
    fn test_vec_iter() {
        let vec = [1, 2, 3];
        let mut i = vec.iter();
        assert_eq!(i.peek().cloned(), i.next().cloned());
        assert_eq!(i.peek_back().cloned(), i.next_back().cloned());
    }

    #[test]
    fn test_peek_iter() {
        let vec = [1, 2, 3];
        let i = vec.iter();
        let peeked = i.peek_iter().copied().collect::<Vec<_>>();

        assert!(peeked.iter().zip(i).all(|(a, b)| *a == *b))
    }

    #[test]
    fn test_peek_into_iter() {
        let i = [1, 2, 3].into_iter();
        let peeked = i.peek_iter().copied().collect::<Vec<_>>();

        assert!(peeked.iter().zip(i).all(|(a, b)| *a == b))
    }
}
