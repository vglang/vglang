use parserc::{
    ensure_keyword, take_till, take_while, ParseContext, Parser, ParserExt, Result, Span,
};

use crate::opcode::Opcode;

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

fn skip_ws(ctx: &mut ParseContext<'_>) -> Result<Option<Span>> {
    let span = take_while(|c| c.is_whitespace()).parse(ctx)?;

    Ok(span)
}

fn skip_comments(ctx: &mut ParseContext<'_>) -> Result<()> {
    while let Some(span) = parse_comment.ok().parse(ctx)? {
        log::debug!("(skip comment): {}", ctx.as_str(span));
    }

    Ok(())
}

fn parse_graphic_fn(ctx: &mut ParseContext<'_>) -> Result<()> {
    skip_comments(ctx)?;

    let start = ensure_keyword("fn").parse(ctx)?;

    skip_ws
        .with_context(ParseError::GraphicFn, start)
        .fatal()
        .parse(ctx)?;

    Ok(())
}

/// The main entry fn of `vglang` parser.
pub fn parse(ctx: &mut ParseContext<'_>) -> Result<Vec<Opcode>> {
    parse_graphic_fn(ctx)?;
    Ok(vec![])
}
