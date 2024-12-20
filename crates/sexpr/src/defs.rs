use crate::{Appliable, Graphic};
use vglang_opcode::operand::{Id, Use};
use vglang_targets::Builder;

impl Appliable for Id {
    fn apply<G, C>(self, graphic: C) -> impl crate::Graphic<G>
    where
        C: crate::Graphic<G>,
        G: Builder,
    {
        |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}

impl<G> Graphic<G> for Use
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}
