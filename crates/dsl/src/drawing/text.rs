use cotati_ir::Text;

use crate::generator::Generator;

use super::{Graphic, WithContent};

impl WithContent for Text {
    fn content<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator,
    {
        |g: &mut G| {
            g.push_text(self);
            graphic.draw(g);
            g.pop(1);
        }
    }
}

/// Create a new `Text` element.
pub fn text() -> Text {
    Text::default()
}
