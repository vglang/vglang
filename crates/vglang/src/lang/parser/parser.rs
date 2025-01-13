use parserc::{
    ensure_keyword, take_till, take_while, ControlFlow, FromSrc, ParseContext, Parser, ParserExt,
    Result, Span,
};

use crate::{lang::ir::Ident, opcode::Opcode};

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

fn skip_comments(ctx: &mut ParseContext<'_>) -> Result<()> {
    while let Some(span) = parse_comment.ok().parse(ctx)? {
        log::debug!("(skip comment): {}", ctx.as_str(span));
    }

    Ok(())
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

fn parse_graphic_fn(ctx: &mut ParseContext<'_>) -> Result<()> {
    skip_comments(ctx)?;

    let start = ensure_keyword("fn").parse(ctx)?;

    skip_ws
        .with_context(ParseError::Fn, start)
        .fatal()
        .parse(ctx)?;

    Ok(())
}

/// The main entry fn of `vglang` parser.
pub fn parse(ctx: &mut ParseContext<'_>) -> Result<Vec<Opcode>> {
    parse_graphic_fn(ctx)?;
    Ok(vec![])
}
