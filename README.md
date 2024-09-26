# fastpeek

This crate aims to provide a way to peek iterators without the overhead of
[Peekable]. It does so by providing the trait [PeekableIter] which are
implemented directly on existing iterators. It does not intercept and buffer
output from [Iterator::next] like [Peekable] and thus there is no overhead
when doing normal iteration.