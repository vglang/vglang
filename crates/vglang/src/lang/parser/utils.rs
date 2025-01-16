use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, ControlFlow, FromSrc, IntoParser,
    ParseContext, Parser, ParserExt, Result, Span,
};

use crate::lang::{
    ir::{Ident, LitBool, LitEnum, LitStr},
    parser::{EnumKind, ParseError},
};

use super::StrKind;

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

impl FromSrc for LitEnum {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let target = Ident::parse(ctx)?;

        skip_ws(ctx)?;

        ensure_char('.')
            .fatal(ParseError::LitEnum(EnumKind::Punct), ctx.span())
            .parse(ctx)?;

        skip_ws(ctx)?;

        let field = Ident::into_parser()
            .fatal(ParseError::LitEnum(EnumKind::Field), ctx.span())
            .parse(ctx)?;

        Ok(Self {
            span: target.1.extend_to_inclusive(field.1),
            target,
            field,
        })
    }
}

impl FromSrc for LitBool {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        ensure_keyword("true")
            .map(|span| LitBool(span, true))
            .or(ensure_keyword("false").map(|span| LitBool(span, false)))
            .parse(ctx)
    }
}

impl FromSrc for LitStr {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_char('"').or(ensure_char('\'')).parse(ctx)?;

        let double_quote = match ctx.as_str(start) {
            "'" => false,
            "\"" => true,
            _ => panic!("not here"),
        };

        let content = take_till(|c| if double_quote { c == '"' } else { c == '\'' }).parse(ctx)?;

        let content = if let Some(content) = content {
            ctx.as_str(content).to_string()
        } else {
            "".to_string()
        };

        let end = ensure_char(if double_quote { '"' } else { '\'' })
            .fatal(
                ParseError::LitStr(if double_quote {
                    StrKind::DoubleQuote
                } else {
                    StrKind::Quote
                }),
                start,
            )
            .parse(ctx)?;

        Ok(LitStr {
            span: start.extend_to_inclusive(end),
            double_quote,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use parserc::{ControlFlow, FromSrc, ParseContext, Span};

    use crate::lang::ir::{Ident, LitBool, LitEnum, LitStr};

    #[test]
    fn test_bool() {
        assert_eq!(
            LitBool::parse(&mut ParseContext::from("true")),
            Ok(LitBool(Span::new(0, 4, 1, 1), true))
        );

        assert_eq!(
            LitBool::parse(&mut ParseContext::from("false")),
            Ok(LitBool(Span::new(0, 5, 1, 1), false))
        );
    }

    #[test]
    fn test_enum() {
        assert_eq!(
            LitEnum::parse(&mut ParseContext::from("color.blue")),
            Ok(LitEnum {
                span: Span::new(0, 10, 1, 1),
                target: Ident("color".to_string(), Span::new(0, 5, 1, 1)),
                field: Ident("blue".to_string(), Span::new(6, 4, 1, 7))
            })
        );

        assert_eq!(
            LitEnum::parse(&mut ParseContext::from("color\t.\nblue")),
            Ok(LitEnum {
                span: Span::new(0, 12, 1, 1),
                target: Ident("color".to_string(), Span::new(0, 5, 1, 1)),
                field: Ident("blue".to_string(), Span::new(8, 4, 2, 1))
            })
        );
    }

    #[test]
    fn test_lit_str() {
        assert_eq!(
            LitStr::parse(&mut ParseContext::from("\"hello world\"")),
            Ok(LitStr {
                span: Span::new(0, 13, 1, 1),
                double_quote: true,
                content: "hello world".to_string()
            })
        );
        assert_eq!(
            LitStr::parse(&mut ParseContext::from("'hello world'")),
            Ok(LitStr {
                span: Span::new(0, 13, 1, 1),
                double_quote: false,
                content: "hello world".to_string()
            })
        );

        assert_eq!(
            LitStr::parse(&mut ParseContext::from("\t'hello world'")),
            Err(ControlFlow::Recoverable)
        );
    }
}
