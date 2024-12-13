use crate::expr::{Animatable, FillRule, Length, Paint, StrokeLineCap, StrokeLineJoin};

/// The ‘fill’ instruction paints the interior of the given graphical element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    /// paints color.
    ///
    /// `Inherited: yes`
    pub paint: Option<Animatable<Paint>>,
    /// fill painting rule, see [`FillRule`] for more information.
    ///
    /// `Inherited: yes`
    pub rule: Option<Animatable<FillRule>>,
}

/// This property affect how an element is stroked.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    /// paints color paints along the outline of the given graphical element.
    ///
    /// `Inherited: yes`
    pub paint: Option<Animatable<Paint>>,
    /// This property specifies the width of the stroke on the current object
    ///
    /// `Inherited: yes`
    pub width: Option<Animatable<Length>>,

    /// specifies the shape to be used at the end of open subpaths when they are stroked.
    ///
    /// `Inherited: yes`
    pub linecap: Option<Animatable<StrokeLineCap>>,

    /// specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
    ///
    /// `Inherited: yes`
    pub linejoin: Option<Animatable<StrokeLineJoin>>,

    /// controls the pattern of dashes and gaps used to stroke paths. `<dasharray>` contains a list of comma and/or
    /// white space separated `<length>s` and `<percentage>s` that specify the lengths of alternating dashes and gaps.
    /// If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
    /// Thus, stroke-dasharray: 5,3,2 is equivalent to stroke-dasharray: 5,3,2,5,3,2.
    ///
    /// `Inherited: yes`
    pub dasharray: Option<Animatable<Vec<Length>>>,
    /// specifies the distance into the dash pattern to start the dash
    ///
    /// `Inherited: yes`
    pub dashoffset: Option<Animatable<Length>>,
}

impl<P> From<P> for Stroke
where
    Paint: From<P>,
{
    fn from(value: P) -> Self {
        Self {
            paint: Some(Animatable::Constant(value.into())),
            ..Default::default()
        }
    }
}