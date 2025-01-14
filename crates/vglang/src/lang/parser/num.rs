use parserc::{
    ensure_char, ensure_keyword, take_while, ControlFlow, ParseContext, ParseOkOr, Parser,
    ParserExt, Result, Span,
};

use crate::lang::ir::{LitExpr, LitInt, LitNum};

use super::ParseError;

/// Parse decimal str
#[inline]
fn parse_decimal(ctx: &mut ParseContext<'_>) -> Result<Span> {
    take_while(|c| c.is_digit(10))
        .ok_or(ParseError::LitDecimal, ctx.span())
        .parse(ctx)
}

/// Parse hex num str
#[inline]
fn parse_hex(ctx: &mut ParseContext<'_>) -> Result<Span> {
    ensure_keyword("0x").parse(ctx)?;

    take_while(|c| c.is_digit(16))
        .ok_or(ParseError::LitDecimal, ctx.span())
        .parse(ctx)
}

/// Parse hex binary str
#[inline]
fn parse_binary(ctx: &mut ParseContext<'_>) -> Result<Span> {
    ensure_keyword("0b").parse(ctx)?;

    take_while(|c| c.is_digit(2))
        .ok_or(ParseError::LitDecimal, ctx.span())
        .parse(ctx)
}

/// Parse literal expr num or int.
#[inline]
pub fn parse_int_or_num(ctx: &mut ParseContext<'_>) -> Result<LitExpr> {
    parse_int_or_num_ext(false).parse(ctx)
}

/// Parse literal expr num or int.
#[inline]
pub fn parse_int_or_num_ext(parse_length: bool) -> impl Parser<Output = LitExpr> + Clone {
    move |ctx: &mut ParseContext<'_>| {
        let start = ctx.span();
        let sign = ensure_char('+')
            .map(|_| 1)
            .or(ensure_char('-').map(|_| -1))
            .ok()
            .parse(ctx)?;

        let trunc_parser = parse_hex
            .map(|span| (span, 16))
            .or(parse_binary.map(|span| (span, 2)))
            .or(parse_decimal.map(|span| (span, 10)));

        let ((body, radix), sign) = if let Some(sign) = sign {
            (trunc_parser.fatal().parse(ctx)?, sign)
        } else {
            (trunc_parser.parse(ctx)?, 1)
        };

        let mut is_float = false;

        let mut end = body;

        if let Some(span) = ensure_char('.').ok().parse(ctx)? {
            is_float = true;
            end = parse_decimal
                .with_context(ParseError::LitNum, span)
                .fatal()
                .parse(ctx)?;
        }

        if !parse_length {
            if let Some(span) = ensure_char('E').or(ensure_char('e')).ok().parse(ctx)? {
                is_float = true;

                ensure_char('+').or(ensure_char('-')).ok().parse(ctx)?;

                end = parse_decimal
                    .with_context(ParseError::LitNum, span)
                    .fatal()
                    .parse(ctx)?;
            }
        }

        let span = body.extend_to_inclusive(end);

        let body = ctx.as_str(span);

        if is_float {
            let v = match body.parse::<f32>() {
                Ok(v) => v,
                Err(_) => {
                    ctx.report_error(ParseError::LitNum, span);
                    return Err(ControlFlow::Fatal);
                }
            };

            return Ok(LitExpr::Number(LitNum(
                start.extend_to_inclusive(end),
                sign as f32 * v,
            )));
        } else {
            let v = match i32::from_str_radix(body, radix) {
                Ok(v) => v,
                Err(_) => {
                    ctx.report_error(ParseError::LitInt, span);
                    return Err(ControlFlow::Fatal);
                }
            };

            return Ok(LitExpr::Int(LitInt(
                start.extend_to_inclusive(end),
                sign * v,
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_int() {
        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 3, 1, 1), -10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("+10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 3, 1, 1), 10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 2, 1, 1), 10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("- 10")),
            Err(ControlFlow::Fatal)
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-10e-10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 7, 1, 1), -10e-10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-10e10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 6, 1, 1), -10e10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-10e 10")),
            Err(ControlFlow::Fatal)
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("0b13")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 3, 1, 1), 1)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-0b13")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 4, 1, 1), -1)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-3.1415e-10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 11, 1, 1), -3.1415e-10)))
        );

        assert_eq!(
            parse_int_or_num(&mut ParseContext::from("-0x11")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 5, 1, 1), -0x11)))
        );
    }
}
