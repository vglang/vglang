use std::fmt::Debug;

/// [`Error`](ParserError) type used by builtin parser combinator.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Kind {
    #[error("expect char('{0}')")]
    Char(char),

    #[error("expect keyword('{0}')")]
    Keyword(String),
}

/// Error type returns by a [`Parser`].
///
/// The combinators use it to control parsing process.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ControlFlow {
    /// Report a recoverable error. a combinator may convert this error to a `None`.
    #[error("Combinator report a recoverable error.")]
    Recoverable,
    /// Reach the end of source code.
    #[error("Combinator report the source code end is reached.")]
    Incomplete,
    /// Meet a fatal error.
    #[error("Combinator report a fatal error.")]
    Fatal,
}

/// Result type returns by parser combinators.
pub type Result<T> = std::result::Result<T, ControlFlow>;
