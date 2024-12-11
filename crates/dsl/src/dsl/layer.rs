use vglang_ir::{Layer, Measurement, PreserveAspectRatio, ViewBox};

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

/// An extension trait for [`ViewBox`].
pub trait ViewBoxDsl {
    fn with_aspect<P>(self, value: P) -> ViewBox
    where
        PreserveAspectRatio: From<P>;
}

impl<X, Y, W, H> ViewBoxDsl for (X, Y, W, H)
where
    Measurement: From<X> + From<Y> + From<W> + From<H>,
{
    fn with_aspect<P>(self, value: P) -> ViewBox
    where
        PreserveAspectRatio: From<P>,
    {
        ViewBox::from(self).aspect(value)
    }
}
