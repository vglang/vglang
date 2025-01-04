use std::{fmt::Display, iter::Peekable, str::CharIndices};

/// A `span` is a reference to a fragment of the [`Input`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Span {
    offset: usize,
    len: usize,
    lines: usize,
    cols: usize,
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
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.lines, self.cols)
    }
}

/// A seekable source code stream.
pub struct Input<'a> {
    /// raw source code.
    source: &'a str,
    /// the char stream iterator.
    iter: Peekable<CharIndices<'a>>,
    /// current reading offset. start with '0'
    offset: usize,
    /// tracking the line no. start with '1'
    lines: usize,
    /// tracking the col no. start with `1`
    cols: usize,
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            source: value,
            iter: value.char_indices().peekable(),
            lines: 1,
            cols: 1,
            offset: 0,
        }
    }
}

impl<'a> Input<'a> {
    /// Return the [`Span`] of the next char.
    ///
    /// Use [`eof`](Span::eof) fn to check if the eof is reached.
    #[inline]
    pub fn span(&mut self) -> Span {
        let (_, span) = self.peek();

        span
    }

    /// Seek to the start of the `span`.
    ///
    /// A seek beyond the end of the stream is not allowed, will cause a panic.
    pub fn seek(&mut self, span: Span) {
        assert!(span.offset <= self.source.len(), "seek: out of range.");
        if self.offset != span.offset {
            self.offset = span.offset;
            self.cols = span.cols;
            self.lines = span.lines;
            self.iter = self.source[span.offset..].char_indices().peekable();
        }
    }

    /// peek up next char in the reading stream.
    pub fn peek(&mut self) -> (Option<char>, Span) {
        if let Some((_, c)) = self.iter.peek() {
            (
                Some(*c),
                Span {
                    offset: self.offset,
                    len: c.len_utf8(),
                    lines: self.lines,
                    cols: self.cols,
                },
            )
        } else {
            (
                None,
                Span {
                    offset: self.offset,
                    len: 0,
                    lines: self.lines,
                    cols: self.cols,
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {

    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn test_span() {
        assert_eq!(Input::from("hello world").span().len(), 1);
        assert!(Input::from("").span().eof());
        assert_eq!(Input::from("你好").span().len(), '你'.len_utf8());
    }

    #[test]
    fn test_seek() {
        Input::from("hello world").seek(Span {
            offset: 11,
            len: 0,
            lines: 1,
            cols: 12,
        });

        let r = catch_unwind(|| {
            Input::from("hello world").seek(Span {
                offset: 12,
                len: 0,
                lines: 1,
                cols: 12,
            });
        });

        assert!(r.is_err());
    }
}
