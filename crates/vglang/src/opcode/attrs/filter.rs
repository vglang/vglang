use crate::opcode::{
    data::{Background, Length},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use crate::opcode::el::*;

/// enables access to the background image
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", vglang_derive::attribute(boxed, Group))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableBackground(pub Background);

impl<T> From<T> for EnableBackground
where
    Background: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl EnableBackground {
    /// Create a new `EnableBackground` property with `new` value.
    pub fn new() -> Self {
        Self(Background::new())
    }
}

/// All filter primitives have attributes ‘x’, ‘y’, ‘width’ and ‘height’ which identify a subregion which
/// restricts calculation and rendering of the given filter primitive. These attributes are defined according
/// to the same rules as other filter primitives' coordinate and length attributes and thus represent values
/// in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
///
/// ‘x’, ‘y’, ‘width’ and ‘height’ default to the union (i.e., tightest fitting bounding box) of the subregions
/// defined for all referenced nodes. If there are no referenced nodes (e.g., for ‘feImage’ or ‘feTurbulence’),
/// or one or more of the referenced nodes is a standard input (one of SourceGraphic, SourceAlpha, BackgroundImage,
/// BackgroundAlpha, FillPaint or StrokePaint), or for ‘feTile’ (which is special because its principal function is
/// to replicate the referenced node in X and Y and thereby produce a usually larger result), the default subregion
/// is 0%,0%,100%,100%, where as a special-case the percentages are relative to the dimensions of the filter region,
/// thus making the the default filter primitive subregion equal to the filter region.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(boxed, FeBlend, FeColorMatrix, FeComponentTransfer, FeComposite)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePrimitive {
    /// The minimum x coordinate for the subregion which restricts calculation and rendering of the given filter primitive.  
    pub x: Option<Variable<Length>>,

    /// The minimum y coordinate for the subregion which restricts calculation and rendering of the given filter primitive
    pub y: Option<Variable<Length>>,

    /// The width of the subregion which restricts calculation and rendering of the given filter primitive.
    pub width: Option<Variable<Length>>,

    /// The height of the subregion which restricts calculation and rendering of the given filter primitive.
    pub height: Option<Variable<Length>>,

    /// Assigned name for this filter primitive. If supplied, then graphics that result from processing this filter primitive can
    /// be referenced by an ‘in’ attribute on a subsequent filter primitive within the same ‘filter’ element. If no value is provided,
    /// the output will only be available for re-use as the implicit input into the next filter primitive if that filter primitive
    /// provides no value for its ‘in’ attribute.
    ///
    /// Note that a `filter-primitive-reference` is not an XML ID; instead, a `filter-primitive-reference` is only meaningful within a
    /// given ‘filter’ element and thus have only local scope. It is legal for the same `filter-primitive-reference` to appear multiple
    /// times within the same ‘filter’ element. When referenced, the `filter-primitive-reference` will use the closest preceding filter
    /// primitive with the given result.
    pub result: Option<Variable<String>>,
}

impl<X, Y, W, H> From<(X, Y, W, H)> for FePrimitive
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

impl<X, Y, W, H, R> From<(X, Y, W, H, R)> for FePrimitive
where
    Length: From<X> + From<Y> + From<W> + From<H>,
    String: From<R>,
{
    fn from(value: (X, Y, W, H, R)) -> Self {
        Self {
            x: Some(Variable::Constant(value.0.into())),
            y: Some(Variable::Constant(value.1.into())),
            width: Some(Variable::Constant(value.2.into())),
            height: Some(Variable::Constant(value.3.into())),
            result: Some(Variable::Constant(value.4.into())),
        }
    }
}

/// Define a fragment by name.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(boxed, Group, Polygon, Text)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithFilter(pub String);

impl<T> From<T> for WithFilter
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
