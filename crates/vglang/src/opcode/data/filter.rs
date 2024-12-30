use super::Number;

/// Data value used by `enable-background` property.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Background {
    /// A meaning of enable-background: accumulate (the initial/default value) depends on context:
    ///
    /// * If an ancestor container element has a property value of enable-background: new, then all
    /// graphics elements within the current container element are rendered both onto the parent container
    /// element's background image canvas and onto the target device.
    ///
    /// * Otherwise, there is no current background image canvas, so it is only necessary to render graphics
    /// elements onto the target device. (No need to render to the background image canvas.)
    Accumulate,
    /// Indicate the subregion of the container element's user space where access to the background image is allowed to happen.
    New {
        x: Option<f32>,
        y: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
    },
}

impl Default for Background {
    fn default() -> Self {
        Self::Accumulate
    }
}

impl<X, Y, W, H> From<(X, Y, W, H)> for Background
where
    Number: From<X> + From<Y> + From<W> + From<H>,
{
    fn from(value: (X, Y, W, H)) -> Self {
        Self::New {
            x: Some(Number::from(value.0).0),
            y: Some(Number::from(value.1).0),
            width: Some(Number::from(value.2).0),
            height: Some(Number::from(value.3).0),
        }
    }
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
    Ref(String),
}

impl Default for FeIn {
    fn default() -> Self {
        Self::SourceGraphic
    }
}

impl<T> From<T> for FeIn
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self::Ref(value.into())
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

/// transfer functions for the rgba channels.
///
/// See [`FeComponentTransfer`](crate::opcode::el::FeComponentTransfer)
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

/// See [`stitch_tiles`](crate::opcode::el::FeTurbulence::stitch_tiles)
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
