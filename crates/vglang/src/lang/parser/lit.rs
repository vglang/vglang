use parserc::{ensure_char, ensure_keyword, take_till, FromSrc, Parser, ParserExt};

use crate::{
    lang::ir::{LitBool, LitCoords, LitExpr, LitLength, LitStr},
    opcode::{Coords, Length},
};

use super::{parse_int_or_num_ext, skip_ws, ParseError};

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

impl FromSrc for LitCoords {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let start = ensure_keyword("coords").parse(ctx)?;

        skip_ws
            .with_context(ParseError::LitCoords, ctx.span())
            .fatal()
            .parse(ctx)?;

        ensure_char('.')
            .with_context(ParseError::LitCoords, ctx.span())
            .fatal()
            .parse(ctx)?;

        skip_ws
            .with_context(ParseError::LitCoords, ctx.span())
            .fatal()
            .parse(ctx)?;

        ensure_keyword("userspace")
            .map(|span| LitCoords(Coords::UserSpaceOnUse, start.extend_to_inclusive(span)))
            .or(ensure_keyword("object")
                .map(|span| LitCoords(Coords::ObjectBoundingBox, start.extend_to_inclusive(span))))
            .with_context(ParseError::LitCoords, ctx.span())
            .fatal()
            .parse(ctx)
    }
}

impl FromSrc for LitLength {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        let value = parse_int_or_num_ext(true)
            .with_context(ParseError::LitLength, ctx.span())
            .parse(ctx)?;

        let (value, start) = match value {
            LitExpr::Number(v) => (v.1, v.0),
            LitExpr::Int(v) => (v.1 as f32, v.0),
            _ => {
                panic!("not here.")
            }
        };

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
            Ok(length)
        } else {
            Ok(LitLength(Length::Px(value), start))
        }
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
    fn test_coords() {
        assert_eq!(
            LitCoords::parse(&mut ParseContext::from("coords.userspace")),
            Ok(LitCoords(Coords::UserSpaceOnUse, Span::new(0, 16, 1, 1)))
        );

        assert_eq!(
            LitCoords::parse(&mut ParseContext::from("coords.object")),
            Ok(LitCoords(Coords::ObjectBoundingBox, Span::new(0, 13, 1, 1)))
        );

        assert_eq!(
            LitCoords::parse(&mut ParseContext::from("coords  \n.\t\nobject")),
            Ok(LitCoords(Coords::ObjectBoundingBox, Span::new(0, 18, 1, 1)))
        );

        assert_eq!(
            LitCoords::parse(&mut ParseContext::from("coords")),
            Err(ControlFlow::Fatal)
        );

        assert_eq!(
            LitCoords::parse(&mut ParseContext::from("coords  \t\n.")),
            Err(ControlFlow::Fatal)
        );
    }

    #[test]
    fn test_length() {
        assert_eq!(
            LitLength::parse(&mut ParseContext::from("2cm")),
            Ok(LitLength(Length::Cm(2.0), Span::new(0, 3, 1, 1)))
        );

        assert_eq!(
            LitLength::parse(&mut ParseContext::from("2.01cm")),
            Ok(LitLength(Length::Cm(2.01), Span::new(0, 6, 1, 1)))
        );

        assert_eq!(
            LitLength::parse(&mut ParseContext::from("2.01e10")),
            Ok(LitLength(Length::Cm(2.01), Span::new(0, 6, 1, 1)))
        );

        assert_eq!(
            LitLength::parse(&mut ParseContext::from("2 cm")),
            Ok(LitLength(Length::Px(2.0), Span::new(0, 1, 1, 1)))
        );

        let tests = [
            ("ex", LitLength(Length::Ex(2.0), Span::new(0, 3, 1, 1))),
            ("px", LitLength(Length::Px(2.0), Span::new(0, 3, 1, 1))),
            ("in", LitLength(Length::Inch(2.0), Span::new(0, 3, 1, 1))),
            ("cm", LitLength(Length::Cm(2.0), Span::new(0, 3, 1, 1))),
            ("mm", LitLength(Length::Mm(2.0), Span::new(0, 3, 1, 1))),
            ("pt", LitLength(Length::Pt(2.0), Span::new(0, 3, 1, 1))),
            ("pc", LitLength(Length::Pc(2.0), Span::new(0, 3, 1, 1))),
            ("%", LitLength(Length::Percent(2.0), Span::new(0, 2, 1, 1))),
        ];

        for (unit, expect) in tests {
            assert_eq!(
                LitLength::parse(&mut ParseContext::from(format!("2{}", unit).as_str())),
                Ok(expect),
                "test {}",
                unit
            );
        }
    }
}
