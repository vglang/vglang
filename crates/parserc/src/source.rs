use std::{iter::Peekable, str::CharIndices};

use crate::{Error, Result};

/// A region of source code
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Span {
    pub lines: usize,
    pub cols: usize,
    pub offset: usize,
    pub len: usize,
}

impl Span {
    /// return true if the `Span` point to `None`.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Represent a source code.
pub struct Source<'a> {
    source: &'a str,
    /// char iterator.
    iter: Peekable<CharIndices<'a>>,
    /// current offset,
    offset: usize,
    /// current lines,
    lines: usize,
    /// current cols,
    cols: usize,
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            source: value,
            iter: value.char_indices().peekable(),
            offset: 0,
            lines: 1,
            cols: 1,
        }
    }
}

impl<'a> Source<'a> {
    /// Returns next char's span.
    pub fn span(&mut self) -> Span {
        if let Some((_, c)) = self.iter.peek() {
            Span {
                lines: self.lines,
                cols: self.cols,
                offset: self.offset,
                len: c.len_utf8(),
            }
        } else {
            Span::default()
        }
    }

    /// Extend span from current position to end.
    pub fn extend(&self, span: Span) -> Span {
        Span {
            lines: span.lines,
            cols: span.cols,
            offset: span.offset,
            len: self.source.len() - span.offset,
        }
    }

    /// Extend span from start's start to end's start.
    pub fn extend_to(&self, start: Span, end: Span) -> Span {
        assert!(!end.is_empty(), "invalid parameter, end is empty.");
        Span {
            lines: start.lines,
            cols: start.cols,
            offset: start.offset,
            len: end.offset - start.offset,
        }
    }

    /// Extend span from start's start to end's end.
    pub fn extend_to_inclusive(&self, start: Span, end: Span) -> Span {
        assert!(!end.is_empty(), "invalid parameter, end is empty.");
        Span {
            lines: start.lines,
            cols: start.cols,
            offset: start.offset,
            len: end.offset + end.len - start.offset,
        }
    }

    /// Convert `Span` to a &str.
    pub fn to_str(&self, start: Span) -> &'a str {
        &self.source[start.offset..(start.offset + start.len)]
    }

    /// Seek in the source by [`span`] which may be returned by [`span`](Self::span) fn.
    pub fn seek(&mut self, span: Span) -> Result<()> {
        if self.source.len() < span.offset {
            return Err(Error::OutOfRange);
        }

        self.iter = self.source[span.offset..].char_indices().peekable();
        self.offset = span.offset;
        self.lines = span.lines;
        self.cols = span.cols;

        Ok(())
    }

    /// Returns next `char` with pos in the source.
    ///
    /// Returns `None` if there is no more data to read.
    pub fn next(&mut self) -> Result<(char, Span)> {
        match self.iter.next() {
            Some((_, c)) => {
                let len = c.len_utf8();
                let span = Span {
                    lines: self.lines,
                    cols: self.cols,
                    offset: self.offset,
                    len,
                };

                self.offset += len;

                if c == '\n' {
                    self.lines += 1;
                    self.cols = 1;
                } else {
                    self.cols += 1;
                }

                Ok((c, span))
            }
            None => Err(Error::Eof),
        }
    }

    /// Parse a [`ParseSource`] type from this source.
    pub fn parse<P>(&mut self) -> std::result::Result<P, P::Error>
    where
        P: ParseSource,
    {
        P::parse(self)
    }
}

/// A type that can be parsed from `source` should implement this trait.
pub trait ParseSource: Sized {
    type Error: From<Error>;
    /// Create self by parse input source.
    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error>;
}

impl<T, E> ParseSource for Option<T>
where
    E: From<Error>,
    T: ParseSource<Error = E>,
{
    type Error = E;
    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let span = source.span();
        match T::parse(source) {
            Ok(v) => Ok(Some(v)),
            Err(_) => {
                // rollback.
                source.seek(span).unwrap();
                Ok(None)
            }
        }
    }
}
