use crate::{
    data::{FillRule, Length, Paint, StrokeLineCap, StrokeLineJoin},
    variable::Variable,
};

/// The ‘fill’ instruction paints the interior of the given graphical element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    /// paints color.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paint: Option<Variable<Paint>>,
    /// fill painting rule, see [`FillRule`] for more information.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rule: Option<Variable<FillRule>>,
}

impl<P> From<P> for Fill
where
    Paint: From<P>,
{
    fn from(value: P) -> Self {
        Self {
            paint: Some(Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}

/// This property affect how an element is stroked.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    /// paints color paints along the outline of the given graphical element.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paint: Option<Variable<Paint>>,
    /// This property specifies the width of the stroke on the current object
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub width: Option<Variable<Length>>,

    /// specifies the shape to be used at the end of open subpaths when they are stroked.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub linecap: Option<Variable<StrokeLineCap>>,

    /// specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub linejoin: Option<Variable<StrokeLineJoin>>,

    /// controls the pattern of dashes and gaps used to stroke paths. `<dasharray>` contains a list of comma and/or
    /// white space separated `<length>s` and `<percentage>s` that specify the lengths of alternating dashes and gaps.
    /// If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
    /// Thus, stroke-dasharray: 5,3,2 is equivalent to stroke-dasharray: 5,3,2,5,3,2.
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dasharray: Option<Variable<Vec<Length>>>,
    /// specifies the distance into the dash pattern to start the dash
    ///
    /// `Inherited: yes`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dashoffset: Option<Variable<Length>>,
}

impl<P> From<P> for Stroke
where
    Paint: From<P>,
{
    fn from(value: P) -> Self {
        Self {
            paint: Some(Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
