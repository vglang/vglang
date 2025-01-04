use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, Diagnostic, FromInput, IntoParser, Kind,
    Parser, ParserError, ParserExt, ToDiagnostic,
};

use crate::opcode::{Ident, LitExpr, LitNum, LitStr};

#[derive(Debug, thiserror::Error, PartialEq, PartialOrd, Clone)]
pub enum MlError {
    #[error(transparent)]
    Parserc(#[from] Kind),

    #[error("expect ident")]
    Ident(Diagnostic),

    #[error("expect literal numeric")]
    LitNum(Diagnostic),

    #[error("hex number body is empty")]
    LitNumHexBody(Diagnostic),

    #[error("expect literal string")]
    LitStr(Diagnostic),
}

impl ParserError for MlError {
    fn diagnostic(&self) -> &parserc::Diagnostic {
        match self {
            MlError::Parserc(kind) => kind.diagnostic(),
            Self::Ident(diagnostic) => diagnostic,
            Self::LitNum(diagnostic) => diagnostic,
            Self::LitStr(diagnostic) => diagnostic,
            Self::LitNumHexBody(diagnostic) => diagnostic,
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

impl FromInput for LitNum {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Some(start) = ensure_keyword("0x").ok().parse(input)? {
            let body = take_while(|c| c.is_ascii_hexdigit())
                .parse(input)?
                .ok_or(MlError::LitNumHexBody(start.fatal()))?;

            assert!(body.len() > 0);

            let numeric = input.as_str(body);

            assert_eq!(numeric.len(), body.len());

            let numeric = usize::from_str_radix(numeric, 16).unwrap();

            return Ok(Self(numeric, Some(start.extend_to_inclusive(body))));
        }

        let (c, start) = input.next();

        if let Some(c) = c {
            if !c.is_ascii_digit() {
                return Err(MlError::LitNum(start.recoverable()));
            }
        } else {
            return Err(MlError::LitNum(start.incomplete()));
        }

        let body = take_while(|c| c.is_ascii_digit()).parse(input)?;

        let span = if let Some(body) = body {
            start.extend_to_inclusive(body)
        } else {
            start
        };

        assert!(span.len() > 0);

        let numeric = input.as_str(span);

        assert_eq!(numeric.len(), span.len());

        let numeric = usize::from_str_radix(numeric, 10).unwrap();

        Ok(Self(numeric, Some(span)))
    }
}

impl FromInput for LitStr {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let (expect, start) = ensure_char('"')
            .map(|span| ('"', span))
            .or(ensure_char('\'').map(|span| ('\'', span)))
            .parse(input)?;

        let body = take_till(|c| c == expect).parse(input)?;

        let end = ensure_char(expect).parse(input)?;

        let span = start.extend_to_inclusive(end);

        if let Some(body) = body {
            Ok(Self(input.as_str(body).to_string(), Some(span)))
        } else {
            Ok(Self("".to_string(), Some(span)))
        }
    }
}

impl FromInput for LitExpr {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        LitStr::into_parser()
            .map(|v| LitExpr::from(v))
            .or(LitNum::into_parser().map(|v| LitExpr::from(v)))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use parserc::{FromInput, Input, Kind, Span, ToDiagnostic};

    use crate::{
        opcode::{Ident, LitNum, LitStr},
        parser::MlError,
    };

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

    #[test]
    fn test_litstr() {
        assert_eq!(
            LitStr::parse(&mut Input::from("'hello world'")),
            Ok(LitStr::from_span("hello world", Span::new(0, 13, 1, 1)))
        );

        assert_eq!(
            LitStr::parse(&mut Input::from("\"hello ' world\"")),
            Ok(LitStr::from_span("hello ' world", Span::new(0, 15, 1, 1)))
        );

        assert_eq!(
            LitStr::parse(&mut Input::from("'hello.")),
            Err(MlError::Parserc(Kind::Char(
                '\'',
                Span::new(7, 0, 1, 8).incomplete()
            )))
        );
    }
    #[test]
    fn test_litnum() {
        assert_eq!(
            LitNum::parse(&mut Input::from("12345")),
            Ok(LitNum::from_span(12345, Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            LitNum::parse(&mut Input::from("0xf20")),
            Ok(LitNum::from_span(0xf20, Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            LitNum::parse(&mut Input::from("0x f20")),
            Err(MlError::LitNumHexBody(Span::new(0, 2, 1, 1).fatal()))
        );

        assert_eq!(
            LitNum::parse(&mut Input::from("123\t45")),
            Ok(LitNum::from_span(123, Span::new(0, 3, 1, 1)))
        );

        assert_eq!(
            LitNum::parse(&mut Input::from("h1234")),
            Err(MlError::LitNum(Span::new(0, 1, 1, 1).recoverable()))
        );
    }
}
