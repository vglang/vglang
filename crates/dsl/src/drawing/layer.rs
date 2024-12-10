use cotati_ir::Layer;

use crate::generator::Generator;

use super::Graphic;

/// Create a layer graphic element.
pub fn layer<G, C>(attrs: Layer, content: C) -> impl Graphic<G>
where
    G: Generator,
    C: Graphic<G>,
{
    |g: &mut G| {
        g.push_from(attrs);
        content.draw(g);
        g.pop(1);
    }
}
