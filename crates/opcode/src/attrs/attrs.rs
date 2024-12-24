use super::{Fill, Stroke};

/// If an attribute want to be applied to a graphic element, it should implement this trait.
pub trait Apply<Target: ?Sized> {
    /// convert self into [`Attr`]
    fn into_attribute(self) -> Attr;
}

macro_rules! impl_apply_to {
    (box,$name: ident, $to: ident) => {
        impl $crate::attrs::Apply<$to> for $name {
            fn into_attribute(self) -> $crate::attrs::Attr {
                $crate::attrs::Attr::$name(Box::new(self))
            }
        }
    };
    ($name: ident, $to: ident) => {
        impl $crate::attrs::Apply<$to> for $name {
            fn into_attribute(self) -> $crate::attrs::Attr {
                $crate::attrs::Attr::$name(self)
            }
        }
    };
    (listof, $name: ident, $to: ident) => {
        concat_idents::concat_idents!(field_name = ListOf, $name {
            impl $crate::attrs::Apply<$to> for Vec<$name> {
                fn into_attribute(self) -> $crate::attrs::Attr {
                    $crate::attrs::Attr::field_name(Box::new(self))
                }
            }
        });
    };
}

pub(crate) use impl_apply_to;

/// the attribute types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attr {
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
}
