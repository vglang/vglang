use vglang_opcode::operand::{Font, Text, TextLayout, TextSpan};
use vglang_targets::Builder;

use crate::{Appliable, Graphic, WithContent};

impl WithContent for Text {
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

impl WithContent for TextSpan {
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

impl Appliable for Font {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Builder,
    {
        move |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}

impl Appliable for TextLayout {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Builder,
    {
        move |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}
