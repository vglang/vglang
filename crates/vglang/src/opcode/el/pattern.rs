use crate::opcode::{
    data::{Coords, Length, Transform},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use super::*;

/// A pattern is used to fill or stroke an object using a pre-defined graphic object which can be replicated ("tiled")
/// at fixed intervals in x and y to cover the areas to be painted. Patterns are defined using a ‘pattern’ element and
/// then referenced by properties ‘fill’ and ‘stroke’ on a given graphics element to indicate that the given element
/// shall be filled or stroked with the referenced pattern.
///
/// Attributes ‘x’, ‘y’, ‘width’, ‘height’ and ‘patternUnits’ define a reference rectangle somewhere on the infinite canvas.
/// The reference rectangle has its top/left at (x, y) and its bottom/right at (x + width, y + height). The tiling
/// theoretically extends a series of such rectangles to infinity in X and Y (positive and negative), with rectangles
/// starting at (x + m*width, y + n* height) for each possible integer value for m and n.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, If, For, Foreach, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If patternUnits="objectBoundingBox", the user coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’
    /// is established using the bounding box of the element to which the pattern is applied (see Object bounding box units)
    /// and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If patternUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the coordinate system
    /// that results from taking the current user coordinate system in place at the time when the ‘pattern’ element
    /// is referenced (i.e., the user coordinate system for the element referencing the ‘pattern’ element via a ‘fill’
    /// or ‘stroke’ property) and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If attribute `units` is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
    pub units: Option<Variable<Coords>>,
    /// Defines the coordinate system for the contents of the ‘pattern’. Note that this attribute has no effect
    /// if attribute ‘viewBox’ is specified.
    ///
    /// If patternContentUnits="userSpaceOnUse", the user coordinate system for the contents of the ‘pattern’
    /// element is the coordinate system that results from taking the current user coordinate system in place
    /// at the time when the ‘pattern’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘pattern’ element via a ‘fill’ or ‘stroke’ property) and then applying the transform
    /// specified by attribute ‘patternTransform’.
    ///
    /// If patternContentUnits="objectBoundingBox", the user coordinate system for the contents of the ‘pattern’
    /// element is established using the bounding box of the element to which the pattern is applied (see Object
    /// bounding box units) and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If attribute `content_units` is not specified, then the effect is as if a value of 'userSpaceOnUse'
    /// were specified.
    pub content_units: Option<Variable<Coords>>,

    /// Contains the definition of an optional additional transformation from the pattern coordinate system onto the
    /// target coordinate system (i.e., 'userSpaceOnUse' or 'objectBoundingBox'). This allows for things such as
    /// skewing the pattern tiles. This additional transformation matrix is post-multiplied to (i.e., inserted to
    /// the right of) any previously defined transformations, including the implicit transformation necessary to convert
    /// from object bounding box units to user space.
    ///
    /// If attribute `transform` is not specified, then the effect is as if an identity transform were specified.
    pub transform: Option<Variable<Transform>>,

    /// ‘x’, ‘y’, ‘width’ and ‘height’ indicate how the pattern tiles are placed and spaced. These attributes represent
    /// coordinates and values in the coordinate space specified by the combination of attributes [`units`](Self::units) and
    /// [`content_units`](Self::content_units).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub x: Option<Variable<Length>>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub y: Option<Variable<Length>>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub width: Option<Variable<Length>>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub height: Option<Variable<Length>>,
}

impl<X, Y, W, H> From<(X, Y, W, H)> for Pattern
where
    Length: From<X> + From<Y> + From<W> + From<H>,
{
    fn from(value: (X, Y, W, H)) -> Self {
        Self {
            x: Some(Variable::Constant(value.0.into())),
            y: Some(Variable::Constant(value.1.into())),
            width: Some(Variable::Constant(value.2.into())),
            height: Some(Variable::Constant(value.3.into())),
            ..Default::default()
        }
    }
}
