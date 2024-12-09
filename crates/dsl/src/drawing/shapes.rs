use cotati_ir::Rect;

use crate::generator::Generator;

use super::Graphic;

/// Create a rect graphic element.
pub fn rect() -> Rect {
    Rect::default()
}

impl<G> Graphic<G> for Rect
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push_rect(self);
    }
}
