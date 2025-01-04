use std::fmt::Debug;

use crate::Span;

/// Error type used by `parserc`
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Kind {
    #[error("expect keyworld({0})")]
    Keyword(&'static str),

    #[error("Expect token({0})")]
    Token(char),
}

/// Error type used by `parserc`
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error<K>
where
    K: From<Kind> + PartialEq + Debug,
{
    /// Some combinator may fix this error by performing a backtracking algorithm.
    #[error("recoverable: {0}. {1:?}")]
    Recoverable(K, Span),

    /// Indicates an unfixable error
    #[error("fatal: {0}. {1:?}")]
    Fatal(K, Span),

    /// When parsing is not completed at the end of the input, the parser reports this error.
    #[error("incomplete: {0}.")]
    Incomplete(K),
}

/// Result type used by `parserc`
pub type Result<T, K> = std::result::Result<T, Error<K>>;
