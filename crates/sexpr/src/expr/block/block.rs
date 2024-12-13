use super::{Text, TextSpan};

/// sexpr to create a block.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Block {
    /// Create a text block.
    Text(Text),
    /// create a text-span block.
    TextSpan(TextSpan),
    /// Pop a block.
    End,
}
