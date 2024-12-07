use crate::{Layer, Text};

/// A type that representation a cotai script instruction.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IR {
    /// Pop the top(n) instructions from the `multiline instruction stack`.
    Pop(usize),
    /// A literal string value.
    String(String),
    /// A reference to animatable variable.
    Animated(String),
    /// A text element.
    Text(Box<Text>),

    /// A layer element.
    Layer(Box<Layer>),
}
