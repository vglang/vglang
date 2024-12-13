use super::Variable;

/// Instruction of arithmetic expression
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Smath {
    /// Operand of an arithmetic expression
    Operand(Variable),
    /// operator `+`
    Add,
    /// operator `-`
    Sub,
    /// operator `*`
    Mul,
    /// operator `/`
    Div,
}
