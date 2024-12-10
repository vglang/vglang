use cotati_ir::{Font, Text};

use crate::generator::Generator;

use super::{Appliable, Graphic, WithContent};

impl WithContent for Text {
    fn with_content<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator,
    {
        |g: &mut G| {
            g.push_from(self);
            graphic.draw(g);
            g.pop(1);
        }
    }
}

impl Appliable for Font {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator,
    {
        move |g: &mut G| {
            g.push_from(self);
            graphic.draw(g);
            g.pop(1);
        }
    }
}
