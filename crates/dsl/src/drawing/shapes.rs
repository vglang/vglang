use cotati_ir::Rect;

use crate::generator::Generator;

use super::Graphic;

/// Create a rect graphic element.
pub fn rect<G, C>(attrs: Rect) -> impl Graphic<G>
where
    G: Generator,
    C: Graphic<G>,
{
    |g: &mut G| {
        g.push_rect(attrs);
    }
}
