use std::str::CharIndices;

use crate::{Error, Result};

/// A region of source code
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub offset: usize,
    pub lines: usize,
    pub cols: usize,
}

impl Span {
    /// Advance span.
    pub fn advance(mut self, steps: i64) -> Result<Self> {
        let offset = self.offset as i64 + steps;

        if offset < 0 {
            return Err(Error::OutOfRange);
        }

        self.offset = offset as usize;

        Ok(self)
    }
}

/// Represent a source code.
pub struct Source<'a> {
    source: &'a str,
    /// char iterator.
    iter: CharIndices<'a>,
    /// current span,
    span: Span,
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            source: value,
            iter: value.char_indices(),
            span: Span {
                offset: 0,
                lines: 1,
                cols: 1,
            },
        }
    }
}

impl<'a> Source<'a> {
    /// Returns current pos.
    pub fn span(&self) -> Span {
        return self.span;
    }

    /// Returns current pos.
    pub fn as_str(&self, start: Span, end: Option<Span>) -> &'a str {
        if let Some(end) = end {
            &self.source[start.offset..end.offset]
        } else {
            &self.source[start.offset..]
        }
    }

    /// Seek in the source by [`span`] which may be returned by [`span`](Self::span) fn.
    pub fn seek(&mut self, span: Span) -> Result<()> {
        if self.source.len() < span.offset {
            return Err(Error::OutOfRange);
        }

        self.iter = self.source[span.offset..].char_indices();
        self.span = span;

        Ok(())
    }

    /// Returns next `char` with pos in the source.
    ///
    /// Returns `None` if there is no more data to read.
    pub fn next(&mut self) -> Option<(char, Span)> {
        match self.iter.next() {
            Some((_, c)) => {
                let span = self.span;
                self.span.offset += c.len_utf8();

                if c == '\n' {
                    self.span.lines += 1;
                    self.span.cols = 1;
                } else {
                    self.span.cols += 1;
                }

                Some((c, span))
            }
            None => None,
        }
    }

    /// Parse a [`ParseSource`] type from this source.
    pub fn parse<P>(&mut self) -> Result<P>
    where
        P: ParseSource,
    {
        P::parse(self)
    }
}

/// A type that can be parsed from `source` should implement this trait.
pub trait ParseSource: Sized {
    /// Create self by parse input source.
    fn parse(source: &mut Source<'_>) -> Result<Self>;
}

impl<T> ParseSource for Option<T>
where
    T: ParseSource,
{
    fn parse(source: &mut Source<'_>) -> Result<Self> {
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
