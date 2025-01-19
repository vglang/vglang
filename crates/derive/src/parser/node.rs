use parserc::FromSrc;

use crate::ir::Node;

impl FromSrc for Node {
    fn parse(_ctx: &mut parserc::ParseContext<'_>) -> parserc::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
