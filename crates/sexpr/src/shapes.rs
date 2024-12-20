use vglang_opcode::operand::{Circle, Polyline, Rect};
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

impl<G> Graphic<G> for Circle
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}

impl<G> Graphic<G> for Polyline
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}
