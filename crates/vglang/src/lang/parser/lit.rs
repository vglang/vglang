use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, ControlFlow, FromSrc, IntoParser,
    ParseContext, Parser, ParserExt, Result, Span,
};

use crate::lang::ir::{LitBool, LitExp, LitRadix, LitSign, LitStr};

use super::ParseError;

impl FromSrc for LitBool {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        ensure_keyword("true")
            .map(|span| LitBool(true, span))
            .or(ensure_keyword("false").map(|span| LitBool(false, span)))
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

        let content = take_till(|c| if double_quote { c == '"' } else { c == '\'' })
            .with_context(ParseError::LitStr(ctx.as_str(start).to_string()), start)
            .fatal()
            .parse(ctx)?;

        let content = if let Some(content) = content {
            ctx.as_str(content).to_string()
        } else {
            "".to_string()
        };

        let end = ensure_char(if double_quote { '"' } else { '\'' })
            .with_context(ParseError::LitStr(ctx.as_str(start).to_string()), start)
            .fatal()
            .parse(ctx)?;

        Ok(LitStr(content, start.extend_to_inclusive(end)))
    }
}

impl FromSrc for LitSign {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        ensure_char('+')
            .map(|span| Self(true, span))
            .or(ensure_char('-').map(|span| Self(false, span)))
            .parse(ctx)
    }
}

fn parse_decimal_integer(ctx: &mut ParseContext<'_>) -> Result<(usize, Span)> {
    let start = ctx.span();

    if let Some(span) = take_while(|c| c.is_ascii_digit())
        .with_context(ParseError::LitDecimal, start)
        .parse(ctx)?
    {
        let body = ctx.as_str(span);

        match usize::from_str_radix(body, 10) {
            Ok(value) => return Ok((value, span)),
            Err(err) => {
                match err.kind() {
                    std::num::IntErrorKind::InvalidDigit => {
                        ctx.report_error(ParseError::InvalidDigit(10), span);
                    }
                    _ => {
                        ctx.report_error(ParseError::LitDecimalOverflow(body.to_string()), span);
                    }
                }

                return Err(ControlFlow::Fatal);
            }
        }
    } else {
        let span = ctx.span();
        ctx.report_error(ParseError::LitDecimal, span);
        return Err(ControlFlow::Recoverable);
    }
}

fn parse_hex_integer(ctx: &mut ParseContext<'_>) -> Result<(usize, Span)> {
    let start = ensure_keyword("0x").parse(ctx)?;

    if let Some(span) = take_while(|c| !c.is_whitespace()).parse(ctx)? {
        match usize::from_str_radix(ctx.as_str(span), 16) {
            Ok(value) => return Ok((value, start.extend_to_inclusive(span))),
            Err(err) => {
                match err.kind() {
                    std::num::IntErrorKind::InvalidDigit => {
                        ctx.report_error(ParseError::InvalidDigit(16), span);
                    }
                    _ => {
                        ctx.report_error(
                            ParseError::LitHexOverflow(ctx.as_str(span).to_string()),
                            span,
                        );
                    }
                }

                return Err(ControlFlow::Fatal);
            }
        }
    } else {
        let span = ctx.span();
        ctx.report_error(ParseError::LitHex, span);
        return Err(ControlFlow::Recoverable);
    }
}

fn parse_binary_integer(ctx: &mut ParseContext<'_>) -> Result<(usize, Span)> {
    let start = ensure_keyword("0b").parse(ctx)?;

    if let Some(span) = take_while(|c| c.is_ascii_digit()).parse(ctx)? {
        match usize::from_str_radix(ctx.as_str(span), 2) {
            Ok(value) => return Ok((value, start.extend_to_inclusive(span))),
            Err(err) => {
                match err.kind() {
                    std::num::IntErrorKind::InvalidDigit => {
                        ctx.report_error(ParseError::InvalidDigit(2), span);
                    }
                    _ => {
                        ctx.report_error(
                            ParseError::LitBinaryOverflow(ctx.as_str(span).to_string()),
                            span,
                        );
                    }
                }

                return Err(ControlFlow::Fatal);
            }
        }
    } else {
        let span = ctx.span();
        ctx.report_error(ParseError::LitBinary, span);
        return Err(ControlFlow::Recoverable);
    }
}

impl FromSrc for LitRadix {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        parse_hex_integer
            .map(|(value, span)| LitRadix::Hex(value, span))
            .or(parse_binary_integer.map(|(value, span)| LitRadix::Binary(value, span)))
            .or(parse_decimal_integer.map(|(value, span)| LitRadix::Decimal(value, span)))
            .parse(ctx)
    }
}

impl FromSrc for LitExp {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_char('E').or(ensure_char('e')).parse(ctx)?;

        let sign = LitSign::into_parser()
            .ok()
            .with_context(ParseError::LitExp, ctx.span())
            .fatal()
            .parse(ctx)?;

        let (value, span) = parse_decimal_integer
            .with_context(ParseError::LitExp, ctx.span())
            .fatal()
            .parse(ctx)?;

        let value = if let Some(LitSign(false, _)) = sign {
            -(value as i64)
        } else {
            value as i64
        };

        Ok(Self(value, start.extend_to_inclusive(span)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parserc::{ControlFlow, ParseContext, Span};

    #[test]
    fn test_bool() {
        assert_eq!(
            LitBool::parse(&mut ParseContext::from("true")),
            Ok(LitBool(true, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            LitBool::parse(&mut ParseContext::from("false")),
            Ok(LitBool(false, Span::new(0, 5, 1, 1)))
        );
    }

    #[test]
    fn test_lit_str() {
        assert_eq!(
            LitStr::parse(&mut ParseContext::from("\"hello world\"")),
            Ok(LitStr("hello world".to_string(), Span::new(0, 13, 1, 1)))
        );
        assert_eq!(
            LitStr::parse(&mut ParseContext::from("'hello world'")),
            Ok(LitStr("hello world".to_string(), Span::new(0, 13, 1, 1)))
        );

        assert_eq!(
            LitStr::parse(&mut ParseContext::from("\t'hello world'")),
            Err(ControlFlow::Recoverable)
        );
    }

    #[test]
    fn test_lit_decimal() {
        assert_eq!(
            parse_decimal_integer(&mut ParseContext::from("1000")),
            Ok((1000usize, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            parse_decimal_integer(&mut ParseContext::from("10000000000000000000000000000000")),
            Err(ControlFlow::Fatal)
        );

        assert_eq!(
            parse_decimal_integer(&mut ParseContext::from("-10")),
            Err(ControlFlow::Recoverable)
        );

        assert_eq!(
            parse_decimal_integer(&mut ParseContext::from("0xff")),
            Ok((0, Span::new(0, 1, 1, 1)))
        );
    }

    #[test]
    fn test_lit_radix() {
        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("1000")),
            Ok(LitRadix::Decimal(1000, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("1000e10")),
            Ok(LitRadix::Decimal(1000, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0b110")),
            Ok(LitRadix::Binary(0b110, Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0b113")),
            Err(ControlFlow::Fatal)
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0x11")),
            Ok(LitRadix::Hex(0x11, Span::new(0, 4, 1, 1)))
        );

        assert_eq!(
            LitRadix::parse(&mut ParseContext::from("0x1g")),
            Err(ControlFlow::Fatal)
        );
    }

    #[test]
    fn test_lit_exp() {
        assert_eq!(
            LitExp::parse(&mut ParseContext::from("e10")),
            Ok(LitExp(10, Span::new(0, 3, 1, 1)))
        );

        assert_eq!(
            LitExp::parse(&mut ParseContext::from("E-1000")),
            Ok(LitExp(-1000, Span::new(0, 6, 1, 1)))
        );

        assert_eq!(
            LitExp::parse(&mut ParseContext::from("Ef")),
            Err(ControlFlow::Fatal)
        );
    }
}
