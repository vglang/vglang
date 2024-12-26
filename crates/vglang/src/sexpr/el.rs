use crate::opcode::el::{Container, Else, If};

use super::{ApplyTo, Graphics};

/// All elements must implement this trait.
pub trait Element {}

/// A wrapper [`Graphics`] returns by shape's apply function.
pub struct ApplyShape<Attrs, Shape> {
    pub attrs: Attrs,
    pub shape: Shape,
}

impl<Attrs, Shape> Graphics for ApplyShape<Attrs, Shape>
where
    Attrs: ApplyTo<Shape> + Graphics,
    Shape: Element + Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.attrs.build(builder);
        self.shape.build(builder);
    }
}

/// A wrapper [`Graphics`] returns by container's apply function.
pub struct ApplyContainer<Attrs, Container> {
    pub attrs: Attrs,
    pub container: Container,
}

impl<Attrs, Container> ApplyContainer<Attrs, Container>
where
    Attrs: ApplyTo<Container> + Graphics,
    Container: Element + Graphics,
    crate::opcode::el::Container: From<Container>,
{
    /// Apply the children node.
    pub fn children<Children>(
        self,
        children: Children,
    ) -> ApplyChildrenContainer<Attrs, Container, Children> {
        ApplyChildrenContainer {
            attrs: self.attrs,
            container: self.container,
            children,
        }
    }
}

impl<Attrs, El> Graphics for ApplyContainer<Attrs, El>
where
    Attrs: ApplyTo<El> + Graphics,
    El: Element + Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.attrs.build(builder);
        self.container.build(builder);
    }
}

/// A wrapper [`Graphics`] returns by calling container's children function.
pub struct ContainerChildren<Container, Children> {
    pub container: Container,
    pub children: Children,
}

impl<Container, Children> ContainerChildren<Container, Children>
where
    Container: Element + Graphics,
    crate::opcode::el::Container: From<Container>,
{
    /// Apply the children node.
    pub fn apply<Attrs>(self, attrs: Attrs) -> ApplyChildrenContainer<Attrs, Container, Children>
    where
        Attrs: ApplyTo<Container>,
    {
        ApplyChildrenContainer {
            attrs,
            container: self.container,
            children: self.children,
        }
    }
}

impl<Container, Children> Graphics for ContainerChildren<Container, Children>
where
    Container: Element + Graphics,
    crate::opcode::el::Container: From<Container>,
    Children: Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.container.build(builder);
        self.children.build(builder);
        builder.pop();
    }
}

/// A wrapper [`Graphics`] returns by [`ApplyContainer::children`] or container's `children` function.
pub struct ApplyChildrenContainer<Attrs, Container, Children> {
    pub attrs: Attrs,
    pub container: Container,
    pub children: Children,
}

impl<Attrs, Container, Children> Graphics for ApplyChildrenContainer<Attrs, Container, Children>
where
    Attrs: ApplyTo<Container> + Graphics,
    Container: Element + Graphics,
    crate::opcode::el::Container: From<Container>,
    Children: Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.attrs.build(builder);
        self.container.build(builder);
        self.children.build(builder);
        builder.pop();
    }
}

impl If {
    pub fn children<C>(self, children: C) -> ContainerChildren<If, C> {
        ContainerChildren {
            container: self,
            children,
        }
    }
}

impl Element for If {}

/// A composite graphics that combine `if` and `else` blocks.
pub struct IfElse<IfExpr, Children>(IfExpr, Children);

impl<IfExpr, Children> Element for IfElse<IfExpr, Children> {}

impl<IfExpr, Children> Graphics for IfElse<IfExpr, Children>
where
    IfExpr: Graphics,
    Children: Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.0.build(builder);
        builder.push(Container::Else(Else));
        self.1.build(builder);
        builder.pop();
    }
}

/// A helper trait that conver accept a content graphics and returns [`IfElse`].
pub trait IntoIfElse {
    #[allow(non_snake_case)]
    fn Else<C>(self, children: C) -> IfElse<Self, C>
    where
        Self: Sized;
}

impl<IfContainer> IntoIfElse for ContainerChildren<If, IfContainer> {
    fn Else<C>(self, children: C) -> IfElse<Self, C>
    where
        Self: Sized,
    {
        IfElse(self, children)
    }
}
