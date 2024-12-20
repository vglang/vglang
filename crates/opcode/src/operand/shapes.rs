use crate::MapCollect;

use super::{Length, Point, Variable};

/// The ‘rect’ element defines a rectangle which is axis-aligned with the current user coordinate system.
/// Rounded rectangles can be achieved by setting appropriate values for attributes ‘rx’ and ‘ry’.
///
///
/// The values used for the x- and y-axis rounded corner radii are determined implicitly if the ‘rx’ or ‘ry’ attributes (or both) are not specified, or are specified but with invalid values. The values are also subject to clamping so that the lengths of the straight segments of the rectangle are never negative. The effective values for ‘rx’ and ‘ry’ are determined by following these steps in order:
///
/// 1. Let rx and ry be length values.
/// 1. If neither ‘rx’ nor ‘ry’ are properly specified, then set both rx and ry to 0. (This will result in square corners.)
/// 1. Otherwise, if a properly specified value is provided for ‘rx’, but not for ‘ry’, then set both rx and ry to the value of ‘rx’.
/// 1. Otherwise, if a properly specified value is provided for ‘ry’, but not for ‘rx’, then set both rx and ry to the value of ‘ry’.
/// 1. Otherwise, both ‘rx’ and ‘ry’ were specified properly. Set rx to the value of ‘rx’ and ry to the value of ‘ry’.
/// 1. If rx is greater than half of ‘width’, then set rx to half of ‘width’.
/// 1. If ry is greater than half of ‘height’, then set ry to half of ‘height’.
/// 1. The effective values of ‘rx’ and ‘ry’ are rx and ry, respectively.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x-axis coordinate of the side of the rectangle which has the smaller x-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x: Variable<Length>,

    /// The y-axis coordinate of the side of the rectangle which has the smaller y-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y: Variable<Length>,

    /// The width of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub width: Variable<Length>,

    /// The height of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub height: Variable<Length>,

    /// For rounded rectangles, the x-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub rx: Variable<Length>,

    /// For rounded rectangles, the y-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub ry: Option<Variable<Length>>,
}

impl<X, Y, W, H> From<(X, Y, W, H)> for Rect
where
    Length: From<X> + From<Y> + From<W> + From<H>,
{
    fn from(value: (X, Y, W, H)) -> Self {
        Self {
            x: Variable::Constant(value.0.into()),
            y: Variable::Constant(value.1.into()),
            width: Variable::Constant(value.2.into()),
            height: Variable::Constant(value.3.into()),
            ..Default::default()
        }
    }
}

/// The ‘circle’ element defines a circle based on a center point and a radius.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    /// The x-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Variable<Length>,

    /// The y-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Variable<Length>,

    /// The radius of the circle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub r: Variable<Length>,
}

impl<X, Y, R> From<(X, Y, R)> for Circle
where
    Length: From<X> + From<Y> + From<R>,
{
    fn from(value: (X, Y, R)) -> Self {
        Self {
            cx: Variable::Constant(value.0.into()),
            cy: Variable::Constant(value.1.into()),
            r: Variable::Constant(value.2.into()),
        }
    }
}

/// The ‘ellipse’ element defines an ellipse which is axis-aligned with the current user coordinate
/// system based on a center point and two radii.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    /// The x-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Variable<Length>,

    /// The y-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Variable<Length>,

    /// The x-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub rx: Variable<Length>,

    /// The y-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub ry: Variable<Length>,
}

/// The ‘line’ element defines a line segment that starts at one point and ends at another.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    /// The x-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x1: Variable<Length>,

    /// The y-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y1: Variable<Length>,

    /// The x-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x2: Variable<Length>,

    /// The y-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y2: Variable<Length>,
}

/// The ‘polygon’ element defines a closed shape consisting of a set of connected straight line segments.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(
    /// The points that make up the polygon. All coordinate values are in the user coordinate system.
    ///
    /// Animatable: yes.
    pub Variable<Vec<Point>>,
);

impl<T> From<T> for Polyline
where
    T: MapCollect<Point>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.map_collect()))
    }
}
