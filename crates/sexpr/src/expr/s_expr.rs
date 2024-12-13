use super::{Sblock, Scall, ScontrolFlow, Svariable};

/// sexpr variant for vglang.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sexpr {
    /// a syscall sexpr.
    Call(Box<Scall>),
    /// a control-flow sexpr.
    ControlFlow(Box<ScontrolFlow>),
    /// Create a block.
    Block(Box<Sblock>),
    /// Mixin an entity by [`Svariable`].
    Mixin(Box<Svariable>),
}

impl From<Svariable> for Sexpr {
    fn from(value: Svariable) -> Self {
        Self::Mixin(Box::new(value))
    }
}

impl From<Sblock> for Sexpr {
    fn from(value: Sblock) -> Self {
        Self::Block(Box::new(value))
    }
}

impl From<ScontrolFlow> for Sexpr {
    fn from(value: ScontrolFlow) -> Self {
        Self::ControlFlow(Box::new(value))
    }
}
impl From<Scall> for Sexpr {
    fn from(value: Scall) -> Self {
        Self::Call(Box::new(value))
    }
}
