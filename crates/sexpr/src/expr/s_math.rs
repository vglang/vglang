use super::Svariable;

/// Instruction of arithmetic expression
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Smath {
    /// Operand of an arithmetic expression
    Operand(Svariable),
    /// operator `+`
    Add,
    /// operator `-`
    Sub,
    /// operator `*`
    Mul,
    /// operator `/`
    Div,
}
