mod text;
pub use text::*;

use super::{Sattribute, Smath, Svariable};

/// Instruction of bool expression.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sbool {
    /// operator `!=`
    Neq,
    /// operator `==`
    Eq,
    /// operator `>`
    Gt,
    /// operator `<`
    Lt,
    /// operator `||`
    Or,
    /// operator `&&`
    And,
    /// Operand of a bool expression.
    Operand(Box<Vec<Smath>>),
}

/// control-flow instruction.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScontrolFlow {
    /// Push a `if` control-flow block.
    If(Vec<Sbool>),

    /// Push a elseif control-flow block.
    ElseIf(Vec<Sbool>),

    /// Push a else control-flow block.
    Else,

    /// Push a `foreach` control-flow block.
    Foreach {
        /// variable name for foreach item.
        item: String,
        /// Operand of foreach expression.
        operand: Svariable,
    },

    /// Push a `for range` control-flow block.
    Range {
        /// index variable name.
        index: String,
        /// The lower bound of the range (inclusive).
        start: Svariable,
        /// The upper bound of the range (exclusive).
        end: Svariable,
    },

    /// Apply an attributes block.
    Attrs(Vec<Sattribute>),

    /// Apply a text block.
    Text(Stext),

    /// Apply a text-span block.
    TextSpan(StextSpan),

    /// Push a `close tag` for the latest control-flow block.
    End,
}
