use std::fmt::Debug;

use crate::Span;

/// Diagnosis Context for parser error.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Diagnostic {
    #[error("error({0})")]
    Recoverable(Span),
    #[error("fatal error({0})")]
    Fatal(Span),
    #[error("eof")]
    Incomplete,
}

/// A parser combinator raised error must implement this trait.
pub trait ParserError: std::error::Error + PartialEq + Debug + From<Kind> {
    /// get diagnosis context data from the error.
    fn diagnostic(&self) -> &Diagnostic;
}

/// [`Error`](ParserError) type used by builtin parser combinator.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Kind {
    #[error("expect char('{0}'), {1}")]
    Char(char, Diagnostic),

    #[error("expect keyword('{0}'), {1}")]
    Keyword(String, Diagnostic),
}

impl ParserError for Kind {
    fn diagnostic(&self) -> &Diagnostic {
        match self {
            Kind::Char(_, diagnostic) => diagnostic,
            Kind::Keyword(_, diagnostic) => diagnostic,
        }
    }
}
