use vglang_opcode::operand::{GradientStop, LinearGradient, RadialGradient};
use vglang_targets::Builder;

use crate::{Appliable, Graphic};

impl Appliable for LinearGradient {
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

impl Appliable for RadialGradient {
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

impl<G> Graphic<G> for GradientStop
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}
