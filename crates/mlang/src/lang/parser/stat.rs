use parserc::{ControlFlow, FromSrc, IntoParser, Parser, ParserExt};

use crate::lang::{
    ir::{ApplyTo, ChildrenOf, Enum, Group, Stat},
    parser::{node::parse_node, utils::skip_ws},
};

impl FromSrc for Stat {
    fn parse(ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        if let Some(opcode) = Enum::into_parser()
            .map(|v| Stat::Enum(Box::new(v)))
            .ok()
            .parse(ctx)?
        {
            return Ok(opcode);
        }

        if let Some(stat) = parse_node.ok().parse(ctx)? {
            return Ok(stat);
        }

        if let Some(group) = Group::into_parser().ok().parse(ctx)? {
            return Ok(Stat::Group(Box::new(group)));
        }

        if let Some(apply_to) = ApplyTo::into_parser().ok().parse(ctx)? {
            return Ok(Stat::ApplyTo(Box::new(apply_to)));
        }

        if let Some(children_of) = ChildrenOf::into_parser().ok().parse(ctx)? {
            return Ok(Stat::ChildrenOf(Box::new(children_of)));
        }

        skip_ws(ctx)?;

        assert_eq!(ctx.remaining(), 0, "Unparsed codes: {} ...", ctx.unparsed());

        return Err(ControlFlow::Recoverable);
    }
}
