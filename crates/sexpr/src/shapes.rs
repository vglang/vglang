use vglang_opcode::operand::Rect;
use vglang_targets::Builder;

use crate::Graphic;

impl<G> Graphic<G> for Rect
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}
