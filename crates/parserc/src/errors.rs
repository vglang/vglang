use std::ops::Range;

/// Error type used by `parserc`
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("{0:?} parser failed")]
    Range(Range<usize>),
    #[error("Repeat({0}) parser failed")]
    Repeat(usize),
    #[error("Seek out of range.")]
    OutOfRange,
}

/// Result type used by `parserc`
pub type Result<T> = std::result::Result<T, Error>;
