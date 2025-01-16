use parserc::{take_while, ControlFlow, FromSrc, ParseContext, Parser, Result, Span};

use crate::lang::{ir::Ident, parser::ParseError};

pub(super) fn skip_ws(ctx: &mut ParseContext<'_>) -> Result<Option<Span>> {
    let span = take_while(|c| c.is_whitespace()).parse(ctx)?;

    Ok(span)
}

impl FromSrc for Ident {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let (c, start) = ctx.next();

        if let Some(c) = c {
            if c != '_' && !c.is_alphabetic() {
                ctx.on_fatal(ParseError::Ident, start);
                return Err(ControlFlow::Recoverable);
            }
        } else {
            ctx.on_fatal(ParseError::Ident, start);
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
