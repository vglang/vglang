use super::{ApplyTo, ContentOf, Graphics};

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

impl<Attrs, Shape, Target> ContentOf<Target> for ApplyShape<Attrs, Shape>
where
    Attrs: ApplyTo<Shape> + Graphics,
    Shape: Element + Graphics + ContentOf<Target>,
    crate::opcode::el::Container: From<Target>,
{
}

/// A wrapper [`Graphics`] returns by container's apply function.
pub struct ApplyContainer<Attrs, Container> {
    pub attrs: Attrs,
    pub container: Container,
}

impl<Attrs, Container, Target> ContentOf<Target> for ApplyContainer<Attrs, Container>
where
    Attrs: ApplyTo<Container> + Graphics,
    Container: Element + Graphics + ContentOf<Target>,
    crate::opcode::el::Container: From<Target>,
{
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
    ) -> ApplyChildrenContainer<Attrs, Container, Children>
    where
        Children: ContentOf<Container>,
    {
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

impl<Container, Children, Target> ContentOf<Target> for ContainerChildren<Container, Children>
where
    Container: Element + Graphics + ContentOf<Target>,
    crate::opcode::el::Container: From<Container> + From<Target>,
    Children: ContentOf<Container>,
{
}

impl<Container, Children> ContainerChildren<Container, Children>
where
    Container: Element + Graphics,
    crate::opcode::el::Container: From<Container>,
    Children: ContentOf<Container>,
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
    Children: ContentOf<Container> + Graphics,
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
    Children: ContentOf<Container> + Graphics,
{
    fn build(self, builder: &mut super::BuildContext) {
        self.attrs.build(builder);
        self.container.build(builder);
        self.children.build(builder);
        builder.pop();
    }
}

impl<Attrs, Container, Children, Target> ContentOf<Target>
    for ApplyChildrenContainer<Attrs, Container, Children>
where
    Attrs: ApplyTo<Container> + Graphics,
    Container: Element + Graphics + ContentOf<Target>,
    crate::opcode::el::Container: From<Container> + From<Target>,
    Children: ContentOf<Container>,
{
}
