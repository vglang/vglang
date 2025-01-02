use parserc::{is_char, keyword, take_until, take_while, ParseSource, Parser, Source, Span};

use crate::opcode::{CallExpr, Comment, Ident, LitExpr, LitNum, LitStr, Property, Type};

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

        Ok(Self::from_span(source.to_str(span).to_string(), span))
    }
}

impl ParseSource for LitNum {
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

        Ok(Self::from_span(numeric, span))
    }
}

impl ParseSource for Comment {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        // comment start with ///
        let start = keyword("///").parse(source)?;

        if let Some(line) = take_until(|c| c == '\n').parse(source)? {
            Ok(Comment::from_span(
                source.to_str(line).trim().to_string(),
                source.extend_to_inclusive(start, line),
            ))
        } else {
            Ok(Comment::from_span("".to_string(), start))
        }
    }
}

impl ParseSource for Type {
    type Error = ParseError;
    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let ty = keyword("bool")
            .map(move |span| Type::Bool(Some(span)))
            .or(keyword("string").map(move |span| Type::String(Some(span))))
            .or(keyword("byte").map(move |span| Type::Byte(Some(span))))
            .or(keyword("ubyte").map(move |span| Type::Ubyte(Some(span))))
            .or(keyword("short").map(move |span| Type::Short(Some(span))))
            .or(keyword("ushort").map(move |span| Type::Ushort(Some(span))))
            .or(keyword("int").map(move |span| Type::Int(Some(span))))
            .or(keyword("uint").map(move |span| Type::Uint(Some(span))))
            .or(keyword("long").map(move |span| Type::Long(Some(span))))
            .or(keyword("ulong").map(move |span| Type::Ulong(Some(span))))
            .or(keyword("float").map(move |span| Type::Float(Some(span))))
            .or(keyword("double").map(move |span| Type::Double(Some(span))))
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
            return Ok(Type::Data(ident));
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

        let numeric = source.parse::<LitNum>()?;

        skip_whitespaces(source)?;

        is_char(']').parse(source)?;

        return Ok(Type::ArrayOf(Box::new(component), numeric));
    }
}

impl ParseSource for LitStr {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let start = source.span().ok_or(parserc::Error::Eof)?;

        let quote = is_char('"')
            .map(|_| '"')
            .or(is_char('\'').map(|_| '\''))
            .parse(source)?;

        let span = take_until(move |c| c == quote).parse(source)?;

        let end = is_char(quote).parse(source)?;

        if let Some(span) = span {
            Ok(LitStr::from_span(
                source.to_str(span).to_string(),
                source.extend_to_inclusive(start, end),
            ))
        } else {
            Ok(LitStr::from_span(
                "".to_string(),
                source.extend_to_inclusive(start, end),
            ))
        }
    }
}

impl ParseSource for LitExpr {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let numeric = |source: &mut Source<'_>| LitNum::parse(source).map(|v| LitExpr::Numeric(v));
        let str = |source: &mut Source<'_>| LitStr::parse(source).map(|v| LitExpr::String(v));

        numeric.or(str).parse(source)
    }
}

impl ParseSource for CallExpr {
    type Error = ParseError;
    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let ident = source.parse::<Ident>()?;

        let mut params = vec![];

        skip_whitespaces(source)?;

        if is_char('(').optional().parse(source)?.is_some() {
            skip_whitespaces(source)?;

            if let Some(param) = source.parse::<Option<LitExpr>>()? {
                params.push(param);
            }

            skip_whitespaces(source)?;

            while let Some(_) = is_char(',').optional().parse(source)? {
                skip_whitespaces(source)?;

                if let Some(ident) = source.parse::<Option<LitExpr>>()? {
                    params.push(ident);
                    skip_whitespaces(source)?;
                } else {
                    break;
                }
            }

            skip_whitespaces(source)?;

            is_char(')').parse(source)?;
        }

        Ok(CallExpr { ident, params })
    }
}

impl ParseSource for Property {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        is_char('#').parse(source)?;
        skip_whitespaces(source)?;
        is_char('[').parse(source)?;
        skip_whitespaces(source)?;

        let mut idents = vec![source.parse::<CallExpr>()?];

        skip_whitespaces(source)?;

        while let Some(_) = is_char(',').optional().parse(source)? {
            skip_whitespaces(source)?;

            if let Some(ident) = source.parse::<Option<CallExpr>>()? {
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
        opcode::{CallExpr, Comment, Ident, LitNum, LitStr, Property, Type},
        parser::ParseError,
    };

    #[test]
    fn test_ident() {
        let mut source = Source::from("hello#");

        let ident = Ident::parse(&mut source).unwrap();

        assert_eq!(
            ident,
            Ident::from_span(
                "hello",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 5
                }
            )
        );

        assert_eq!(
            Ident::parse(&mut Source::from("hello_")),
            Ok(Ident::from_span(
                "hello_",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 6
                }
            ))
        );

        assert_eq!(
            Ident::parse(&mut Source::from("_hello_")),
            Ok(Ident::from_span(
                "_hello_",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 7
                }
            ))
        );

        assert_eq!(
            Ident::parse(&mut Source::from("_hello1234_-")),
            Ok(Ident::from_span(
                "_hello1234_",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 11
                }
            ))
        );

        Ident::parse(&mut Source::from("#hello")).expect_err("start with #");
        Ident::parse(&mut Source::from("3hello")).expect_err("start with number");
    }

    #[test]
    fn test_numeric() {
        assert_eq!(
            LitNum::parse(&mut Source::from("1234#")).unwrap(),
            LitNum(
                1234,
                Some(Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 4
                })
            )
        );

        LitNum::parse(&mut Source::from("#123")).expect_err("start with #");
        LitNum::parse(&mut Source::from("h324")).expect_err("start with alphabetic");
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            Comment::parse(&mut Source::from("/// hello world  \n")),
            Ok(Comment::from_span(
                "hello world",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 17
                }
            ))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("/// hello world")),
            Ok(Comment::from_span(
                "hello world",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 15
                }
            ))
        );

        assert_eq!(
            Comment::parse(&mut Source::from("/// \thello world")),
            Ok(Comment::from_span(
                "hello world",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 16
                }
            ))
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
        assert_eq!(
            Type::parse(&mut Source::from("bool#")),
            Ok(Type::Bool(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 4
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("byte#")),
            Ok(Type::Byte(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 4
            })))
        );

        assert_eq!(
            Type::parse(&mut Source::from("ubyte#")),
            Ok(Type::Ubyte(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 5
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("short#")),
            Ok(Type::Short(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 5
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("ushort#")),
            Ok(Type::Ushort(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 6
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("int#")),
            Ok(Type::Int(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 3
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("uint#")),
            Ok(Type::Uint(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 4
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("long#")),
            Ok(Type::Long(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 4
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("ulong#")),
            Ok(Type::Ulong(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 5
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("float#")),
            Ok(Type::Float(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 5
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("double#")),
            Ok(Type::Double(Some(Span {
                lines: 1,
                cols: 1,
                offset: 0,
                len: 6
            })))
        );
        assert_eq!(
            Type::parse(&mut Source::from("hello_world#")),
            Ok(Type::Data(Ident::from_span(
                "hello_world",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 11
                }
            )))
        );

        assert_eq!(
            Type::parse(&mut Source::from("vec[bool]")),
            Ok(Type::ListOf(Box::new(Type::Bool(Some(Span {
                lines: 1,
                cols: 5,
                offset: 4,
                len: 4
            })))))
        );
        assert_eq!(
            Type::parse(&mut Source::from("vec[[float;3]]")),
            Ok(Type::ListOf(Box::new(Type::ArrayOf(
                Box::new(Type::Float(Some(Span {
                    lines: 1,
                    cols: 6,
                    offset: 5,
                    len: 5
                }))),
                LitNum::from_span(
                    3usize,
                    Span {
                        lines: 1,
                        cols: 12,
                        offset: 11,
                        len: 1
                    }
                )
            ))))
        );

        assert_eq!(
            Type::parse(&mut Source::from("vec[  [\n_hello ;\t3]\r\n]")),
            Ok(Type::ListOf(Box::new(Type::ArrayOf(
                Box::new(Type::Data(Ident::from_span(
                    "_hello",
                    Span {
                        lines: 2,
                        cols: 1,
                        offset: 8,
                        len: 6
                    }
                ))),
                LitNum::from_span(
                    3usize,
                    Span {
                        lines: 2,
                        cols: 10,
                        offset: 17,
                        len: 1
                    }
                )
            ))))
        );
    }

    #[test]
    fn test_property() {
        assert_eq!(
            Property::parse(&mut Source::from("#[ hello, world\t]")),
            Ok(Property::from([
                Ident::from_span(
                    "hello",
                    Span {
                        lines: 1,
                        cols: 4,
                        offset: 3,
                        len: 5
                    }
                ),
                Ident::from_span(
                    "world",
                    Span {
                        lines: 1,
                        cols: 11,
                        offset: 10,
                        len: 5
                    }
                ),
            ]))
        );

        assert_eq!(
            Property::parse(&mut Source::from(
                "#[ hello('hello'), world(1,     \"hello\" )\t]"
            )),
            Ok(Property::from([
                CallExpr::from(Ident::from_span(
                    "hello",
                    Span {
                        lines: 1,
                        cols: 4,
                        offset: 3,
                        len: 5
                    }
                ),)
                .param(LitStr::from_span(
                    "hello",
                    Span {
                        lines: 1,
                        cols: 10,
                        offset: 9,
                        len: 7
                    }
                )),
                CallExpr::from(Ident::from_span(
                    "world",
                    Span {
                        lines: 1,
                        cols: 20,
                        offset: 19,
                        len: 5
                    }
                ),)
                .param(LitNum::from_span(
                    1usize,
                    Span {
                        lines: 1,
                        cols: 26,
                        offset: 25,
                        len: 1
                    }
                ))
                .param(LitStr::from_span(
                    "hello",
                    Span {
                        lines: 1,
                        cols: 33,
                        offset: 32,
                        len: 7
                    }
                )),
            ]))
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
