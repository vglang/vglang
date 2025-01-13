use parserc::{ensure_char, ensure_keyword, take_till, FromInput, Parser, ParserExt};

use crate::lang::ir::{LitBool, LitInt, LitStr};

use super::ParseError;

impl FromInput for LitBool {
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

impl FromInput for LitStr {
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

impl FromInput for LitInt {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        todo!()
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
}
