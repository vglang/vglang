use super::{Characters, Group, Text, TextSpan};

/// the container element types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Container {
    Group(Group),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
}

/// the graphic types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape {
    Characters(Box<Characters>),
}
