use parserc::{
    ensure_char, ensure_keyword, FromSrc, IntoParser, ParseContext, Parser, ParserExt, Result,
};

use crate::{
    ir::{Enum, Fields, Ident, Node, Stat},
    parser::{
        utils::{parse_prefix, skip_ws},
        EnumKind, NodeKind, ParseError,
    },
};

fn parse_node_body(ctx: &mut ParseContext<'_>) -> Result<Node> {
    let start = ctx.span();

    let (comments, properties) = parse_prefix(ctx)?;

    let ident = Ident::parse(ctx)?;

    skip_ws(ctx)?;

    let mixin = if let Some(_) = ensure_keyword("mixin").ok().parse(ctx)? {
        skip_ws(ctx)?;
        let mixin = Ident::into_parser()
            .fatal(ParseError::Node(NodeKind::MixinIdent), ctx.span())
            .parse(ctx)?;

        skip_ws(ctx)?;

        Some(mixin)
    } else {
        None
    };

    let fields = Fields::into_parser()
        .fatal(ParseError::Node(NodeKind::Fields), ctx.span())
        .parse(ctx)?;

    let end = ctx.span();

    Ok(Node {
        span: start.extend_to(end),
        comments,
        mixin,
        properties,
        ident,
        fields,
    })
}

pub(super) fn parse_node(ctx: &mut ParseContext<'_>) -> Result<Stat> {
    let keyword = ensure_keyword("el")
        .or(ensure_keyword("leaf"))
        .or(ensure_keyword("attr"))
        .or(ensure_keyword("data"))
        .or(ensure_keyword("mixin"))
        .parse(ctx)?;

    skip_ws(ctx)?;

    let node = parse_node_body(ctx)?;

    match ctx.as_str(keyword) {
        "el" => Ok(Stat::Element(Box::new(node))),
        "leaf" => Ok(Stat::Leaf(Box::new(node))),
        "attr" => Ok(Stat::Attr(Box::new(node))),
        "data" => Ok(Stat::Data(Box::new(node))),
        "mixin" => Ok(Stat::Mixin(Box::new(node))),
        _ => panic!("inner errro."),
    }
}

impl FromSrc for Enum {
    fn parse(ctx: &mut ParseContext<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let start = ctx.span();

        let (comments, properties) = parse_prefix(ctx)?;

        ensure_keyword("enum").parse(ctx)?;

        skip_ws(ctx)?;

        let ident = Ident::into_parser()
            .fatal(ParseError::Enum(EnumKind::Ident), ctx.span())
            .parse(ctx)?;

        skip_ws(ctx)?;

        ensure_char('{')
            .fatal(ParseError::Enum(EnumKind::BodyStart), ctx.span())
            .parse(ctx)?;

        let mut fields = vec![];

        while let Some(field) = parse_node_body.ok().parse(ctx)? {
            fields.push(field);

            skip_ws(ctx)?;

            skip_ws.parse(ctx)?;

            if ensure_char(',').ok().parse(ctx)?.is_none() {
                break;
            }

            skip_ws.parse(ctx)?;
        }

        let end = ensure_char('}')
            .fatal(ParseError::Enum(EnumKind::BodyEnd), ctx.span())
            .parse(ctx)?;

        Ok(Enum {
            span: start.extend_to_inclusive(end),
            comments,
            properties,
            ident,
            fields,
        })
    }
}
