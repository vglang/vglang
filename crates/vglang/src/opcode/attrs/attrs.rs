use super::*;

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
    WithMask(Box<WithMask>),
    Opacity(Box<Opacity>),
    WithClipPath(Box<WithClipPath>),
    TextLayout(Box<TextLayout>),
    WithTransform(Box<WithTransform>),
    EnableBackground(Box<EnableBackground>),
    WithFilter(Box<WithFilter>),
}
