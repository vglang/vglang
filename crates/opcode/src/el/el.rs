use super::{Characters, Group, Text, TextSpan};

/// If an element want to be applied as a child of a graphic element, it should implement this trait.
pub trait ContentOf<Target> {
    type E;
    /// convert self into [`Attr`]
    fn into_element(self) -> Self::E;
}

macro_rules! impl_content_of {
    (box,$name: ident, $to: ident, $el: ident) => {
        impl $crate::el::ContentOf<$to> for $name {
            type E = $el;
            fn into_element(self) -> Self::E {
                Self::E::$name(Box::new(self))
            }
        }
    };
    ($name: ident, $to: ident, $el: ident) => {
        impl $crate::el::ContentOf<$to> for $name {
            type E = $el;
            fn into_element(self) -> Self::E {
                Self::E::$name(self)
            }
        }
    };
    (listof, $name: ident, $to: ident, $el: ident) => {
        concat_idents::concat_idents!(field_name = ListOf, $name {
            impl $crate::el::ContentOf<$to> for Vec<$name> {
                type E = $el;
                fn into_element(self) -> Self::E {
                    Self::E::field_name(Box::new(self))
                }
            }
        });
    };
}

pub(super) use impl_content_of;

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
