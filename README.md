# fastpeek [![rust-version-badge][]][rust-version]

This crate aims to provide a way to peek iterators without the overhead of
Peekable on normal iteration. It does so by providing the trait PeekableIter
which are implemented directly on iterators that provide access to the
underlying data. This allows us to do normal iteration without any overhead,
as peek() does not have to call next() and buffer the result.

A blanket implementations is provided for Iterators that implement AsRef<[T]>.

[rust-version-badge]: https://img.shields.io/badge/rust-latest%20stable-blue.svg?style=flat-square

[rust-version]: https://github.com/grelner/fastpeek#rust-version-policy