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

/// A trait that can be implemented for iterators that allow accessing the underlying data
/// without calling next_back(). There are blanket implementations for all iterators that implement
/// AsRef<[Self::Item]>
pub trait PeekBack<'a, T>: Iterator {
    type PeekItem;
    fn peek_back(&'a self) -> Option<Self::PeekItem>;
}
