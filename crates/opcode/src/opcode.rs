//! Defines the optimised intermediate instructions.

use crate::operand::{
    Canvas, Circle, Fill, Font, Path, Rect, Stroke, Text, TextLayout, TextSpan, Transform, Variable,
};

/// Opcodes for vglang.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    /// Create a new `canvas` to draw vector graphics.
    Canvas(Box<Canvas>),
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
    /// Render a `rect` element.
    Rect(Box<Rect>),

    /// Render a `circle` element.
    Circle(Box<Circle>),

    /// Apply inheritable property `Font` to all children instructions.
    Font(Box<Font>),

    /// Apply inheritable property `TextLayout` to all children instructions.
    TextLayout(Box<TextLayout>),

    Transform(Box<Variable<Transform>>),

    Path(Box<Path>),

    /// Popup elements, indicates that the popup elements ared fully rendered.
    Pop(usize),
}

impl From<Path> for Opcode {
    fn from(value: Path) -> Self {
        Self::Path(Box::new(value))
    }
}

impl From<Transform> for Opcode {
    fn from(value: Transform) -> Self {
        Self::Transform(Box::new(Variable::Constant(value)))
    }
}

impl From<Circle> for Opcode {
    fn from(value: Circle) -> Self {
        Self::Circle(Box::new(value))
    }
}

impl From<TextLayout> for Opcode {
    fn from(value: TextLayout) -> Self {
        Self::TextLayout(Box::new(value))
    }
}

impl From<Font> for Opcode {
    fn from(value: Font) -> Self {
        Self::Font(Box::new(value))
    }
}

impl From<Rect> for Opcode {
    fn from(value: Rect) -> Self {
        Self::Rect(Box::new(value))
    }
}

impl From<Canvas> for Opcode {
    fn from(value: Canvas) -> Self {
        Self::Canvas(Box::new(value))
    }
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
