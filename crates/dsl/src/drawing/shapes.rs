use cotati_ir::Rect;

use crate::generator::Generator;

use super::Graphic;

impl<G> Graphic<G> for Rect
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push_from(self);
    }
}
