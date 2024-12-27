use super::{ClipPathed, Fill, Font, Id, Masked, Opacity, Stroke, TextLayout, ViewBox};

/// the attribute types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attr {
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
    Id(Box<Id>),
    Font(Box<Font>),
    ViewBox(Box<ViewBox>),
    Masked(Box<Masked>),
    Opacity(Box<Opacity>),
    ClipPathed(Box<ClipPathed>),
    TextLayout(Box<TextLayout>),
}
