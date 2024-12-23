use super::{
    Bool, ChannelSelector, Color, Coords, FuncIRI, Integer, Length, Number, NumberOptNumber,
    PreserveAspectRatio, Rgb, Variable,
};

/// This attribute takes the form x-pixels [y-pixels], and indicates the width and height
/// of the intermediate images in pixels. If not provided, then the user agent will use
/// reasonable values to produce a high-quality result on the output device.
///
/// Care should be taken when assigning a non-default value to this attribute. Too small
/// of a value may result in unwanted pixelation in the result. Too large of a value may
/// result in slow processing and large memory usage.
///
/// Negative values are an error (see Error processing). Zero values disable rendering of
/// the element which referenced the filter.
///
/// Non-integer values are truncated, i.e rounded to the closest integer value towards zero.
///
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilterRes {
    /// `x-pixels`
    pub x: f32,
    /// optional `y-pixels`
    pub y: Option<f32>,
}

/// A filter effect consists of a series of graphics operations that are applied to a given source graphic to
/// produce a modified graphical result. The result of the filter effect is rendered to the target device
/// instead of the original source graphic. The following illustrates the process:
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If units="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the current user
    /// coordinate system in place at the time when the ‘filter’ is referenced (i.e., the user coordinate system
    /// for the element referencing the ‘filter’ via a ‘filter’ property).
    ///
    /// If units="objectBoundingBox", then ‘x’, ‘y’, ‘width’ and ‘height’ represent fractions or percentages
    /// of the bounding box on the referencing element (see Object bounding box units).
    ///
    /// If attribute units is not specified, then the effect is if a value of 'objectBoundingBox' were
    /// specified.
    pub units: Option<Variable<Coords>>,

    /// Specifies the coordinate system for the various length values within the filter primitives and for the
    /// attributes that define the filter primitive subregion.
    ///
    /// If primitive_units="userSpaceOnUse", any length values within the filter definitions represent values in
    /// the current user coordinate system in place at the time when the ‘filter’ element is referenced (i.e.,
    /// the user coordinate system for the element referencing the ‘filter’ element via a ‘filter’ property).
    ///
    /// If primitive_units="objectBoundingBox", then any length values within the filter definitions represent
    /// fractions or percentages of the bounding box on the referencing element (see Object bounding box units).
    /// Note that if only one number was specified in a <number-optional-number> value this number is expanded out
    /// before the ‘primitiveUnits’ computation takes place.
    ///
    /// If attribute primitive_units is not specified, then the effect is as if a value of userSpaceOnUse were specified.
    pub primitive_units: Option<Variable<Coords>>,

    /// These attributes define a rectangular region on the canvas to which this filter applies.
    ///
    /// The amount of memory and processing time required to apply the filter are related to the size of this rectangle
    /// and the ‘filterRes’ attribute of the filter.
    ///
    /// The coordinate system for these attributes depends on the value for attribute ‘filterUnits’.
    ///
    /// Negative values for ‘width’ or ‘height’ are an error (see Error processing). Zero values disable rendering of the
    /// element which referenced the filter.
    ///
    /// The bounds of this rectangle act as a hard clipping region for each filter primitive included with a given ‘filter’
    /// element; thus, if the effect of a given filter primitive would extend beyond the bounds of the rectangle (this
    /// sometimes happens when using a ‘feGaussianBlur’ filter primitive with a very large ‘stdDeviation’), parts of the
    /// effect will get clipped.
    ///
    /// If ‘x’ or ‘y’ is not specified, the effect is as if a value of -10% were specified.
    ///
    /// If ‘width’ or ‘height’ is not specified, the effect is as if a value of 120% were specified.    
    pub x: Option<Variable<Length>>,

    /// See [`x`](Self::x)
    pub y: Option<Variable<Length>>,

    /// See [`x`](Self::x)
    pub width: Option<Variable<Length>>,

    /// See [`x`](Self::x)
    pub height: Option<Variable<Length>>,

    /// This attribute takes the form x-pixels [y-pixels], and indicates the width and height of the
    /// intermediate images in pixels. If not provided, then the user agent will use reasonable values
    /// to produce a high-quality result on the output device.
    ///
    /// Care should be taken when assigning a non-default value to this attribute. Too small of a value
    /// may result in unwanted pixelation in the result. Too large of a value may result in slow
    /// processing and large memory usage.
    ///
    /// Negative values are an error (see Error processing). Zero values disable rendering of the
    /// element which referenced the filter.
    ///
    /// Non-integer values are truncated, i.e rounded to the closest integer value towards zero.
    pub res: Option<Variable<FilterRes>>,
}

/// Identifies input for the given filter primitive. The value can be either one of six keywords or
/// can be a string which matches a previous ‘result’ attribute value within the same ‘filter’ element.
/// If no value is provided and this is the first filter primitive, then this filter primitive will use
/// SourceGraphic as its input. If no value is provided and this is a subsequent filter primitive, then
/// this filter primitive will use the result from the previous filter primitive as its input.
///
/// If the value for ‘result’ appears multiple times within a given ‘filter’ element, then a reference
/// to that result will use the closest preceding filter primitive with the given value for attribute
/// ‘result’. Forward references to results are an error.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeIn {
    /// This keyword represents the graphics elements that were the original input into the ‘filter’ element.
    /// For raster effects filter primitives, the graphics elements will be rasterized into an initially clear
    /// RGBA raster in image space. Pixels left untouched by the original graphic will be left clear. The image
    /// is specified to be rendered in linear RGBA pixels. The alpha channel of this image captures any
    /// anti-aliasing specified by SVG. (Since the raster is linear, the alpha channel of this image will
    /// represent the exact percent coverage of each pixel.)
    SourceGraphic,
    /// This keyword represents the graphics elements that were the original input into the ‘filter’ element.
    /// SourceAlpha has all of the same rules as SourceGraphic except that only the alpha channel is used.
    /// The input image is an RGBA image consisting of implicitly black color values for the RGB channels,
    /// but whose alpha channel is the same as SourceGraphic. If this option is used, then some implementations
    /// might need to rasterize the graphics elements in order to extract the alpha channel.
    SourceAlpha,
    /// This keyword represents an image snapshot of the canvas under the filter region at the time that the
    /// ‘filter’ element was invoked.
    BackgroundImage,
    /// Same as BackgroundImage except only the alpha channel is used. See SourceAlpha and Accessing the background image.
    BackgroundAlpha,
    /// This keyword represents the value of the ‘fill’ property on the target element for the filter effect.
    /// The FillPaint image has conceptually infinite extent. Frequently this image is opaque everywhere,
    /// but it might not be if the "paint" itself has alpha, as in the case of a gradient or pattern which
    /// itself includes transparent or semi-transparent parts.
    FillPaint,
    /// This keyword represents the value of the ‘stroke’ property on the target element for the filter effect.
    /// The StrokePaint image has conceptually infinite extent. Frequently this image is opaque everywhere,
    /// but it might not be if the "paint" itself has alpha, as in the case of a gradient or pattern which
    /// itself includes transparent or semi-transparent parts.
    StrokePaint,

    /// Reference to named register for other filter-primitive result .
    Ref(FePrimitiveRef),
}

impl Default for FeIn {
    fn default() -> Self {
        Self::SourceGraphic
    }
}

/// Assign output to a named register. otherwise the filter output will only be referenced by next filter primitive.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeOut {
    Position,
    Named(String),
}

impl Default for FeOut {
    fn default() -> Self {
        Self::Position
    }
}

/// A name assigned for the filter primitive.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePrimitiveRef(pub String);

impl From<String> for FePrimitiveRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FePrimitiveRef {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
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
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
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
    /// Note that a <filter-primitive-reference> is not an XML ID; instead, a <filter-primitive-reference> is only meaningful within a
    /// given ‘filter’ element and thus have only local scope. It is legal for the same <filter-primitive-reference> to appear multiple
    /// times within the same ‘filter’ element. When referenced, the <filter-primitive-reference> will use the closest preceding filter
    /// primitive with the given result.
    pub result: Option<Variable<FePrimitiveRef>>,
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
    FePrimitiveRef: From<R>,
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

/// Defines a distant light source that can be used within a lighting filter primitive:
/// [`FeDiffuseLighting`] or [`FeSpecularLighting`].
///
/// The following diagram illustrates the angles which ‘azimuth’ and ‘elevation’ represent in an XYZ coordinate system.
///
/// ![`distance light source`](https://www.w3.org/TR/SVG11/images/filters/azimuth-elevation.png)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDistantLight {
    /// Direction angle for the light source on the XY plane (clockwise), in degrees from the x axis.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub azimuth: Option<Variable<Number>>,

    /// Direction angle for the light source from the XY plane towards the z axis, in degrees. Note the positive Z-axis points towards the viewer of the content.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub elevation: Option<Variable<Number>>,
}

/// Defines a point light source that can be used within a lighting filter primitive:
/// [`FeDiffuseLighting`] or [`FeSpecularLighting`].
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePointLight {
    /// X location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub x: Option<Variable<Number>>,

    /// Y location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub y: Option<Variable<Number>>,

    /// Z location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element,
    /// assuming that, in the initial coordinate system, the positive Z-axis comes out towards the person viewing the content and
    /// assuming that one unit along the Z-axis equals one unit in X and Y.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub z: Option<Variable<Number>>,
}

/// Defines a spot light source that can be used within a lighting filter primitive:
/// [`FeDiffuseLighting`] or [`FeSpecularLighting`].
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpotLight {
    /// X location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub x: Option<Variable<Number>>,

    /// Y location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub y: Option<Variable<Number>>,

    /// Z location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element,
    /// assuming that, in the initial coordinate system, the positive Z-axis comes out towards the person viewing the content and
    /// assuming that one unit along the Z-axis equals one unit in X and Y.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub z: Option<Variable<Number>>,

    /// X location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub point_at_x: Option<Variable<Number>>,

    /// Y location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub point_at_y: Option<Variable<Number>>,

    /// Z location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing, assuming that, in the initial coordinate system, the positive Z-axis comes out towards the
    /// person viewing the content and assuming that one unit along the Z-axis equals one unit in X and Y.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub point_at_z: Option<Variable<Number>>,

    /// Exponent value controlling the focus for the light source.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub specular_exponent: Option<Variable<Number>>,

    /// A limiting cone which restricts the region where the light is projected. No light is projected outside the cone.
    /// ‘limitingConeAngle’ represents the angle in degrees between the spot light axis (i.e. the axis between the light
    /// source and the point to which it is pointing at) and the spot light cone. User agents should apply a smoothing
    /// technique such as anti-aliasing at the boundary of the cone.
    ///
    /// If no value is specified, then no limiting cone will be applied.
    pub limiting_cone_angle: Option<Variable<Number>>,
}

/// The ‘lighting-color’ property defines the color of the light source for filter primitives ‘feDiffuseLighting’ and
/// ‘feSpecularLighting’.
#[cfg(feature = "sexpr")]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightingColor(pub Variable<Rgb>);

impl<C> From<C> for LightingColor
where
    Rgb: From<C>,
{
    fn from(value: C) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

/// Image blending modes
/// For the compositing formulas below, the following definitions apply:
/// * cr = Result color (RGB) - premultiplied
/// * qa = Opacity value at a given pixel for image A
/// * qb = Opacity value at a given pixel for image B
/// * ca = Color (RGB) at a given pixel for image A - premultiplied
/// * cb = Color (RGB) at a given pixel for image B - premultiplied
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeBlendMode {
    /// cr = (1 - qa) * cb + ca
    Normal,
    /// cr = (1-qa)*cb + (1-qb)*ca + ca*cb
    Multiply,
    /// cr = cb + ca - ca * cb
    Screen,
    /// cr = Min ((1 - qa) * cb + ca, (1 - qb) * ca + cb)
    Darken,
    /// cr = Max ((1 - qa) * cb + ca, (1 - qb) * ca + cb)
    Lighten,
}

impl Default for FeBlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// This filter composites two objects together using commonly used imaging software blending modes.
/// It performs a pixel-wise combination of two input images.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeBlend {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// Image blending mode
    pub mode: Option<Variable<FeBlendMode>>,

    /// The first input image to the blending operation.
    pub r#in: Option<Variable<FeIn>>,

    /// The second input image to the blending operation. This attribute can take on the same values as the ‘in’ attribute.
    pub in2: Option<Variable<FeIn>>,
}

impl<T> From<T> for FeBlend
where
    FePrimitive: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            primitive: value.into(),
            ..Default::default()
        }
    }
}

/// Values of FeColorMatrix.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeColorMatrixValues {
    /// a list of 20 matrix values.
    Matrix([f32; 20]),

    /// `Saturate` is a single real number value (0 to 1).
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    Saturate(f32),

    /// `HueRotate` is a single one real number value (degrees)
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    HueRotate(f32),

    /// `LuminanceToAlpha`  is not applicable.
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    LuminanceToAlpha,
}

impl Default for FeColorMatrixValues {
    fn default() -> Self {
        Self::Matrix([
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
        ])
    }
}

/// This filter applies a matrix transformation.
///
/// on the RGBA color and alpha values of every pixel on the input graphics to produce a result with a new
/// set of RGBA color and alpha values.
///
/// The calculations are performed on non-premultiplied color values. If the input graphics consists of
/// premultiplied color values, those values are automatically converted into non-premultiplied color values
/// for this operation.
///
/// These matrices often perform an identity mapping in the alpha channel. If that is the case, an implementation
/// can avoid the costly undoing and redoing of the premultiplication for all pixels with A = 1.
///
/// See [`feColorMatrix`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement).
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeColorMatrix {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Variable<FeIn>,

    /// The contents of ‘values’ depends on the value of attribute ‘type’:
    pub values: Variable<FeColorMatrixValues>,
}

impl<T> From<T> for FeColorMatrix
where
    FePrimitive: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            primitive: value.into(),
            ..Default::default()
        }
    }
}

/// transfer functions for the rgba channels.
///
/// See [`FeComponentTransfer`]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeFunc {
    /// C' = C
    Identity,

    /// For table, the function is defined by linear interpolation between values given in the attribute ‘tableValues’.
    /// The table has n+1 values (i.e., v0 to vn) specifying the start and end values for n evenly sized interpolation regions.
    /// Interpolations use the following formula:
    ///
    /// For a value C < 1 find k such that:
    ///
    /// > k/n <= C < (k+1)/n
    ///
    /// The result C' is given by:
    ///
    /// > C' = vk + (C - k/n)*n * (vk+1 - vk)
    ///
    /// If C = 1 then:
    ///
    /// > C' = vn.
    Table(Vec<f32>),

    /// For discrete, the function is defined by the step function given in the attribute ‘tableValues’,
    /// which provides a list of n values (i.e., v0 to vn-1) in order to identify a step function consisting of n steps.
    /// The step function is defined by the following formula:
    ///
    /// For a value C < 1 find k such that:
    ///
    /// > k/n <= C < (k+1)/n
    ///
    /// The result C' is given by:
    ///
    /// > C' = vk
    ///
    /// If C = 1 then:
    ///
    /// > C' = vn-1.
    Discrete(Vec<f32>),

    /// For linear, the function is defined by the following linear equation:
    ///
    /// > C' = slope * C + intercept
    Linear {
        /// the slope of the linear function.
        slope: f32,
        /// the intercept of the linear function.
        intercept: f32,
    },

    /// For gamma, the function is defined by the following exponential function:
    ///
    /// > C' = amplitude * pow(C, exponent) + offset
    Gamma {
        /// the amplitude of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
        amplitude: f32,
        /// the exponent of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
        exponent: f32,
        /// the offset of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
        offset: f32,
    },
}

impl Default for FeFunc {
    fn default() -> Self {
        Self::Identity
    }
}

/// This filter primitive performs component-wise remapping of data as follows:
///
/// > R' = feFuncR( R )
///
/// > G' = feFuncG( G )
///
/// > B' = feFuncB( B )
///
/// > A' = feFuncA( A )
///
/// for every pixel. It allows operations like brightness adjustment, contrast adjustment, color balance or thresholding.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComponentTransfer {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,
}

impl<T> From<T> for FeComponentTransfer
where
    FePrimitive: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            primitive: value.into(),
            ..Default::default()
        }
    }
}

macro_rules! define_func_x {
    ($compoent: ident, $name: ident) => {
        /// transfer function for the
        #[doc = stringify!($compoent)]
        /// component of the input graphic
        ///
        /// See [`FeFunc`], [`FeComponentTransfer`]
        #[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(pub FeFunc);

        impl<F> From<F> for $name
        where
            FeFunc: From<F>,
        {
            fn from(value: F) -> Self {
                $name(value.into())
            }
        }
    };
}

define_func_x!(red, FeFuncR);
define_func_x!(green, FeFuncG);
define_func_x!(blue, FeFuncB);
define_func_x!(alpha, FeFuncA);

/// The compositing operation that is to be performed. All of the ‘operator’ types except arithmetic match the
/// corresponding operation as described in `PORTERDUFF`. The arithmetic operator is described above. If attribute
/// ‘operator’ is not specified, then the effect is as if a value of over were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeCompositeOperator {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic {
        /// Only applicable if operator="arithmetic".
        /// If the attribute is not specified, the effect is as if a value of 0 were specified.
        k1: Number,

        /// Only applicable if operator="arithmetic".
        /// If the attribute is not specified, the effect is as if a value of 0 were specified.
        k2: Number,

        /// Only applicable if operator="arithmetic".
        /// If the attribute is not specified, the effect is as if a value of 0 were specified.
        k3: Number,

        /// Only applicable if operator="arithmetic".
        /// If the attribute is not specified, the effect is as if a value of 0 were specified.
        k4: Number,
    },
}

impl Default for FeCompositeOperator {
    fn default() -> Self {
        Self::Over
    }
}

/// This filter performs the combination of the two input images pixel-wise in image space using one of the Porter-Duff [`PORTERDUFF`]
/// compositing operations: over, in, atop, out, xor [`SVG-COMPOSITING`]. Additionally, a component-wise arithmetic operation (with
/// the result clamped between [0..1]) can be applied.
///
/// See [`feComposite`].
///
/// [`feComposite`]: https://www.w3.org/TR/SVG11/filters.html#feCompositeElement
/// [`PORTERDUFF`]: https://www.w3.org/TR/SVG11/refs.html#ref-PORTERDUFF
/// [`SVG-COMPOSITING`]: https://www.w3.org/TR/SVG11/refs.html#ref-SVG-COMPOSITING
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComposite {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// The second input image to the compositing operation. This attribute can take on the same values as the [`a`](Self::a) attribute.
    pub in2: Variable<FeIn>,

    /// See [`FeCompositeOperator`]
    pub operator: Option<Variable<FeCompositeOperator>>,
}

impl<T> From<T> for FeComposite
where
    FePrimitive: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            primitive: value.into(),
            ..Default::default()
        }
    }
}

/// Determines how to extend the input image as necessary with color values so that the matrix operations
/// can be applied when the kernel is positioned at or near the edge of the input image.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeConvolveMatrixEdgeMode {
    Duplicate,
    Wrap,
    None,
}

impl Default for FeConvolveMatrixEdgeMode {
    fn default() -> Self {
        Self::Duplicate
    }
}

/// feConvolveMatrix applies a matrix convolution filter effect. A convolution combines pixels
/// in the input image with neighboring pixels to produce a resulting image. A wide variety
/// of imaging operations can be achieved through convolutions, including blurring, edge detection,
/// sharpening, embossing and beveling.
///
/// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrix {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,
    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// Indicates the number of cells in each dimension for ‘kernelMatrix’. The values provided must be `integer`s greater than zero.
    /// The first number, `orderX`, indicates the number of columns in the matrix. The second number, `orderY`, indicates the number
    /// of rows in the matrix. If `orderY` is not provided, it defaults to `orderX`.
    ///
    /// A typical value is order="3". It is recommended that only small values (e.g., 3) be used; higher values may result in very
    /// high CPU overhead and usually do not produce results that justify the impact on performance.
    ///
    /// If the attribute is not specified, the effect is as if a value of 3 were specified.
    pub order: Variable<NumberOptNumber>,
    /// The list of `number`s that make up the kernel matrix for the convolution. Values are separated by space
    /// characters and/or a comma. The number of entries in the list must equal `orderX` times `orderY`.
    pub kernel: Variable<Vec<Number>>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number, that number is divided by ‘divisor’
    /// to yield the final destination color value. A divisor that is the sum of all the matrix values tends to have
    /// an evening effect on the overall color intensity of the result. It is an error to specify a divisor of zero.
    /// The default value is the sum of all values in kernelMatrix, with the exception that if the sum is zero, then
    /// the divisor is set to 1.
    pub divisor: Variable<Number>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number and applying the ‘divisor’, the ‘bias’
    /// attribute is added to each component. One application of ‘bias’ is when it is desirable to have .5 gray value
    /// be the zero response of the filter. The bias property shifts the range of the filter. This allows representation
    /// of values that would otherwise be clamped to 0 or 1. If ‘bias’ is not specified, then the effect is as if a
    /// value of 0 were specified.
    pub bias: Variable<Number>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number and applying the ‘divisor’, the ‘bias’
    /// attribute is added to each component. One application of ‘bias’ is when it is desirable to have .5 gray value
    /// be the zero response of the filter. The bias property shifts the range of the filter. This allows representation
    /// of values that would otherwise be clamped to 0 or 1. If ‘bias’ is not specified, then the effect is as if a
    /// value of 0 were specified.
    pub target_x: Variable<Integer>,

    /// Determines the positioning in Y of the convolution matrix relative to a given target pixel in the input image.
    /// The topmost row of the matrix is row number zero. The value must be such that: 0 <= targetY < orderY. By default,
    /// the convolution matrix is centered in Y over each pixel of the input image (i.e., targetY = floor ( orderY / 2 )).
    pub target_y: Variable<Integer>,

    /// Determines how to extend the input image as necessary with color values so that the matrix operations can be applied
    /// when the kernel is positioned at or near the edge of the input image.
    pub edge_mode: Variable<FeConvolveMatrixEdgeMode>,

    /// The first number is the `dx` value. The second number is the `dy` value. If the `dy` value is not specified,
    /// it defaults to the same value as `dx`. Indicates the intended distance in current filter units (i.e., units
    /// as determined by the value of attribute ‘primitiveUnits’) between successive columns and rows, respectively,
    /// in the ‘kernelMatrix’. By specifying value(s) for ‘kernelUnitLength’, the kernel becomes defined in a
    /// scalable, abstract coordinate system. If ‘kernelUnitLength’ is not specified, the default value is one pixel
    /// in the offscreen bitmap, which is a pixel-based coordinate system, and thus potentially not scalable. For
    /// some level of consistency across display media and user agents, it is necessary that a value be provided for
    /// at least one of ‘filterRes’ and ‘kernelUnitLength’. In some implementations, the most consistent results and
    /// the fastest performance will be achieved if the pixel grid of the temporary offscreen images aligns with the
    /// pixel grid of the kernel.
    ///
    /// A negative or zero value is an error (see Error processing).
    pub kernel_unit_len: Option<Variable<NumberOptNumber>>,

    /// A value of false indicates that the convolution will apply to all channels, including the alpha channel.
    ///
    /// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)
    pub preserve_alpha: Variable<Bool>,
}

/// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feDiffuseLightingElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDiffuseLighting {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Variable<FeIn>,

    /// height of surface when Ain = 1.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub surface_scale: Option<Variable<Number>>,

    /// kd in Phong lighting model. In SVG, this can be any non-negative number.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub diffuse_constant: Option<Variable<Number>>,

    /// The first number is the `dx` value. The second number is the `dy` value. If the `dy` value is not specified,
    /// it defaults to the same value as `dx`. Indicates the intended distance in current filter units (i.e., units
    /// as determined by the value of attribute ‘primitiveUnits’) for dx and dy, respectively, in the surface normal
    /// calculation formulas. By specifying value(s) for ‘kernelUnitLength’, the kernel becomes defined in a scalable,
    /// abstract coordinate system. If ‘kernelUnitLength’ is not specified, the dx and dy values should represent
    /// very small deltas relative to a given (x,y) position, which might be implemented in some cases as one pixel
    /// in the intermediate image offscreen bitmap, which is a pixel-based coordinate system, and thus potentially
    /// not scalable. For some level of consistency across display media and user agents, it is necessary that a value
    /// be provided for at least one of ‘filterRes’ and ‘kernelUnitLength’. Discussion of intermediate images are in the
    /// Introduction and in the description of attribute ‘filterRes’.
    ///
    /// A negative or zero value is an error (see Error processing).
    pub kernel_unit_len: Option<Variable<NumberOptNumber>>,
}

/// This filter primitive uses the pixels values from the image from ‘in2’ to spatially displace the image from ‘in’.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDisplacementMap {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// See [`FeIn`]
    pub in2: Variable<FeIn>,

    /// Displacement scale factor. The amount is expressed in the coordinate system established by attribute ‘primitiveUnits’
    /// on the ‘filter’ element.
    ///
    /// When the value of this attribute is 0, this operation has no effect on the source image.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub scale: Option<Variable<Number>>,

    /// Indicates which channel from ‘in2’ to use to displace the pixels in ‘in’ along the x-axis.
    /// If attribute ‘xChannelSelector’ is not specified, then the effect is as if a value of A were
    /// specified.
    pub x_channel_selector: Option<Variable<ChannelSelector>>,

    /// Indicates which channel from ‘in2’ to use to displace the pixels in ‘in’ along the y-axis.
    /// If attribute ‘yChannelSelector’ is not specified, then the effect is as if a value of A were
    /// specified.
    pub y_channel_selector: Option<Variable<ChannelSelector>>,
}

/// This filter primitive creates a rectangle filled with the color and opacity values from properties ‘flood-color’ and ‘flood-opacity’.
/// The rectangle is as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the
/// ‘feFlood’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFlood {
    pub color: Variable<Rgb>,
    pub opacity: Option<Variable<Number>>,
}

impl Default for FeFlood {
    fn default() -> Self {
        Self {
            color: Variable::Constant(Color::black.into()),
            opacity: None,
        }
    }
}

/// This filter primitive performs a Gaussian blur on the input image.
///
/// See [`feGaussianBlur`](https://www.w3.org/TR/SVG11/filters.html#feGaussianBlurElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeGaussianBlur {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// The standard deviation for the blur operation. If two `number`s are provided, the first number represents
    /// a standard deviation value along the x-axis of the coordinate system established by attribute ‘primitiveUnits’
    /// on the ‘filter’ element. The second value represents a standard deviation in Y. If one number is provided,
    /// then that value is used for both X and Y.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables the effect of the given filter
    /// primitive (i.e., the result is the filter input image). If ‘stdDeviation’ is 0 in only one of X or Y, then the
    /// effect is that the blur is only applied in the direction that has a non-zero value.
    pub std_deviation: Option<Variable<NumberOptNumber>>,
}

/// This filter primitive composites input image layers on top of each other using the over operator with Input1
/// (corresponding to the first ‘feMergeNode’ child element) on the bottom and the last specified input, InputN
/// (corresponding to the last ‘feMergeNode’ child element), on top.
///
/// See [`feMerge`](https://www.w3.org/TR/SVG11/filters.html#feMergeElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMerge(pub FePrimitive);

/// This filter primitive refers to a graphic external to this filter element, which is loaded or rendered into an RGBA
/// raster and becomes the result of the filter primitive.
///
/// See [`feImage`](https://www.w3.org/TR/SVG11/filters.html#feImageElement)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeImage {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// An IRI reference to the image source.
    pub href: Variable<FuncIRI>,

    /// See [`PreserveAspectRatio`].
    pub aspect: Variable<PreserveAspectRatio>,
}

/// See [`FeMerge`]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMergeItem(pub Option<Variable<FeIn>>);

/// A keyword indicating whether to erode (i.e., thin) or dilate (fatten) the source graphic.
/// If attribute `mode` is not specified, then the effect is as if a value of erode were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeMorphologyOperator {
    Erode,
    Dilate,
}

impl Default for FeMorphologyOperator {
    fn default() -> Self {
        Self::Erode
    }
}

/// This filter primitive performs "fattening" or "thinning" of artwork.
/// It is particularly useful for fattening or thinning an alpha channel.
///
/// See [`feMorphology`](https://www.w3.org/TR/SVG11/filters.html#feMorphologyElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMorphology {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// See [`FeMorphologyMode`]
    pub mode: Option<Variable<FeMorphologyOperator>>,

    /// The radius (or radii) for the operation. If two `number`s are provided, the first number represents
    /// a x-radius and the second value represents a y-radius. If one number is provided, then that value
    /// is used for both X and Y. The values are in the coordinate system established by attribute
    /// ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables the effect of the given
    /// filter primitive (i.e., the result is a transparent black image).
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub radius: Option<Variable<NumberOptNumber>>,
}

/// This filter primitive offsets the input image relative to its current position in the image space by the specified vector.
///
/// This is important for effects like drop shadows.
///
/// See [`feOffset`](https://www.w3.org/TR/SVG11/filters.html#feOffsetElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeOffset {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// The amount to offset the input graphic along the x-axis. The offset amount is expressed in the coordinate system established
    /// by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub dx: Option<Variable<Number>>,

    /// The amount to offset the input graphic along the y-axis. The offset amount is expressed in the coordinate system established
    /// by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub dy: Option<Variable<Number>>,
}

/// This filter primitive lights a source graphic using the alpha channel as a bump map.
/// The resulting image is an RGBA image based on the light color. The lighting calculation follows the standard specular component of
/// the Phong lighting model. The resulting image depends on the light color, light position and surface geometry of the input bump map.
/// The result of the lighting calculation is added. The filter primitive assumes that the viewer is at infinity in the z direction (i.e.,
/// the unit vector in the eye direction is (0,0,1) everywhere).
///
/// See [`feSpecularLighting`](https://www.w3.org/TR/SVG11/filters.html#feSpecularLightingElement)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpecularLighting {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,

    /// height of surface when Ain = 1.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub surface_scale: Option<Variable<Number>>,

    /// height of surface when Ain = 1.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub specular_constant: Option<Variable<Number>>,

    /// Exponent for specular term, larger is more "shiny". Range 1.0 to 128.0.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub specular_exponent: Option<Variable<Number>>,

    /// The first number is the `dx` value. The second number is the `dy` value. If the `dy` value is not specified,
    /// it defaults to the same value as `dx`. Indicates the intended distance in current filter units (i.e., units
    /// as determined by the value of attribute ‘primitiveUnits’) for dx and dy, respectively, in the surface normal
    /// calculation formulas. By specifying value(s) for ‘kernelUnitLength’, the kernel becomes defined in a scalable,
    /// abstract coordinate system. If ‘kernelUnitLength’ is not specified, the dx and dy values should represent very
    /// small deltas relative to a given (x,y) position, which might be implemented in some cases as one pixel in the
    /// intermediate image offscreen bitmap, which is a pixel-based coordinate system, and thus potentially not scalable.
    /// For some level of consistency across display media and user agents, it is necessary that a value be provided
    /// for at least one of ‘filterRes’ and ‘kernelUnitLength’. Discussion of intermediate images are in the Introduction
    /// and in the description of attribute ‘filterRes’.
    ///
    /// A negative or zero value is an error (see Error processing).
    pub kernel_unit_len: Option<Variable<NumberOptNumber>>,
}

/// This filter primitive fills a target rectangle with a repeated, tiled pattern of an input image. The target rectangle is
/// as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the ‘feTile’
/// element.
///
/// See [`feTitle`](https://www.w3.org/TR/SVG11/filters.html#feTitleElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTile {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Option<Variable<FeIn>>,
}

/// See [`stitch_tiles`](FeTurbulence::stitch_tiles)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeStitchTiles {
    /// If stitchTiles="stitch", then the user agent will automatically adjust baseFrequency-x and baseFrequency-y values
    /// such that the feTurbulence node's width and height (i.e., the width and height of the current subregion) contains
    /// an integral number of the Perlin tile width and height for the first octave. The baseFrequency will be adjusted up
    /// or down depending on which way has the smallest relative (not absolute) change as follows: Given the frequency,
    /// calculate lowFreq=floor(width*frequency)/width and hiFreq=ceil(width*frequency)/width. If frequency/lowFreq < hiFreq/frequency
    /// then use lowFreq, else use hiFreq. While generating turbulence values, generate lattice vectors as normal for Perlin Noise,
    /// except for those lattice points that lie on the right or bottom edges of the active area (the size of the resulting tile).
    /// In those cases, copy the lattice vector from the opposite edge of the active area.
    Stitch,
    /// If stitchTiles="noStitch", no attempt it made to achieve smooth transitions at the border of tiles which contain a turbulence
    /// function. Sometimes the result will show clear discontinuities at the tile borders.
    NoStitch,
}

impl Default for FeStitchTiles {
    fn default() -> Self {
        Self::NoStitch
    }
}

// Indicates whether the filter primitive should perform a noise or turbulence function.
// If attribute ‘type’ is not specified, then the effect is as if a value of turbulence were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeTurbulenceType {
    FractalNoise,
    Turbulence,
}

impl Default for FeTurbulenceType {
    fn default() -> Self {
        Self::Turbulence
    }
}

/// This filter primitive creates an image using the Perlin turbulence function.
/// It allows the synthesis of artificial textures like clouds or marble. For a detailed description the of the Perlin turbulence
/// function, see "Texturing and Modeling", Ebert et al, AP Professional, 1994. The resulting image will fill the entire filter
/// primitive subregion for this filter primitive.
///
/// See [`feTurbulence`](https://www.w3.org/TR/SVG11/filters.html#feTurbulenceElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTurbulence {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// The base frequency (frequencies) parameter(s) for the noise function. If two `number`s are provided, the first number
    /// represents a base frequency in the X direction and the second value represents a base frequency in the Y direction.
    /// If one number is provided, then that value is used for both X and Y.
    ///
    /// A negative value for base frequency is an error (see Error processing).
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specifie.
    pub base_frequency: Option<Variable<NumberOptNumber>>,

    /// The numOctaves parameter for the noise function.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub num_octaves: Option<Variable<Integer>>,

    /// The starting number for the pseudo random number generator.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    /// When the seed number is handed over to the algorithm above it must first be truncated, i.e.
    /// rounded to the closest integer value towards zero.
    pub seed: Option<Variable<Number>>,

    /// See [`FeStitchTiles`]
    pub stitch_tiles: Option<Variable<FeStitchTiles>>,

    /// See [`FeStitchTiles`]
    pub r#type: Option<Variable<FeTurbulenceType>>,
}
