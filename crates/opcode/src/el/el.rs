use crate::Opcode;

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

/// If an element want to be applied as a child of a graphic element, it should implement this trait.
pub trait ContentOf<Target: ?Sized> {
    /// convert self into [`Attr`]
    fn into_opcode(self) -> Opcode;
}

macro_rules! impl_content_of {
    (box,$name: ident, $to: ident, $el: ident) => {
        impl $crate::el::ContentOf<$to> for $name {
                    fn into_opcode(self) -> $crate::Opcode {
                        $el::$name(Box::new(self)).into()
                    }
                }
    };
    ($name: ident, $to: ident, $el: ident) => {
        impl $crate::el::ContentOf<$to> for $name {
                    fn into_opcode(self) -> $crate::Opcode {
                        $el::$name(self).into()
                    }
                }
    };
    (listof, $name: ident, $to: ident, $el: ident) => {
        concat_idents::concat_idents!(field_name = ListOf, $name {
            impl $crate::el::ContentOf<$to> for Vec<$name> {
                fn into_opcode(self) -> $crate::Opcode {
                    $el::$name(self).into()
                }
            }
        });
    };
}

pub(super) use impl_content_of;

/// A marker trait that indicates the implementor is a graphics element.
pub trait Element {
    fn into_opcode(self) -> Opcode;
}

/// A marker trait that indicates the implementor is a shape element.
pub trait ShapeElement: Element {}

macro_rules! impl_shape_element {
    (box,$name: ident) => {
        impl Element for $name {
            fn into_opcode(self) -> Opcode {
                Shape::$name(Box::new(self)).into()
            }
        }

        impl ShapeElement for $name {}
    };
    ($name: ident) => {
        impl Element for $name {
            fn into_opcode(self) -> Opcode {
                Shape::$name(self).into()
            }
        }

        impl ShapeElement for $name {}
    };
}

impl_shape_element!(box, Characters);

/// A marker trait that indicates the implementor is a container element.
pub trait ContainerElement: Element {}

macro_rules! impl_container_element {
    (box, $name: ident) => {
        impl Element for $name {
            fn into_opcode(self) -> Opcode {
                Container::$name(Box::new(self)).into()
            }
        }

        impl ContainerElement for $name {}
    };
    ($name: ident) => {
        impl Element for $name {
            fn into_opcode(self) -> Opcode {
                Container::$name(self).into()
            }
        }

        impl ContainerElement for $name {}
    };
}

impl_container_element!(Group);
impl_container_element!(box, Text);
impl_container_element!(box, TextSpan);
