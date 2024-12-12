use super::{Scall, ScontrolFlow, Svariable};

/// sexpr variant for vglang.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sexpr {
    /// a syscall sexpr.
    Call(Box<Scall>),
    /// a control-flow sexpr.
    ControlFlow(Box<ScontrolFlow>),
    /// Mixin an entity by [`Svariable`].
    Mixin(Svariable),
}
