use std::fmt::Display;

use super::{Length, Number, Variable};

/// see [`svg`] document for more information.
///
/// [`svg`]: https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet,
    Slice,
}

impl Display for MeetOrSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeetOrSlice::Meet => write!(f, "meet"),
            MeetOrSlice::Slice => write!(f, "slice"),
        }
    }
}

/// In some cases, typically when using the ‘viewBox’ attribute, i
/// t is desirable that the graphics stretch to fit non-uniformly
/// to take up the entire viewport. In other cases, it is desirable
/// that uniform scaling be used for the purposes of preserving
/// the aspect ratio of the graphics.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    /// Force uniform scaling
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMinYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMidYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMaxYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMinYMid(MeetOrSlice),
    /// Force uniform scaling(the default).
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMidYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMaxYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMinYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMidYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMaxYMax(MeetOrSlice),
}

impl Display for PreserveAspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreserveAspectRatio::xMinYMin(meet_or_slice) => {
                write!(f, "xMinYMin {}", meet_or_slice)
            }
            PreserveAspectRatio::xMidYMin(meet_or_slice) => {
                write!(f, "xMidYMin {}", meet_or_slice)
            }
            PreserveAspectRatio::xMaxYMin(meet_or_slice) => {
                write!(f, "xMaxYMin {}", meet_or_slice)
            }
            PreserveAspectRatio::xMinYMid(meet_or_slice) => {
                write!(f, "xMinYMid {}", meet_or_slice)
            }
            PreserveAspectRatio::xMidYMid(meet_or_slice) => {
                write!(f, "xMidYMid {}", meet_or_slice)
            }
            PreserveAspectRatio::xMaxYMid(meet_or_slice) => {
                write!(f, "xMaxYMid {}", meet_or_slice)
            }
            PreserveAspectRatio::xMinYMax(meet_or_slice) => {
                write!(f, "xMinYMax {}", meet_or_slice)
            }
            PreserveAspectRatio::xMidYMax(meet_or_slice) => {
                write!(f, "xMidYMax {}", meet_or_slice)
            }
            PreserveAspectRatio::xMaxYMax(meet_or_slice) => {
                write!(f, "xMaxYMax {}", meet_or_slice)
            }
        }
    }
}

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self::xMidYMid(MeetOrSlice::Meet)
    }
}

/// It is often desirable to specify that a given set of graphics stretch to fit a particular container element.
/// The ‘viewBox’ attribute provides this capability.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

/// Create a new layer into which the backend render child elements.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canvas {
    /// a number (usually an integer) that represents the width of the rendering layer.
    pub width: Variable<Length>,
    /// a number (usually an integer) that represents the height of the rendering layer.
    pub height: Variable<Length>,
    /// stretch to fit a particular container element.
    pub viewbox: Option<ViewBox>,
}

impl<W, H> From<(W, H)> for Canvas
where
    Length: From<W> + From<H>,
{
    fn from(value: (W, H)) -> Self {
        Canvas {
            width: Variable::Constant(value.0.into()),
            height: Variable::Constant(value.1.into()),
            viewbox: None,
        }
    }
}
