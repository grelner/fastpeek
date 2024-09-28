#![no_std]
mod adapter;
mod blanket_impl;
mod std_impl;

pub use adapter::*;

extern crate core;

/// A trait for peeking the next value of an iterator.
pub trait Peek<'a, T>: Iterator {
    type PeekItem;
    fn peek(&'a self) -> Option<Self::PeekItem>;
}

/// A trait for peeking the last value of an iterator.
pub trait PeekBack<'a, T>: Iterator {
    type PeekItem;
    fn peek_back(&'a self) -> Option<Self::PeekItem>;
}

/// A trait for peeking all values of an iterator.
pub trait PeekIter<'a, T>: Iterator {
    type Iter: Iterator;
    fn peek_iter(&'a self) -> Self::Iter;
}
