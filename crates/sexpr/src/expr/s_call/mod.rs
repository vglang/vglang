use super::Sexpr;

/// sexpr to create syscall instruction.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Scall {}

impl From<Scall> for Sexpr {
    fn from(value: Scall) -> Self {
        Self::Call(Box::new(value))
    }
}
