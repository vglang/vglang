use vglang_opcode::{
    attrs::{Apply, Attr},
    el::ShapeElement,
};
use vglang_surface::Builder;

use crate::Graphics;

/// Extension trait to add shape element `sexpr`s.
pub trait Sshape<E> {
    /// [`child`](Scontainer::child) function returns type.
    type Output;

    /// Apply a attributes to this container element.
    fn apply<A>(self, attr: A) -> Self::Output
    where
        A: Apply<E>;
}

/// A wrapper of [`ShapeElement`] returned by `sexpr`s.
pub struct SshapeElement<E>
where
    E: ShapeElement,
{
    attrs: Vec<Attr>,
    element: E,
}

impl<E> Sshape<E> for SshapeElement<E>
where
    E: ShapeElement,
{
    type Output = Self;

    fn apply<A>(mut self, attr: A) -> Self::Output
    where
        A: Apply<E>,
    {
        self.attrs.push(attr.into_attribute());
        self
    }
}

impl<E> Sshape<E> for E
where
    E: ShapeElement,
{
    type Output = SshapeElement<E>;

    fn apply<A>(self, attr: A) -> Self::Output
    where
        A: Apply<E>,
    {
        SshapeElement {
            attrs: vec![attr.into_attribute()],
            element: self,
        }
    }
}

impl<B, E> Graphics<B> for SshapeElement<E>
where
    B: Builder,
    E: ShapeElement,
{
    fn build(self, builder: &mut B) {
        for attr in self.attrs {
            builder.push(attr.into());
        }

        builder.push(self.element.into_opcode());
    }
}
