use parserc::{ensure_char, IntoParser, ParseContext, Parser, ParserExt, Result, Span};

use crate::{
    lang::{ir::Ident, parser::ParseError},
    opcode::variable::Path,
};

/// Parse named register path.
pub fn parse_named_register(ctx: &mut ParseContext<'_>) -> Result<(Path, Span)> {
    let start = ensure_char('$').parse(ctx)?;

    let name = Ident::into_parser()
        .with_context(ParseError::HexColor, start)
        .fatal()
        .parse(ctx)?;

    let path = Path::Named(name.0);

    let span = start.extend_to_inclusive(name.1);

    Ok((path, span))
}

#[cfg(test)]
mod tests {
    use parserc::ParseContext;

    use super::parse_named_register;

    #[test]
    fn test_named_register() {
        parse_named_register(&mut ParseContext::from("$hello")).expect("$hello");

        parse_named_register(&mut ParseContext::from("$ hello")).expect_err("$ hello");
        parse_named_register(&mut ParseContext::from("$1hello")).expect_err("$1hello");
    }
}
