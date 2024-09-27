# fastpeek [![rust-version-badge][]][rust-version]

When you need to do fast iteration, std::iter::Peekable adds some overhead because it has to check if it has a buffered
value on each call to next(). This crate provides a simple trait that can be implemented for iterators where the aim
is to peek without calling next().

The crate does not necessarily aim to make peeking faster, but to remove the overhead of std::iter::Peekable when doing
normal iteration.

A blanket implementation is provided for Iterators that implement AsRef<[T]>. In addition, Iterator adapters using
various strategies to provide Peek are provided.

[rust-version-badge]: https://img.shields.io/badge/rust-latest%20stable-blue.svg?style=flat-square

[rust-version]: https://github.com/grelner/fastpeek#rust-version-policy