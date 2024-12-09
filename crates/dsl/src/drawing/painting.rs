use cotati_ir::{Fill, Stroke};

use crate::generator::Generator;

use super::{Appliable, Graphic};

impl Appliable for Fill {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator,
    {
        |g: &mut G| {
            g.push_fill(self);
            graphic.draw(g);
            g.pop(1);
        }
    }
}

impl Appliable for Stroke {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator,
    {
        |g: &mut G| {
            g.push_stroke(self);
            graphic.draw(g);
            g.pop(1);
        }
    }
}
