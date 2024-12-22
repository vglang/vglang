use std::fmt::Display;

use super::{Coords, FuncIRI, Length, Number, Variable};

/// used as an alpha mask for compositing the current object into the background.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mask {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If maskUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the current user coordinate system
    /// in place at the time when the ‘mask’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘mask’ element via the ‘mask’ property).
    ///
    /// If maskUnits="objectBoundingBox", ‘x’, ‘y’, ‘width’ and ‘height’ represent fractions or percentages of the bounding box
    /// of the element to which the mask is applied. (See Object bounding box units.)
    ///
    /// If attribute ‘maskUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
    pub units: Option<Variable<Coords>>,

    /// Defines the coordinate system for the contents of the ‘mask’.
    ///
    /// If maskContentUnits="userSpaceOnUse", the user coordinate system for the contents of the ‘mask’ element is the current user
    /// coordinate system in place at the time when the ‘mask’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘mask’ element via the ‘mask’ property).
    ///
    /// If maskContentUnits="objectBoundingBox", the user coordinate system for the contents of the ‘mask’ is established using the
    /// bounding box of the element to which the mask is applied. (See Object bounding box units.)
    ///
    /// If attribute ‘maskContentUnits’ is not specified, then the effect is as if a value of 'userSpaceOnUse' were specified.
    pub content_units: Option<Variable<Coords>>,

    /// The x-axis coordinate of one corner of the rectangle for the largest possible offscreen buffer. Note that the clipping
    /// path used to render any graphics within the mask will consist of the intersection of the current clipping path
    /// associated with the given object and the rectangle defined by ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If the attribute is not specified, the effect is as if a value of '-10%' were specified.
    pub x: Option<Variable<Length>>,

    /// The y-axis coordinate of one corner of the rectangle for the largest possible offscreen buffer.
    ///
    /// If the attribute is not specified, the effect is as if a value of '-10%' were specified.
    pub y: Option<Variable<Length>>,

    /// The width of the largest possible offscreen buffer. Note that the clipping path used to render any graphics within the
    /// mask will consist of the intersection of the current clipping path associated with the given object and the rectangle
    /// defined by ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// If the attribute is not specified, the effect is as if a value of '120%' were specified.
    pub width: Option<Variable<Length>>,

    /// The height of the largest possible offscreen buffer.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// If the attribute is not specified, the effect is as if a value of '120%' were specified.
    pub height: Option<Variable<Length>>,
}

impl<X, Y, W, H> From<(X, Y, W, H)> for Mask
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

/// Use mask to a element.
/// Use mask to a element.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Masked(pub Variable<FuncIRI>);

impl<T> From<T> for Masked
where
    FuncIRI: From<T>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

///Sspecifies object/group opacity
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(pub Variable<Number>);

impl<T> From<T> for Opacity
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        let mut value = Number::from(value);
        if value.0 > 1.0 {
            value.0 = 1.0;
        }

        if value.0 < 0.0 {
            value.0 = 0.0;
        }

        Self(Variable::Constant(value))
    }
}

/// A clipping path is defined with a ‘clipPath’ element.
/// A clipping path is used/referenced using the ‘clip-path’ property.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPath(
    /// Defines the coordinate system for the contents of the ‘clipPath’.
    ///
    /// If clipPathUnits="userSpaceOnUse", the contents of the ‘clipPath’ represent values in the current user coordinate
    /// system in place at the time when the ‘clipPath’ element is referenced (i.e., the user coordinate system for the
    /// element referencing the ‘clipPath’ element via the ‘clip-path’ property).
    ///
    /// If clipPathUnits="objectBoundingBox", then the user coordinate system for the contents of the ‘clipPath’ element
    /// is established using the bounding box of the element to which the clipping path is applied (see Object bounding
    /// box units).
    ///
    /// If attribute ‘clipPathUnits’ is not specified, then the effect is as if a value of 'userSpaceOnUse' were specified.
    pub Option<Variable<Coords>>,
);

impl Default for ClipPath {
    fn default() -> Self {
        Self(Some(Variable::Constant(Coords::UserSpaceOnUse)))
    }
}

impl From<Coords> for ClipPath {
    fn from(value: Coords) -> Self {
        Self(Some(Variable::Constant(value)))
    }
}

/// Use mask to a element.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPathed(pub Variable<FuncIRI>);

impl<T> From<T> for ClipPathed
where
    FuncIRI: From<T>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

/// The  property only applies to graphics elements that are contained within a [`ClipPath`] element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipRule {
    Nonzero,
    EvenOdd,
}

impl Display for ClipRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipRule::Nonzero => write!(f, "nonzero"),
            ClipRule::EvenOdd => write!(f, "evenodd"),
        }
    }
}
