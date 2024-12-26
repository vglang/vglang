use super::{Characters, Else, For, Foreach, Group, If, Text, TextSpan};

/// the container element types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Container {
    If(Box<If>),
    Else(Else),
    For(Box<For>),
    Foreach(Box<Foreach>),
    Group(Group),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
}

/// the graphic types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape {
    Characters(Box<Characters>),
}
