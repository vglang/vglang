use vglang_opcode::operand::{Fill, Paint, RefBy, Stroke};
use vglang_targets::Builder;

use super::{Appliable, Graphic};

impl Appliable for Fill {
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

impl Appliable for Stroke {
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

/// A helper trait that convert self into [`Paint`](vglang_opcode::operand::Paint)
pub trait Sgradient {
    fn gradient(self) -> Paint;
}

impl Sgradient for String {
    fn gradient(self) -> Paint {
        Paint::Gradient(RefBy::Named(self))
    }
}

impl Sgradient for &str {
    fn gradient(self) -> Paint {
        Paint::Gradient(RefBy::Named(self.to_owned()))
    }
}
