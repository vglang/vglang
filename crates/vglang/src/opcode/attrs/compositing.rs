#[cfg(feature = "sexpr")]
use crate::opcode::el::*;
use crate::opcode::{
    data::{FuncIRI, Number},
    variable::Variable,
};

/// Use mask to a element.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", vglang_derive::attribute(boxed, Canvas))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithClipPath(pub Variable<FuncIRI>);

impl<T> From<T> for WithClipPath
where
    FuncIRI: From<T>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

/// Use mask to a element.
/// Use mask to a element.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", vglang_derive::attribute(boxed, Canvas, Use))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithMask(pub Variable<FuncIRI>);

impl<T> From<T> for WithMask
where
    FuncIRI: From<T>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

///Sspecifies object/group opacity
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(boxed, Canvas, Circle, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(pub Variable<Number>);

impl<T> From<T> for Opacity
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        let mut value = Number::from(value);
        if value.0 > 1.0 {
            value.0 = 1.0;
        }

        if value.0 < 0.0 {
            value.0 = 0.0;
        }

        Self(Variable::Constant(value))
    }
}
