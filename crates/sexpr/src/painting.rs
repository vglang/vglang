use vglang_opcode::operand::{Fill, FuncIRI, Paint, Stroke};
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

impl<T> Sgradient for T
where
    FuncIRI: From<T>,
{
    fn gradient(self) -> Paint {
        Paint::Gradient(self.into())
    }
}

/// A helper trait that convert self into [`Paint`](vglang_opcode::operand::Paint)
pub trait Spattern {
    fn pattern(self) -> Paint;
}

impl<T> Spattern for T
where
    FuncIRI: From<T>,
{
    fn pattern(self) -> Paint {
        Paint::Pattern(self.into())
    }
}
