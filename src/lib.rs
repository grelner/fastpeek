pub trait PeekableIter: Iterator {
    fn peek(&self) -> Option<Self::Item>;
}