use super::{Color, Length, RefBy, Rgb, Variable};

/// ‘fill’ and ‘stroke’ take on a value of type [`Paint`], which is specified as follows:
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    /// the explicit color to be used to paint the current object
    Color(Rgb),
    /// A gradient entity reference.
    Gradient(RefBy),
    /// A pattern entity reference.
    Pattern(RefBy),
}

impl From<Color> for Paint {
    fn from(value: Color) -> Self {
        Self::Color(value.into())
    }
}
impl From<Rgb> for Paint {
    fn from(value: Rgb) -> Self {
        Self::Color(value)
    }
}

impl From<(u8, u8, u8)> for Paint {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::Color(value.into())
    }
}

//// The ‘fill-rule’ property indicates the algorithm which is to be used to determine what parts of the canvas are
//// included inside the shape. For a simple, non-intersecting path, it is intuitively clear what region lies "inside";
//// however, for a more complex path, such as a path that intersects itself or where one subpath encloses another,
//// the interpretation of "inside" is not so obvious.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    /// This rule determines the "insideness" of a point on the canvas by drawing a ray from that point to infinity in
    /// any direction and then examining the places where a segment of the shape crosses the ray. Starting with a count of zero,
    /// add one each time a path segment crosses the ray from left to right and subtract one each time a path segment crosses
    /// the ray from right to left. After counting the crossings, if the result is zero then the point is outside the path.
    /// Otherwise, it is inside.
    Nonzero,
    /// This rule determines the "insideness" of a point on the canvas by drawing a ray from that point to infinity in any direction
    /// and counting the number of path segments from the given shape that the ray crosses. If this number is odd, the point is inside;
    /// if even, the point is outside.
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::Nonzero
    }
}

/// Specifies the shape to be used at the end of open subpaths when they are stroked
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}

impl Default for StrokeLineCap {
    fn default() -> Self {
        Self::Butt
    }
}

/// Specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineJoin {
    Miter(f32),
    Round,
    Bevel,
}

impl Default for StrokeLineJoin {
    fn default() -> Self {
        Self::Miter(4.0)
    }
}

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
