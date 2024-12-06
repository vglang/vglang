use cotati_ir::Text;

use crate::generator::Generator;

use super::Graphic;

/// Create a text graphic element.
pub fn text<G, C>(attrs: Text, content: C) -> impl Graphic<G>
where
    G: Generator,
    C: Graphic<G>,
{
    |g: &mut G| {
        g.push_text(attrs);
        content.draw(g);
        g.pop(1);
    }
}
