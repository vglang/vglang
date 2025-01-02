use std::ops::Range;

use crate::Span;

/// Error type used by `parserc`
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("{0:?} parser failed")]
    Range(Range<usize>),
    #[error("Repeat({0}) parser failed")]
    Repeat(usize),
    #[error("Seek out of range.")]
    OutOfRange,
    #[error("Reach the end of the source.")]
    Eof,
    #[error("Expect keyword({0}). {1:?}")]
    Keyword(&'static str, Span),

    #[error("Expect char({0}). {1:?}")]
    Char(char, Span),
}

/// Result type used by `parserc`
pub type Result<T> = std::result::Result<T, Error>;
