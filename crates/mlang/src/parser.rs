use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, ControlFlow, FromSrc, IntoParser, Kind,
    ParseContext, Parser, ParserExt, Result, Span,
};

use crate::opcode::{
    ApplyTo, CallExpr, ChildrenOf, Comment, Enum, Fields, Group, Ident, LitExpr, LitNum, LitStr,
    NamedField, Node, Opcode, Property, Type, UnnamedField,
};

/// Error type returns by `mlang` parser combinators.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd, Clone)]
pub enum MlParseError {
    #[error(transparent)]
    Parserc(#[from] Kind),

    #[error("expect ident")]
    Ident,

    #[error("expect literal numeric")]
    LitNum,

    #[error("hex number body is empty")]
    LitNumHexBody,

    #[error("expect literal string")]
    LitStr,

    #[error("Invalid call expression")]
    CallExpr,

    #[error("Expect `:=` operator.")]
    GroupAssign,

    #[error("Syntax error: expect `(...)`")]
    TupleExpr,

    #[error("Syntax error: expect `apply...to...`")]
    ApplyExpr,

    #[error("Syntax error: expect `children...of...`")]
    ChildrenOf,

    #[error("Expect `:<-` operator.")]
    ApplyAssign,

    #[error("Invalid property expression")]
    Property,

    #[error("Invalid field")]
    Field,

    #[error("Invalid type")]
    Type,

    #[error("Parse node error")]
    Node,

    #[error("expect node keyword")]
    NodeKeyWord,

    #[error("Invalid named field syntax.")]
    NamedField,

    #[error("Invalid unnamed field syntax.")]
    UnnamedField,

    #[error("Parse enum error")]
    Enum,

    #[error("Unexpect key word `{0}`")]
    UnexpectKeyWord(String),

    #[error("Unknown ident `{0}`")]
    UnknownSymbol(String),
}

fn skip_ws(input: &mut ParseContext<'_>) -> Result<Option<Span>> {
    let span = take_while(|c| c.is_whitespace()).parse(input)?;

    Ok(span)
}

enum Prefix {
    Property(Property),
    Comment(Comment),
}

fn parse_prefix(input: &mut ParseContext<'_>) -> Result<(Vec<Comment>, Vec<Property>)> {
    let mut properties = vec![];
    let mut comments = vec![];

    skip_ws(input)?;

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

fn parse_node(parse_enum_field: bool) -> impl Parser<Output = (Option<Span>, Node)> + Clone {
    move |input: &mut ParseContext<'_>| parse_node_inner(parse_enum_field, input)
}

fn parse_node_inner(
    parse_enum_field: bool,
    input: &mut ParseContext<'_>,
) -> Result<(Option<Span>, Node)> {
    let (comments, properties) = parse_prefix(input)?;

    skip_ws(input)?;

    let mut start = input.span();

    let keyword = ensure_keyword("el")
        .or(ensure_keyword("leaf"))
        .or(ensure_keyword("attr"))
        .or(ensure_keyword("data"))
        .or(ensure_keyword("mixin"))
        .ok()
        .parse(input)?;

    if let Some(kw) = keyword {
        start = kw;
    } else {
        if !parse_enum_field {
            input.report_error(MlParseError::NodeKeyWord, start);
            return Err(ControlFlow::Recoverable);
        }
    }

    skip_ws
        .with_context(MlParseError::Node, start)
        .fatal()
        .parse(input)?;

    let ident = if parse_enum_field {
        Ident::into_parser().parse(input)?
    } else {
        Ident::into_parser()
            .with_context(MlParseError::Node, start)
            .fatal()
            .parse(input)?
    };

    skip_ws
        .with_context(MlParseError::Node, start)
        .fatal()
        .parse(input)?;

    let mixin = if let Some(span) = ensure_keyword("mixin")
        .ok()
        .with_context(MlParseError::Node, start)
        .fatal()
        .parse(input)?
    {
        skip_ws
            .with_context(MlParseError::Node, span)
            .fatal()
            .parse(input)?;

        let mixin = Ident::into_parser()
            .with_context(MlParseError::Node, span)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::Node, span)
            .fatal()
            .parse(input)?;

        Some(mixin)
    } else {
        None
    };

    let fields = Fields::into_parser()
        .with_context(MlParseError::Node, start)
        .parse(input)?;

    if !parse_enum_field && fields.is_tuple() {
        ensure_char(';')
            .with_context(MlParseError::Node, start)
            .fatal()
            .fatal()
            .parse(input)?;
    }

    Ok((
        keyword,
        Node {
            comments,
            mixin,
            properties,
            ident,
            fields,
        },
    ))
}

fn parse_tuple_expr(input: &mut ParseContext<'_>) -> Result<(Vec<Ident>, Span)> {
    let start = ensure_char('(')
        .with_context(MlParseError::TupleExpr, input.span())
        .parse(input)?;

    skip_ws
        .with_context(MlParseError::TupleExpr, start)
        .fatal()
        .parse(input)?;

    let mut children = vec![];

    while let Some(child) = Ident::into_parser()
        .ok()
        .with_context(MlParseError::TupleExpr, start)
        .fatal()
        .parse(input)?
    {
        children.push(child);

        skip_ws
            .with_context(MlParseError::TupleExpr, start)
            .fatal()
            .parse(input)?;

        if ensure_char(',')
            .ok()
            .with_context(MlParseError::TupleExpr, start)
            .fatal()
            .parse(input)?
            .is_none()
        {
            break;
        }

        skip_ws
            .with_context(MlParseError::TupleExpr, start)
            .fatal()
            .parse(input)?;
    }

    let end = ensure_char(')')
        .with_context(MlParseError::TupleExpr, start)
        .fatal()
        .parse(input)?;

    Ok((children, start.extend_to_inclusive(end)))
}

impl FromSrc for Ident {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (c, start) = input.next();

        if let Some(c) = c {
            if c != '_' && !c.is_alphabetic() {
                input.report_error(MlParseError::Ident, start);
                return Err(ControlFlow::Recoverable);
            }
        } else {
            input.report_error(MlParseError::Ident, start);
            return Err(ControlFlow::Incomplete);
        }

        let body = take_while(|c| c == '_' || c.is_alphanumeric()).parse(input)?;

        let span = if let Some(body) = body {
            start.extend_to_inclusive(body)
        } else {
            start
        };

        assert!(span.len() > 0);

        let ident = input.as_str(span);

        match ident {
            "el" | "leaf" | "attr" | "group" | "mixin" | "data" | "enum" | "apply" | "to"
            | "of" | "string" | "bool" | "byte" | "ubyte" | "short" | "ushort" | "int" | "uint"
            | "long" | "ulong" | "float" | "double" | "vec" => {
                input.report_error(MlParseError::UnexpectKeyWord(ident.to_string()), start);
                return Err(ControlFlow::Fatal);
            }
            _ => {}
        }

        assert_eq!(ident.len(), span.len());

        Ok(Ident(ident.to_string(), span))
    }
}

impl FromSrc for LitNum {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(start) = ensure_keyword("0x").ok().parse(input)? {
            let body = take_while(|c| c.is_ascii_hexdigit())
                .parse(input)?
                .ok_or_else(|| {
                    input.report_error(MlParseError::LitNumHexBody, start);
                    ControlFlow::Fatal
                })?;

            assert!(body.len() > 0);

            let numeric = input.as_str(body);

            assert_eq!(numeric.len(), body.len());

            let numeric = usize::from_str_radix(numeric, 16).unwrap();

            return Ok(Self(numeric, start.extend_to_inclusive(body)));
        }

        let (c, start) = input.next();

        if let Some(c) = c {
            if !c.is_ascii_digit() {
                input.report_error(MlParseError::LitNum, start);
                return Err(ControlFlow::Recoverable);
            }
        } else {
            input.report_error(MlParseError::LitNum, start);
            return Err(ControlFlow::Incomplete);
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

        Ok(Self(numeric, span))
    }
}

impl FromSrc for LitStr {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
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
            Ok(Self(input.as_str(body).to_string(), span))
        } else {
            Ok(Self("".to_string(), span))
        }
    }
}

impl FromSrc for LitExpr {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        LitStr::into_parser()
            .map(|v| LitExpr::from(v))
            .or(LitNum::into_parser().map(|v| LitExpr::from(v)))
            .parse(input)
    }
}

impl FromSrc for CallExpr {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let ident = Ident::parse(input)?;

        let start = ident.1;

        skip_ws(input)?;

        if ensure_char('(').ok().parse(input)?.is_none() {
            return Ok(CallExpr {
                span: ident.1,
                ident,
                params: vec![],
            });
        }

        skip_ws(input)?;

        let mut params = vec![];

        while let Some(expr) = LitExpr::into_parser()
            .ok()
            .with_context(MlParseError::CallExpr, start)
            .fatal()
            .parse(input)?
        {
            params.push(expr);

            skip_ws(input)?;

            if ensure_char(',')
                .ok()
                .with_context(MlParseError::CallExpr, start)
                .fatal()
                .parse(input)?
                .is_none()
            {
                skip_ws(input)?;
                break;
            }

            skip_ws(input)?;
        }

        let end = ensure_char(')')
            .with_context(MlParseError::CallExpr, start)
            .fatal()
            .parse(input)?;

        let span = start.extend_to_inclusive(end);

        Ok(Self {
            span,
            ident,
            params,
        })
    }
}

impl FromSrc for Property {
    fn parse(input: &mut parserc::ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_keyword("#[").parse(input)?;

        skip_ws(input)?;

        let mut params = vec![];

        while let Some(expr) = CallExpr::into_parser()
            .ok()
            .with_context(MlParseError::Property, start)
            .fatal()
            .parse(input)?
        {
            params.push(expr);

            skip_ws(input)?;

            if ensure_char(',')
                .ok()
                .with_context(MlParseError::Property, start)
                .fatal()
                .parse(input)?
                .is_none()
            {
                skip_ws(input)?;
                break;
            }

            skip_ws(input)?;
        }

        let end = ensure_char(']')
            .with_context(MlParseError::Property, start)
            .fatal()
            .parse(input)?;

        let span = start.extend_to_inclusive(end);

        Ok(Self { span, params })
    }
}

impl FromSrc for Comment {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
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

        Ok(Comment(content.to_string(), span))
    }
}

impl FromSrc for Type {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let ty = ensure_keyword("bool")
            .map(|span| Type::Bool(span))
            .or(ensure_keyword("string").map(|span| Type::String(span)))
            .or(ensure_keyword("byte").map(|span| Type::Byte(span)))
            .or(ensure_keyword("ubyte").map(|span| Type::Ubyte(span)))
            .or(ensure_keyword("short").map(|span| Type::Short(span)))
            .or(ensure_keyword("ushort").map(|span| Type::Ushort(span)))
            .or(ensure_keyword("int").map(|span| Type::Int(span)))
            .or(ensure_keyword("uint").map(|span| Type::Uint(span)))
            .or(ensure_keyword("long").map(|span| Type::Long(span)))
            .or(ensure_keyword("ulong").map(|span| Type::Ulong(span)))
            .or(ensure_keyword("float").map(|span| Type::Float(span)))
            .or(ensure_keyword("double").map(|span| Type::Double(span)))
            .ok()
            .parse(input)?;

        if let Some(ty) = ty {
            return Ok(ty);
        }

        if let Some(start) = ensure_keyword("vec").ok().parse(input)? {
            skip_ws(input)?;

            ensure_char('[')
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            skip_ws(input)?;

            let component = Type::into_parser()
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            return Ok(Type::ListOf(
                Box::new(component),
                start.extend_to_inclusive(end),
            ));
        }

        if let Some(start) = ensure_char('[').ok().parse(input)? {
            skip_ws(input)?;

            let component = Type::into_parser()
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            skip_ws(input)?;

            ensure_char(';')
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            skip_ws(input)?;

            let len = LitNum::into_parser()
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .with_context(MlParseError::Type, start)
                .fatal()
                .parse(input)?;

            return Ok(Type::ArrayOf(
                Box::new(component),
                len,
                start.extend_to_inclusive(end),
            ));
        }

        let start = input.span();

        // try parse as ident at last.
        Ident::into_parser()
            .with_context(MlParseError::Type, start)
            .parse(input)
            .map(|ident| Type::Data(ident))
    }
}

impl FromSrc for NamedField {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = input.span();

        let (comments, properties) = parse_prefix(input)?;

        skip_ws
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        let ident = Ident::into_parser()
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        ensure_char(':')
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        let ty = Type::into_parser()
            .with_context(MlParseError::NamedField, start)
            .fatal()
            .parse(input)?;

        Ok(NamedField {
            comments,
            properties,
            ident,
            ty,
        })
    }
}

impl FromSrc for UnnamedField {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = input.span();

        let (comments, properties) = parse_prefix(input)?;

        skip_ws
            .with_context(MlParseError::UnnamedField, start)
            .fatal()
            .parse(input)?;

        let ty = Type::into_parser()
            .with_context(MlParseError::UnnamedField, start)
            .parse(input)?;

        Ok(UnnamedField {
            comments,
            properties,
            ty,
        })
    }
}

impl FromSrc for Fields {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_char('{')
            .map(|span| (span, false))
            .or(ensure_char('(').map(|span| (span, true)))
            .ok()
            .parse(input)?;

        let (start, is_tuple) = match start {
            Some(v) => v,
            _ => return Ok(Fields::None),
        };

        skip_ws
            .with_context(MlParseError::Field, start)
            .fatal()
            .parse(input)?;

        if is_tuple {
            let mut fields = vec![];

            while let Some(field) = UnnamedField::into_parser().catch_fatal().parse(input)? {
                fields.push(field);

                skip_ws
                    .with_context(MlParseError::Field, start)
                    .fatal()
                    .parse(input)?;

                if ensure_char(',').ok().parse(input)?.is_none() {
                    break;
                }
            }

            skip_ws
                .with_context(MlParseError::Field, start)
                .fatal()
                .parse(input)?;

            ensure_char(')')
                .with_context(MlParseError::Field, start)
                .fatal()
                .parse(input)?;

            if fields.is_empty() {
                return Ok(Fields::None);
            } else {
                return Ok(Fields::Unnamed(fields));
            }
        } else {
            let mut fields = vec![];

            while let Some(field) = NamedField::into_parser().catch_fatal().parse(input)? {
                fields.push(field);

                skip_ws
                    .with_context(MlParseError::Field, start)
                    .fatal()
                    .parse(input)?;

                if ensure_char(',').ok().parse(input)?.is_none() {
                    break;
                }
            }

            skip_ws
                .with_context(MlParseError::Field, start)
                .fatal()
                .parse(input)?;

            ensure_char('}')
                .with_context(MlParseError::Field, start)
                .fatal()
                .parse(input)?;

            if fields.is_empty() {
                return Ok(Fields::None);
            } else {
                return Ok(Fields::Named(fields));
            }
        }
    }
}

impl FromSrc for Enum {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (comments, properties) = parse_prefix(input)?;

        skip_ws(input)?;

        let keyword = ensure_keyword("enum").parse(input)?;

        skip_ws
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        let ident = Ident::into_parser()
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        ensure_char('{')
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        let mut fields = vec![];

        while let Some((kw, field)) = parse_node(true)
            .ok()
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?
        {
            if let Some(kw) = kw {
                input.report_error(
                    MlParseError::UnexpectKeyWord(input.as_str(kw).to_string()),
                    kw,
                );
                return Err(ControlFlow::Fatal);
            }

            fields.push(field);

            skip_ws
                .with_context(MlParseError::Enum, keyword)
                .fatal()
                .parse(input)?;

            if ensure_char(',')
                .ok()
                .with_context(MlParseError::Enum, keyword)
                .fatal()
                .parse(input)?
                .is_none()
            {
                break;
            }

            skip_ws
                .with_context(MlParseError::Enum, keyword)
                .fatal()
                .parse(input)?;
        }

        ensure_char('}')
            .with_context(MlParseError::Enum, keyword)
            .fatal()
            .parse(input)?;

        Ok(Enum {
            comments,
            properties,
            ident,
            fields,
        })
    }
}

impl FromSrc for Group {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (comments, properties) = parse_prefix(input)?;

        skip_ws(input)?;

        let start = ensure_keyword("group").parse(input)?;

        skip_ws
            .with_context(MlParseError::GroupAssign, start)
            .fatal()
            .parse(input)?;

        let ident = Ident::into_parser()
            .with_context(MlParseError::GroupAssign, start)
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::GroupAssign, start)
            .fatal()
            .parse(input)?;

        ensure_keyword(":=")
            .with_context(MlParseError::GroupAssign, ident.1)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::GroupAssign, start)
            .fatal()
            .parse(input)?;

        let (children, _) = parse_tuple_expr
            .with_context(MlParseError::GroupAssign, start)
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::TupleExpr, start)
            .fatal()
            .parse(input)?;

        let end = ensure_char(';')
            .with_context(MlParseError::TupleExpr, start)
            .fatal()
            .parse(input)?;

        Ok(Self {
            comments,
            properties,
            span: start.extend_to_inclusive(end),
            ident,
            children,
        })
    }
}

impl FromSrc for ApplyTo {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (comments, properties) = parse_prefix(input)?;

        skip_ws(input)?;

        let start = ensure_keyword("apply").parse(input)?;

        skip_ws
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        let from = Ident::into_parser()
            .map(|v| vec![v])
            .or(parse_tuple_expr.map(|(v, _)| v))
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        ensure_keyword("to")
            .with_context(MlParseError::ApplyExpr, input.span())
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        let (to, _) = Ident::into_parser()
            .map(|v| {
                let span = v.1;
                (vec![v], span)
            })
            .or(parse_tuple_expr)
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        let end = ensure_char(';')
            .with_context(MlParseError::ApplyExpr, start)
            .fatal()
            .parse(input)?;

        Ok(Self {
            properties,
            comments,
            span: start.extend_to_inclusive(end),
            from,
            to,
        })
    }
}

impl FromSrc for ChildrenOf {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (comments, properties) = parse_prefix(input)?;

        skip_ws(input)?;

        let start = ensure_keyword("children").parse(input)?;

        skip_ws
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        let from = Ident::into_parser()
            .map(|v| vec![v])
            .or(parse_tuple_expr.map(|(v, _)| v))
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        ensure_keyword("of")
            .with_context(MlParseError::ChildrenOf, input.span())
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        let (to, _) = Ident::into_parser()
            .map(|v| {
                let span = v.1;
                (vec![v], span)
            })
            .or(parse_tuple_expr)
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        skip_ws
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        let end = ensure_char(';')
            .with_context(MlParseError::ChildrenOf, start)
            .fatal()
            .parse(input)?;

        Ok(Self {
            properties,
            comments,
            span: start.extend_to_inclusive(end),
            from,
            to,
        })
    }
}

impl FromSrc for Opcode {
    fn parse(input: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(opcode) = Enum::into_parser()
            .map(|v| Opcode::Enum(Box::new(v)))
            .ok()
            .parse(input)?
        {
            return Ok(opcode);
        }

        if let Some((keyword, node)) = parse_node(false).ok().parse(input)? {
            match keyword.map(|kw| input.as_str(kw)) {
                Some("el") => return Ok(Opcode::Element(Box::new(node))),
                Some("leaf") => return Ok(Opcode::Leaf(Box::new(node))),
                Some("attr") => return Ok(Opcode::Attr(Box::new(node))),
                Some("mixin") => return Ok(Opcode::Mixin(Box::new(node))),
                Some("data") => return Ok(Opcode::Data(Box::new(node))),
                _ => {}
            }
        }

        if let Some(group) = Group::into_parser().ok().parse(input)? {
            return Ok(Opcode::Group(Box::new(group)));
        }

        if let Some(apply_to) = ApplyTo::into_parser().ok().parse(input)? {
            return Ok(Opcode::ApplyTo(Box::new(apply_to)));
        }

        if let Some(children_of) = ChildrenOf::into_parser().ok().parse(input)? {
            return Ok(Opcode::ChildrenOf(Box::new(children_of)));
        }

        skip_ws(input)?;

        assert_eq!(
            input.remaining(),
            0,
            "inner error: unparsed length must be zero."
        );

        return Err(ControlFlow::Incomplete);
    }
}

/// Parse input source code.
pub fn parse(input: &mut ParseContext<'_>) -> Result<Vec<Opcode>> {
    let mut opcodes = vec![];

    while let Some(opcode) = Opcode::into_parser().ok().parse(input)? {
        opcodes.push(opcode);
    }

    Ok(opcodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        Group::into_parser()
            .expect(Group {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 19, 1, 1),
                ident: Ident::from_span("hello", Span::new(6, 5, 1, 7)),
                children: vec![],
            })
            .parse(&mut ParseContext::from("group hello := () ;"))
            .unwrap();

        Group::into_parser()
            .expect(Group {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 31, 1, 1),
                ident: Ident::from_span("hello", Span::new(6, 5, 1, 7)),
                children: vec![
                    Ident::from_span("word", Span::new(17, 4, 1, 18)),
                    Ident::from_span("large", Span::new(23, 5, 1, 24)),
                ],
            })
            .parse(&mut ParseContext::from("group hello := ( word, large) ;"))
            .unwrap();
    }

    #[test]
    fn test_group2() {
        Group::into_parser()
            .ok()
            .expect_err()
            .parse(&mut ParseContext::from(
                "group Shape(Rect,Circle,Ellipse,Line,Polyline,Polygon);",
            ))
            .unwrap();
    }

    #[test]
    fn test_apply_to() {
        ApplyTo::into_parser()
            .expect(ApplyTo {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 21, 1, 1),
                from: vec![Ident::from_span("hello", Span::new(6, 5, 1, 7))],
                to: vec![Ident::from_span("world", Span::new(15, 5, 1, 16))],
            })
            .parse(&mut ParseContext::from("apply hello to world;"))
            .unwrap();

        ApplyTo::into_parser()
            .expect(ApplyTo {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 38, 1, 1),
                from: vec![
                    Ident::from_span("hello", Span::new(7, 5, 1, 8)),
                    Ident::from_span("world", Span::new(13, 5, 1, 14)),
                ],
                to: vec![
                    Ident::from_span("hello", Span::new(24, 5, 1, 25)),
                    Ident::from_span("world", Span::new(30, 5, 1, 31)),
                ],
            })
            .parse(&mut ParseContext::from(
                "apply (hello,world) to (hello,world,);",
            ))
            .unwrap();
    }

    #[test]
    fn test_childrenof() {
        ChildrenOf::into_parser()
            .expect(ChildrenOf {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 24, 1, 1),
                from: vec![Ident::from_span("hello", Span::new(9, 5, 1, 10))],
                to: vec![Ident::from_span("world", Span::new(18, 5, 1, 19))],
            })
            .parse(&mut ParseContext::from("children hello of world;"))
            .unwrap();

        ChildrenOf::into_parser()
            .expect(ChildrenOf {
                properties: vec![],
                comments: vec![],
                span: Span::new(0, 41, 1, 1),
                from: vec![
                    Ident::from_span("hello", Span::new(10, 5, 1, 11)),
                    Ident::from_span("world", Span::new(16, 5, 1, 17)),
                ],
                to: vec![
                    Ident::from_span("hello", Span::new(27, 5, 1, 28)),
                    Ident::from_span("world", Span::new(33, 5, 1, 34)),
                ],
            })
            .parse(&mut ParseContext::from(
                "children (hello,world) of (hello,world,);",
            ))
            .unwrap();
    }
}

//     #[test]
//     fn test_ident() {
//         assert_eq!(
//             Ident::parse(&mut ParseContext::from("hello")),
//             Ok(Ident::from_span("hello", Span::new(0, 5, 1, 1)))
//         );

//         assert_eq!(
//             Ident::parse(&mut ParseContext::from("_hello")),
//             Ok(Ident::from_span("_hello", Span::new(0, 6, 1, 1)))
//         );

//         assert_eq!(
//             Ident::parse(&mut ParseContext::from("hello123#")),
//             Ok(Ident::from_span("hello123", Span::new(0, 8, 1, 1)))
//         );

//         assert_eq!(
//             Ident::parse(&mut ParseContext::from("123hello123#")),
//             Err(MlError::Ident(Span::new(0, 1, 1, 1).recoverable()))
//         );

//         assert_eq!(
//             Ident::parse(&mut ParseContext::from("")),
//             Err(MlError::Ident(Span::new(0, 0, 1, 1).incomplete()))
//         );
//     }

//     #[test]
//     fn test_litstr() {
//         assert_eq!(
//             LitStr::parse(&mut ParseContext::from("'hello world'")),
//             Ok(LitStr::from_span("hello world", Span::new(0, 13, 1, 1)))
//         );

//         assert_eq!(
//             LitStr::parse(&mut ParseContext::from("\"hello ' world\"")),
//             Ok(LitStr::from_span("hello ' world", Span::new(0, 15, 1, 1)))
//         );

//         assert_eq!(
//             LitStr::parse(&mut ParseContext::from("'hello.")),
//             Err(MlError::Parserc(Kind::Char(
//                 '\'',
//                 Span::new(7, 0, 1, 8).incomplete()
//             )))
//         );
//     }
//     #[test]
//     fn test_litnum() {
//         assert_eq!(
//             LitNum::parse(&mut ParseContext::from("12345")),
//             Ok(LitNum::from_span(12345, Span::new(0, 5, 1, 1)))
//         );

//         assert_eq!(
//             LitNum::parse(&mut ParseContext::from("0xf20")),
//             Ok(LitNum::from_span(0xf20, Span::new(0, 5, 1, 1)))
//         );

//         assert_eq!(
//             LitNum::parse(&mut ParseContext::from("0x f20")),
//             Err(MlError::LitNumHexBody(Span::new(0, 2, 1, 1).fatal()))
//         );

//         assert_eq!(
//             LitNum::parse(&mut ParseContext::from("123\t45")),
//             Ok(LitNum::from_span(123, Span::new(0, 3, 1, 1)))
//         );

//         assert_eq!(
//             LitNum::parse(&mut ParseContext::from("h1234")),
//             Err(MlError::LitNum(Span::new(0, 1, 1, 1).recoverable()))
//         );
//     }

//     #[test]
//     fn test_litexpr() {
//         let mut input = ParseContext::from("12345'hello world'");
//         assert_eq!(
//             LitExpr::parse(&mut input),
//             Ok(LitExpr::Numeric(LitNum::from_span(
//                 12345,
//                 Span::new(0, 5, 1, 1)
//             )))
//         );

//         assert_eq!(
//             LitExpr::parse(&mut input),
//             Ok(LitExpr::String(LitStr::from_span(
//                 "hello world",
//                 Span::new(5, 13, 1, 6)
//             )))
//         );
//     }

//     #[test]
//     fn test_callexpr() {
//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello")),
//             Ok(CallExpr::from_span(
//                 Ident::from_span("hello", Span::new(0, 5, 1, 1)),
//                 Span::new(0, 5, 1, 1)
//             ))
//         );

//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello( )")),
//             Ok(CallExpr::from_span(
//                 Ident::from_span("hello", Span::new(0, 5, 1, 1)),
//                 Span::new(0, 8, 1, 1)
//             ))
//         );

//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello( 'world' )")),
//             Ok(CallExpr::from_span(
//                 Ident::from_span("hello", Span::new(0, 5, 1, 1)),
//                 Span::new(0, 16, 1, 1)
//             )
//             .param(LitStr::from_span("world", Span::new(7, 7, 1, 8))))
//         );

//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello( 'world' , 1234 \t)")),
//             Ok(CallExpr::from_span(
//                 Ident::from_span("hello", Span::new(0, 5, 1, 1)),
//                 Span::new(0, 24, 1, 1)
//             )
//             .param(LitStr::from_span("world", Span::new(7, 7, 1, 8)))
//             .param(LitNum::from_span(1234, Span::new(17, 4, 1, 18))))
//         );

//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello('world', 1234,)")),
//             Ok(CallExpr::from_span(
//                 Ident::from_span("hello", Span::new(0, 5, 1, 1)),
//                 Span::new(0, 21, 1, 1)
//             )
//             .param(LitStr::from_span("world", Span::new(6, 7, 1, 7)))
//             .param(LitNum::from_span(1234, Span::new(15, 4, 1, 16))))
//         );

//         assert_eq!(
//             CallExpr::parse(&mut ParseContext::from("hello(goo)")),
//             Err(MlError::CallExpr(
//                 Box::new(Kind::Char(')', Span::new(6, 1, 1, 7).recoverable()).into()),
//                 Span::new(0, 5, 1, 1).fatal()
//             ))
//         );
//     }

//     #[test]
//     fn test_property() {
//         assert_eq!(
//             Property::parse(&mut ParseContext::from("#[  ]")),
//             Ok(Property::from_span(Span::new(0, 5, 1, 1)))
//         );
//         assert_eq!(
//             Property::parse(&mut ParseContext::from("#[\t\n]")),
//             Ok(Property::from_span(Span::new(0, 5, 1, 1)))
//         );

//         assert_eq!(
//             Property::parse(&mut ParseContext::from("#[")),
//             Err(MlError::Property(
//                 Box::new(Kind::Char(']', Span::new(2, 0, 1, 3).incomplete()).into()),
//                 Span::new(0, 2, 1, 1).fatal()
//             ))
//         );

//         assert_eq!(
//             Property::parse(&mut ParseContext::from("")),
//             Err(Kind::Keyword("#[".to_string(), Span::new(0, 0, 1, 1).incomplete()).into())
//         );

//         assert_eq!(
//             Property::parse(&mut ParseContext::from("#[hello('123',123),]")),
//             Ok(Property::from_span(Span::new(0, 20, 1, 1)).param(
//                 CallExpr::from_span(
//                     Ident::from_span("hello", Span::new(2, 5, 1, 3)),
//                     Span::new(2, 16, 1, 3)
//                 )
//                 .param(LitStr::from_span("123", Span::new(8, 5, 1, 9)))
//                 .param(LitNum::from_span(123, Span::new(14, 3, 1, 15)))
//             ))
//         );

//         assert_eq!(
//             Property::parse(&mut ParseContext::from("#[hello('123',\n123)]")),
//             Ok(Property::from_span(Span::new(0, 20, 1, 1)).param(
//                 CallExpr::from_span(
//                     Ident::from_span("hello", Span::new(2, 5, 1, 3)),
//                     Span::new(2, 17, 1, 3)
//                 )
//                 .param(LitStr::from_span("123", Span::new(8, 5, 1, 9)))
//                 .param(LitNum::from_span(123, Span::new(15, 3, 2, 1)))
//             ))
//         );
//     }

//     #[test]
//     fn test_comments() {
//         assert_eq!(
//             Comment::parse(&mut ParseContext::from("///")),
//             Ok(Comment("".to_owned(), Some(Span::new(0, 3, 1, 1))))
//         );

//         assert_eq!(
//             Comment::parse(&mut ParseContext::from("///\t\n")),
//             Ok(Comment("".to_owned(), Some(Span::new(0, 4, 1, 1))))
//         );

//         assert_eq!(
//             Comment::parse(&mut ParseContext::from("///\thello world\n")),
//             Ok(Comment(
//                 "hello world".to_owned(),
//                 Some(Span::new(0, 15, 1, 1))
//             ))
//         );
//     }

//     #[test]
//     fn test_ty() {
//         assert_eq!(
//             Type::parse(&mut ParseContext::from("bool")),
//             Ok(Type::Bool(Some(Span::new(0, 4, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("123")),
//             Err(MlError::Type(
//                 Box::new(MlError::Ident(Span::new(0, 1, 1, 1).recoverable())),
//                 Span::new(0, 1, 1, 1).recoverable()
//             ))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("")),
//             Err(MlError::Type(
//                 Box::new(MlError::Ident(Span::new(0, 0, 1, 1).incomplete())),
//                 Span::new(0, 0, 1, 1).recoverable()
//             ))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("string")),
//             Ok(Type::String(Some(Span::new(0, 6, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("byte")),
//             Ok(Type::Byte(Some(Span::new(0, 4, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("ubyte")),
//             Ok(Type::Ubyte(Some(Span::new(0, 5, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("short")),
//             Ok(Type::Short(Some(Span::new(0, 5, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("ushort")),
//             Ok(Type::Ushort(Some(Span::new(0, 6, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("int")),
//             Ok(Type::Int(Some(Span::new(0, 3, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("uint")),
//             Ok(Type::Uint(Some(Span::new(0, 4, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("long")),
//             Ok(Type::Long(Some(Span::new(0, 4, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("ulong")),
//             Ok(Type::Ulong(Some(Span::new(0, 5, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("float")),
//             Ok(Type::Float(Some(Span::new(0, 5, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("double")),
//             Ok(Type::Double(Some(Span::new(0, 6, 1, 1))))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("_hello")),
//             Ok(Type::Data(Ident::from_span(
//                 "_hello",
//                 Span::new(0, 6, 1, 1)
//             )))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("vec[ bool\n ]")),
//             Ok(Type::ListOf(
//                 Box::new(Type::Bool(Some(Span::new(5, 4, 1, 6)))),
//                 Some(Span::new(0, 12, 1, 1))
//             ))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("vec")),
//             Err(MlError::Type(
//                 Box::new(Kind::Char('[', Span::new(3, 0, 1, 4).incomplete()).into()),
//                 Span::new(0, 3, 1, 1).fatal()
//             ))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("vec[")),
//             Err(MlError::Type(
//                 Box::new(MlError::Type(
//                     Box::new(MlError::Ident(Span::new(4, 0, 1, 5).incomplete())),
//                     Span::new(4, 0, 1, 5).recoverable()
//                 )),
//                 Span::new(0, 3, 1, 1).fatal()
//             ))
//         );

//         assert_eq!(
//             Type::parse(&mut ParseContext::from("[ bool ; 30 ]")),
//             Ok(Type::ArrayOf(
//                 Box::new(Type::Bool(Some(Span::new(2, 4, 1, 3)))),
//                 LitNum::from_span(30, Span::new(9, 2, 1, 10)),
//                 Some(Span::new(0, 13, 1, 1))
//             ))
//         );
//     }

//     #[test]
//     fn test_field() {
//         assert_eq!(
//             Field::parse(&mut ParseContext::from("/// hello world\nhello: bool")),
//             Ok(Field {
//                 comments: vec![Comment(
//                     "hello world".to_string(),
//                     Some(Span::new(0, 15, 1, 1))
//                 )],
//                 properties: vec![],
//                 ident: Some(Ident::from_span("hello", Span::new(16, 5, 2, 1))),
//                 ty: Type::Bool(Some(Span::new(23, 4, 2, 8)))
//             })
//         );

//         assert_eq!(
//             Field::parse(&mut ParseContext::from("/// hello world\n#[hello]hello")),
//             Ok(Field {
//                 comments: vec![Comment(
//                     "hello world".to_string(),
//                     Some(Span::new(0, 15, 1, 1))
//                 )],
//                 properties: vec![Property::from_span(Span::new(16, 8, 2, 1)).param(
//                     CallExpr::from_span(
//                         Ident::from_span("hello", Span::new(18, 5, 2, 3)),
//                         Span::new(18, 5, 2, 3)
//                     )
//                 )],
//                 ident: None,
//                 ty: Type::Data(Ident::from_span("hello", Span::new(24, 5, 2, 9)))
//             })
//         );
//     }

//     #[test]
//     fn test_node() {
//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from(
//                 "el hello\n(/// hello world\nhello: bool)"
//             )),
//             Err(MlError::TupleField(Span::new(26, 5, 3, 1).fatal()))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from(
//                 "el hello\n{/// hello world\nworld}"
//             )),
//             Err(MlError::NotTupleField(Span::new(26, 5, 3, 1).fatal()))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("struct hello\n(bool,a:int)")),
//             Err(MlError::NodeKeyWord(Span::new(0, 1, 1, 1).recoverable()))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("hello\n{bool,}")),
//             Err(MlError::NodeKeyWord(Span::new(0, 1, 1, 1).recoverable()))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("data hello(bool)")),
//             Err(MlError::Node(
//                 Box::new(Kind::Char(';', Span::new(16, 0, 1, 17).incomplete()).into()),
//                 Span::new(0, 4, 1, 1).fatal()
//             ))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("attr hello;")),
//             Ok((
//                 Some(Span::new(0, 4, 1, 1)),
//                 Node {
//                     comments: vec![],
//                     mixin: None,
//                     properties: vec![],
//                     ident: Ident::from_span("hello", Span::new(5, 5, 1, 6)),
//                     fields: vec![]
//                 }
//             ))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("attr hello();")),
//             Ok((
//                 Some(Span::new(0, 4, 1, 1)),
//                 Node {
//                     comments: vec![],
//                     mixin: None,
//                     properties: vec![],
//                     ident: Ident::from_span("hello", Span::new(5, 5, 1, 6)),
//                     fields: vec![]
//                 }
//             ))
//         );

//         assert_eq!(
//             parse_node(false).parse(&mut ParseContext::from("attr hello {  }")),
//             Ok((
//                 Some(Span::new(0, 4, 1, 1)),
//                 Node {
//                     comments: vec![],
//                     mixin: None,
//                     properties: vec![],
//                     ident: Ident::from_span("hello", Span::new(5, 5, 1, 6)),
//                     fields: vec![]
//                 }
//             ))
//         );

//         assert_eq!(
//             parse_node(true).parse(&mut ParseContext::from(
//                 "CubicBezier{ ctrl1: Point, ctrl2: Point, to: Point }"
//             )),
//             Ok((
//                 None,
//                 Node {
//                     comments: vec![],
//                     mixin: None,
//                     properties: vec![],
//                     ident: Ident::from_span("CubicBezier", Span::new(0, 11, 1, 1)),
//                     fields: vec![
//                         Field {
//                             comments: vec![],
//                             properties: vec![],
//                             ident: Some(Ident::from_span("ctrl1", Span::new(13, 5, 1, 14))),
//                             ty: Type::Data(Ident::from_span("Point", Span::new(20, 5, 1, 21)))
//                         },
//                         Field {
//                             comments: vec![],
//                             properties: vec![],
//                             ident: Some(Ident::from_span("ctrl2", Span::new(27, 5, 1, 28))),
//                             ty: Type::Data(Ident::from_span("Point", Span::new(34, 5, 1, 35)))
//                         },
//                         Field {
//                             comments: vec![],
//                             properties: vec![],
//                             ident: Some(Ident::from_span("to", Span::new(41, 2, 1, 42))),
//                             ty: Type::Data(Ident::from_span("Point", Span::new(45, 5, 1, 46)))
//                         }
//                     ]
//                 }
//             ))
//         );

//         assert_eq!(
//             parse_node(true).parse(&mut ParseContext::from("Polyline(vec[Point])")),
//             Ok((
//                 None,
//                 Node {
//                     comments: vec![],
//                     mixin: None,
//                     properties: vec![],
//                     ident: Ident::from_span("Polyline", Span::new(0, 8, 1, 1)),
//                     fields: vec![Field {
//                         comments: vec![],
//                         properties: vec![],
//                         ident: None,
//                         ty: Type::ListOf(
//                             Box::new(Type::Data(Ident::from_span(
//                                 "Point",
//                                 Span::new(13, 5, 1, 14)
//                             ))),
//                             Some(Span::new(9, 10, 1, 10))
//                         )
//                     }]
//                 }
//             ))
//         );
//     }
// }
