use parserc::{
    ensure_char, ensure_keyword, take_while, ControlFlow, FromSrc, IntoParser, ParseContext,
    ParseOkOr, Parser, ParserExt, Result,
};

use crate::lang::ir::{Color, Ident, LitColor, Rgb};

use super::{utils::skip_ws, ParseError};

/// Parse color from hex format.
pub fn parse_hex_color(ctx: &mut ParseContext<'_>) -> Result<LitColor> {
    let start = ensure_char('#').parse(ctx)?;

    let body = take_while(|c| c.is_ascii_hexdigit())
        .ok_or(ParseError::HexColor, start)
        .parse(ctx)?;

    let span = start.extend_to_inclusive(body);

    match body.len() {
        3 => {
            let mut body = ctx.as_str(body).chars();

            let r = body.next().unwrap();
            let g = body.next().unwrap();
            let b = body.next().unwrap();

            let r: u8 = u8::from_str_radix(&format!("{}{}", r, r), 16).unwrap();
            let g: u8 = u8::from_str_radix(&format!("{}{}", g, g), 16).unwrap();
            let b: u8 = u8::from_str_radix(&format!("{}{}", b, b), 16).unwrap();

            return Ok(LitColor::Rgb(span, Rgb(r, g, b)));
        }
        6 => {
            let body = ctx.as_str(body);

            let r: u8 = u8::from_str_radix(&body[0..2], 16).unwrap();
            let g: u8 = u8::from_str_radix(&body[2..4], 16).unwrap();
            let b: u8 = u8::from_str_radix(&body[4..6], 16).unwrap();

            return Ok(LitColor::Rgb(span, Rgb(r, g, b)));
        }
        _ => {
            ctx.on_fatal(ParseError::HexColor, start);
            return Err(ControlFlow::Fatal);
        }
    }
}

/// Parse recognized color: color.blue, etc.
pub fn parse_recognized_color(ctx: &mut ParseContext<'_>) -> Result<LitColor> {
    let start = ensure_keyword("color").parse(ctx)?;

    skip_ws(ctx)?;

    ensure_char('.')
        .fatal(ParseError::RecognizedColor, start)
        .parse(ctx)?;

    skip_ws(ctx)?;

    let ident = Ident::into_parser()
        .fatal(ParseError::RecognizedColor, start)
        .parse(ctx)?;

    let color = match ident.0.as_str() {
        "aliceblue" => Color::Aliceblue,
        "antiquewhite" => Color::Antiquewhite,
        "aqua" => Color::Aqua,
        "aquamarine" => Color::Aquamarine,
        "azure" => Color::Azure,
        "beige" => Color::Beige,
        "bisque" => Color::Bisque,
        "black" => Color::Black,
        "blanchedalmond" => Color::Blanchedalmond,
        "blue" => Color::Blue,
        "blueviolet" => Color::Blueviolet,
        "brown" => Color::Brown,
        "burlywood" => Color::Burlywood,
        "cadetblue" => Color::Cadetblue,
        "chartreuse" => Color::Chartreuse,
        "chocolate" => Color::Chocolate,
        "coral" => Color::Coral,
        "cornflowerblue" => Color::Cornflowerblue,
        "cornsilk" => Color::Cornsilk,
        "crimson" => Color::Crimson,
        "cyan" => Color::Cyan,
        "darkblue" => Color::Darkblue,
        "darkcyan" => Color::Darkcyan,
        "darkgoldenrod" => Color::Darkgoldenrod,
        "darkgray" => Color::Darkgray,
        "darkgreen" => Color::Darkgreen,
        "darkgrey" => Color::Darkgrey,
        "darkkhaki" => Color::Darkkhaki,
        "darkmagenta" => Color::Darkmagenta,
        "darkolivegreen" => Color::Darkolivegreen,
        "darkorange" => Color::Darkorange,
        "darkorchid" => Color::Darkorchid,
        "darkred" => Color::Darkred,
        "darksalmon" => Color::Darksalmon,
        "darkseagreen" => Color::Darkseagreen,
        "darkslateblue" => Color::Darkslateblue,
        "darkslategray" => Color::Darkslategray,
        "darkslategrey" => Color::Darkslategrey,
        "darkturquoise" => Color::Darkturquoise,
        "darkviolet" => Color::Darkviolet,
        "deeppink" => Color::Deeppink,
        "deepskyblue" => Color::Deepskyblue,
        "dimgray" => Color::Dimgray,
        "dimgrey" => Color::Dimgrey,
        "dodgerblue" => Color::Dodgerblue,
        "firebrick" => Color::Firebrick,
        "floralwhite" => Color::Floralwhite,
        "forestgreen" => Color::Forestgreen,
        "fuchsia" => Color::Fuchsia,
        "gainsboro" => Color::Gainsboro,
        "ghostwhite" => Color::Ghostwhite,
        "gold" => Color::Gold,
        "goldenrod" => Color::Goldenrod,
        "gray" => Color::Gray,
        "grey" => Color::Grey,
        "green" => Color::Green,
        "greenyellow" => Color::Greenyellow,
        "honeydew" => Color::Honeydew,
        "hotpink" => Color::Hotpink,
        "indianred" => Color::Indianred,
        "indigo" => Color::Indigo,
        "ivory" => Color::Ivory,
        "khaki" => Color::Khaki,
        "lavender" => Color::Lavender,
        "lavenderblush" => Color::Lavenderblush,
        "lawngreen" => Color::Lawngreen,
        "lemonchiffon" => Color::Lemonchiffon,
        "lightblue" => Color::Lightblue,
        "lightcoral" => Color::Lightcoral,
        "lightcyan" => Color::Lightcyan,
        "lightgoldenrodyellow" => Color::Lightgoldenrodyellow,
        "lightgray" => Color::Lightgray,
        "lightgreen" => Color::Lightgreen,
        "lightgrey" => Color::Lightgrey,
        "lightpink" => Color::Lightpink,
        "lightsalmon" => Color::Lightsalmon,
        "lightseagreen" => Color::Lightseagreen,
        "lightskyblue" => Color::Lightskyblue,
        "lightslategray" => Color::Lightslategray,
        "lightslategrey" => Color::Lightslategrey,
        "lightsteelblue" => Color::Lightsteelblue,
        "lightyellow" => Color::Lightyellow,
        "lime" => Color::Lime,
        "limegreen" => Color::Limegreen,
        "linen" => Color::Linen,
        "magenta" => Color::Magenta,
        "maroon" => Color::Maroon,
        "mediumaquamarine" => Color::Mediumaquamarine,
        "mediumblue" => Color::Mediumblue,
        "mediumorchid" => Color::Mediumorchid,
        "mediumpurple" => Color::Mediumpurple,
        "mediumseagreen" => Color::Mediumseagreen,
        "mediumslateblue" => Color::Mediumslateblue,
        "mediumspringgreen" => Color::Mediumspringgreen,
        "mediumturquoise" => Color::Mediumturquoise,
        "mediumvioletred" => Color::Mediumvioletred,
        "midnightblue" => Color::Midnightblue,
        "mintcream" => Color::Mintcream,
        "mistyrose" => Color::Mistyrose,
        "moccasin" => Color::Moccasin,
        "navajowhite" => Color::Navajowhite,
        "navy" => Color::Navy,
        "oldlace" => Color::Oldlace,
        "olive" => Color::Olive,
        "olivedrab" => Color::Olivedrab,
        "orange" => Color::Orange,
        "orangered" => Color::Orangered,
        "orchid" => Color::Orchid,
        "palegoldenrod" => Color::Palegoldenrod,
        "palegreen" => Color::Palegreen,
        "paleturquoise" => Color::Paleturquoise,
        "palevioletred" => Color::Palevioletred,
        "papayawhip" => Color::Papayawhip,
        "peachpuff" => Color::Peachpuff,
        "peru" => Color::Peru,
        "pink" => Color::Pink,
        "plum" => Color::Plum,
        "powderblue" => Color::Powderblue,
        "purple" => Color::Purple,
        "red" => Color::Red,
        "rosybrown" => Color::Rosybrown,
        "royalblue" => Color::Royalblue,
        "saddlebrown" => Color::Saddlebrown,
        "salmon" => Color::Salmon,
        "sandybrown" => Color::Sandybrown,
        "seagreen" => Color::Seagreen,
        "seashell" => Color::Seashell,
        "sienna" => Color::Sienna,
        "silver" => Color::Silver,
        "skyblue" => Color::Skyblue,
        "slateblue" => Color::Slateblue,
        "slategray" => Color::Slategray,
        "slategrey" => Color::Slategrey,
        "snow" => Color::Snow,
        "springgreen" => Color::Springgreen,
        "steelblue" => Color::Steelblue,
        "tan" => Color::Tan,
        "teal" => Color::Teal,
        "thistle" => Color::Thistle,
        "tomato" => Color::Tomato,
        "turquoise" => Color::Turquoise,
        "violet" => Color::Violet,
        "wheat" => Color::Wheat,
        "white" => Color::White,
        "whitesmoke" => Color::Whitesmoke,
        "yellow" => Color::Yellow,
        "yellowgreen" => Color::Yellowgreen,
        _ => {
            ctx.on_fatal(ParseError::RecognizedColor, start);
            return Err(ControlFlow::Fatal);
        }
    };

    Ok(LitColor::Recognized(
        start.extend_to_inclusive(ident.1),
        color,
    ))
}

impl FromSrc for LitColor {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        parse_hex_color.or(parse_recognized_color).parse(ctx)
    }
}

#[cfg(test)]
mod tests {
    use parserc::{FromSrc, ParseContext};

    use crate::lang::ir::LitColor;

    #[test]
    fn test_color() {
        let mut ctx = ParseContext::from("#a");
        LitColor::parse(&mut ctx).expect_err("invalid color");

        println!("{:?}", ctx.last_error());
    }
}
