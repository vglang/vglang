use crate::opcode::data::Transform;

#[cfg(feature = "sexpr")]
use crate::opcode::el::*;

/// support for various international writing directions, such as left-to-right (e.g., Latin scripts) and
/// bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts).
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(
        boxed, Group, Text, TextSpan, Characters, Rect, Circle, Line, Polyline, Polygon
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transformed(pub Vec<Transform>);
