use super::{Block, ControlFlow, Property, Svariable, Syscall};

/// sexpr variant for vglang.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sexpr {
    /// a syscall sexpr.
    Syscall(Box<Syscall>),
    /// a control-flow sexpr.
    ControlFlow(Box<ControlFlow>),
    /// Create a block.
    Block(Box<Block>),
    /// Create a inherit property.
    Property(Box<Property>),
    /// Mixin an entity by [`Svariable`].
    Mixin(Box<Svariable>),
}

impl From<Svariable> for Sexpr {
    fn from(value: Svariable) -> Self {
        Self::Mixin(Box::new(value))
    }
}

impl From<Block> for Sexpr {
    fn from(value: Block) -> Self {
        Self::Block(Box::new(value))
    }
}

impl From<ControlFlow> for Sexpr {
    fn from(value: ControlFlow) -> Self {
        Self::ControlFlow(Box::new(value))
    }
}
impl From<Syscall> for Sexpr {
    fn from(value: Syscall) -> Self {
        Self::Syscall(Box::new(value))
    }
}

impl From<Property> for Sexpr {
    fn from(value: Property) -> Self {
        Self::Property(Box::new(value))
    }
}
