use std::fmt::Debug;

/// [`Error`](ParserError) type used by builtin parser combinator.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Kind {
    #[error("expect char('{0}')")]
    Char(char),

    #[error("expect keyword('{0}')")]
    Keyword(String),
}

/// Error type to control combinator parsing procedure.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ControlFlow {
    #[error("combinator report a recoverable error.")]
    Recoverable,
    #[error("combinator report that reached the end of the source code.")]
    Incomplete,
    #[error("combinator report a fatal error.")]
    Fatal,
}

/// `Result` type that used by parser combinator.
pub type Result<T> = std::result::Result<T, ControlFlow>;
