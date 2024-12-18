use vglang_opcode::operand::Canvas;
use vglang_targets::Builder;

use crate::{Graphic, WithContent};

impl WithContent for Canvas {
    fn with_content<G, C>(self, graphic: C) -> impl Graphic<G>
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
