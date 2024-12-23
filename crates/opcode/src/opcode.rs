use crate::{attrs::Attr, el::Container, el::Shape};

/// hight level `opcode` for vglang.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    /// Push an attributes into drawing stack.
    ///
    /// The drawing stack is clean up by executing the [`Opcode::Group`] or [`Opcode::Draw`] instruction.
    Apply(Attr),
    /// Draws a graphics elements group.
    Group(Container),
    /// Pop up a group of graphics elements, and draw them.
    Pop,
    /// Draws a shape.
    Draw(Shape),
}

impl From<Attr> for Opcode {
    fn from(value: Attr) -> Self {
        Self::Apply(value)
    }
}

impl From<Container> for Opcode {
    fn from(value: Container) -> Self {
        Self::Group(value)
    }
}

impl From<Shape> for Opcode {
    fn from(value: Shape) -> Self {
        Self::Draw(value)
    }
}
