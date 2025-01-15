use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, ControlFlow, FromSrc, IntoParser,
    ParseContext, Parser, ParserExt, Result, Span,
};

use crate::{
    lang::{
        ir::{Attr, CallExpr, Ident, LitBool, LitCoords, LitExpr, LitStr},
        parser::{parse_lit_num_int_length, parse_named_register, parser_color},
    },
    opcode::{variable::Target, Opcode},
};

use super::ParseError;

fn parse_comment(ctx: &mut ParseContext<'_>) -> Result<Span> {
    skip_ws(ctx)?;
    let start = ensure_keyword("///").parse(ctx)?;

    let content = take_till(|c| c == '\n').parse(ctx)?;

    let span = if let Some(content) = content {
        start.extend_to_inclusive(content)
    } else {
        start
    };

    skip_ws(ctx)?;

    Ok(span)
}

pub(super) fn skip_ws(ctx: &mut ParseContext<'_>) -> Result<Option<Span>> {
    let span = take_while(|c| c.is_whitespace()).parse(ctx)?;

    Ok(span)
}

fn parse_attrs(input: &mut ParseContext<'_>) -> Result<Vec<Attr>> {
    let mut attrs = vec![];

    skip_ws(input)?;

    let property_parser = Attr::into_parser().map(|p| Some(p));

    let comment_parser = parse_comment.map(|_| None);

    while let Some(prefix) = property_parser
        .clone()
        .or(comment_parser.clone())
        .ok()
        .parse(input)?
    {
        if let Some(attr) = prefix {
            attrs.push(attr);
        }

        skip_ws(input)?;
    }

    Ok(attrs)
}

impl FromSrc for Ident {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (c, start) = ctx.next();

        if let Some(c) = c {
            if c != '_' && !c.is_alphabetic() {
                ctx.report_error(ParseError::Ident, start);
                return Err(ControlFlow::Recoverable);
            }
        } else {
            ctx.report_error(ParseError::Ident, start);
            return Err(ControlFlow::Incomplete);
        }

        let body = take_while(|c| c == '_' || c.is_alphanumeric()).parse(ctx)?;

        let span = if let Some(body) = body {
            start.extend_to_inclusive(body)
        } else {
            start
        };

        assert!(span.len() > 0);

        let ident = ctx.as_str(span);

        assert_eq!(ident.len(), span.len());

        Ok(Ident(ident.to_string(), span))
    }
}

impl FromSrc for LitExpr {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        parse_named_register
            .map(|(path, span)| LitExpr::Variable(path, Target::Register, span))
            .or(parse_lit_num_int_length)
            .or(LitBool::into_parser().map(|v| LitExpr::Bool(v)))
            .or(parser_color.map(|v| LitExpr::Color(v)))
            .or(LitStr::into_parser().map(|v| LitExpr::Str(v)))
            .or(LitCoords::into_parser().map(|v| LitExpr::Coords(v)))
            .parse(ctx)
    }
}

impl FromSrc for Attr {
    fn parse(_ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl FromSrc for CallExpr {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = ctx.span();

        let attrs = parse_attrs(ctx)?;

        let target = Ident::parse(ctx)?;

        skip_ws(ctx)?;

        if ensure_char('(').ok().parse(ctx)?.is_none() {
            return Ok(CallExpr {
                span: start.extend_to_inclusive(target.1),
                target,
                attrs,
                params: vec![],
            });
        }

        let mut params = vec![];

        while let Some(lit) = LitExpr::into_parser().ok().parse(ctx)? {
            params.push(lit);

            skip_ws(ctx)?;

            if ensure_char(',').ok().parse(ctx)?.is_none() {
                skip_ws(ctx)?;
                break;
            }

            skip_ws(ctx)?;
        }

        // parse param list

        let end = ensure_char(')')
            .with_context(ParseError::CallBodyEnd, ctx.span())
            .parse(ctx)?;

        Ok(CallExpr {
            span: start.extend_to_inclusive(end),
            attrs,
            target,
            params,
        })
    }
}

/// The main entry fn of `vglang` parser.
pub fn parse(_ctx: &mut ParseContext<'_>) -> Result<Vec<Opcode>> {
    Ok(vec![])
}
