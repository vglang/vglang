use parserc::{take_while, ParseSource, Parser, Source, Span};

use crate::opcode::Ident;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Parserc(#[from] parserc::Error),

    #[error("ident must start with alphabetic.")]
    Indent(Span),
}

/// Implement `ParseSource` for [`Ident`]
impl ParseSource for Ident {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        // at least one char.
        let (c, start) = source.next()?;

        if !c.is_alphabetic() {
            return Err(ParseError::Indent(start));
        }

        let tail = take_while(|c| c.is_ascii_alphanumeric()).parse(source)?;

        let span = source.extend_to_inclusive(start, tail);

        Ok(Self(source.to_str(span).to_string()))
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ParseSource, Source};

    use crate::opcode::Ident;

    #[test]
    fn test_ident() {
        let mut source = Source::from("hello#");

        let ident = Ident::parse(&mut source).unwrap();

        assert_eq!(ident, Ident::from("hello"));

        Ident::parse(&mut Source::from("#hello")).expect_err("start with #");
        Ident::parse(&mut Source::from("3hello")).expect_err("start with number");
    }
}
