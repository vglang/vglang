use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, FromSrc, IntoParser, ParseContext, Parser,
    ParserExt, Result, Span,
};

use crate::ir::{Comment, Ident, LitUint, Property, Type};

use super::ParseError;

pub(super) fn skip_ws(ctx: &mut ParseContext<'_>) -> Result<Option<Span>> {
    let span = take_while(|c| c.is_whitespace()).parse(ctx)?;

    Ok(span)
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

        Ok(Comment(span, content.to_string()))
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
                .fatal(ParseError::Type(super::TypeKind::SquareBracketStart), start)
                .parse(input)?;

            skip_ws(input)?;

            let component = Type::into_parser().parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .fatal(ParseError::Type(super::TypeKind::SquareBracketEnd), start)
                .parse(input)?;

            return Ok(Type::ListOf(
                Box::new(component),
                start.extend_to_inclusive(end),
            ));
        }

        if let Some(start) = ensure_char('[').ok().parse(input)? {
            skip_ws(input)?;

            let component = Type::into_parser().parse(input)?;

            skip_ws(input)?;

            ensure_char(';')
                .fatal(ParseError::Type(super::TypeKind::Semicolons), start)
                .parse(input)?;

            skip_ws(input)?;

            let len = LitUint::into_parser()
                .fatal(ParseError::Type(super::TypeKind::Uint), start)
                .parse(input)?;

            skip_ws(input)?;

            let end = ensure_char(']')
                .fatal(ParseError::Type(super::TypeKind::SquareBracketEnd), start)
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
            .fatal(ParseError::Type(super::TypeKind::Data), start)
            .parse(input)
            .map(|ident| Type::Data(ident))
    }
}

#[allow(unused)]
pub(super) enum Prefix {
    Property(Property),
    Comment(Comment),
}

#[allow(unused)]
pub(super) fn parse_prefix(input: &mut ParseContext<'_>) -> Result<(Vec<Comment>, Vec<Property>)> {
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
