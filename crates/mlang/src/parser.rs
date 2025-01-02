use parserc::{is_char, keyword, take_until, take_while, ParseSource, Parser, Source, Span};

use crate::opcode::{Comment, Ident, Numeric, Property, Type};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    #[error(transparent)]
    Parserc(#[from] parserc::Error),

    #[error("ident must start with alphabetic. {0:?}")]
    Indent(Span),

    #[error("expect numeric. {0:?}")]
    Numeric(Span),

    #[error("invalid vec declare. {0:?}")]
    Vec(Span),

    #[error("invalid array declare. {0:?}")]
    Array(Span),
    #[error("expect a type declare. {0:?}")]
    Type(Span),
}

fn skip_whitespaces(source: &mut Source<'_>) -> Result<Option<Span>, ParseError> {
    let span = take_while(char::is_whitespace).parse(source)?;

    Ok(span)
}

/// Implement `ParseSource` for [`Ident`]
impl ParseSource for Ident {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        // at least one char.
        let (c, start) = source.next()?;

        if !c.is_alphabetic() && c != '_' {
            return Err(ParseError::Indent(start));
        }

        let span = if let Some(tail) =
            take_while(|c| c.is_ascii_alphanumeric() || c == '_').parse(source)?
        {
            source.extend_to_inclusive(start, tail)
        } else {
            start
        };

        Ok(Self(source.to_str(span).to_string()))
    }
}

impl ParseSource for Numeric {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        // at least one char.
        let (c, start) = source.next()?;

        if !c.is_ascii_digit() {
            return Err(ParseError::Numeric(start));
        }

        let span = if let Some(tail) = take_while(|c| c.is_ascii_alphanumeric()).parse(source)? {
            source.extend_to_inclusive(start, tail)
        } else {
            start
        };

        let numeric = usize::from_str_radix(source.to_str(span), 10).unwrap();

        Ok(Self(numeric))
    }
}

impl ParseSource for Comment {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        // comment start with ///
        _ = keyword("///").parse(source)?;

        if let Some(line) = take_until(|c| c == '\n').parse(source)? {
            Ok(Comment(source.to_str(line).trim().to_string()))
        } else {
            Ok(Comment("".to_string()))
        }
    }
}

impl ParseSource for Type {
    type Error = ParseError;
    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let ty = keyword("bool")
            .map(|_| Type::Bool)
            .or(keyword("string").map(|_| Type::String))
            .or(keyword("byte").map(|_| Type::Byte))
            .or(keyword("ubyte").map(|_| Type::Ubyte))
            .or(keyword("short").map(|_| Type::Short))
            .or(keyword("ushort").map(|_| Type::Ushort))
            .or(keyword("int").map(|_| Type::Int))
            .or(keyword("uint").map(|_| Type::Uint))
            .or(keyword("long").map(|_| Type::Long))
            .or(keyword("ulong").map(|_| Type::Ulong))
            .or(keyword("float").map(|_| Type::Float))
            .or(keyword("double").map(|_| Type::Double))
            .optional()
            .parse(source)?;

        if let Some(ty) = ty {
            return Ok(ty);
        }

        if let Some(span) = keyword("vec").optional().parse(source)? {
            match Self::parse_vec(source) {
                Ok(ty) => return Ok(ty),
                Err(_) => {
                    source.seek(span)?;
                    return Err(ParseError::Vec(span));
                }
            }
        }

        if let Some(span) = is_char('[').optional().parse(source)? {
            match Self::parse_array(source) {
                Ok(ty) => return Ok(ty),
                Err(_) => {
                    source.seek(span)?;
                    return Err(ParseError::Array(span));
                }
            }
        }

        if let Some(ident) = source.parse::<Option<Ident>>()? {
            return Ok(Type::Ref(ident));
        } else {
            return Err(ParseError::Type(source.span().ok_or(parserc::Error::Eof)?));
        }
    }
}

impl Type {
    fn parse_vec(source: &mut Source<'_>) -> Result<Type, ParseError> {
        skip_whitespaces(source)?;

        is_char('[').parse(source)?;

        skip_whitespaces(source)?;

        let component = Type::parse(source)?;

        skip_whitespaces(source)?;

        is_char(']').parse(source)?;

        return Ok(Type::ListOf(Box::new(component)));
    }

    fn parse_array(source: &mut Source<'_>) -> Result<Type, ParseError> {
        skip_whitespaces(source)?;

        let component = Type::parse(source)?;

        skip_whitespaces(source)?;

        is_char(';').parse(source)?;

        skip_whitespaces(source)?;

        let numeric = source.parse::<Numeric>()?;

        skip_whitespaces(source)?;

        is_char(']').parse(source)?;

        return Ok(Type::ArrayOf(Box::new(component), numeric));
    }
}

impl ParseSource for Property {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        is_char('#').parse(source)?;
        skip_whitespaces(source)?;
        is_char('[').parse(source)?;
        skip_whitespaces(source)?;

        let mut idents = vec![source.parse::<Ident>()?];

        skip_whitespaces(source)?;

        while let Some(_) = is_char(',').optional().parse(source)? {
            skip_whitespaces(source)?;

            if let Some(ident) = source.parse::<Option<Ident>>()? {
                idents.push(ident);
                skip_whitespaces(source)?;
            } else {
                break;
            }
        }

        skip_whitespaces(source)?;

        is_char(']').parse(source)?;

        Ok(Self(idents))
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ParseSource, Source, Span};

    use crate::{
        opcode::{Comment, Ident, Numeric, Property, Type},
        parser::ParseError,
    };

    #[test]
    fn test_ident() {
        let mut source = Source::from("hello#");

        let ident = Ident::parse(&mut source).unwrap();

        assert_eq!(ident, Ident::from("hello"));

        assert_eq!(
            Ident::parse(&mut Source::from("hello_")),
            Ok(Ident::from("hello_"))
        );

        assert_eq!(
            Ident::parse(&mut Source::from("_hello_")),
            Ok(Ident::from("_hello_"))
        );

        assert_eq!(
            Ident::parse(&mut Source::from("_hello1234_-")),
            Ok(Ident::from("_hello1234_"))
        );

        Ident::parse(&mut Source::from("#hello")).expect_err("start with #");
        Ident::parse(&mut Source::from("3hello")).expect_err("start with number");
    }

    #[test]
    fn test_numeric() {
        assert_eq!(
            Numeric::parse(&mut Source::from("1234#")).unwrap(),
            Numeric(1234)
        );

        Numeric::parse(&mut Source::from("#123")).expect_err("start with #");
        Numeric::parse(&mut Source::from("h324")).expect_err("start with alphabetic");
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            Comment::parse(&mut Source::from("/// hello world  \n")),
            Ok(Comment("hello world".into()))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("/// hello world")),
            Ok(Comment("hello world".into()))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("/// \thello world")),
            Ok(Comment("hello world".into()))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("\thello world")),
            Err(ParseError::Parserc(parserc::Error::Keyword(
                "///",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 1
                }
            )))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("//hello world")),
            Err(ParseError::Parserc(parserc::Error::Keyword(
                "///",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 1
                }
            )))
        );
    }

    #[test]
    fn test_ty() {
        assert_eq!(
            Type::parse(&mut Source::from("")),
            Err(ParseError::Parserc(parserc::Error::Eof))
        );

        assert_eq!(
            Type::parse(&mut Source::from("#")),
            Err(ParseError::Type(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 1,
            }))
        );
        assert_eq!(Type::parse(&mut Source::from("bool#")), Ok(Type::Bool));
        assert_eq!(Type::parse(&mut Source::from("byte#")), Ok(Type::Byte));
        assert_eq!(Type::parse(&mut Source::from("ubyte#")), Ok(Type::Ubyte));
        assert_eq!(Type::parse(&mut Source::from("short#")), Ok(Type::Short));
        assert_eq!(Type::parse(&mut Source::from("ushort#")), Ok(Type::Ushort));
        assert_eq!(Type::parse(&mut Source::from("int#")), Ok(Type::Int));
        assert_eq!(Type::parse(&mut Source::from("uint#")), Ok(Type::Uint));
        assert_eq!(Type::parse(&mut Source::from("long#")), Ok(Type::Long));
        assert_eq!(Type::parse(&mut Source::from("ulong#")), Ok(Type::Ulong));
        assert_eq!(Type::parse(&mut Source::from("float#")), Ok(Type::Float));
        assert_eq!(Type::parse(&mut Source::from("double#")), Ok(Type::Double));
        assert_eq!(
            Type::parse(&mut Source::from("hello_world#")),
            Ok(Type::Ref(Ident::from("hello_world")))
        );

        assert_eq!(
            Type::parse(&mut Source::from("vec[bool]")),
            Ok(Type::ListOf(Box::new(Type::Bool)))
        );
        assert_eq!(
            Type::parse(&mut Source::from("vec[[float;3]]")),
            Ok(Type::ListOf(Box::new(Type::ArrayOf(
                Box::new(Type::Float),
                Numeric(3)
            ))))
        );

        assert_eq!(
            Type::parse(&mut Source::from("vec[  [\n_hello ;\t3]\r\n]")),
            Ok(Type::ListOf(Box::new(Type::ArrayOf(
                Box::new(Type::Ref("_hello".into())),
                Numeric(3)
            ))))
        );
    }

    #[test]
    fn test_property() {
        assert_eq!(
            Property::parse(&mut Source::from("#[ hello, world\t]")),
            Ok(Property::from(["hello", "world"]))
        );

        assert_eq!(
            Property::parse(&mut Source::from("#[]")),
            Err(ParseError::Indent(Span {
                lines: 1,
                cols: 3,
                offset: 2,
                len: 1
            }))
        );
    }
}
