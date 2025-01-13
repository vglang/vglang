use parserc::{
    ensure_char, ensure_keyword, take_while, ControlFlow, FromInput, IntoParser, ParseContext,
    Parser, ParserExt, Result,
};

use crate::{
    lang::{
        ir::Ident,
        parser::{skip_ws, ParseError},
    },
    opcode::Rgb,
};

/// Parse color from hex format.
pub fn parse_hex_color(ctx: &mut ParseContext<'_>) -> Result<Rgb> {
    let start = ensure_char('#').parse(ctx)?;

    let body = take_while(|c| c.is_ascii_hexdigit())
        .with_context(ParseError::Fn, start)
        .fatal()
        .parse(ctx)?;

    let body = match body {
        Some(body) => body,
        _ => {
            ctx.report_error(ParseError::HexColor, start);
            return Err(ControlFlow::Fatal);
        }
    };

    match body.len() {
        3 => {
            let mut body = ctx.as_str(body).chars();

            let r = body.next().unwrap();
            let g = body.next().unwrap();
            let b = body.next().unwrap();

            let r: u8 = u8::from_str_radix(&format!("{}{}", r, r), 16).unwrap();
            let g: u8 = u8::from_str_radix(&format!("{}{}", g, g), 16).unwrap();
            let b: u8 = u8::from_str_radix(&format!("{}{}", b, b), 16).unwrap();

            return Ok(Rgb(r, g, b));
        }
        6 => {
            let body = ctx.as_str(body);

            let r: u8 = u8::from_str_radix(&body[0..2], 16).unwrap();
            let g: u8 = u8::from_str_radix(&body[2..4], 16).unwrap();
            let b: u8 = u8::from_str_radix(&body[4..6], 16).unwrap();

            return Ok(Rgb(r, g, b));
        }
        _ => {
            ctx.report_error(ParseError::HexColor, start);
            return Err(ControlFlow::Fatal);
        }
    }
}

/// Parse recognized color: color.blue, etc.
pub fn parse_recognized_color(ctx: &mut ParseContext<'_>) -> Result<Rgb> {
    let start = ensure_keyword("color").parse(ctx)?;

    skip_ws(ctx)?;

    ensure_char('.')
        .with_context(ParseError::RecognizedColor, start)
        .fatal()
        .parse(ctx)?;

    skip_ws(ctx)?;

    let ident = Ident::into_parser()
        .with_context(ParseError::RecognizedColor, start)
        .fatal()
        .parse(ctx)?;

    

    todo!()
}

#[cfg(test)]
mod tests {
    use parserc::ParseContext;

    use super::parse_hex_color;

    #[test]
    fn test_hex_color() {
        parse_hex_color(&mut ParseContext::from("#000")).expect("hex3 color");

        parse_hex_color(&mut ParseContext::from("#0000")).expect_err("invalid hex color");

        parse_hex_color(&mut ParseContext::from("#ffffff")).expect("hex6 color");

        parse_hex_color(&mut ParseContext::from("#0000fff")).expect_err("invalid hex color");
    }
}
