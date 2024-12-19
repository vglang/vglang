use vglang_opcode::operand::Transform;
use vglang_targets::Builder;

use crate::Appliable;

impl Appliable for Transform {
    fn apply<G, C>(self, graphic: C) -> impl crate::Graphic<G>
    where
        C: crate::Graphic<G>,
        G: Builder,
    {
        move |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}
