use parserc::{
    is_char, keyword, take_until, take_while, IntoParser, ParseSource, Parser, Source, Span,
};

use crate::opcode::{
    CallExpr, Comment, Enum, Field, Ident, LitExpr, LitNum, LitStr, Mixin, Node, Opcode, Property,
    Type,
};

static MAX_COMMENTS: usize = 1000;
static MAX_PROPERTIES: usize = 200;
static MAX_FIELDS: usize = 200;
static MAX_NODES: usize = 10000;

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

    #[error("Unexpect keyword: {0}. {1:?}")]
    UnexpectKeyWord(String, Span),

    #[error("Unexpect ident: {0}. {1:?}")]
    UnexpectIdent(String, Span),

    #[error("Expect keyword `el`,`leaf`,`data`,.etc {0:?}")]
    ExpectKeyWord(Span),
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
        // skip leading blanks.
        skip_whitespaces(source)?;
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
        // skip leading blanks.
        skip_whitespaces(source)?;
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

impl ParseSource for Field {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let mut comments = vec![];

        // parse comments.
        for comment in Comment::into_parser().repeat(..MAX_COMMENTS) {
            if let Some(comment) = comment.optional().parse(source)? {
                let comment = comment.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                comments.push(comment);
            } else {
                break;
            }
        }

        let mut properties = vec![];

        // parse properties.
        for property in Property::into_parser().repeat(..MAX_PROPERTIES) {
            if let Some(property) = property.optional().parse(source)? {
                let property = property.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                properties.push(property);
            } else {
                break;
            }
        }

        skip_whitespaces(source)?;

        let ident = if let Some(ident) = source.parse::<Option<Ident>>()? {
            skip_whitespaces(source)?;

            if let Some(_) = is_char(':').optional().parse(source)? {
                Some(ident)
            } else {
                source.seek(ident.1.unwrap())?;
                None
            }
        } else {
            None
        };

        skip_whitespaces(source)?;

        let ty = source.parse::<Type>()?;

        skip_whitespaces(source)?;

        Ok(Self {
            comments,
            properties,
            ident,
            ty,
        })
    }
}

impl ParseSource for Mixin {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let mut comments = vec![];

        // parse comments.
        for comment in Comment::into_parser().repeat(..MAX_COMMENTS) {
            if let Some(comment) = comment.optional().parse(source)? {
                let comment = comment.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                comments.push(comment);
            } else {
                break;
            }
        }

        let mut properties = vec![];

        // parse properties.
        for property in Property::into_parser().repeat(..MAX_PROPERTIES) {
            if let Some(property) = property.optional().parse(source)? {
                let property = property.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                properties.push(property);
            } else {
                break;
            }
        }

        skip_whitespaces(source)?;

        keyword("mixin").parse(source)?;

        skip_whitespaces(source)?;

        let ident = source.parse::<Ident>()?;

        skip_whitespaces(source)?;

        keyword("{").parse(source)?;

        let mut fields = vec![];

        // parse fields.
        for field in Field::into_parser().repeat(..MAX_FIELDS) {
            if let Some(field) = field.optional().parse(source)? {
                let field = field.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                fields.push(field);
            } else {
                break;
            }
        }

        keyword("}").parse(source)?;

        Ok(Self {
            comments,
            properties,
            fields,
            ident,
        })
    }
}

impl ParseSource for Enum {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        let mut comments = vec![];

        // parse comments.
        for comment in Comment::into_parser().repeat(..MAX_COMMENTS) {
            if let Some(comment) = comment.optional().parse(source)? {
                let comment = comment.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                comments.push(comment);
            } else {
                break;
            }
        }

        let mut properties = vec![];

        // parse properties.
        for property in Property::into_parser().repeat(..MAX_PROPERTIES) {
            if let Some(property) = property.optional().parse(source)? {
                let property = property.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
                properties.push(property);
            } else {
                break;
            }
        }

        skip_whitespaces(source)?;

        keyword("enum").parse(source)?;

        skip_whitespaces(source)?;

        let ident = source.parse::<Ident>()?;

        skip_whitespaces(source)?;

        is_char('{').parse(source)?;

        let mut fields = vec![];

        // parse fields.
        for field in parse_node.repeat(..MAX_FIELDS) {
            if let Some(Some((field, node_type))) = field.optional().parse(source)? {
                if let Some(node_type) = node_type {
                    return Err(ParseError::UnexpectKeyWord(
                        source.to_str(node_type).to_string(),
                        node_type,
                    ));
                }
                fields.push(field);

                if is_char(',').optional().parse(source)?.is_none() {
                    break;
                }
            } else {
                break;
            }
        }

        skip_whitespaces(source)?;

        is_char('}').parse(source)?;

        Ok(Self {
            comments,
            properties,
            fields,
            ident,
        })
    }
}

/// Parse element node.
pub fn parse_node(source: &mut Source<'_>) -> Result<(Node, Option<Span>), ParseError> {
    let mut comments = vec![];

    // parse comments.
    for comment in Comment::into_parser().repeat(..MAX_COMMENTS) {
        if let Some(comment) = comment.optional().parse(source)? {
            let comment = comment.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
            comments.push(comment);
        } else {
            break;
        }
    }

    let mut properties = vec![];

    // parse properties.
    for property in Property::into_parser().repeat(..MAX_PROPERTIES) {
        if let Some(property) = property.optional().parse(source)? {
            let property = property.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
            properties.push(property);
        } else {
            break;
        }
    }

    skip_whitespaces(source)?;

    let node_type = keyword("el")
        .or(keyword("leaf"))
        .or(keyword("attr"))
        .or(keyword("data"))
        .optional()
        .parse(source)?;

    skip_whitespaces(source)?;

    let ident = source.parse::<Ident>()?;

    skip_whitespaces(source)?;

    let mixin = if let Some(_) = keyword("mixin").optional().parse(source)? {
        skip_whitespaces(source)?;
        let ident = source.parse::<Ident>()?;
        skip_whitespaces(source)?;
        Some(ident)
    } else {
        None
    };

    skip_whitespaces(source)?;

    let is_tuple = is_char('{')
        .map(|_| false)
        .or(is_char('(').map(|_| true))
        .optional()
        .parse(source)?;

    let mut fields = vec![];

    if let Some(is_tuple) = is_tuple {
        // parse fields.
        for field in Field::into_parser().repeat(..MAX_FIELDS) {
            let field = field.parse(source)?;

            let field = field.ok_or(ParseError::Parserc(parserc::Error::Eof))?;
            if is_tuple {
                if let Some(ident) = field.ident {
                    return Err(ParseError::UnexpectIdent(
                        ident.0,
                        ident.1.unwrap_or(Span::default()),
                    ));
                }
            }
            fields.push(field);

            is_char(',').optional().parse(source)?;

            skip_whitespaces(source)?;

            if is_char(if is_tuple { ')' } else { '}' })
                .optional()
                .parse(source)?
                .is_some()
            {
                break;
            }
        }
    }

    let node = Node {
        comments,
        mixin,
        ident,
        properties,
        fields,
    };

    Ok((node, node_type))
}

impl ParseSource for Opcode {
    type Error = ParseError;

    fn parse(source: &mut Source<'_>) -> std::result::Result<Self, Self::Error> {
        if let Some(opcode) = Mixin::into_parser()
            .map(|v| Opcode::Mixin(Box::new(v)))
            .or(Enum::into_parser().map(|v| Opcode::Enum(Box::new(v))))
            .optional()
            .parse(source)?
        {
            skip_whitespaces(source)?;
            return Ok(opcode);
        }

        let (node, span) = parse_node.parse(source)?;

        if let Some(span) = span {
            // tuple or empty field.
            if node.fields.is_empty() || node.fields.first().unwrap().ident.is_none() {
                is_char(';').parse(source)?;
            }

            skip_whitespaces(source)?;

            match source.to_str(span) {
                "el" => return Ok(Opcode::Element(Box::new(node))),
                "leaf" => return Ok(Opcode::Leaf(Box::new(node))),
                "attr" => return Ok(Opcode::Attr(Box::new(node))),
                "data" => return Ok(Opcode::Data(Box::new(node))),
                _ => {
                    panic!("inner error.")
                }
            }
        }

        return Err(ParseError::ExpectKeyWord(node.ident.1.unwrap()));
    }
}

/// Parse a mlang source code.
pub fn parse<'a, S>(source: S) -> Result<Vec<Opcode>, ParseError>
where
    Source<'a>: From<S>,
{
    let mut source = source.into();

    let mut opcodes = vec![];

    for parser in Opcode::into_parser().repeat(..MAX_NODES) {
        if let Some(opcode) = parser.parse(&mut source)? {
            opcodes.push(opcode);
        } else {
            break;
        }
    }

    return Ok(opcodes);
}

#[cfg(test)]
mod tests {
    use parserc::{ParseSource, Parser, Source, Span};

    use crate::{
        opcode::{CallExpr, Comment, Enum, Ident, LitNum, LitStr, Property, Type},
        parser::ParseError,
    };

    use super::parse_node;

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
                    cols: 2,
                    offset: 1,
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

    #[test]
    fn test_node() {
        let mut source = Source::from(include_str!("./tests/node.ml"));
        for (index, parser) in parse_node.repeat(..3).enumerate() {
            let (node, span) = parser.parse(&mut source).unwrap().unwrap();

            assert_eq!(node.ident.0, "TextPath");
            assert_eq!(source.to_str(span.unwrap()), "leaf");

            if index == 1 {
                assert_eq!(node.mixin.unwrap().0, "Hello");
                assert_eq!(node.properties.len(), 1);
            } else {
                assert_eq!(node.mixin, None);
            }
        }
    }

    #[test]
    fn test_enum() {
        let mut source = Source::from(include_str!("./tests/enum.ml"));

        source.parse::<Enum>().unwrap();
    }
}
