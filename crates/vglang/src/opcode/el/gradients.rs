use crate::opcode::{
    data::{Coords, Length, Number, Rgb, SpreadMethod, Transform},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use super::*;

/// Linear gradients are defined by a ‘linearGradient’ element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, If, For, Foreach, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub units: Option<Variable<Coords>>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Variable: yes.
    pub transform: Option<Variable<Transform>>,

    /// ‘x1’, ‘y1’, ‘x2’ and ‘y2’ define a gradient vector for the linear gradient.
    /// This gradient vector provides starting and ending points onto which the gradient stops are mapped. The values
    /// of ‘x1’, ‘y1’, ‘x2’ and ‘y2’ can be either numbers or percents.
    ///
    /// If the attribute is not specified, the effect is as if a value of '0%' were specified.
    ///
    /// Variable: yes.
    pub x1: Option<Variable<Length>>,

    /// See [`x1`](LinearGradient::x1)
    pub y1: Option<Variable<Length>>,

    /// See [`x1`](LinearGradient::x1)
    pub x2: Option<Variable<Length>>,

    /// See [`x1`](LinearGradient::x1)
    pub y2: Option<Variable<Length>>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Option<Variable<SpreadMethod>>,
}

impl<X1, Y1, X2, Y2> From<(X1, Y1, X2, Y2)> for LinearGradient
where
    Length: From<X1> + From<Y1> + From<X2> + From<Y2>,
{
    fn from(value: (X1, Y1, X2, Y2)) -> Self {
        Self {
            x1: Some(Variable::Constant(value.0.into())),
            y1: Some(Variable::Constant(value.1.into())),
            x2: Some(Variable::Constant(value.2.into())),
            y2: Some(Variable::Constant(value.3.into())),

            ..Default::default()
        }
    }
}

/// Radial gradients are defined by a ‘radialGradient’ element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, If, For, Foreach, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub unit: Option<Variable<Coords>>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Variable: yes.
    pub transform: Option<Variable<Transform>>,

    /// ‘cx’, ‘cy’ and ‘r’ define the largest (i.e., outermost) circle for the radial gradient.
    /// The gradient will be drawn such that the 100% gradient stop is mapped to the perimeter
    /// of this largest (i.e., outermost) circle.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Variable: yes.
    pub cx: Option<Variable<Length>>,

    /// See [`cx`](RadialGradient::cx)
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    pub cy: Option<Variable<Length>>,

    /// See [`cx`](RadialGradient::cx)
    ///
    /// A negative value is an error (see Error processing). A value of zero will cause the area to be painted as a single color
    /// using the color and opacity of the last gradient stop.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Variable: yes.
    pub r: Option<Variable<Length>>,

    /// ‘fx’ and ‘fy’ define the focal point for the radial gradient. The gradient will be drawn such that the
    /// 0% gradient stop is mapped to (fx, fy).
    ///
    /// If attribute ‘fx’ is not specified, ‘fx’ will coincide with the presentational value of ‘cx’ for the element whether the value
    /// for 'cx' was inherited or not. If the element references an element that specifies a value for 'fx', then the value of 'fx'
    /// is inherited from the referenced element.
    ///
    /// Variable: yes.
    pub fx: Option<Variable<Length>>,

    /// See [`fx`](RadialGradient::fx)
    ///
    /// If attribute ‘fy’ is not specified, ‘fy’ will coincide with the presentational value of ‘cy’ for the element whether the value
    /// for 'cy' was inherited or not. If the element references an element that specifies a value for 'fy', then the value of 'fy'
    /// is inherited from the referenced element.
    ///
    /// Variable: yes.
    pub fy: Option<Variable<Length>>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Option<Variable<SpreadMethod>>,
}

impl From<Coords> for RadialGradient {
    fn from(value: Coords) -> Self {
        Self {
            unit: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl<CX, CY, R, FX, FY> From<(CX, CY, R, FX, FY)> for RadialGradient
where
    Length: From<CX> + From<CY> + From<R> + From<FX> + From<FY>,
{
    fn from(value: (CX, CY, R, FX, FY)) -> Self {
        Self {
            cx: Some(Variable::Constant(value.0.into())),
            cy: Some(Variable::Constant(value.1.into())),
            r: Some(Variable::Constant(value.2.into())),
            fx: Some(Variable::Constant(value.3.into())),
            fy: Some(Variable::Constant(value.4.into())),
            ..Default::default()
        }
    }
}

/// The ramp of colors to use on a gradient is defined by the ‘stop’ elements that are child elements
/// to either the ‘linearGradient’ element or the ‘radialGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::shape_element(boxed, If, For, Foreach, LinearGradient, RadialGradient)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    /// The ‘offset’ attribute is either a `<number>` (usually ranging from 0 to 1) or a `<percent>`
    /// (usually ranging from 0% to 100%) which indicates where the gradient stop is placed.
    /// For linear gradients, the ‘offset’ attribute represents a location along the gradient vector.
    /// For radial gradients, it represents a percent distance from (fx,fy) to the edge of the
    /// outermost/largest circle.
    ///
    /// Variable: yes.
    pub offset: Variable<Number>,

    /// indicates what color to use at that gradient stop
    pub color: Option<Variable<Rgb>>,

    pub opacity: Option<Variable<Number>>,
}

impl<T> From<T> for GradientStop
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            offset: Variable::Constant(value.into()),
            color: None,
            opacity: None,
        }
    }
}
