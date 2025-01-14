use parserc::{
    ensure_char, ensure_keyword, take_while, ControlFlow, ParseContext, Parser, ParserExt, Result,
    Span,
};

use crate::{
    lang::{
        ir::{LitExpr, LitInt, LitLength, LitNum},
        parser::ParseError,
    },
    opcode::Length,
};

enum Prefix {
    Binary(Span),
    Hex(Span),
}

impl Prefix {
    fn radix(&self) -> u32 {
        match self {
            Prefix::Binary(_) => 2,
            Prefix::Hex(_) => 16,
        }
    }

    fn span(&self) -> Span {
        match self {
            Prefix::Binary(span) => *span,
            Prefix::Hex(span) => *span,
        }
    }
}

fn parse_decimal(ctx: &mut ParseContext<'_>) -> Result<(i32, Span)> {
    let start = ctx.span();

    if let Some(span) = take_while(|c| c.is_ascii_digit()).parse(ctx)? {
        if let Ok(v) = i32::from_str_radix(ctx.as_str(span), 10) {
            return Ok((v, span));
        } else {
            ctx.report_error(ParseError::LitIntOverflow, start);
            return Err(ControlFlow::Fatal);
        }
    } else {
        ctx.report_error(ParseError::LitInt, start);
        return Err(ControlFlow::Recoverable);
    }
}

/// Parse num/int/length.
pub fn parse_lit_num_int_length(ctx: &mut ParseContext<'_>) -> Result<LitExpr> {
    let (sign, start, has_sign) = ensure_char('+')
        .map(|span| (1, span, true))
        .or(ensure_char('-').map(|span| (-1, span, true)))
        .ok()
        .parse(ctx)?
        .unwrap_or((1, ctx.span(), false));

    let prefix = ensure_keyword("0x")
        .map(|span| Prefix::Hex(span))
        .or(ensure_keyword("0b").map(|span| Prefix::Binary(span)))
        .ok()
        .parse(ctx)?;

    // binary and hex prefix indicate that this is a literal int type.
    if let Some(prefix) = prefix {
        let radix = prefix.radix();

        if let Some(span) = take_while(move |c| c.is_digit(radix)).parse(ctx)? {
            if let Ok(v) = i32::from_str_radix(ctx.as_str(span), radix) {
                return Ok(LitExpr::Int(LitInt(
                    start.extend_to_inclusive(span),
                    sign * v,
                )));
            } else {
                ctx.report_error(ParseError::LitIntOverflow, prefix.span());
                return Err(ControlFlow::Fatal);
            }
        } else {
            ctx.report_error(ParseError::LitInt, prefix.span());
            return Err(ControlFlow::Fatal);
        }
    }

    let (trunc, span) = if has_sign {
        parse_decimal.fatal().parse(ctx)?
    } else {
        parse_decimal(ctx)?
    };

    let mut end = span;

    let franc = if ensure_char('.').ok().parse(ctx)?.is_some() {
        Some(parse_decimal.fatal().parse(ctx)?)
    } else {
        None
    };

    let mut value = trunc as f32;

    if let Some((franc, span)) = &franc {
        value += (*franc as f32) / 10f32.powi(span.len() as i32);
        end = *span;
    }

    value *= sign as f32;

    let length = ensure_keyword("em")
        .map(|span| LitLength(Length::Em(value), start.extend_to_inclusive(span)))
        .or(ensure_keyword("ex")
            .map(|span| LitLength(Length::Ex(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("px")
            .map(|span| LitLength(Length::Px(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("in")
            .map(|span| LitLength(Length::Inch(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("cm")
            .map(|span| LitLength(Length::Cm(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("mm")
            .map(|span| LitLength(Length::Mm(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("pt")
            .map(|span| LitLength(Length::Pt(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("pc")
            .map(|span| LitLength(Length::Pc(value), start.extend_to_inclusive(span))))
        .or(ensure_keyword("%")
            .map(|span| LitLength(Length::Percent(value), start.extend_to_inclusive(span))))
        .ok()
        .parse(ctx)?;

    if let Some(length) = length {
        return Ok(LitExpr::Length(length));
    }

    if ensure_char('e').ok().parse(ctx)?.is_none() {
        if franc.is_some() {
            return Ok(LitExpr::Number(LitNum(
                start.extend_to_inclusive(end),
                value,
            )));
        }

        return Ok(LitExpr::Int(LitInt(
            start.extend_to_inclusive(end),
            sign * trunc,
        )));
    }

    let exp_sign = ensure_char('+')
        .map(|_| 1)
        .or(ensure_char('-').map(|_| -1))
        .ok()
        .parse(ctx)?
        .unwrap_or(1);

    let (exp_num, end) = parse_decimal.fatal().parse(ctx)?;

    if exp_sign > 0 {
        value *= 10f32.powi(exp_num);
    } else {
        value /= 10f32.powi(exp_num);
    }

    return Ok(LitExpr::Number(LitNum(
        start.extend_to_inclusive(end),
        value,
    )));
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lang::ir::{LitExpr, LitInt};

    #[test]
    fn test_int() {
        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("-10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 3, 1, 1), -10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("-0x10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 5, 1, 1), -0x10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("0b10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 4, 1, 1), 0b10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("0b10e10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 4, 1, 1), 0b10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("0x10e10")),
            Ok(LitExpr::Int(LitInt(Span::new(0, 7, 1, 1), 0x10e10)))
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("-3.1415e-10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 11, 1, 1), -3.1415e-10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("-3e-10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 6, 1, 1), -3e-10)))
        );

        assert_eq!(
            parse_lit_num_int_length(&mut ParseContext::from("3.1415e-10")),
            Ok(LitExpr::Number(LitNum(Span::new(0, 10, 1, 1), 3.1415e-10)))
        );
    }

    #[test]
    fn test_length() {
        let tests = [
            (
                "ex",
                LitExpr::Length(LitLength(Length::Ex(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "px",
                LitExpr::Length(LitLength(Length::Px(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "in",
                LitExpr::Length(LitLength(Length::Inch(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "cm",
                LitExpr::Length(LitLength(Length::Cm(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "mm",
                LitExpr::Length(LitLength(Length::Mm(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "pt",
                LitExpr::Length(LitLength(Length::Pt(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "pc",
                LitExpr::Length(LitLength(Length::Pc(2.0), Span::new(0, 3, 1, 1))),
            ),
            (
                "%",
                LitExpr::Length(LitLength(Length::Percent(2.0), Span::new(0, 2, 1, 1))),
            ),
        ];

        for (unit, expect) in tests {
            assert_eq!(
                parse_lit_num_int_length(&mut ParseContext::from(format!("2{}", unit).as_str())),
                Ok(expect),
                "test {}",
                unit
            );
        }
    }
}
