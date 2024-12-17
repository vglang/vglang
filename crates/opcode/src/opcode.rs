//! Defines the optimised intermediate instructions.

use crate::operand::{Fill, Stroke, Text, TextSpan, Variable};

/// Opcodes for vglang.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    /// Create a new `canvas` to draw vector graphics.
    Canvas(usize),
    /// Render a `text` element.
    Text(Box<Text>),
    /// Render a `text-span` element.
    TextSpan(Box<TextSpan>),
    /// characters content of the text/text-span element.
    Characters(Box<Variable<String>>),
    /// Apply inheritable property `Fill` to all children instructions.
    Fill(Box<Fill>),
    /// Apply inheritable property `Stroke` to all children instructions.
    Stroke(Box<Stroke>),
    /// Popup elements, indicates that the popup elements ared fully rendered.
    Pop(usize),
}

impl From<Text> for Opcode {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}

impl From<TextSpan> for Opcode {
    fn from(value: TextSpan) -> Self {
        Self::TextSpan(Box::new(value))
    }
}

impl From<Fill> for Opcode {
    fn from(value: Fill) -> Self {
        Self::Fill(Box::new(value))
    }
}

impl From<Stroke> for Opcode {
    fn from(value: Stroke) -> Self {
        Self::Stroke(Box::new(value))
    }
}

impl From<Variable<String>> for Opcode {
    fn from(value: Variable<String>) -> Self {
        Self::Characters(Box::new(value))
    }
}
