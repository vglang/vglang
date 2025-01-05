use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, Diagnostic, FromInput, Input, IntoParser,
    Kind, Parser, ParserError, ParserExt, Span, ToDiagnostic,
};

use crate::opcode::{CallExpr, Comment, Field, Ident, LitExpr, LitNum, LitStr, Property, Type};

/// [`ParserError`] defined by `mlang` crate.
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

    #[error("Invalid call expression, {0}")]
    CallExpr(Box<MlError>, Diagnostic),

    #[error("Invalid property expression, {0}")]
    Property(Box<MlError>, Diagnostic),

    #[error("Invalid field, {0}")]
    Field(Box<MlError>, Diagnostic),

    #[error("Invalid type, {0}")]
    Type(Box<MlError>, Diagnostic),
}

impl ParserError for MlError {
    fn diagnostic(&self) -> &parserc::Diagnostic {
        match self {
            MlError::Parserc(kind) => kind.diagnostic(),
            Self::Ident(diagnostic) => diagnostic,
            Self::LitNum(diagnostic) => diagnostic,
            Self::LitStr(diagnostic) => diagnostic,
            Self::LitNumHexBody(diagnostic) => diagnostic,
            Self::CallExpr(_, diagnostic) => diagnostic,
            Self::Property(_, diagnostic) => diagnostic,
            Self::Field(_, diagnostic) => diagnostic,
            Self::Type(_, diagnostic) => diagnostic,
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

impl FromInput for CallExpr {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let ident = Ident::parse(input)?;

        let start = ident.1.clone().unwrap();

        skip_ws(input)?;

        if ensure_char('(').ok().parse(input)?.is_none() {
            return Ok(CallExpr {
                span: Some(ident.1.clone().unwrap()),
                ident,
                params: vec![],
            });
        }

        skip_ws(input)?;

        let mut params = vec![];

        while let Some(expr) = LitExpr::into_parser()
            .ok()
            .map_err(|err| MlError::CallExpr(Box::new(err.into()), start.fatal()))
            .parse(input)?
        {
            params.push(expr);

            skip_ws(input)?;

            if ensure_char(',')
                .ok()
                .map_err(|err| MlError::CallExpr(Box::new(err.into()), start.fatal()))
                .parse(input)?
                .is_none()
            {
                skip_ws(input)?;
                break;
            }

            skip_ws(input)?;
        }

        let end = ensure_char(')')
            .map_err(|err| MlError::CallExpr(Box::new(err.into()), start.fatal()))
            .parse(input)?;

        let span = start.extend_to_inclusive(end);

        Ok(Self {
            span: Some(span),
            ident,
            params,
        })
    }
}

impl FromInput for Property {
    type Error = MlError;

    fn parse(input: &mut parserc::Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let start = ensure_keyword("#[").parse(input)?;

        skip_ws(input)?;

        let mut params = vec![];

        while let Some(expr) = CallExpr::into_parser()
            .ok()
            .map_err(|err| MlError::Property(Box::new(err.into()), start.fatal()))
            .parse(input)?
        {
            params.push(expr);

            skip_ws(input)?;

            if ensure_char(',')
                .ok()
                .map_err(|err| MlError::Property(Box::new(err.into()), start.fatal()))
                .parse(input)?
                .is_none()
            {
                skip_ws(input)?;
                break;
            }

            skip_ws(input)?;
        }

        let end = ensure_char(']')
            .map_err(|err| MlError::Property(Box::new(err.into()), start.fatal()))
            .parse(input)?;

        let span = start.extend_to_inclusive(end);

        Ok(Self {
            span: Some(span),
            params,
        })
    }
}

impl FromInput for Comment {
    type Error = MlError;

    fn parse(input: &mut Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let start = ensure_keyword("///").parse(input)?;

        let content = take_till(|c| c == '\n').parse(input)?;

        let (content, span) = if let Some(content) = content {
            (
                input.as_str(content).trim(),
                start.extend_to_inclusive(content),
            )
        } else {
            ("", start)
        };

        Ok(Comment(content.to_string(), Some(span)))
    }
}

impl FromInput for Type {
    type Error = MlError;
    fn parse(input: &mut Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let ty = ensure_keyword("bool")
            .map(|span| Type::Bool(Some(span)))
            .or(ensure_keyword("string").map(|span| Type::String(Some(span))))
            .or(ensure_keyword("byte").map(|span| Type::Byte(Some(span))))
            .or(ensure_keyword("ubyte").map(|span| Type::Ubyte(Some(span))))
            .or(ensure_keyword("short").map(|span| Type::Short(Some(span))))
            .or(ensure_keyword("ushort").map(|span| Type::Ushort(Some(span))))
            .or(ensure_keyword("int").map(|span| Type::Int(Some(span))))
            .or(ensure_keyword("uint").map(|span| Type::Uint(Some(span))))
            .or(ensure_keyword("long").map(|span| Type::Long(Some(span))))
            .or(ensure_keyword("ulong").map(|span| Type::Ulong(Some(span))))
            .or(ensure_keyword("float").map(|span| Type::Float(Some(span))))
            .or(ensure_keyword("double").map(|span| Type::Double(Some(span))))
            .ok()
            .parse(input)?;

        if let Some(ty) = ty {
            return Ok(ty);
        }

        if let Some(start) = ensure_keyword("vec").ok().parse(input)? {
            skip_ws(input)?;

            ensure_char('[')
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            skip_ws(input)?;

            let component = Type::into_parser()
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            return Ok(Type::ListOf(
                Box::new(component),
                Some(start.extend_to_inclusive(end)),
            ));
        }

        if let Some(start) = ensure_char('[').ok().parse(input)? {
            skip_ws(input)?;

            let component = Type::into_parser()
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            skip_ws(input)?;

            ensure_char(';')
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            skip_ws(input)?;

            let len = LitNum::into_parser()
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .map_err(|err| MlError::Type(Box::new(err.into()), start.fatal()))
                .parse(input)?;

            return Ok(Type::ArrayOf(
                Box::new(component),
                len,
                Some(start.extend_to_inclusive(end)),
            ));
        }

        let start = input.span();

        // try parse as ident at last.
        Ident::into_parser()
            .map_err(|err| MlError::Type(Box::new(err.into()), start.recoverable()))
            .parse(input)
            .map(|ident| Type::Data(ident))
    }
}

impl FromInput for Field {
    type Error = MlError;
    fn parse(input: &mut Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let (comments, properties) = parse_prefix(input)?;

        skip_ws(input)?;

        let ident = Ident::into_parser().ok().parse(input)?;

        let ty = if let Some(ident) = ident.as_ref() {
            skip_ws(input)?;

            let semi_colon = ensure_char(':')
                .ok()
                .map_err(|err| MlError::Property(Box::new(err.into()), ident.1.unwrap().fatal()))
                .parse(input)?;

            if semi_colon.is_none() {
                return Ok(Self {
                    comments,
                    properties,
                    ident: None,
                    ty: Type::Data(ident.clone()),
                });
            }

            skip_ws(input)?;

            Type::into_parser()
                .map_err(|err| MlError::Property(Box::new(err.into()), ident.1.unwrap().fatal()))
                .parse(input)?
        } else {
            Type::parse(input)?
        };

        Ok(Self {
            comments,
            properties,
            ident,
            ty,
        })
    }
}

fn skip_ws(input: &mut Input<'_>) -> Result<Option<Span>, MlError> {
    let span = take_while(|c| c.is_whitespace()).parse(input)?;

    Ok(span)
}

enum Prefix {
    Property(Property),
    Comment(Comment),
}

#[allow(unused)]
fn parse_prefix(input: &mut Input<'_>) -> Result<(Vec<Comment>, Vec<Property>), MlError> {
    let mut properties = vec![];
    let mut comments = vec![];

    let property_parser = Property::into_parser().map(|p| Prefix::Property(p));

    let comment_parser = Comment::into_parser().map(|p| Prefix::Comment(p));

    while let Some(prefix) = property_parser
        .clone()
        .or(comment_parser.clone())
        .ok()
        .parse(input)?
    {
        match prefix {
            Prefix::Property(property) => properties.push(property),
            Prefix::Comment(comment) => comments.push(comment),
        }

        skip_ws(input)?;
    }

    Ok((comments, properties))
}

#[cfg(test)]
mod tests {
    use parserc::{FromInput, Input, Kind, Span, ToDiagnostic};

    use crate::{
        opcode::{CallExpr, Comment, Field, Ident, LitExpr, LitNum, LitStr, Property, Type},
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

    #[test]
    fn test_litexpr() {
        let mut input = Input::from("12345'hello world'");
        assert_eq!(
            LitExpr::parse(&mut input),
            Ok(LitExpr::Numeric(LitNum::from_span(
                12345,
                Span::new(0, 5, 1, 1)
            )))
        );

        assert_eq!(
            LitExpr::parse(&mut input),
            Ok(LitExpr::String(LitStr::from_span(
                "hello world",
                Span::new(5, 13, 1, 6)
            )))
        );
    }

    #[test]
    fn test_callexpr() {
        assert_eq!(
            CallExpr::parse(&mut Input::from("hello")),
            Ok(CallExpr::from_span(
                Ident::from_span("hello", Span::new(0, 5, 1, 1)),
                Span::new(0, 5, 1, 1)
            ))
        );

        assert_eq!(
            CallExpr::parse(&mut Input::from("hello( )")),
            Ok(CallExpr::from_span(
                Ident::from_span("hello", Span::new(0, 5, 1, 1)),
                Span::new(0, 8, 1, 1)
            ))
        );

        assert_eq!(
            CallExpr::parse(&mut Input::from("hello( 'world' )")),
            Ok(CallExpr::from_span(
                Ident::from_span("hello", Span::new(0, 5, 1, 1)),
                Span::new(0, 16, 1, 1)
            )
            .param(LitStr::from_span("world", Span::new(7, 7, 1, 8))))
        );

        assert_eq!(
            CallExpr::parse(&mut Input::from("hello( 'world' , 1234 \t)")),
            Ok(CallExpr::from_span(
                Ident::from_span("hello", Span::new(0, 5, 1, 1)),
                Span::new(0, 24, 1, 1)
            )
            .param(LitStr::from_span("world", Span::new(7, 7, 1, 8)))
            .param(LitNum::from_span(1234, Span::new(17, 4, 1, 18))))
        );

        assert_eq!(
            CallExpr::parse(&mut Input::from("hello('world', 1234,)")),
            Ok(CallExpr::from_span(
                Ident::from_span("hello", Span::new(0, 5, 1, 1)),
                Span::new(0, 21, 1, 1)
            )
            .param(LitStr::from_span("world", Span::new(6, 7, 1, 7)))
            .param(LitNum::from_span(1234, Span::new(15, 4, 1, 16))))
        );

        assert_eq!(
            CallExpr::parse(&mut Input::from("hello(goo)")),
            Err(MlError::CallExpr(
                Box::new(Kind::Char(')', Span::new(6, 1, 1, 7).recoverable()).into()),
                Span::new(0, 5, 1, 1).fatal()
            ))
        );
    }

    #[test]
    fn test_property() {
        assert_eq!(
            Property::parse(&mut Input::from("#[  ]")),
            Ok(Property::from_span(Span::new(0, 5, 1, 1)))
        );
        assert_eq!(
            Property::parse(&mut Input::from("#[\t\n]")),
            Ok(Property::from_span(Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            Property::parse(&mut Input::from("#[")),
            Err(MlError::Property(
                Box::new(Kind::Char(']', Span::new(2, 0, 1, 3).incomplete()).into()),
                Span::new(0, 2, 1, 1).fatal()
            ))
        );

        assert_eq!(
            Property::parse(&mut Input::from("")),
            Err(Kind::Keyword("#[".to_string(), Span::new(0, 0, 1, 1).incomplete()).into())
        );

        assert_eq!(
            Property::parse(&mut Input::from("#[hello('123',123),]")),
            Ok(Property::from_span(Span::new(0, 20, 1, 1)).param(
                CallExpr::from_span(
                    Ident::from_span("hello", Span::new(2, 5, 1, 3)),
                    Span::new(2, 16, 1, 3)
                )
                .param(LitStr::from_span("123", Span::new(8, 5, 1, 9)))
                .param(LitNum::from_span(123, Span::new(14, 3, 1, 15)))
            ))
        );

        assert_eq!(
            Property::parse(&mut Input::from("#[hello('123',\n123)]")),
            Ok(Property::from_span(Span::new(0, 20, 1, 1)).param(
                CallExpr::from_span(
                    Ident::from_span("hello", Span::new(2, 5, 1, 3)),
                    Span::new(2, 17, 1, 3)
                )
                .param(LitStr::from_span("123", Span::new(8, 5, 1, 9)))
                .param(LitNum::from_span(123, Span::new(15, 3, 2, 1)))
            ))
        );
    }

    #[test]
    fn test_comments() {
        assert_eq!(
            Comment::parse(&mut Input::from("///")),
            Ok(Comment("".to_owned(), Some(Span::new(0, 3, 1, 1))))
        );

        assert_eq!(
            Comment::parse(&mut Input::from("///\t\n")),
            Ok(Comment("\t".to_owned(), Some(Span::new(0, 4, 1, 1))))
        );

        assert_eq!(
            Comment::parse(&mut Input::from("///\thello world\n")),
            Ok(Comment(
                "\thello world".to_owned(),
                Some(Span::new(0, 15, 1, 1))
            ))
        );
    }

    #[test]
    fn test_ty() {
        assert_eq!(
            Type::parse(&mut Input::from("bool")),
            Ok(Type::Bool(Some(Span::new(0, 4, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("123")),
            Err(MlError::Type(
                Box::new(MlError::Ident(Span::new(0, 1, 1, 1).recoverable())),
                Span::new(0, 1, 1, 1).recoverable()
            ))
        );

        assert_eq!(
            Type::parse(&mut Input::from("")),
            Err(MlError::Type(
                Box::new(MlError::Ident(Span::new(0, 0, 1, 1).incomplete())),
                Span::new(0, 0, 1, 1).recoverable()
            ))
        );

        assert_eq!(
            Type::parse(&mut Input::from("string")),
            Ok(Type::String(Some(Span::new(0, 6, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("byte")),
            Ok(Type::Byte(Some(Span::new(0, 4, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("ubyte")),
            Ok(Type::Ubyte(Some(Span::new(0, 5, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("short")),
            Ok(Type::Short(Some(Span::new(0, 5, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("ushort")),
            Ok(Type::Ushort(Some(Span::new(0, 6, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("int")),
            Ok(Type::Int(Some(Span::new(0, 3, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("uint")),
            Ok(Type::Uint(Some(Span::new(0, 4, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("long")),
            Ok(Type::Long(Some(Span::new(0, 4, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("ulong")),
            Ok(Type::Ulong(Some(Span::new(0, 5, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("float")),
            Ok(Type::Float(Some(Span::new(0, 5, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("double")),
            Ok(Type::Double(Some(Span::new(0, 6, 1, 1))))
        );

        assert_eq!(
            Type::parse(&mut Input::from("_hello")),
            Ok(Type::Data(Ident::from_span(
                "_hello",
                Span::new(0, 6, 1, 1)
            )))
        );

        assert_eq!(
            Type::parse(&mut Input::from("vec[ bool\n ]")),
            Ok(Type::ListOf(
                Box::new(Type::Bool(Some(Span::new(5, 4, 1, 6)))),
                Some(Span::new(0, 12, 1, 1))
            ))
        );

        assert_eq!(
            Type::parse(&mut Input::from("vec")),
            Err(MlError::Type(
                Box::new(Kind::Char('[', Span::new(3, 0, 1, 4).incomplete()).into()),
                Span::new(0, 3, 1, 1).fatal()
            ))
        );

        assert_eq!(
            Type::parse(&mut Input::from("vec[")),
            Err(MlError::Type(
                Box::new(MlError::Type(
                    Box::new(MlError::Ident(Span::new(4, 0, 1, 5).incomplete())),
                    Span::new(4, 0, 1, 5).recoverable()
                )),
                Span::new(0, 3, 1, 1).fatal()
            ))
        );

        assert_eq!(
            Type::parse(&mut Input::from("[ bool ; 30 ]")),
            Ok(Type::ArrayOf(
                Box::new(Type::Bool(Some(Span::new(5, 4, 1, 6)))),
                LitNum::from_span(30, Span::new(9, 2, 1, 10)),
                Some(Span::new(0, 1, 1, 1))
            ))
        );
    }

    #[test]
    fn test_field() {
        assert_eq!(
            Field::parse(&mut Input::from("/// hello world\nhello: bool")),
            Ok(Field {
                comments: vec![Comment(
                    "hello world".to_string(),
                    Some(Span::new(0, 15, 1, 1))
                )],
                properties: vec![],
                ident: Some(Ident::from_span("hello", Span::new(16, 5, 2, 1))),
                ty: Type::Bool(Some(Span::new(23, 4, 2, 8)))
            })
        );

        assert_eq!(
            Field::parse(&mut Input::from("/// hello world\n#[hello]hello")),
            Ok(Field {
                comments: vec![Comment(
                    "hello world".to_string(),
                    Some(Span::new(0, 15, 1, 1))
                )],
                properties: vec![Property::from_span(Span::new(16, 8, 2, 1)).param(
                    CallExpr::from_span(
                        Ident::from_span("hello", Span::new(18, 5, 2, 3)),
                        Span::new(18, 5, 2, 3)
                    )
                )],
                ident: None,
                ty: Type::Data(Ident::from_span("hello", Span::new(24, 5, 2, 9)))
            })
        );
    }
}
