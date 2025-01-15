use parserc::{
    ensure_char, ensure_keyword, take_while, ControlFlow, FromSrc, IntoParser, Parser, ParserExt,
};

use crate::lang::{
    ir::{LitExp, LitNum, LitRadix, LitSign},
    parser::ParseError,
};

impl FromSrc for LitSign {
    #[inline(always)]
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        ensure_char('+')
            .map(|span| LitSign(span, false))
            .or(ensure_char('-').map(|span| LitSign(span, true)))
            .parse(ctx)
    }
}

impl FromSrc for LitRadix {
    #[inline(always)]
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        if let Some(start) = ensure_keyword("0x").ok().parse(ctx)? {
            let body = take_while(|c| c.is_ascii_hexdigit()).parse(ctx)?;

            if let Some(body) = body {
                return Ok(Self::Hex(
                    u64::from_str_radix(ctx.as_str(body), 16).unwrap(),
                    start.extend_to_inclusive(body),
                ));
            } else {
                ctx.report_error(ParseError::HexBody, start);
                return Err(ControlFlow::Fatal);
            }
        }

        if let Some(start) = ensure_keyword("0b").ok().parse(ctx)? {
            let body = take_while(|c| c.is_digit(2)).parse(ctx)?;

            if let Some(body) = body {
                return Ok(Self::Binary(
                    u64::from_str_radix(ctx.as_str(body), 2).unwrap(),
                    start.extend_to_inclusive(body),
                ));
            } else {
                ctx.report_error(ParseError::BinaryBody, start);
                return Err(ControlFlow::Fatal);
            }
        }

        let body = take_while(|c| c.is_ascii_digit()).parse(ctx)?;

        if let Some(body) = body {
            return Ok(Self::Decimal(
                u64::from_str_radix(ctx.as_str(body), 10).unwrap(),
                body,
            ));
        } else {
            let start = ctx.span();
            ctx.report_error(ParseError::Decimal, start);
            return Err(ControlFlow::Recoverable);
        }
    }
}

impl FromSrc for LitExp {
    #[inline(always)]
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_char('E').or(ensure_char('e')).parse(ctx)?;

        let sign = LitSign::into_parser().ok().parse(ctx)?;

        let body = take_while(|c| c.is_ascii_digit()).parse(ctx)?;

        if let Some(body) = body {
            return Ok(Self {
                span: start.extend_to_inclusive(body),
                sign,
                number: u64::from_str_radix(ctx.as_str(body), 10).unwrap(),
            });
        } else {
            let start = ctx.span();
            ctx.report_error(ParseError::Decimal, start);
            return Err(ControlFlow::Fatal);
        }
    }
}

impl FromSrc for LitNum {
    #[inline(always)]
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let start = ctx.span();

        let sign = LitSign::into_parser().ok().parse(ctx)?;

        let integer_part = if let Some(sign) = &sign {
            LitRadix::into_parser()
                .with_context(ParseError::Number, sign.0)
                .fatal()
                .parse(ctx)?
        } else {
            LitRadix::into_parser().parse(ctx)?
        };

        match &integer_part {
            LitRadix::Decimal(_, _) => {}
            _ => {
                return Ok(Self {
                    span: start.extend_to(ctx.span()),
                    sign,
                    integer_part,
                    frac_part: None,
                    exp_part: None,
                })
            }
        }

        ensure_char('.')
            .with_context(ParseError::Number, ctx.span())
            .fatal()
            .parse(ctx)?;

        let frac_part = LitRadix::into_parser()
            .ok()
            .with_context(ParseError::Number, ctx.span())
            .fatal()
            .parse(ctx)?;

        let exp_part = LitExp::into_parser()
            .ok()
            .with_context(ParseError::Number, ctx.span())
            .fatal()
            .parse(ctx)?;

        Ok(Self {
            span: start.extend_to(ctx.span()),
            sign,
            integer_part,
            frac_part,
            exp_part,
        })
    }
}

#[cfg(test)]
mod tests {
    use parserc::{FromSrc, ParseContext, Span};

    use crate::lang::ir::{LitExp, LitNum, LitRadix, LitSign};

    #[test]
    fn test_radix() {
        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("10")),
            Ok(LitRadix::Decimal(10, Span::new(0, 2, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0b10")),
            Ok(LitRadix::Binary(0b10, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0x011")),
            Ok(LitRadix::Hex(0x011, Span::new(0, 5, 1, 1)))
        );
    }

    #[test]
    fn test_exp() {
        assert_eq!(
            LitExp::parse(&mut ParseContext::from("e1000")),
            Ok(LitExp {
                span: Span::new(0, 5, 1, 1),
                sign: None,
                number: 1000
            })
        );

        assert_eq!(
            LitExp::parse(&mut ParseContext::from("E-10")),
            Ok(LitExp {
                span: Span::new(0, 4, 1, 1),
                sign: Some(LitSign(Span::new(1, 1, 1, 2), true)),
                number: 10
            })
        );

        assert_eq!(
            LitExp::parse(&mut ParseContext::from("E+10")),
            Ok(LitExp {
                span: Span::new(0, 4, 1, 1),
                sign: Some(LitSign(Span::new(1, 1, 1, 2), false)),
                number: 10
            })
        );
    }

    #[test]
    fn test_num() {
        assert_eq!(
            LitNum::parse(&mut ParseContext::from("2.1012e10")),
            Ok(LitNum {
                span: Span::new(0, 9, 1, 1),
                sign: None,
                integer_part: LitRadix::Decimal(2, Span::new(0, 1, 1, 1)),
                frac_part: Some(LitRadix::Decimal(1012, Span::new(2, 4, 1, 3))),
                exp_part: Some(LitExp {
                    span: Span::new(6, 3, 1, 7),
                    sign: None,
                    number: 10
                })
            })
        );
    }
}
