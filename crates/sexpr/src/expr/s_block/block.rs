use super::{Fill, Stext, StextSpan, Stroke};

/// sexpr to create a block.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sblock {
    /// Create a text block.
    Text(Stext),
    /// create a text-span block.
    TextSpan(StextSpan),
    /// Create a fill block.
    Fill(Fill),
    /// Create a fill block.
    Stroke(Stroke),
    /// Pop a block.
    End,
}
