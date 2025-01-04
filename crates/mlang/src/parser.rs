use parserc::{take_while, Diagnostic, FromInput, Kind, Parser, ParserError, ToDiagnostic};

use crate::opcode::Ident;

#[derive(Debug, thiserror::Error, PartialEq, PartialOrd, Clone)]
pub enum MlError {
    #[error(transparent)]
    Parserc(#[from] Kind),

    #[error("expect ident")]
    Ident(Diagnostic),
}

impl ParserError for MlError {
    fn diagnostic(&self) -> &parserc::Diagnostic {
        match self {
            MlError::Parserc(kind) => kind.diagnostic(),
            Self::Ident(diagnostic) => diagnostic,
        }
    }
}

impl FromInput for Ident {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let (c, start) = input.next();

        if let Some(c) = c {
            if c != '_' && !c.is_alphabetic() {
                return Err(MlError::Ident(start.recoverable()));
            }
        } else {
            return Err(MlError::Ident(start.incomplete()));
        }

        let body = take_while(|c| c == '_' || c.is_alphanumeric()).parse(input)?;

        let span = if let Some(body) = body {
            start.extend_to_inclusive(body)
        } else {
            start
        };

        assert!(span.len() > 0);

        let ident = input.as_str(span);

        assert_eq!(ident.len(), span.len());

        Ok(Ident(ident.to_string(), Some(span)))
    }
}

#[cfg(test)]
mod tests {
    use parserc::{FromInput, Input, Span, ToDiagnostic};

    use crate::{opcode::Ident, parser::MlError};

    #[test]
    fn test_ident() {
        assert_eq!(
            Ident::parse(&mut Input::from("hello")),
            Ok(Ident::from_span("hello", Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            Ident::parse(&mut Input::from("_hello")),
            Ok(Ident::from_span("_hello", Span::new(0, 6, 1, 1)))
        );

        assert_eq!(
            Ident::parse(&mut Input::from("hello123#")),
            Ok(Ident::from_span("hello123", Span::new(0, 8, 1, 1)))
        );

        assert_eq!(
            Ident::parse(&mut Input::from("123hello123#")),
            Err(MlError::Ident(Span::new(0, 1, 1, 1).recoverable()))
        );

        assert_eq!(
            Ident::parse(&mut Input::from("")),
            Err(MlError::Ident(Span::new(0, 0, 1, 1).incomplete()))
        );
    }
}
