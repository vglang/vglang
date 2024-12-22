use vglang_opcode::operand::Pattern;
use vglang_targets::Builder;

use crate::{Appliable, Graphic};

impl Appliable for Pattern {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Builder,
    {
        |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}
