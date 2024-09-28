#![no_std]
mod adapter;
mod blanket_impl;
mod std_impl;

pub use adapter::*;

extern crate core;

/// A trait for peeking the next value of an iterator.
///
/// # Examples
/// ```
/// use fastpeek::Peek;
///
/// let mut i = [1,2,3].into_iter();
/// assert_eq!(i.peek().copied(), i.next());
/// ```
pub trait Peek<'a, T>: Iterator {
    type PeekItem;
    fn peek(&'a self) -> Option<Self::PeekItem>;
}

/// A trait for peeking the last value of an iterator.
///
/// # Examples
/// ```
/// use fastpeek::PeekBack;
///
/// let mut i = [1,2,3].into_iter();
/// assert_eq!(i.peek_back().copied(), i.next_back());
/// ```
pub trait PeekBack<'a, T>: Iterator {
    type PeekItem;
    fn peek_back(&'a self) -> Option<Self::PeekItem>;
}

/// A trait for peeking all values of an iterator.
///
/// # Examples
/// ```
/// use fastpeek::PeekIter;
///
/// let mut i = [1,2,3].into_iter();
/// assert!(i.clone().peek_iter().zip(i).all(|(a,b)| *a == b));
/// ```
pub trait PeekIter<'a, T>: Iterator {
    type Iter: Iterator;
    fn peek_iter(&'a self) -> Self::Iter;
}
