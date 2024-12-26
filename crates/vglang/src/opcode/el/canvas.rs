use crate::opcode::{data::Length, variable::Variable};

#[cfg(feature = "sexpr")]
use super::*;

/// Create a new layer into which the backend render child elements.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, If, For, Foreach, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canvas {
    /// a number (usually an integer) that represents the width of the rendering layer.
    pub width: Variable<Length>,
    /// a number (usually an integer) that represents the height of the rendering layer.
    pub height: Variable<Length>,
}

impl<W, H> From<(W, H)> for Canvas
where
    Length: From<W> + From<H>,
{
    fn from(value: (W, H)) -> Self {
        Canvas {
            width: Variable::Constant(value.0.into()),
            height: Variable::Constant(value.1.into()),
        }
    }
}
