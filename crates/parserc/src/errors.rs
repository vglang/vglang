use std::fmt::Debug;

use crate::Span;

/// Diagnosis Context for parser error.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Diagnostic {
    #[error("error({0})")]
    Recoverable(Span),
    #[error("fatal error({0})")]
    Fatal(Span),
    #[error("eof")]
    Incomplete(Span),
}

/// A helper trait that convert [`Span`] to [`Diagnostic`]
pub trait ToDiagnostic {
    /// Convert to [`Recoverable`](Diagnostic::Recoverable)
    fn recoverable(self) -> Diagnostic;

    /// Convert to [`Fatal`](Diagnostic::Fatal)
    fn fatal(self) -> Diagnostic;

    /// Convert to [`Incomplete`](Diagnostic::Incomplete)
    fn incomplete(self) -> Diagnostic;
}

impl ToDiagnostic for Span {
    fn recoverable(self) -> Diagnostic {
        Diagnostic::Recoverable(self)
    }

    fn fatal(self) -> Diagnostic {
        Diagnostic::Fatal(self)
    }

    fn incomplete(self) -> Diagnostic {
        Diagnostic::Incomplete(self)
    }
}

/// The returns error types of the parser combinator must implement this trait.
///
/// See [`Parser`](crate::Parser).
///
/// # Examples
///
/// ```
/// use parserc::*;
///
/// /// Custom parser errors.
/// #[derive(Debug, thiserror::Error, PartialEq)]
/// pub enum MyError {
///     #[error(transparent)]
///     Parserc(#[from] parserc::Kind),
///     #[error("expect ident, {0}")]
///     Ident(Diagnostic),
/// }
///
/// impl ParserError for MyError {
///     fn diagnostic(&self) -> &Diagnostic {
///         match self {
///             Self::Parserc(kind) => kind.diagnostic(),
///             Self::Ident(diagnostic) => diagnostic,
///         }
///     }
/// }
///
/// ```
pub trait ParserError: std::error::Error + PartialEq + Debug + From<Kind> {
    /// get diagnosis context data from the error.
    fn diagnostic(&self) -> &Diagnostic;
}

/// [`Error`](ParserError) type used by builtin parser combinator.
#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
