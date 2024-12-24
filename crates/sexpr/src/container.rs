use vglang_opcode::{
    attrs::{Apply, Attr},
    el::{ContainerElement, ContentOf},
    Opcode,
};
use vglang_surface::Builder;

use crate::Graphics;

/// Extension trait to add container element `sexpr`s.
pub trait Scontainer<E>
where
    E: ContainerElement,
{
    /// [`child`](Scontainer::child) function returns type.
    type Output;

    /// Apply a attributes to this container element.
    fn apply<A>(self, attr: A) -> Self::Output
    where
        A: Apply<E>;

    /// Append a new child to this container.
    fn child<C>(self, child: C) -> Self::Output
    where
        C: ContentOf<E>;
}

/// A wrapper of [`ContainerElement`] returned by `sexpr`s.
pub struct ScontainerElement<E>
where
    E: ContainerElement,
{
    element: E,
    attrs: Vec<Attr>,
    children: Vec<Opcode>,
}

impl<E> Scontainer<E> for ScontainerElement<E>
where
    E: ContainerElement,
{
    type Output = Self;

    fn apply<A>(mut self, attr: A) -> Self::Output
    where
        A: Apply<E>,
    {
        self.attrs.push(attr.into_attribute());
        self
    }

    fn child<C>(mut self, child: C) -> Self::Output
    where
        C: ContentOf<E>,
    {
        self.children.push(child.into_opcode());
        self
    }
}

impl<B, E> Graphics<B> for ScontainerElement<E>
where
    B: Builder,
    E: ContainerElement,
{
    fn build(self, builder: &mut B) {
        for attr in self.attrs {
            builder.push(attr.into());
        }

        builder.push(self.element.into_opcode());

        for child in self.children {
            builder.push(child);
        }

        builder.pop();
    }
}

impl<E> Scontainer<E> for E
where
    E: ContainerElement,
{
    type Output = ScontainerElement<E>;

    fn apply<A>(self, attr: A) -> Self::Output
    where
        A: Apply<E>,
    {
        ScontainerElement {
            element: self,
            attrs: vec![attr.into_attribute()],
            children: vec![],
        }
    }

    fn child<C>(self, child: C) -> Self::Output
    where
        C: ContentOf<E>,
    {
        ScontainerElement {
            element: self,
            children: vec![child.into_opcode()],
            attrs: vec![],
        }
    }
}
