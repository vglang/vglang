use vglang_opcode::operand::Canvas;
use vglang_targets::Builder;

use crate::Graphic;

/// Create a layer graphic element.
pub fn canvas<G, C>(attrs: Canvas, content: C) -> impl Graphic<G>
where
    G: Builder,
    C: Graphic<G>,
{
    |g: &mut G| {
        g.push(attrs);
        content.draw(g);
        g.pop();
    }
}
