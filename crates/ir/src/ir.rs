use crate::{Fill, Font, Layer, Rect, Stroke, Text};

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

    /// Basic rect shape.
    Rect(Box<Rect>),

    /// Fill commander.
    Fill(Box<Fill>),

    /// Stroke commander.
    Stroke(Box<Stroke>),

    /// Font properties.
    Font(Box<Font>),
}

impl From<Text> for IR {
    fn from(value: Text) -> Self {
        IR::Text(Box::new(value))
    }
}

impl From<Layer> for IR {
    fn from(value: Layer) -> Self {
        IR::Layer(Box::new(value))
    }
}

impl From<Rect> for IR {
    fn from(value: Rect) -> Self {
        IR::Rect(Box::new(value))
    }
}

impl From<Fill> for IR {
    fn from(value: Fill) -> Self {
        IR::Fill(Box::new(value))
    }
}

impl From<Stroke> for IR {
    fn from(value: Stroke) -> Self {
        IR::Stroke(Box::new(value))
    }
}

impl From<Font> for IR {
    fn from(value: Font) -> Self {
        IR::Font(Box::new(value))
    }
}
