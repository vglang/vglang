use crate::opcode::{
    data::{Number, PreserveAspectRatio},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use crate::opcode::el::*;

/// It is often desirable to specify that a given set of graphics stretch to fit a particular container element.
/// The ‘viewBox’ attribute provides this capability.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", vglang_derive::attribute(boxed, Canvas))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBox {
    /// ViewBox left-top x coordinate,
    pub minx: Variable<Number>,
    /// ViewBox left-top y coordinate,
    pub miny: Variable<Number>,
    /// ViewBox width dimension.
    pub width: Variable<Number>,
    /// ViewBox height dimension.
    pub height: Variable<Number>,
    /// clip preserve aspect ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aspect: Option<Variable<PreserveAspectRatio>>,
}

impl<X, Y, W, H> From<(X, Y, W, H)> for ViewBox
where
    Number: From<X> + From<Y> + From<W> + From<H>,
{
    fn from(value: (X, Y, W, H)) -> Self {
        Self {
            minx: Variable::Constant(value.0.into()),
            miny: Variable::Constant(value.1.into()),
            width: Variable::Constant(value.2.into()),
            height: Variable::Constant(value.3.into()),
            aspect: None,
        }
    }
}

impl<X, Y, W, H, P> From<(X, Y, W, H, P)> for ViewBox
where
    Number: From<X> + From<Y> + From<W> + From<H>,
    PreserveAspectRatio: From<P>,
{
    fn from(value: (X, Y, W, H, P)) -> Self {
        Self {
            minx: Variable::Constant(value.0.into()),
            miny: Variable::Constant(value.1.into()),
            width: Variable::Constant(value.2.into()),
            height: Variable::Constant(value.3.into()),
            aspect: Some(Variable::Constant(value.4.into())),
        }
    }
}
