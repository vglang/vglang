use super::*;

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

impl<T> From<T> for StrokeLineJoin
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        Self::Miter(Number::from(value).0)
    }
}
