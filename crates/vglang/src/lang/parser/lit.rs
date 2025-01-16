use parserc::{FromSrc, IntoParser, Parser, ParserExt};

use crate::lang::ir::{LitBool, LitColor, LitEnum, LitExpr, LitNum, LitStr};

impl FromSrc for LitExpr {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        LitBool::into_parser()
            .map(|lit| LitExpr::Bool(lit))
            .or(LitStr::into_parser().map(|lit| LitExpr::Str(lit)))
            .or(LitNum::into_parser().map(|lit| LitExpr::Num(lit)))
            .or(LitEnum::into_parser().map(|lit| LitExpr::Enum(lit)))
            .or(LitColor::into_parser().map(|lit| LitExpr::Color(lit)))
            .parse(ctx)
    }
}
