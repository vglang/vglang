mod painting;
pub use painting::*;

/// Apply inherit property to a block.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Property {
    /// Create a fill block.
    Fill(Fill),
    /// Create a fill block.
    Stroke(Stroke),
}
