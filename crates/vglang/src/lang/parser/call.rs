use parserc::{ensure_char, FromSrc, IntoParser, Parser, ParserExt};

use crate::lang::{
    ir::{CallBody, Ident, LitExpr},
    parser::{utils::skip_ws, CallBodyKind, ParseError},
};

impl FromSrc for CallBody {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let start = ctx.span();
        let target = Ident::parse(ctx)?;

        skip_ws(ctx)?;

        if ensure_char('(')
            .ok()
            .fatal(
                ParseError::CallBody(CallBodyKind::ParamListStart),
                ctx.span(),
            )
            .parse(ctx)?
            .is_none()
        {
            return Ok(Self {
                span: target.1,
                target,
                params: vec![],
            });
        }

        let mut params = vec![];

        while let Some(param) = LitExpr::into_parser()
            .ok()
            .fatal(ParseError::CallBody(CallBodyKind::Param), ctx.span())
            .parse(ctx)?
        {
            params.push(param);

            skip_ws(ctx)?;

            if ensure_char(',')
                .ok()
                .fatal(
                    ParseError::CallBody(CallBodyKind::ParamListStart),
                    ctx.span(),
                )
                .parse(ctx)?
                .is_none()
            {
                break;
            }

            skip_ws(ctx)?;
        }

        let end = ensure_char(')')
            .fatal(
                ParseError::CallBody(CallBodyKind::ParamListStart),
                ctx.span(),
            )
            .parse(ctx)?;

        let span = start.extend_to_inclusive(end);

        Ok(Self {
            span,
            target,
            params,
        })
    }
}

#[cfg(test)]
mod tests {
    use parserc::{FromSrc, ParseContext, Span};

    use crate::lang::ir::{CallBody, Ident, LitBool, LitExpr, LitStr};

    #[test]
    fn test_call_body() {
        assert_eq!(
            CallBody::parse(&mut ParseContext::from("hello")),
            Ok(CallBody {
                span: Span::new(0, 5, 1, 1),
                target: Ident("hello".into(), Span::new(0, 5, 1, 1)),
                params: vec![]
            })
        );

        assert_eq!(
            CallBody::parse(&mut ParseContext::from("hello\t('world',\ntrue)")),
            Ok(CallBody {
                span: Span::new(0, 21, 1, 1),
                target: Ident("hello".into(), Span::new(0, 5, 1, 1)),
                params: vec![
                    LitExpr::Str(LitStr {
                        span: Span::new(7, 7, 1, 8),
                        double_quote: false,
                        content: "world".into()
                    }),
                    LitExpr::Bool(LitBool(Span::new(16, 4, 2, 1), true))
                ]
            })
        );
    }
}
