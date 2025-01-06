use std::fmt::Display;

use crate::Kind;

/// A `span` is a reference to a fragment of the [`Input`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    pub offset: usize,
    pub len: usize,
    pub lines: usize,
    pub cols: usize,
}

impl Span {
    /// Create a new `Span` instance.
    ///
    /// A length of `0` indicates that the span reference to the `eof`
    pub fn new(offset: usize, len: usize, lines: usize, cols: usize) -> Self {
        Self {
            offset,
            len,
            lines,
            cols,
        }
    }

    /// Returns true if the `len` is `0`.
    pub fn eof(&self) -> bool {
        self.len == 0
    }

    /// Returns the postion`(lines,cols)` referenced by this span.
    pub fn position(&self) -> (usize, usize) {
        (self.lines, self.cols)
    }

    /// Return the length of this span in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Extend self to `other`'s start offset.
    pub fn extend_to(self, other: Span) -> Span {
        assert!(
            !(self.offset > other.offset),
            "extend_to: self.offset < other.offset."
        );

        Span {
            offset: self.offset,
            len: other.offset - self.offset,
            lines: self.lines,
            cols: self.cols,
        }
    }

    /// Extend self to `other`'s end offset.
    pub fn extend_to_inclusive(self, other: Span) -> Span {
        assert!(
            !(self.offset > other.offset),
            "extend_to: self.offset < other.offset."
        );

        Span {
            offset: self.offset,
            len: other.offset + other.len - self.offset,
            lines: self.lines,
            cols: self.cols,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.lines, self.cols)
    }
}

/// An abstract of source code.
pub trait Source<'a> {
    /// Return the [`Span`] of the next char.
    ///
    /// Use [`eof`](Span::eof) fn to check if the eof is reached.
    fn span(&mut self) -> Span;

    /// Convert span into &str.
    fn as_str(&self, span: Span) -> &str;

    /// Seek to the start of the `span`.
    ///
    /// A seek beyond the end of the stream is not allowed, will cause a panic.
    fn seek(&mut self, span: Span);

    /// Returns a tuple where the first element is the reading offset,
    /// and second element is the total length of the source code.
    fn size_hint(&mut self) -> (usize, usize);

    /// Returns the unparsed length.
    fn remaining(&mut self) -> usize;

    /// peek up next char in the reading stream.
    fn peek(&mut self) -> (Option<char>, Span);

    /// Returns the next character and its corresponding [`Span`].
    fn next(&mut self) -> (Option<char>, Span);
}

/// An abstract of error report interface.
pub trait Report {
    /// Report error type.
    type Error: From<Kind>;

    /// Report a new error with source code `span`.
    fn report_err(&mut self, error: Self::Error, span: Span);

    /// pop the last error.
    fn pop_err(&mut self) -> Option<(Self::Error, Span)>;

    /// Map last error corresponding to `op` fn.
    ///
    /// If the error report stack is empty, this fn does nothing!
    fn map_err<F>(&mut self, op: F)
    where
        F: FnOnce(Self::Error, Span) -> (Self::Error, Span),
    {
        if let Some((err, span)) = self.pop_err() {
            let (err, span) = op(err, span);
            self.report_err(err, span);
        }
    }
}

/// The context data of the pase task.
pub trait ParseContext<'a>: Report + Source<'a> {}

impl<'a, T> ParseContext<'a> for T where T: Report + Source<'a> {}
