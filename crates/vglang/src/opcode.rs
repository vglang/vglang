#[doc = " Angles are specified in one of two ways depending upon"]
#[doc = " whether they are used in CSS property syntax or SVG"]
#[doc = " presentation attribute syntax:"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    Deg(Option<f32>),
    Grad(Option<f32>),
    Rad(Option<f32>),
}
#[doc = " A length is a distance Length, given as a number along with a unit which may be optional."]
#[doc = " "]
#[doc = " See [`length`](https://www.w3.org/TR/SVG11/types.html#DataTypeLength)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Length {
    #[doc = " Represents the calculated font-size of the element. If used on the font-size property itself,"]
    #[doc = " it represents the inherited font-size of the element."]
    Em(Option<f32>),
    #[doc = " Represents the x-height of the element's font. In fonts with the x letter, this is generally"]
    #[doc = " the height of lowercase letters in the font; 1ex ≈ 0.5em in many fonts."]
    Ex(Option<f32>),
    #[doc = " Pixels"]
    Px(Option<f32>),
    #[doc = " Inches"]
    Inch(Option<f32>),
    #[doc = " Centimeters"]
    Cm(Option<f32>),
    #[doc = " Millimeters"]
    Mm(Option<f32>),
    #[doc = " Points, 1pt = 1/72nd of 1in"]
    Pt(Option<f32>),
    #[doc = " Picas, \t1pc = 1/6th of 1in"]
    Pc(Option<f32>),
    #[doc = " A percentage value"]
    Percent(Option<f32>),
}
#[doc = " A color represents with read,green and blue components."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb(pub Option<u8>, pub Option<u8>, pub Option<u8>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Iri(pub Option<String>);
#[doc = " Functional notation for a reference. The syntax for this reference is the same as the [`CSS URI`]."]
#[doc = " "]
#[doc = " [`CSS URI`]: https://developer.mozilla.org/en-US/docs/Web/CSS/url_value"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FuncIri {
    Iri(Option<Iri>),
    Path(Option<String>),
}
#[doc = " A 2d coordinate point."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point(pub Option<f32>, pub Option<f32>);
#[doc = " Percentages are specified as a number followed by a \"%\" character:"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Percent(pub Option<f32>);
#[doc = " ‘fill’ and ‘stroke’ take on a value of type [`Paint`], which is specified as follows:"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    #[doc = " the explicit color to be used to paint the current object"]
    Color(Option<Rgb>),
    #[doc = " A reference to a paint server."]
    Server(Option<FuncIri>),
}
#[doc = " A pair of `number`s, where the second `number` is optional."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber(pub Option<f32>, pub Option<f32>);
#[doc = " Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’."]
#[doc = " "]
#[doc = " If attribute ‘gradientUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Coords {
    #[doc = " If gradientUnits=\"userSpaceOnUse\", ‘x1’, ‘y1’, ‘x2’ and ‘y2’ represent values in the coordinate system"]
    #[doc = " that results from taking the current user coordinate system in place at the time when the gradient element"]
    #[doc = " is referenced (i.e., the user coordinate system for the element referencing the gradient element via a ‘fill’"]
    #[doc = " or ‘stroke’ property) and then applying the transform specified by attribute ‘gradientTransform’."]
    UserSpaceOnUse(),
    #[doc = " If gradientUnits=\"objectBoundingBox\", the user coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’ is"]
    #[doc = " established using the bounding box of the element to which the gradient is applied (see Object bounding box units)"]
    #[doc = " and then applying the transform specified by attribute ‘gradientTransform’."]
    #[doc = " "]
    #[doc = " When gradientUnits=\"objectBoundingBox\" and ‘gradientTransform’ is the identity matrix, the normal of the linear"]
    #[doc = " gradient is perpendicular to the gradient vector in object bounding box space (i.e., the abstract coordinate"]
    #[doc = " system where (0,0) is at the top/left of the object bounding box and (1,1) is at the bottom/right of the object bounding box)."]
    #[doc = " When the object's bounding box is not square, the gradient normal which is initially perpendicular to the gradient vector"]
    #[doc = " within object bounding box space may render non-perpendicular relative to the gradient vector in user space. If the gradient"]
    #[doc = " vector is parallel to one of the axes of the bounding box, the gradient normal will remain perpendicular. This transformation"]
    #[doc = " is due to application of the non-uniform scaling transformation from bounding box space to user space."]
    ObjectBoundingBox(),
}
#[doc = " A `transform` matrix type."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transform {
    Translate(Option<f32>, Option<f32>),
    Matrix(Option<[f32; 6usize]>),
    Scale(Option<f32>, Option<f32>),
    Rotate {
        angle: Option<f32>,
        cx: Option<f32>,
        cy: Option<f32>,
    },
    SkewX(Option<f32>),
    SkewY(Option<f32>),
}
#[doc = " Indicates which channel of rgba is selected."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    R(),
    G(),
    B(),
    A(),
}
#[doc = " The  property only applies to graphics elements that are contained within a [`ClipPath`](ClipPath) element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipRule {
    Nonzero(),
    EvenOdd(),
}
#[doc = " A direction that representation a path drawing commander."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    #[doc = " Close the current subpath by drawing a straight line from the current point to current subpath's initial point."]
    Close(),
    #[doc = " (absolute) Start a new sub-path at the given (x,y) coordinate."]
    MoveTo(Option<Point>),
    #[doc = " (relative) Start a new sub-path at the given (x,y) coordinate."]
    MoveToRelative(Option<Point>),
    #[doc = " (absolute) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point."]
    LineTo(Option<Point>),
    #[doc = " (relative) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point."]
    LineToRelative(Option<Point>),
    #[doc = " (absolute) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided."]
    Polyline(Option<Vec<Point>>),
    #[doc = " (relative) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided."]
    PolylineRelative(Option<Vec<Point>>),
    #[doc = " (absolute) Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezier {
        ctrl1: Option<Point>,
        ctrl2: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (relative) Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezierRelative {
        ctrl1: Option<Point>,
        ctrl2: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (smooth) Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezierSmooth {
        ctrl2: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (smooth, relative)Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezierSmoothRelative {
        ctrl2: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (absolute) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezier {
        ctrl: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (relative) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezierRelative {
        ctrl: Option<Point>,
        to_point: Option<Point>,
    },
    #[doc = " (smooth) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezierSmooth(Option<Point>),
    #[doc = " (smooth,relative) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezierSmoothRelative(Option<Point>),
    #[doc = " Draws an elliptical arc from the current point to `to` point."]
    #[doc = " "]
    #[doc = " The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints"]
    #[doc = " imposed by the other parameters."]
    Arc {
        rx: Option<f32>,
        ry: Option<f32>,
        x_rotation: Option<f32>,
        large_arc: Option<bool>,
        sweep: Option<bool>,
        to_point: Option<Point>,
    },
    #[doc = " (relative) Draws an elliptical arc from the current point to `to` point."]
    #[doc = " "]
    #[doc = " The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints"]
    #[doc = " imposed by the other parameters."]
    ArcRelative {
        rx: Option<f32>,
        ry: Option<f32>,
        x_rotation: Option<f32>,
        large_arc: Option<bool>,
        sweep: Option<bool>,
        to_point: Option<Point>,
    },
}
#[doc = " The ‘fill-rule’ property indicates the algorithm which is to be used to determine what parts of the canvas are"]
#[doc = " included inside the shape. For a simple, non-intersecting path, it is intuitively clear what region lies \"inside\";"]
#[doc = " however, for a more complex path, such as a path that intersects itself or where one subpath encloses another,"]
#[doc = " the interpretation of \"inside\" is not so obvious."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    #[doc = " This rule determines the \"insideness\" of a point on the canvas by drawing a ray from that point to infinity in"]
    #[doc = " any direction and then examining the places where a segment of the shape crosses the ray. Starting with a count of zero,"]
    #[doc = " add one each time a path segment crosses the ray from left to right and subtract one each time a path segment crosses"]
    #[doc = " the ray from right to left. After counting the crossings, if the result is zero then the point is outside the path."]
    #[doc = " Otherwise, it is inside."]
    Nonzero(),
    #[doc = " This rule determines the \"insideness\" of a point on the canvas by drawing a ray from that point to infinity in any direction"]
    #[doc = " and counting the number of path segments from the given shape that the ray crosses. If this number is odd, the point is inside;"]
    #[doc = " if even, the point is outside."]
    EvenOdd(),
}
#[doc = " Specifies the shape to be used at the end of open subpaths when they are stroked"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt(),
    Round(),
    Square(),
}
#[doc = " Specifies the shape to be used at the corners of paths or basic shapes when they are stroked."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineJoin {
    Miter(Option<f32>),
    Round(),
    Bevel(),
}
#[doc = " Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle."]
#[doc = " Possible values are: 'pad', which says to use the terminal colors of the gradient to fill the remainder of the target region,"]
#[doc = " 'reflect', which says to reflect the gradient pattern start-to-end, end-to-start, start-to-end, etc. continuously until the"]
#[doc = " target rectangle is filled, and repeat, which says to repeat the gradient pattern start-to-end, start-to-end, start-to-end,"]
#[doc = " etc. continuously until the target region is filled."]
#[doc = " If the attribute is not specified, the effect is as if a value of 'pad' were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpreadMethod {
    Pad(),
    Reflect(),
    Repeat(),
}
#[doc = " See [`css2`](https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-font-style)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal(),
    Italic(),
    Oblique(),
}
#[doc = " Same syntax and semantics as the ‘font-variant’ descriptor within an @font-face rule."]
#[doc = " Indication of whether this face is the small-caps variant of a font. Takes on the same values"]
#[doc = " as the ‘font-variant’ property, except that a comma-separated list is permitted."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of 'normal' were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    Normal(),
    SmallCaps(),
}
#[doc = " Same syntax and semantics as the ‘font-weight’ descriptor within an @font-face rule."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Normal(),
    Bold(),
    Bolder(),
    Lighter(),
    W100(),
    W200(),
    W300(),
    W400(),
    W500(),
    W600(),
    W700(),
    W800(),
    W900(),
}
#[doc = " This property specifies a prioritized font family names and/or generic family names."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    Serif(),
    SansSerif(),
    Cursive(),
    Fantasy(),
    Monospace(),
    Generic(Option<String>),
}
#[doc = " Same syntax and semantics as the ‘font-stretch’ descriptor within an @font-face rule."]
#[doc = " Indication of the condensed or expanded nature of the face relative to others in the same font family."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Normal(),
    Wider(),
    Narrower(),
    UltraCondensed(),
    ExtraCondensed(),
    Condensed(),
    SemiCondensed(),
    SemiExpanded(),
    Expanded(),
    ExtraExpanded(),
    UltraExpanded(),
}
#[doc = " Data value used by `enable-background` property."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Background {
    #[doc = " A meaning of enable-background: accumulate (the initial/default value) depends on context:"]
    #[doc = " "]
    #[doc = " * If an ancestor container element has a property value of enable-background: new, then all"]
    #[doc = " graphics elements within the current container element are rendered both onto the parent container"]
    #[doc = " element's background image canvas and onto the target device."]
    #[doc = " "]
    #[doc = " * Otherwise, there is no current background image canvas, so it is only necessary to render graphics"]
    #[doc = " elements onto the target device. (No need to render to the background image canvas.)"]
    Accumulate(),
    #[doc = " Indicate the subregion of the container element's user space where access to the background image is allowed to happen."]
    New {
        x: Option<f32>,
        y: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
    },
}
#[doc = " Identifies input for the given filter primitive. The value can be either one of six keywords or"]
#[doc = " can be a string which matches a previous ‘result’ attribute value within the same ‘filter’ element."]
#[doc = " If no value is provided and this is the first filter primitive, then this filter primitive will use"]
#[doc = " SourceGraphic as its input. If no value is provided and this is a subsequent filter primitive, then"]
#[doc = " this filter primitive will use the result from the previous filter primitive as its input."]
#[doc = " "]
#[doc = " If the value for ‘result’ appears multiple times within a given ‘filter’ element, then a reference"]
#[doc = " to that result will use the closest preceding filter primitive with the given value for attribute"]
#[doc = " ‘result’. Forward references to results are an error."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeIn {
    #[doc = " This keyword represents the graphics elements that were the original input into the ‘filter’ element."]
    #[doc = " For raster effects filter primitives, the graphics elements will be rasterized into an initially clear"]
    #[doc = " RGBA raster in image space. Pixels left untouched by the original graphic will be left clear. The image"]
    #[doc = " is specified to be rendered in linear RGBA pixels. The alpha channel of this image captures any"]
    #[doc = " anti-aliasing specified by SVG. (Since the raster is linear, the alpha channel of this image will"]
    #[doc = " represent the exact percent coverage of each pixel.)"]
    SourceGraphic(),
    #[doc = " This keyword represents the graphics elements that were the original input into the ‘filter’ element."]
    #[doc = " SourceAlpha has all of the same rules as SourceGraphic except that only the alpha channel is used."]
    #[doc = " The input image is an RGBA image consisting of implicitly black color values for the RGB channels,"]
    #[doc = " but whose alpha channel is the same as SourceGraphic. If this option is used, then some implementations"]
    #[doc = " might need to rasterize the graphics elements in order to extract the alpha channel."]
    SourceAlpha(),
    #[doc = " This keyword represents an image snapshot of the canvas under the filter region at the time that the"]
    #[doc = " ‘filter’ element was invoked."]
    BackgroundImage(),
    #[doc = " Same as BackgroundImage except only the alpha channel is used. See SourceAlpha and Accessing the background image."]
    BackgroundAlpha(),
    #[doc = " This keyword represents the value of the ‘fill’ property on the target element for the filter effect."]
    #[doc = " The FillPaint image has conceptually infinite extent. Frequently this image is opaque everywhere,"]
    #[doc = " but it might not be if the \"paint\" itself has alpha, as in the case of a gradient or pattern which"]
    #[doc = " itself includes transparent or semi-transparent parts."]
    FillPaint(),
    #[doc = " This keyword represents the value of the ‘stroke’ property on the target element for the filter effect."]
    #[doc = " The StrokePaint image has conceptually infinite extent. Frequently this image is opaque everywhere,"]
    #[doc = " but it might not be if the \"paint\" itself has alpha, as in the case of a gradient or pattern which"]
    #[doc = " itself includes transparent or semi-transparent parts."]
    StrokePaint(),
    #[doc = " Reference to another filter-primitive result ."]
    Result(Option<String>),
}
#[doc = " Assign output to a named register. otherwise the filter output will only be referenced by next filter primitive."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeOut {
    Position(),
    Named(Option<String>),
}
#[doc = " Image blending modes"]
#[doc = " For the compositing formulas below, the following definitions apply:"]
#[doc = " * cr = Result color (RGB) - premultiplied"]
#[doc = " * qa = Opacity value at a given pixel for image A"]
#[doc = " * qb = Opacity value at a given pixel for image B"]
#[doc = " * ca = Color (RGB) at a given pixel for image A - premultiplied"]
#[doc = " * cb = Color (RGB) at a given pixel for image B - premultiplied"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeBlendMode {
    #[doc = " cr = (1 - qa) * cb + ca"]
    Normal(),
    #[doc = " cr = (1-qa)*cb + (1-qb)*ca + ca*cb"]
    Multiply(),
    #[doc = " cr = cb + ca - ca * cb"]
    Screen(),
    #[doc = " cr = Min ((1 - qa) * cb + ca, (1 - qb) * ca + cb)"]
    Darken(),
    #[doc = " cr = Max ((1 - qa) * cb + ca, (1 - qb) * ca + cb)"]
    Lighten(),
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextLengthAdjust {
    #[doc = " 'spacing' indicates that only the advance values are adjusted. The glyphs themselves are not stretched or compressed."]
    Spacing(),
    #[doc = " 'spacingAndGlyphs' indicates that the advance values are adjusted and the glyphs themselves stretched or compressed"]
    #[doc = " in one axis (i.e., a direction parallel to the inline-progression-direction)."]
    SpacingAndGlyphs(),
}
#[doc = " The ‘writing-mode’ property specifies whether the initial inline-progression-direction for a ‘text’ element shall be"]
#[doc = " left-to-right, right-to-left, or top-to-bottom. The ‘writing-mode’ property applies only to ‘text’ elements;"]
#[doc = " the property is ignored for ‘tspan’, ‘tref’, ‘altGlyph’ and ‘textPath’ sub-elements. (Note that the inline-progression-direction"]
#[doc = " can change within a ‘text’ element due to the Unicode bidirectional algorithm and properties ‘direction’ and ‘unicode-bidi’."]
#[doc = " For more on bidirectional text, see Relationship with bidirectionality.)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WritingMode {
    #[doc = " Sets the initial inline-progression-direction to left-to-right, as is common in most Latin-based documents."]
    #[doc = " For most characters, the current text position is advanced from left to right after each glyph is rendered."]
    #[doc = " (When the character data includes characters which are subject to the Unicode bidirectional algorithm, the text"]
    #[doc = " advance rules are more complex. See Relationship with bidirectionality)."]
    LrTb(),
    #[doc = " Sets the initial inline-progression-direction to right-to-left, as is common in Arabic or Hebrew scripts."]
    #[doc = " (See Relationship with bidirectionality.)"]
    RlTb(),
    #[doc = " Sets the initial inline-progression-direction to top-to-bottom, as is common in some Asian scripts,"]
    #[doc = " such as Chinese and Japanese. Though hardly as frequent as horizontal, this type of vertical layout also occurs"]
    #[doc = " in Latin based documents, particularly in table column or row labels. In most cases, the vertical baselines"]
    #[doc = " running through the middle of each glyph are aligned."]
    TbRl(),
    #[doc = " See [`LrTb`](WritingMode::LrTb)"]
    Lr(),
    #[doc = " See [`RlTb`](WritingMode::RlTb)"]
    Rl(),
    #[doc = " See [`TbRl`](WritingMode::TbRl)"]
    Tb(),
}
#[doc = " Within text content elements, the alignment of text with regards to the ‘text-anchor’ property is determined by"]
#[doc = " the value of the ‘direction’ property. For example, given a ‘text’ element with a ‘text-anchor’ value of \"end\","]
#[doc = " for a ‘direction’ value of \"ltr\", the text will extend to the left of the position of the ‘text’ element's ‘x’"]
#[doc = " attribute value, while for ‘direction’ value of \"rtl\", the text will extend to the right of the position of the"]
#[doc = " ‘text’ element's ‘x’ attribute value."]
#[doc = " "]
#[doc = " A more complete discussion of bidirectionality can be found in the Text direction section of"]
#[doc = " [`CSS 2`](https://www.w3.org/TR/CSS2/visuren.html#direction)."]
#[doc = " "]
#[doc = " See [`direction`](https://www.w3.org/TR/SVG11/text.html#DirectionProperty)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    Ltr(),
    Rtl(),
}
#[doc = " Except for any additional information provided in this specification, the normative definition of the"]
#[doc = " [`unicode-bidi`] property is in CSS2"]
#[doc = " "]
#[doc = " [`unicode-bidi`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/visuren.html#propdef-unicode-bidi"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeBidi {
    Normal(),
    Embed(),
    BidiOverride(),
}
#[doc = " The ‘text-anchor’ property is used to align (start-, middle- or end-alignment) a string of text relative to a given point."]
#[doc = " "]
#[doc = " The ‘text-anchor’ property is applied to each individual text chunk within a given ‘text’ element. Each text chunk has an"]
#[doc = " initial current text position, which represents the point in the user coordinate system resulting from (depending on context)"]
#[doc = " application of the ‘x’ and ‘y’ attributes on the ‘text’ element, any ‘x’ or ‘y’ attribute values on a ‘tspan’, ‘tref’ or"]
#[doc = " ‘altGlyph’ element assigned explicitly to the first rendered character in a text chunk, or determination of the initial current"]
#[doc = " text position for a ‘textPath’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAnchor {
    #[doc = " The rendered characters are aligned such that the start of the resulting rendered text is at the initial current text position."]
    #[doc = " For an element with a ‘direction’ property value of \"ltr\" (typical for most European languages), the left side of the text is"]
    #[doc = " rendered at the initial text position. For an element with a ‘direction’ property value of \"rtl\" (typical for Arabic and Hebrew),"]
    #[doc = " the right side of the text is rendered at the initial text position. For an element with a vertical primary text direction"]
    #[doc = " (often typical for Asian text), the top side of the text is rendered at the initial text position."]
    Start(),
    #[doc = " The rendered characters are aligned such that the geometric middle of the resulting rendered text is at the initial"]
    #[doc = " current text position."]
    Middle(),
    #[doc = " The rendered characters are aligned such that the end of the resulting rendered text is at the initial current text position."]
    #[doc = " For an element with a ‘direction’ property value of \"ltr\" (typical for most European languages), the right side of the text is"]
    #[doc = " rendered at the initial text position. For an element with a ‘direction’ property value of \"rtl\" (typical for Arabic and Hebrew),"]
    #[doc = " the left side of the text is rendered at the initial text position. For an element with a vertical primary text direction (often"]
    #[doc = " typical for Asian text), the bottom of the text is rendered at the initial text position."]
    End(),
}
#[doc = " See [`baseline`](https://www.w3.org/TR/SVG11/text.html#BaselineAlignmentProperties)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DominantBaseline {
    #[doc = " If this property occurs on a ‘text’ element, then the computed value depends on the value of the ‘writing-mode’ property."]
    #[doc = " If the 'writing-mode' is horizontal, then the value of the dominant-baseline component is 'alphabetic', else if the"]
    #[doc = " 'writing-mode' is vertical, then the value of the dominant-baseline component is 'central'."]
    #[doc = " "]
    #[doc = " If this property occurs on a ‘tspan’, ‘tref’, ‘altGlyph’ or ‘textPath’ element, then the dominant-baseline and the"]
    #[doc = " baseline-table components remain the same as those of the parent text content element. If the computed ‘baseline-shift’"]
    #[doc = " value actually shifts the baseline, then the baseline-table font-size component is set to the value of the ‘font-size’"]
    #[doc = " property on the element on which the ‘dominant-baseline’ property occurs, otherwise the baseline-table font-size remains"]
    #[doc = " the same as that of the element. If there is no parent text content element, the scaled-baseline-table value is"]
    #[doc = " constructed as above for ‘text’ elements."]
    Auto(),
    #[doc = " The dominant-baseline and the baseline-table components are set by determining the predominant script of the character"]
    #[doc = " data content. The ‘writing-mode’, whether horizontal or vertical, is used to select the appropriate set of baseline-tables"]
    #[doc = " and the dominant baseline is used to select the baseline-table that corresponds to that baseline. The baseline-table"]
    #[doc = " font-size component is set to the value of the ‘font-size’ property on the element on which the ‘dominant-baseline’ property"]
    #[doc = " occurs."]
    UseScript(),
    #[doc = " The dominant-baseline, the baseline-table, and the baseline-table font-size remain the same as that of the parent text"]
    #[doc = " content element."]
    NoChange(),
    #[doc = " The dominant-baseline and the baseline-table remain the same, but the baseline-table font-size is changed to the value"]
    #[doc = " of the ‘font-size’ property on this element. This re-scales the baseline-table for the current ‘font-size’."]
    ResetSize(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'ideographic', the derived baseline-table is constructed"]
    #[doc = " using the 'ideographic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Ideographic(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'alphabetic', the derived baseline-table is constructed"]
    #[doc = " using the 'alphabetic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Alphabetic(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'hanging', the derived baseline-table is constructed using"]
    #[doc = " the 'hanging' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    Hanging(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'mathematical', the derived baseline-table is constructed"]
    #[doc = " using the 'mathematical' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Mathematical(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'central'. The derived baseline-table is constructed from"]
    #[doc = " the defined baselines in a baseline-table in the nominal font. That font baseline-table is chosen using the following"]
    #[doc = " priority order of baseline-table names: 'ideographic', 'alphabetic', 'hanging', 'mathematical'. The baseline-table font-size"]
    #[doc = " is changed to the value of the ‘font-size’ property on this element."]
    Central(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'middle'. The derived baseline-table is constructed from"]
    #[doc = " the defined baselines in a baseline-table in the nominal font. That font baseline -table is chosen using the following"]
    #[doc = " priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'. The baseline-table"]
    #[doc = " font-size is changed to the value of the ‘font-size’ property on this element."]
    Middle(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'text-after-edge'. The derived baseline-table is constructed"]
    #[doc = " from the defined baselines in a baseline-table in the nominal font. The choice of which font baseline-table to use from the"]
    #[doc = " baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    #[doc = " "]
    #[doc = " NOTE: using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'"]
    #[doc = " is probably a reasonable strategy for determining which font baseline-table to use."]
    TextAfterEdge(),
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'text-before-edge'. The derived baseline-table is constructed"]
    #[doc = " from the defined baselines in a baseline-table in the nominal font. The choice of which baseline-table to use from the"]
    #[doc = " baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    #[doc = " "]
    #[doc = " NOTE: Using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'"]
    #[doc = " is probably a reasonable strategy for determining which font baseline-table to use."]
    TextBeforeEdge(),
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignmentBaseline {
    #[doc = " The value is the dominant-baseline of the script to which the character belongs - i.e.,"]
    #[doc = " use the dominant-baseline of the parent."]
    Auto(),
    #[doc = " The alignment-point of the object being aligned is aligned with the dominant-baseline of"]
    #[doc = " the parent text content element."]
    Baseline(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"before-edge\" baseline of"]
    #[doc = " the parent text content element."]
    BeforeEdge(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"text-before-edge\" baseline of"]
    #[doc = " the parent text content element."]
    TextBeforeEdge(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"middle\" baseline of the parent text content element."]
    Middle(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"central\" baseline of the parent text content element."]
    Central(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"after-edge\" baseline of the parent text content element."]
    AfterEdge(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"text-after-edge\" baseline of the parent text content element."]
    TextAfterEdge(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"ideographic\" baseline of the parent text content element."]
    Ideographic(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"alphabetic\" baseline of the parent text content element."]
    Alphabetic(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"hanging\" baseline of the parent text content element."]
    Hanging(),
    #[doc = " The alignment-point of the object being aligned is aligned with the \"mathematical\" baseline of the parent text content element."]
    Mathematical(),
}
#[doc = " The ‘baseline-shift’ property allows repositioning of the dominant-baseline relative to the dominant-baseline of"]
#[doc = " the parent text content element. The shifted object might be a sub- or superscript. Within the shifted object,"]
#[doc = " the whole baseline-table is offset; not just a single baseline. The amount of the shift is determined from information"]
#[doc = " from the parent text content element, the sub- or superscript offset from the nominal font of the parent text content"]
#[doc = " element, percent of the \"line-height\" of the parent text content element or an absolute value."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaselineShift {
    #[doc = " There is no baseline shift; the dominant-baseline remains in its original position."]
    Baseline(),
    #[doc = " The dominant-baseline is shifted to the default position for subscripts. The offset to this position"]
    #[doc = " is determined using the font data for the nominal font. Because in most fonts the subscript position"]
    #[doc = " is normally given relative to the \"alphabetic\" baseline, the user agent may compute the effective"]
    #[doc = " position for subscripts for superscripts when some other baseline is dominant. The suggested computation"]
    #[doc = " is to subtract the difference between the position of the dominant baseline and the position of the"]
    #[doc = " \"alphabetic\" baseline from the position of the subscript. The resulting offset is determined by multiplying"]
    #[doc = " the effective subscript position by the dominant baseline-table font-size. If there is no applicable font"]
    #[doc = " data the user agent may use heuristics to determine the offset."]
    Sub(),
    #[doc = " The dominant-baseline is shifted to the default position for superscripts. The offset to this position is"]
    #[doc = " determined using the font data for the nominal font. Because in most fonts the superscript position is normally"]
    #[doc = " given relative to the \"alphabetic\" baseline, the user agent may compute the effective position for superscripts"]
    #[doc = " when some other baseline is dominant. The suggested computation is to subtract the difference between the"]
    #[doc = " position of the dominant baseline and the position of the \"alphabetic\" baseline from the position of the"]
    #[doc = " superscript. The resulting offset is determined by multiplying the effective superscript position by the dominant"]
    #[doc = " baseline-table font-size. If there is no applicable font data the user agent may use heuristics to determine the"]
    #[doc = " offset."]
    Super(),
    #[doc = " The computed value of the property is this percentage multiplied by the computed \"line-height\" of the ‘text’ element."]
    #[doc = " The dominant-baseline is shifted in the shift direction (positive value) or opposite to the shift direction"]
    #[doc = " (negative value) of the parent text content element by the computed value. A value of \"0\" is equivalent to \"baseline\"."]
    Value(Option<Length>),
}
#[doc = " This property describes decorations that are added to the text of an element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDecoration {
    Underline(),
    Overline(),
    LineThrough(),
    Blink(),
}
#[doc = " Indicates the method by which text should be rendered along the path."]
#[doc = " "]
#[doc = " A value of align indicates that the glyphs should be rendered using simple 2x3 transformations such"]
#[doc = " that there is no stretching/warping of the glyphs. Typically, supplemental rotation, scaling and"]
#[doc = " translation transformations are done for each glyph to be rendered. As a result, with align, fonts"]
#[doc = " where the glyphs are designed to be connected (e.g., cursive fonts), the connections may not align"]
#[doc = " properly when text is rendered along a path."]
#[doc = " "]
#[doc = " A value of stretch indicates that the glyph outlines will be converted into paths, and then all end"]
#[doc = " points and control points will be adjusted to be along the perpendicular vectors from the path,"]
#[doc = " thereby stretching and possibly warping the glyphs. With this approach, connected glyphs, such as in"]
#[doc = " cursive scripts, will maintain their connections."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of align were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathMethod {
    Align(),
    Stretch(),
}
#[doc = " Indicates how the user agent should determine the spacing between glyphs that are to be rendered along a path."]
#[doc = " "]
#[doc = " A value of exact indicates that the glyphs should be rendered exactly according to the spacing rules as specified"]
#[doc = " in Text on a path layout rules."]
#[doc = " "]
#[doc = " "]
#[doc = " A value of auto indicates that the user agent should use text-on-a-path layout algorithms to adjust the spacing"]
#[doc = " between glyphs in order to achieve visually appealing results."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of exact were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathSpacing {
    Auto(),
    Exact(),
}
#[doc = " The letter-spacing attribute controls spacing between text characters."]
#[doc = " "]
#[doc = " See [`letter-spacing`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/letter-spacing)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LetterSpacing {
    Normal(),
    Length(Option<Length>),
}
#[doc = " The word-spacing attribute specifies spacing behavior between words."]
#[doc = " "]
#[doc = " See [`word-spacing`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/word-spacing)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WordSpacing {
    Normal(),
    Length(Option<Length>),
}
#[doc = " see [`svg`] document for more information."]
#[doc = " "]
#[doc = " [`svg`]: https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet(),
    Slice(),
}
#[doc = " In some cases, typically when using the ‘viewBox’ attribute, i"]
#[doc = " t is desirable that the graphics stretch to fit non-uniformly"]
#[doc = " to take up the entire viewport. In other cases, it is desirable"]
#[doc = " that uniform scaling be used for the purposes of preserving"]
#[doc = " the aspect ratio of the graphics."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    #[doc = " Force uniform scaling"]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMinYMin(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMidYMin(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMaxYMin(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMinYMid(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling(the default)."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMidYMid(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMaxYMid(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMinYMax(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMidYMax(Option<MeetOrSlice>),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMaxYMax(Option<MeetOrSlice>),
}
#[doc = " support for various international writing directions, such as left-to-right (e.g., Latin scripts) and"]
#[doc = " bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts)."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLayout {
    pub write_mode: Option<WritingMode>,
    pub direction: Option<TextDirection>,
    pub unicode_bidi: Option<UnicodeBidi>,
    pub anchor: Option<variable::Variable<TextAnchor>>,
    pub dominant_baseline: Option<variable::Variable<DominantBaseline>>,
    pub alignment_baseline: Option<variable::Variable<AlignmentBaseline>>,
    pub baseline_shift: Option<variable::Variable<BaselineShift>>,
    pub decoration: Option<variable::Variable<TextDecoration>>,
    pub letter_spacing: Option<variable::Variable<LetterSpacing>>,
    pub word_spacing: Option<variable::Variable<WordSpacing>>,
}
#[doc = " support for various international writing directions, such as left-to-right (e.g., Latin scripts) and"]
#[doc = " bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts)."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithTransform(pub Option<Vec<Transform>>);
#[doc = " Define a fragment with name."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub Option<String>);
#[doc = " The ‘fill’ instruction paints the interior of the given graphical element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    pub paint: Option<Paint>,
    pub rule: Option<FillRule>,
    pub opacity: Option<f32>,
}
#[doc = " This property affect how an element is stroked."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    pub paint: Option<variable::Variable<Paint>>,
    pub width: Option<variable::Variable<Length>>,
    pub linecap: Option<variable::Variable<StrokeLineCap>>,
    pub linejoin: Option<variable::Variable<StrokeLineJoin>>,
    pub dasharray: Option<variable::Variable<Vec<Length>>>,
    pub dashoffset: Option<variable::Variable<Length>>,
}
#[doc = " Shorthand property for setting ‘font-style’, ‘font-variant’, ‘font-weight’, ‘font-size’, ‘line-height’ and ‘font-family’."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    pub family: Option<variable::Variable<Vec<FontFamily>>>,
    pub style: Option<variable::Variable<FontStyle>>,
    pub variant: Option<variable::Variable<FontVariant>>,
    pub weight: Option<variable::Variable<FontWeight>>,
    pub size: Option<variable::Variable<Length>>,
    pub stretch: Option<variable::Variable<FontStretch>>,
}
#[doc = " enables access to the background image"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableBackground(pub Option<Background>);
#[doc = " Define a fragment by name."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithFilter(pub Option<String>);
#[doc = " Use mask to a element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithClipPath(pub Option<FuncIri>);
#[doc = " Use mask to a element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithMask(pub Option<FuncIri>);
#[doc = " Sspecifies object/group opacity"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(pub Option<f32>);
#[doc = " It is often desirable to specify that a given set of graphics stretch to fit a particular container element."]
#[doc = " The ‘viewBox’ attribute provides this capability."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBox {
    pub minx: Option<variable::Variable<f32>>,
    pub miny: Option<variable::Variable<f32>>,
    pub width: Option<variable::Variable<f32>>,
    pub height: Option<variable::Variable<f32>>,
    pub aspect: Option<variable::Variable<PreserveAspectRatio>>,
}
#[doc = " Create a new layer into which the backend render child elements."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canvas {
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
}
#[doc = " used as an alpha mask for compositing the current object into the background."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mask {
    pub units: Option<variable::Variable<Coords>>,
    pub content_units: Option<variable::Variable<Coords>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
}
#[doc = " A clipping path is defined with a ‘clipPath’ element."]
#[doc = " A clipping path is used/referenced using the ‘clip-path’ property."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPath(pub Option<variable::Variable<Coords>>);
#[doc = " A filter effect consists of a series of graphics operations that are applied to a given source graphic to"]
#[doc = " produce a modified graphical result. The result of the filter effect is rendered to the target device"]
#[doc = " instead of the original source graphic. The following illustrates the process:"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    pub units: Option<variable::Variable<Coords>>,
    pub primitive_units: Option<variable::Variable<Coords>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub res: Option<variable::Variable<NumberOptNumber>>,
}
#[doc = " Defines a distant light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[doc = " "]
#[doc = " The following diagram illustrates the angles which ‘azimuth’ and ‘elevation’ represent in an XYZ coordinate system."]
#[doc = " "]
#[doc = " ![`distance light source`](https://www.w3.org/TR/SVG11/images/filters/azimuth-elevation.png)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDistantLight {
    pub azimuth: Option<variable::Variable<f32>>,
    pub elevation: Option<variable::Variable<f32>>,
}
#[doc = " Defines a point light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePointLight {
    pub x: Option<variable::Variable<f32>>,
    pub y: Option<variable::Variable<f32>>,
    pub z: Option<variable::Variable<f32>>,
}
#[doc = " Defines a spot light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpotLight {
    pub x: Option<variable::Variable<f32>>,
    pub y: Option<variable::Variable<f32>>,
    pub z: Option<variable::Variable<f32>>,
    pub point_at_x: Option<variable::Variable<f32>>,
    pub point_at_y: Option<variable::Variable<f32>>,
    pub point_at_z: Option<variable::Variable<f32>>,
    pub specular_exponent: Option<variable::Variable<f32>>,
    pub limiting_cone_angle: Option<variable::Variable<f32>>,
}
#[doc = " This filter composites two objects together using commonly used imaging software blending modes."]
#[doc = " It performs a pixel-wise combination of two input images."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeBlend {
    pub mode: Option<variable::Variable<FeBlendMode>>,
    pub r#in: Option<variable::Variable<FeIn>>,
    pub in2: Option<variable::Variable<FeIn>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " Values of FeColorMatrix."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeColorMatrixValues {
    #[doc = " a list of 20 matrix values."]
    Matrix(Option<[f32; 20usize]>),
    #[doc = " `Saturate` is a single real number value (0 to 1)."]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    Saturate(Option<f32>),
    #[doc = " `HueRotate` is a single one real number value (degrees)"]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    HueRotate(Option<f32>),
    #[doc = " `LuminanceToAlpha`  is not applicable."]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    LuminanceToAlpha(),
}
#[doc = " This filter applies a matrix transformation."]
#[doc = " "]
#[doc = " on the RGBA color and alpha values of every pixel on the input graphics to produce a result with a new"]
#[doc = " set of RGBA color and alpha values."]
#[doc = " "]
#[doc = " The calculations are performed on non-premultiplied color values. If the input graphics consists of"]
#[doc = " premultiplied color values, those values are automatically converted into non-premultiplied color values"]
#[doc = " for this operation."]
#[doc = " "]
#[doc = " These matrices often perform an identity mapping in the alpha channel. If that is the case, an implementation"]
#[doc = " can avoid the costly undoing and redoing of the premultiplication for all pixels with A = 1."]
#[doc = " "]
#[doc = " See [`feColorMatrix`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeColorMatrix {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub values: Option<variable::Variable<FeColorMatrixValues>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " transfer functions for the rgba channels."]
#[doc = " "]
#[doc = " See [`FeComponentTransfer`](FeComponentTransfer)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeFunc {
    #[doc = " C' = C"]
    Identity(),
    #[doc = " For table, the function is defined by linear interpolation between values given in the attribute ‘tableValues’."]
    #[doc = " The table has n+1 values (i.e., v0 to vn) specifying the start and end values for n evenly sized interpolation regions."]
    #[doc = " Interpolations use the following formula:"]
    #[doc = " "]
    #[doc = " For a value C < 1 find k such that:"]
    #[doc = " "]
    #[doc = " > k/n <= C < (k+1)/n"]
    #[doc = " "]
    #[doc = " The result C' is given by:"]
    #[doc = " "]
    #[doc = " > C' = vk + (C - k/n)*n * (vk+1 - vk)"]
    #[doc = " "]
    #[doc = " If C = 1 then:"]
    #[doc = " "]
    #[doc = " > C' = vn."]
    Table(Option<Vec<f32>>),
    #[doc = " For discrete, the function is defined by the step function given in the attribute ‘tableValues’,"]
    #[doc = " which provides a list of n values (i.e., v0 to vn-1) in order to identify a step function consisting of n steps."]
    #[doc = " The step function is defined by the following formula:"]
    #[doc = " "]
    #[doc = " For a value C < 1 find k such that:"]
    #[doc = " "]
    #[doc = " > k/n <= C < (k+1)/n"]
    #[doc = " "]
    #[doc = " The result C' is given by:"]
    #[doc = " "]
    #[doc = " > C' = vk"]
    #[doc = " "]
    #[doc = " If C = 1 then:"]
    #[doc = " "]
    #[doc = " > C' = vn-1."]
    Discrete(Option<Vec<f32>>),
    #[doc = " For linear, the function is defined by the following linear equation:"]
    #[doc = " "]
    #[doc = " > C' = slope * C + intercept"]
    Linear {
        slope: Option<f32>,
        intercept: Option<f32>,
    },
    #[doc = " For gamma, the function is defined by the following exponential function:"]
    #[doc = " "]
    #[doc = " > C' = amplitude * pow(C, exponent) + offset"]
    Gamma {
        amplitude: Option<f32>,
        exponent: Option<f32>,
        offset: Option<f32>,
    },
}
#[doc = " The compositing operation that is to be performed. All of the ‘operator’ types except arithmetic match the"]
#[doc = " corresponding operation as described in `PORTERDUFF`. The arithmetic operator is described above. If attribute"]
#[doc = " ‘operator’ is not specified, then the effect is as if a value of over were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeCompositeOperator {
    Over(),
    In(),
    Out(),
    Atop(),
    Xor(),
    Arithmetic {
        k1: Option<f32>,
        k2: Option<f32>,
        k3: Option<f32>,
        k4: Option<f32>,
    },
}
#[doc = " Determines how to extend the input image as necessary with color values so that the matrix operations"]
#[doc = " can be applied when the kernel is positioned at or near the edge of the input image."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeConvolveMatrixEdgeMode {
    Duplicate(),
    Wrap(),
    None(),
}
#[doc = " A keyword indicating whether to erode (i.e., thin) or dilate (fatten) the source graphic."]
#[doc = " If attribute `mode` is not specified, then the effect is as if a value of erode were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeMorphologyOperator {
    Erode(),
    Dilate(),
}
#[doc = " See [`stitch_tiles`](FeTurbulence::stitch_tiles)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeStitchTiles {
    #[doc = " If stitchTiles=\"stitch\", then the user agent will automatically adjust baseFrequency-x and baseFrequency-y values"]
    #[doc = " such that the feTurbulence node's width and height (i.e., the width and height of the current subregion) contains"]
    #[doc = " an integral number of the Perlin tile width and height for the first octave. The baseFrequency will be adjusted up    /// or down depending on which way has the smallest relative (not absolute) change as follows: Given the frequency,"]
    #[doc = " calculate lowFreq=floor(width*frequency)/width and hiFreq=ceil(width*frequency)/width. If frequency/lowFreq < hiFreq/frequency"]
    #[doc = " then use lowFreq, else use hiFreq. While generating turbulence values, generate lattice vectors as normal for Perlin Noise,"]
    #[doc = " except for those lattice points that lie on the right or bottom edges of the active area (the size of the resulting tile)."]
    #[doc = " In those cases, copy the lattice vector from the opposite edge of the active area."]
    Stitch(),
    #[doc = " If stitchTiles=\"noStitch\", no attempt it made to achieve smooth transitions at the border of tiles which contain a turbulence"]
    #[doc = " function. Sometimes the result will show clear discontinuities at the tile borders."]
    NoStitch(),
}
#[doc = " Indicates whether the filter primitive should perform a noise or turbulence function."]
#[doc = " If attribute ‘type’ is not specified, then the effect is as if a value of turbulence were specified."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeTurbulenceType {
    FractalNoise(),
    Turbulence(),
}
#[doc = " This filter primitive performs component-wise remapping of data as follows:"]
#[doc = " "]
#[doc = " > R' = feFuncR( R )"]
#[doc = " "]
#[doc = " > G' = feFuncG( G )"]
#[doc = " "]
#[doc = " > B' = feFuncB( B )"]
#[doc = " "]
#[doc = " > A' = feFuncA( A )"]
#[doc = " "]
#[doc = " for every pixel. It allows operations like brightness adjustment, contrast adjustment, color balance or thresholding."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComponentTransfer(pub Option<variable::Variable<FeIn>>);
#[doc = " transfer function for the alpha component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncA(pub Option<FeFunc>);
#[doc = " transfer function for the red component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncR(pub Option<FeFunc>);
#[doc = " transfer function for the green component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncG(pub Option<FeFunc>);
#[doc = " transfer function for the blue component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncB(pub Option<FeFunc>);
#[doc = " This filter performs the combination of the two input images pixel-wise in image space using one of the Porter-Duff [`PORTERDUFF`]"]
#[doc = " compositing operations: over, in, atop, out, xor [`SVG-COMPOSITING`]. Additionally, a component-wise arithmetic operation (with"]
#[doc = " the result clamped between [0..1]) can be applied."]
#[doc = " "]
#[doc = " See [`feComposite`]."]
#[doc = " "]
#[doc = " [`feComposite`]: https://www.w3.org/TR/SVG11/filters.html#feCompositeElement"]
#[doc = " [`PORTERDUFF`]: https://www.w3.org/TR/SVG11/refs.html#ref-PORTERDUFF"]
#[doc = " [`SVG-COMPOSITING`]: https://www.w3.org/TR/SVG11/refs.html#ref-SVG-COMPOSITING"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComposite {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub in2: Option<variable::Variable<FeIn>>,
    pub operator: Option<variable::Variable<FeCompositeOperator>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " feConvolveMatrix applies a matrix convolution filter effect. A convolution combines pixels"]
#[doc = " in the input image with neighboring pixels to produce a resulting image. A wide variety"]
#[doc = " of imaging operations can be achieved through convolutions, including blurring, edge detection,"]
#[doc = " sharpening, embossing and beveling."]
#[doc = " "]
#[doc = " See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrix {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub order: Option<variable::Variable<NumberOptNumber>>,
    pub kernel: Option<variable::Variable<Vec<f32>>>,
    pub divisor: Option<variable::Variable<f32>>,
    pub bias: Option<variable::Variable<f32>>,
    pub target_x: Option<variable::Variable<i32>>,
    pub target_y: Option<variable::Variable<i32>>,
    pub edge_mode: Option<variable::Variable<FeConvolveMatrixEdgeMode>>,
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    pub preserve_alpha: Option<variable::Variable<bool>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feDiffuseLightingElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDiffuseLighting {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub surface_scale: Option<variable::Variable<f32>>,
    pub diffuse_constant: Option<variable::Variable<f32>>,
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive uses the pixels values from the image from ‘in2’ to spatially displace the image from ‘in’."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDisplacementMap {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub in2: Option<variable::Variable<FeIn>>,
    pub scale: Option<variable::Variable<f32>>,
    pub x_channel_selector: Option<variable::Variable<Channel>>,
    pub y_channel_selector: Option<variable::Variable<Channel>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive creates a rectangle filled with the color and opacity values from properties ‘flood-color’ a ‘flood-opacity’."]
#[doc = " The rectangle is as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the"]
#[doc = " ‘feFlood’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFlood {
    pub color: Option<variable::Variable<Rgb>>,
    pub opacity: Option<variable::Variable<f32>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive performs a Gaussian blur on the input image."]
#[doc = " "]
#[doc = " See [`feGaussianBlur`](https://www.w3.org/TR/SVG11/filters.html#feGaussianBlurElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeGaussianBlur {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub std_deviation: Option<variable::Variable<NumberOptNumber>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive composites input image layers on top of each other using the over operator with Input1"]
#[doc = " (corresponding to the first ‘feMergeNode’ child element) on the bottom and the last specified input, InputN"]
#[doc = " (corresponding to the last ‘feMergeNode’ child element), on top."]
#[doc = " "]
#[doc = " See [`feMerge`](https://www.w3.org/TR/SVG11/filters.html#feMergeElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMerge {
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " See [`FeMerge`]"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMergeNode(pub Option<variable::Variable<FeIn>>);
#[doc = " This filter primitive refers to a graphic external to this filter element, which is loaded or rendered into an RGBA"]
#[doc = " raster and becomes the result of the filter primitive."]
#[doc = " "]
#[doc = " See [`feImage`](https://www.w3.org/TR/SVG11/filters.html#feImageElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeImage {
    pub href: Option<variable::Variable<FuncIri>>,
    pub aspect: Option<variable::Variable<PreserveAspectRatio>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive performs \"fattening\" or \"thinning\" of artwork."]
#[doc = " It is particularly useful for fattening or thinning an alpha channel."]
#[doc = " "]
#[doc = " See [`feMorphology`](https://www.w3.org/TR/SVG11/filters.html#feMorphologyElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMorphology {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub mode: Option<variable::Variable<FeMorphologyOperator>>,
    pub radius: Option<variable::Variable<NumberOptNumber>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive offsets the input image relative to its current position in the image space by the specified vector."]
#[doc = " "]
#[doc = " This is important for effects like drop shadows."]
#[doc = " "]
#[doc = " See [`feOffset`](https://www.w3.org/TR/SVG11/filters.html#feOffsetElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeOffset {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub dx: Option<variable::Variable<f32>>,
    pub dy: Option<variable::Variable<f32>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive lights a source graphic using the alpha channel as a bump map."]
#[doc = " The resulting image is an RGBA image based on the light color. The lighting calculation follows the standard specular component of"]
#[doc = " the Phong lighting model. The resulting image depends on the light color, light position and surface geometry of the input bump map."]
#[doc = " The result of the lighting calculation is added. The filter primitive assumes that the viewer is at infinity in the z direction (i.e.,"]
#[doc = " the unit vector in the eye direction is (0,0,1) everywhere)."]
#[doc = " "]
#[doc = " See [`feSpecularLighting`](https://www.w3.org/TR/SVG11/filters.html#feSpecularLightingElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpecularLighting {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub surface_scale: Option<variable::Variable<f32>>,
    pub specular_constant: Option<variable::Variable<f32>>,
    pub specular_exponent: Option<variable::Variable<f32>>,
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive fills a target rectangle with a repeated, tiled pattern of an input image. The target rectangle is"]
#[doc = " as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the ‘feTile’"]
#[doc = " element."]
#[doc = " "]
#[doc = " See [`feTitle`](https://www.w3.org/TR/SVG11/filters.html#feTitleElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTile {
    pub r#in: Option<variable::Variable<FeIn>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " This filter primitive creates an image using the Perlin turbulence function."]
#[doc = " It allows the synthesis of artificial textures like clouds or marble. For a detailed description the of the Perlin turbulence"]
#[doc = " function, see \"Texturing and Modeling\", Ebert et al, AP Professional, 1994. The resulting image will fill the entire filter"]
#[doc = " primitive subregion for this filter primitive."]
#[doc = " "]
#[doc = " See [`feTurbulence`](https://www.w3.org/TR/SVG11/filters.html#feTurbulenceElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTurbulence {
    pub base_frequency: Option<variable::Variable<NumberOptNumber>>,
    pub num_octaves: Option<variable::Variable<i32>>,
    pub seed: Option<variable::Variable<f32>>,
    pub stitch_tiles: Option<variable::Variable<FeStitchTiles>>,
    pub r#type: Option<variable::Variable<FeTurbulenceType>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub result: Option<variable::Variable<String>>,
}
#[doc = " Linear gradients are defined by a ‘linearGradient’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    pub units: Option<variable::Variable<Coords>>,
    pub transform: Option<variable::Variable<Transform>>,
    pub x1: Option<variable::Variable<Length>>,
    pub y1: Option<variable::Variable<Length>>,
    pub x2: Option<variable::Variable<Length>>,
    pub y2: Option<variable::Variable<Length>>,
    pub spread: Option<variable::Variable<SpreadMethod>>,
}
#[doc = " Radial gradients are defined by a ‘radialGradient’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    pub unit: Option<variable::Variable<Coords>>,
    pub transform: Option<variable::Variable<Transform>>,
    pub cx: Option<variable::Variable<Length>>,
    pub cy: Option<variable::Variable<Length>>,
    pub r: Option<variable::Variable<Length>>,
    pub fx: Option<variable::Variable<Length>>,
    pub fy: Option<variable::Variable<Length>>,
    pub spread: Option<variable::Variable<SpreadMethod>>,
}
#[doc = " The ramp of colors to use on a gradient is defined by the ‘stop’ elements that are child elements"]
#[doc = " to either the ‘linearGradient’ element or the ‘radialGradient’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    pub offset: Option<variable::Variable<f32>>,
    pub color: Option<variable::Variable<Rgb>>,
    pub opacity: Option<variable::Variable<f32>>,
}
#[doc = " A container element for grouping together related graphics elements."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group();
#[doc = " Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,"]
#[doc = " or any combination of the three."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    pub events: Option<variable::Variable<Vec<PathEvent>>>,
    pub length: Option<variable::Variable<Length>>,
}
#[doc = " A pattern is used to fill or stroke an object using a pre-defined graphic object which can be replicated (\"tiled\")"]
#[doc = " at fixed intervals in x and y to cover the areas to be painted. Patterns are defined using a ‘pattern’ element and"]
#[doc = " then referenced by properties ‘fill’ and ‘stroke’ on a given graphics element to indicate that the given element"]
#[doc = " shall be filled or stroked with the referenced pattern."]
#[doc = " "]
#[doc = " Attributes ‘x’, ‘y’, ‘width’, ‘height’ and ‘patternUnits’ define a reference rectangle somewhere on the infinite canvas."]
#[doc = " The reference rectangle has its top/left at (x, y) and its bottom/right at (x + width, y + height). The tiling"]
#[doc = " theoretically extends a series of such rectangles to infinity in X and Y (positive and negative), with rectangles"]
#[doc = " starting at (x + m*width, y + n* height) for each possible integer value for m and n."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    pub units: Option<variable::Variable<Coords>>,
    pub content_units: Option<variable::Variable<Coords>>,
    pub transform: Option<variable::Variable<Transform>>,
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
}
#[doc = " Use a fragment by name."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Use(pub Option<variable::Variable<Iri>>);
#[doc = " The ‘rect’ element defines a rectangle which is axis-aligned with the current user coordinate system."]
#[doc = " Rounded rectangles can be achieved by setting appropriate values for attributes ‘rx’ and ‘ry’."]
#[doc = " "]
#[doc = " "]
#[doc = " The values used for the x- and y-axis rounded corner radii are determined implicitly if the ‘rx’ or ‘ry’ attributes (or both) are not specified, or are specified but with invalid values. The values are also subject to clamping so that the lengths of the straight segments of the rectangle are never negative. The effective values for ‘rx’ and ‘ry’ are determined by following these steps in order:"]
#[doc = " "]
#[doc = " 1. Let rx and ry be length values."]
#[doc = " 1. If neither ‘rx’ nor ‘ry’ are properly specified, then set both rx and ry to 0. (This will result in square corners.)"]
#[doc = " 1. Otherwise, if a properly specified value is provided for ‘rx’, but not for ‘ry’, then set both rx and ry to the value of ‘rx’."]
#[doc = " 1. Otherwise, if a properly specified value is provided for ‘ry’, but not for ‘rx’, then set both rx and ry to the value of ‘ry’."]
#[doc = " 1. Otherwise, both ‘rx’ and ‘ry’ were specified properly. Set rx to the value of ‘rx’ and ry to the value of ‘ry’."]
#[doc = " 1. If rx is greater than half of ‘width’, then set rx to half of ‘width’."]
#[doc = " 1. If ry is greater than half of ‘height’, then set ry to half of ‘height’."]
#[doc = " 1. The effective values of ‘rx’ and ‘ry’ are rx and ry, respectively."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub x: Option<variable::Variable<Length>>,
    pub y: Option<variable::Variable<Length>>,
    pub width: Option<variable::Variable<Length>>,
    pub height: Option<variable::Variable<Length>>,
    pub rx: Option<variable::Variable<Length>>,
    pub ry: Option<variable::Variable<Length>>,
}
#[doc = " The ‘circle’ element defines a circle based on a center point and a radius."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    pub cx: Option<variable::Variable<Length>>,
    pub cy: Option<variable::Variable<Length>>,
    pub r: Option<variable::Variable<Length>>,
}
#[doc = " The ‘ellipse’ element defines an ellipse which is axis-aligned with the current user coordinate"]
#[doc = " system based on a center point and two radii."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    pub cx: Option<variable::Variable<Length>>,
    pub cy: Option<variable::Variable<Length>>,
    pub rx: Option<variable::Variable<Length>>,
    pub ry: Option<variable::Variable<Length>>,
}
#[doc = " The ‘line’ element defines a line segment that starts at one point and ends at another."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    pub x1: Option<variable::Variable<Length>>,
    pub y1: Option<variable::Variable<Length>>,
    pub x2: Option<variable::Variable<Length>>,
    pub y2: Option<variable::Variable<Length>>,
}
#[doc = " The ‘polyline’ element defines a set of connected straight line segments. Typically, ‘polyline’ elements define open shapes."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(pub Option<variable::Variable<Vec<Point>>>);
#[doc = " The ‘polygon’ element defines a closed shape consisting of a set of connected straight line segments."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon(pub Option<variable::Variable<Vec<Point>>>);
#[doc = " The ‘text’ element defines a graphics element consisting of text."]
#[doc = " "]
#[doc = " See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    pub x: Option<variable::Variable<Vec<Length>>>,
    pub y: Option<variable::Variable<Vec<Length>>>,
    pub dx: Option<variable::Variable<Vec<Length>>>,
    pub dy: Option<variable::Variable<Vec<Length>>>,
    pub rotate: Option<variable::Variable<Vec<Angle>>>,
    pub text_length: Option<variable::Variable<Vec<Length>>>,
    pub length_adjust: Option<variable::Variable<TextLengthAdjust>>,
}
#[doc = " The ‘text’ element defines a graphics element consisting of text."]
#[doc = " "]
#[doc = " See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)"]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpan {
    pub x: Option<variable::Variable<Vec<Length>>>,
    pub y: Option<variable::Variable<Vec<Length>>>,
    pub dx: Option<variable::Variable<Vec<Length>>>,
    pub dy: Option<variable::Variable<Vec<Length>>>,
    pub rotate: Option<variable::Variable<Vec<Angle>>>,
    pub text_length: Option<variable::Variable<Vec<Length>>>,
    pub length_adjust: Option<variable::Variable<TextLengthAdjust>>,
}
#[doc = " Text content chars."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Characters(pub Option<String>);
#[doc = " In addition to text drawn in a straight line, SVG also includes the ability to place text along the"]
#[doc = " shape of a ‘path’ element. To specify that a block of text is to be rendered along the shape of a ‘path’,"]
#[doc = " include the given text within a ‘textPath’ element which includes an href’ attribute with an IRI"]
#[doc = " reference to a ‘path’ element."]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPath {
    pub start_offset: Option<variable::Variable<Length>>,
    pub method: Option<variable::Variable<TextPathMethod>>,
    pub spacing: Option<variable::Variable<TextPathSpacing>>,
    pub href: Option<variable::Variable<Iri>>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Bool(bool),
    ListOfBool(Box<Vec<bool>>),
    String(String),
    ListOfString(Box<Vec<String>>),
    Byte(i8),
    ListOfByte(Box<Vec<i8>>),
    Ubyte(u8),
    ListOfUbyte(Box<Vec<u8>>),
    Short(i16),
    ListOfShort(Box<Vec<i16>>),
    Ushort(u16),
    ListOfUshort(Box<Vec<u16>>),
    Int(i32),
    ListOfInt(Box<Vec<i32>>),
    Uint(u32),
    ListOfUint(Box<Vec<u32>>),
    Long(i64),
    ListOfLong(Box<Vec<i64>>),
    Ulong(u64),
    ListOfUlong(Box<Vec<u64>>),
    Float(f32),
    ListOfFloat(Box<Vec<f32>>),
    Double(f64),
    ListOfDouble(Box<Vec<f64>>),
    Angle(Box<Angle>),
    ListOfAngle(Box<Vec<Angle>>),
    Length(Box<Length>),
    ListOfLength(Box<Vec<Length>>),
    Rgb(Box<Rgb>),
    ListOfRgb(Box<Vec<Rgb>>),
    Iri(Box<Iri>),
    ListOfIri(Box<Vec<Iri>>),
    FuncIri(Box<FuncIri>),
    ListOfFuncIri(Box<Vec<FuncIri>>),
    Point(Box<Point>),
    ListOfPoint(Box<Vec<Point>>),
    Percent(Box<Percent>),
    ListOfPercent(Box<Vec<Percent>>),
    Paint(Box<Paint>),
    ListOfPaint(Box<Vec<Paint>>),
    NumberOptNumber(Box<NumberOptNumber>),
    ListOfNumberOptNumber(Box<Vec<NumberOptNumber>>),
    Coords(Box<Coords>),
    ListOfCoords(Box<Vec<Coords>>),
    Transform(Box<Transform>),
    ListOfTransform(Box<Vec<Transform>>),
    Channel(Box<Channel>),
    ListOfChannel(Box<Vec<Channel>>),
    ClipRule(Box<ClipRule>),
    ListOfClipRule(Box<Vec<ClipRule>>),
    PathEvent(Box<PathEvent>),
    ListOfPathEvent(Box<Vec<PathEvent>>),
    FillRule(Box<FillRule>),
    ListOfFillRule(Box<Vec<FillRule>>),
    StrokeLineCap(Box<StrokeLineCap>),
    ListOfStrokeLineCap(Box<Vec<StrokeLineCap>>),
    StrokeLineJoin(Box<StrokeLineJoin>),
    ListOfStrokeLineJoin(Box<Vec<StrokeLineJoin>>),
    SpreadMethod(Box<SpreadMethod>),
    ListOfSpreadMethod(Box<Vec<SpreadMethod>>),
    FontStyle(Box<FontStyle>),
    ListOfFontStyle(Box<Vec<FontStyle>>),
    FontVariant(Box<FontVariant>),
    ListOfFontVariant(Box<Vec<FontVariant>>),
    FontWeight(Box<FontWeight>),
    ListOfFontWeight(Box<Vec<FontWeight>>),
    FontFamily(Box<FontFamily>),
    ListOfFontFamily(Box<Vec<FontFamily>>),
    FontStretch(Box<FontStretch>),
    ListOfFontStretch(Box<Vec<FontStretch>>),
    Background(Box<Background>),
    ListOfBackground(Box<Vec<Background>>),
    FeIn(Box<FeIn>),
    ListOfFeIn(Box<Vec<FeIn>>),
    FeOut(Box<FeOut>),
    ListOfFeOut(Box<Vec<FeOut>>),
    FeBlendMode(Box<FeBlendMode>),
    ListOfFeBlendMode(Box<Vec<FeBlendMode>>),
    TextLengthAdjust(Box<TextLengthAdjust>),
    ListOfTextLengthAdjust(Box<Vec<TextLengthAdjust>>),
    WritingMode(Box<WritingMode>),
    ListOfWritingMode(Box<Vec<WritingMode>>),
    TextDirection(Box<TextDirection>),
    ListOfTextDirection(Box<Vec<TextDirection>>),
    UnicodeBidi(Box<UnicodeBidi>),
    ListOfUnicodeBidi(Box<Vec<UnicodeBidi>>),
    TextAnchor(Box<TextAnchor>),
    ListOfTextAnchor(Box<Vec<TextAnchor>>),
    DominantBaseline(Box<DominantBaseline>),
    ListOfDominantBaseline(Box<Vec<DominantBaseline>>),
    AlignmentBaseline(Box<AlignmentBaseline>),
    ListOfAlignmentBaseline(Box<Vec<AlignmentBaseline>>),
    BaselineShift(Box<BaselineShift>),
    ListOfBaselineShift(Box<Vec<BaselineShift>>),
    TextDecoration(Box<TextDecoration>),
    ListOfTextDecoration(Box<Vec<TextDecoration>>),
    TextPathMethod(Box<TextPathMethod>),
    ListOfTextPathMethod(Box<Vec<TextPathMethod>>),
    TextPathSpacing(Box<TextPathSpacing>),
    ListOfTextPathSpacing(Box<Vec<TextPathSpacing>>),
    LetterSpacing(Box<LetterSpacing>),
    ListOfLetterSpacing(Box<Vec<LetterSpacing>>),
    WordSpacing(Box<WordSpacing>),
    ListOfWordSpacing(Box<Vec<WordSpacing>>),
    MeetOrSlice(Box<MeetOrSlice>),
    ListOfMeetOrSlice(Box<Vec<MeetOrSlice>>),
    PreserveAspectRatio(Box<PreserveAspectRatio>),
    ListOfPreserveAspectRatio(Box<Vec<PreserveAspectRatio>>),
    FeColorMatrixValues(Box<FeColorMatrixValues>),
    ListOfFeColorMatrixValues(Box<Vec<FeColorMatrixValues>>),
    FeFunc(Box<FeFunc>),
    ListOfFeFunc(Box<Vec<FeFunc>>),
    FeCompositeOperator(Box<FeCompositeOperator>),
    ListOfFeCompositeOperator(Box<Vec<FeCompositeOperator>>),
    FeConvolveMatrixEdgeMode(Box<FeConvolveMatrixEdgeMode>),
    ListOfFeConvolveMatrixEdgeMode(Box<Vec<FeConvolveMatrixEdgeMode>>),
    FeMorphologyOperator(Box<FeMorphologyOperator>),
    ListOfFeMorphologyOperator(Box<Vec<FeMorphologyOperator>>),
    FeStitchTiles(Box<FeStitchTiles>),
    ListOfFeStitchTiles(Box<Vec<FeStitchTiles>>),
    FeTurbulenceType(Box<FeTurbulenceType>),
    ListOfFeTurbulenceType(Box<Vec<FeTurbulenceType>>),
}
impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}
impl From<Vec<bool>> for Data {
    fn from(value: Vec<bool>) -> Self {
        Data::ListOfBool(Box::new(value))
    }
}
impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::String(value)
    }
}
impl From<Vec<String>> for Data {
    fn from(value: Vec<String>) -> Self {
        Data::ListOfString(Box::new(value))
    }
}
impl From<i8> for Data {
    fn from(value: i8) -> Self {
        Data::Byte(value)
    }
}
impl From<Vec<i8>> for Data {
    fn from(value: Vec<i8>) -> Self {
        Data::ListOfByte(Box::new(value))
    }
}
impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Data::Ubyte(value)
    }
}
impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::ListOfUbyte(Box::new(value))
    }
}
impl From<i16> for Data {
    fn from(value: i16) -> Self {
        Data::Short(value)
    }
}
impl From<Vec<i16>> for Data {
    fn from(value: Vec<i16>) -> Self {
        Data::ListOfShort(Box::new(value))
    }
}
impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Data::Ushort(value)
    }
}
impl From<Vec<u16>> for Data {
    fn from(value: Vec<u16>) -> Self {
        Data::ListOfUshort(Box::new(value))
    }
}
impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Int(value)
    }
}
impl From<Vec<i32>> for Data {
    fn from(value: Vec<i32>) -> Self {
        Data::ListOfInt(Box::new(value))
    }
}
impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Data::Uint(value)
    }
}
impl From<Vec<u32>> for Data {
    fn from(value: Vec<u32>) -> Self {
        Data::ListOfUint(Box::new(value))
    }
}
impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Long(value)
    }
}
impl From<Vec<i64>> for Data {
    fn from(value: Vec<i64>) -> Self {
        Data::ListOfLong(Box::new(value))
    }
}
impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Data::Ulong(value)
    }
}
impl From<Vec<u64>> for Data {
    fn from(value: Vec<u64>) -> Self {
        Data::ListOfUlong(Box::new(value))
    }
}
impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data::Float(value)
    }
}
impl From<Vec<f32>> for Data {
    fn from(value: Vec<f32>) -> Self {
        Data::ListOfFloat(Box::new(value))
    }
}
impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data::Double(value)
    }
}
impl From<Vec<f64>> for Data {
    fn from(value: Vec<f64>) -> Self {
        Data::ListOfDouble(Box::new(value))
    }
}
impl From<Angle> for Data {
    fn from(value: Angle) -> Self {
        Data::Angle(Box::new(value))
    }
}
impl From<Vec<Angle>> for Data {
    fn from(value: Vec<Angle>) -> Self {
        Data::ListOfAngle(Box::new(value))
    }
}
impl From<Length> for Data {
    fn from(value: Length) -> Self {
        Data::Length(Box::new(value))
    }
}
impl From<Vec<Length>> for Data {
    fn from(value: Vec<Length>) -> Self {
        Data::ListOfLength(Box::new(value))
    }
}
impl From<Rgb> for Data {
    fn from(value: Rgb) -> Self {
        Data::Rgb(Box::new(value))
    }
}
impl From<Vec<Rgb>> for Data {
    fn from(value: Vec<Rgb>) -> Self {
        Data::ListOfRgb(Box::new(value))
    }
}
impl From<Iri> for Data {
    fn from(value: Iri) -> Self {
        Data::Iri(Box::new(value))
    }
}
impl From<Vec<Iri>> for Data {
    fn from(value: Vec<Iri>) -> Self {
        Data::ListOfIri(Box::new(value))
    }
}
impl From<FuncIri> for Data {
    fn from(value: FuncIri) -> Self {
        Data::FuncIri(Box::new(value))
    }
}
impl From<Vec<FuncIri>> for Data {
    fn from(value: Vec<FuncIri>) -> Self {
        Data::ListOfFuncIri(Box::new(value))
    }
}
impl From<Point> for Data {
    fn from(value: Point) -> Self {
        Data::Point(Box::new(value))
    }
}
impl From<Vec<Point>> for Data {
    fn from(value: Vec<Point>) -> Self {
        Data::ListOfPoint(Box::new(value))
    }
}
impl From<Percent> for Data {
    fn from(value: Percent) -> Self {
        Data::Percent(Box::new(value))
    }
}
impl From<Vec<Percent>> for Data {
    fn from(value: Vec<Percent>) -> Self {
        Data::ListOfPercent(Box::new(value))
    }
}
impl From<Paint> for Data {
    fn from(value: Paint) -> Self {
        Data::Paint(Box::new(value))
    }
}
impl From<Vec<Paint>> for Data {
    fn from(value: Vec<Paint>) -> Self {
        Data::ListOfPaint(Box::new(value))
    }
}
impl From<NumberOptNumber> for Data {
    fn from(value: NumberOptNumber) -> Self {
        Data::NumberOptNumber(Box::new(value))
    }
}
impl From<Vec<NumberOptNumber>> for Data {
    fn from(value: Vec<NumberOptNumber>) -> Self {
        Data::ListOfNumberOptNumber(Box::new(value))
    }
}
impl From<Coords> for Data {
    fn from(value: Coords) -> Self {
        Data::Coords(Box::new(value))
    }
}
impl From<Vec<Coords>> for Data {
    fn from(value: Vec<Coords>) -> Self {
        Data::ListOfCoords(Box::new(value))
    }
}
impl From<Transform> for Data {
    fn from(value: Transform) -> Self {
        Data::Transform(Box::new(value))
    }
}
impl From<Vec<Transform>> for Data {
    fn from(value: Vec<Transform>) -> Self {
        Data::ListOfTransform(Box::new(value))
    }
}
impl From<Channel> for Data {
    fn from(value: Channel) -> Self {
        Data::Channel(Box::new(value))
    }
}
impl From<Vec<Channel>> for Data {
    fn from(value: Vec<Channel>) -> Self {
        Data::ListOfChannel(Box::new(value))
    }
}
impl From<ClipRule> for Data {
    fn from(value: ClipRule) -> Self {
        Data::ClipRule(Box::new(value))
    }
}
impl From<Vec<ClipRule>> for Data {
    fn from(value: Vec<ClipRule>) -> Self {
        Data::ListOfClipRule(Box::new(value))
    }
}
impl From<PathEvent> for Data {
    fn from(value: PathEvent) -> Self {
        Data::PathEvent(Box::new(value))
    }
}
impl From<Vec<PathEvent>> for Data {
    fn from(value: Vec<PathEvent>) -> Self {
        Data::ListOfPathEvent(Box::new(value))
    }
}
impl From<FillRule> for Data {
    fn from(value: FillRule) -> Self {
        Data::FillRule(Box::new(value))
    }
}
impl From<Vec<FillRule>> for Data {
    fn from(value: Vec<FillRule>) -> Self {
        Data::ListOfFillRule(Box::new(value))
    }
}
impl From<StrokeLineCap> for Data {
    fn from(value: StrokeLineCap) -> Self {
        Data::StrokeLineCap(Box::new(value))
    }
}
impl From<Vec<StrokeLineCap>> for Data {
    fn from(value: Vec<StrokeLineCap>) -> Self {
        Data::ListOfStrokeLineCap(Box::new(value))
    }
}
impl From<StrokeLineJoin> for Data {
    fn from(value: StrokeLineJoin) -> Self {
        Data::StrokeLineJoin(Box::new(value))
    }
}
impl From<Vec<StrokeLineJoin>> for Data {
    fn from(value: Vec<StrokeLineJoin>) -> Self {
        Data::ListOfStrokeLineJoin(Box::new(value))
    }
}
impl From<SpreadMethod> for Data {
    fn from(value: SpreadMethod) -> Self {
        Data::SpreadMethod(Box::new(value))
    }
}
impl From<Vec<SpreadMethod>> for Data {
    fn from(value: Vec<SpreadMethod>) -> Self {
        Data::ListOfSpreadMethod(Box::new(value))
    }
}
impl From<FontStyle> for Data {
    fn from(value: FontStyle) -> Self {
        Data::FontStyle(Box::new(value))
    }
}
impl From<Vec<FontStyle>> for Data {
    fn from(value: Vec<FontStyle>) -> Self {
        Data::ListOfFontStyle(Box::new(value))
    }
}
impl From<FontVariant> for Data {
    fn from(value: FontVariant) -> Self {
        Data::FontVariant(Box::new(value))
    }
}
impl From<Vec<FontVariant>> for Data {
    fn from(value: Vec<FontVariant>) -> Self {
        Data::ListOfFontVariant(Box::new(value))
    }
}
impl From<FontWeight> for Data {
    fn from(value: FontWeight) -> Self {
        Data::FontWeight(Box::new(value))
    }
}
impl From<Vec<FontWeight>> for Data {
    fn from(value: Vec<FontWeight>) -> Self {
        Data::ListOfFontWeight(Box::new(value))
    }
}
impl From<FontFamily> for Data {
    fn from(value: FontFamily) -> Self {
        Data::FontFamily(Box::new(value))
    }
}
impl From<Vec<FontFamily>> for Data {
    fn from(value: Vec<FontFamily>) -> Self {
        Data::ListOfFontFamily(Box::new(value))
    }
}
impl From<FontStretch> for Data {
    fn from(value: FontStretch) -> Self {
        Data::FontStretch(Box::new(value))
    }
}
impl From<Vec<FontStretch>> for Data {
    fn from(value: Vec<FontStretch>) -> Self {
        Data::ListOfFontStretch(Box::new(value))
    }
}
impl From<Background> for Data {
    fn from(value: Background) -> Self {
        Data::Background(Box::new(value))
    }
}
impl From<Vec<Background>> for Data {
    fn from(value: Vec<Background>) -> Self {
        Data::ListOfBackground(Box::new(value))
    }
}
impl From<FeIn> for Data {
    fn from(value: FeIn) -> Self {
        Data::FeIn(Box::new(value))
    }
}
impl From<Vec<FeIn>> for Data {
    fn from(value: Vec<FeIn>) -> Self {
        Data::ListOfFeIn(Box::new(value))
    }
}
impl From<FeOut> for Data {
    fn from(value: FeOut) -> Self {
        Data::FeOut(Box::new(value))
    }
}
impl From<Vec<FeOut>> for Data {
    fn from(value: Vec<FeOut>) -> Self {
        Data::ListOfFeOut(Box::new(value))
    }
}
impl From<FeBlendMode> for Data {
    fn from(value: FeBlendMode) -> Self {
        Data::FeBlendMode(Box::new(value))
    }
}
impl From<Vec<FeBlendMode>> for Data {
    fn from(value: Vec<FeBlendMode>) -> Self {
        Data::ListOfFeBlendMode(Box::new(value))
    }
}
impl From<TextLengthAdjust> for Data {
    fn from(value: TextLengthAdjust) -> Self {
        Data::TextLengthAdjust(Box::new(value))
    }
}
impl From<Vec<TextLengthAdjust>> for Data {
    fn from(value: Vec<TextLengthAdjust>) -> Self {
        Data::ListOfTextLengthAdjust(Box::new(value))
    }
}
impl From<WritingMode> for Data {
    fn from(value: WritingMode) -> Self {
        Data::WritingMode(Box::new(value))
    }
}
impl From<Vec<WritingMode>> for Data {
    fn from(value: Vec<WritingMode>) -> Self {
        Data::ListOfWritingMode(Box::new(value))
    }
}
impl From<TextDirection> for Data {
    fn from(value: TextDirection) -> Self {
        Data::TextDirection(Box::new(value))
    }
}
impl From<Vec<TextDirection>> for Data {
    fn from(value: Vec<TextDirection>) -> Self {
        Data::ListOfTextDirection(Box::new(value))
    }
}
impl From<UnicodeBidi> for Data {
    fn from(value: UnicodeBidi) -> Self {
        Data::UnicodeBidi(Box::new(value))
    }
}
impl From<Vec<UnicodeBidi>> for Data {
    fn from(value: Vec<UnicodeBidi>) -> Self {
        Data::ListOfUnicodeBidi(Box::new(value))
    }
}
impl From<TextAnchor> for Data {
    fn from(value: TextAnchor) -> Self {
        Data::TextAnchor(Box::new(value))
    }
}
impl From<Vec<TextAnchor>> for Data {
    fn from(value: Vec<TextAnchor>) -> Self {
        Data::ListOfTextAnchor(Box::new(value))
    }
}
impl From<DominantBaseline> for Data {
    fn from(value: DominantBaseline) -> Self {
        Data::DominantBaseline(Box::new(value))
    }
}
impl From<Vec<DominantBaseline>> for Data {
    fn from(value: Vec<DominantBaseline>) -> Self {
        Data::ListOfDominantBaseline(Box::new(value))
    }
}
impl From<AlignmentBaseline> for Data {
    fn from(value: AlignmentBaseline) -> Self {
        Data::AlignmentBaseline(Box::new(value))
    }
}
impl From<Vec<AlignmentBaseline>> for Data {
    fn from(value: Vec<AlignmentBaseline>) -> Self {
        Data::ListOfAlignmentBaseline(Box::new(value))
    }
}
impl From<BaselineShift> for Data {
    fn from(value: BaselineShift) -> Self {
        Data::BaselineShift(Box::new(value))
    }
}
impl From<Vec<BaselineShift>> for Data {
    fn from(value: Vec<BaselineShift>) -> Self {
        Data::ListOfBaselineShift(Box::new(value))
    }
}
impl From<TextDecoration> for Data {
    fn from(value: TextDecoration) -> Self {
        Data::TextDecoration(Box::new(value))
    }
}
impl From<Vec<TextDecoration>> for Data {
    fn from(value: Vec<TextDecoration>) -> Self {
        Data::ListOfTextDecoration(Box::new(value))
    }
}
impl From<TextPathMethod> for Data {
    fn from(value: TextPathMethod) -> Self {
        Data::TextPathMethod(Box::new(value))
    }
}
impl From<Vec<TextPathMethod>> for Data {
    fn from(value: Vec<TextPathMethod>) -> Self {
        Data::ListOfTextPathMethod(Box::new(value))
    }
}
impl From<TextPathSpacing> for Data {
    fn from(value: TextPathSpacing) -> Self {
        Data::TextPathSpacing(Box::new(value))
    }
}
impl From<Vec<TextPathSpacing>> for Data {
    fn from(value: Vec<TextPathSpacing>) -> Self {
        Data::ListOfTextPathSpacing(Box::new(value))
    }
}
impl From<LetterSpacing> for Data {
    fn from(value: LetterSpacing) -> Self {
        Data::LetterSpacing(Box::new(value))
    }
}
impl From<Vec<LetterSpacing>> for Data {
    fn from(value: Vec<LetterSpacing>) -> Self {
        Data::ListOfLetterSpacing(Box::new(value))
    }
}
impl From<WordSpacing> for Data {
    fn from(value: WordSpacing) -> Self {
        Data::WordSpacing(Box::new(value))
    }
}
impl From<Vec<WordSpacing>> for Data {
    fn from(value: Vec<WordSpacing>) -> Self {
        Data::ListOfWordSpacing(Box::new(value))
    }
}
impl From<MeetOrSlice> for Data {
    fn from(value: MeetOrSlice) -> Self {
        Data::MeetOrSlice(Box::new(value))
    }
}
impl From<Vec<MeetOrSlice>> for Data {
    fn from(value: Vec<MeetOrSlice>) -> Self {
        Data::ListOfMeetOrSlice(Box::new(value))
    }
}
impl From<PreserveAspectRatio> for Data {
    fn from(value: PreserveAspectRatio) -> Self {
        Data::PreserveAspectRatio(Box::new(value))
    }
}
impl From<Vec<PreserveAspectRatio>> for Data {
    fn from(value: Vec<PreserveAspectRatio>) -> Self {
        Data::ListOfPreserveAspectRatio(Box::new(value))
    }
}
impl From<FeColorMatrixValues> for Data {
    fn from(value: FeColorMatrixValues) -> Self {
        Data::FeColorMatrixValues(Box::new(value))
    }
}
impl From<Vec<FeColorMatrixValues>> for Data {
    fn from(value: Vec<FeColorMatrixValues>) -> Self {
        Data::ListOfFeColorMatrixValues(Box::new(value))
    }
}
impl From<FeFunc> for Data {
    fn from(value: FeFunc) -> Self {
        Data::FeFunc(Box::new(value))
    }
}
impl From<Vec<FeFunc>> for Data {
    fn from(value: Vec<FeFunc>) -> Self {
        Data::ListOfFeFunc(Box::new(value))
    }
}
impl From<FeCompositeOperator> for Data {
    fn from(value: FeCompositeOperator) -> Self {
        Data::FeCompositeOperator(Box::new(value))
    }
}
impl From<Vec<FeCompositeOperator>> for Data {
    fn from(value: Vec<FeCompositeOperator>) -> Self {
        Data::ListOfFeCompositeOperator(Box::new(value))
    }
}
impl From<FeConvolveMatrixEdgeMode> for Data {
    fn from(value: FeConvolveMatrixEdgeMode) -> Self {
        Data::FeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl From<Vec<FeConvolveMatrixEdgeMode>> for Data {
    fn from(value: Vec<FeConvolveMatrixEdgeMode>) -> Self {
        Data::ListOfFeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl From<FeMorphologyOperator> for Data {
    fn from(value: FeMorphologyOperator) -> Self {
        Data::FeMorphologyOperator(Box::new(value))
    }
}
impl From<Vec<FeMorphologyOperator>> for Data {
    fn from(value: Vec<FeMorphologyOperator>) -> Self {
        Data::ListOfFeMorphologyOperator(Box::new(value))
    }
}
impl From<FeStitchTiles> for Data {
    fn from(value: FeStitchTiles) -> Self {
        Data::FeStitchTiles(Box::new(value))
    }
}
impl From<Vec<FeStitchTiles>> for Data {
    fn from(value: Vec<FeStitchTiles>) -> Self {
        Data::ListOfFeStitchTiles(Box::new(value))
    }
}
impl From<FeTurbulenceType> for Data {
    fn from(value: FeTurbulenceType) -> Self {
        Data::FeTurbulenceType(Box::new(value))
    }
}
impl From<Vec<FeTurbulenceType>> for Data {
    fn from(value: Vec<FeTurbulenceType>) -> Self {
        Data::ListOfFeTurbulenceType(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attr {
    TextLayout(Box<TextLayout>),
    WithTransform(Box<WithTransform>),
    Id(Box<Id>),
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
    Font(Box<Font>),
    EnableBackground(Box<EnableBackground>),
    WithFilter(Box<WithFilter>),
    WithClipPath(Box<WithClipPath>),
    WithMask(Box<WithMask>),
    Opacity(Box<Opacity>),
    ViewBox(Box<ViewBox>),
}
impl From<TextLayout> for Attr {
    fn from(value: TextLayout) -> Self {
        Self::TextLayout(Box::new(value))
    }
}
impl From<WithTransform> for Attr {
    fn from(value: WithTransform) -> Self {
        Self::WithTransform(Box::new(value))
    }
}
impl From<Id> for Attr {
    fn from(value: Id) -> Self {
        Self::Id(Box::new(value))
    }
}
impl From<Fill> for Attr {
    fn from(value: Fill) -> Self {
        Self::Fill(Box::new(value))
    }
}
impl From<Stroke> for Attr {
    fn from(value: Stroke) -> Self {
        Self::Stroke(Box::new(value))
    }
}
impl From<Font> for Attr {
    fn from(value: Font) -> Self {
        Self::Font(Box::new(value))
    }
}
impl From<EnableBackground> for Attr {
    fn from(value: EnableBackground) -> Self {
        Self::EnableBackground(Box::new(value))
    }
}
impl From<WithFilter> for Attr {
    fn from(value: WithFilter) -> Self {
        Self::WithFilter(Box::new(value))
    }
}
impl From<WithClipPath> for Attr {
    fn from(value: WithClipPath) -> Self {
        Self::WithClipPath(Box::new(value))
    }
}
impl From<WithMask> for Attr {
    fn from(value: WithMask) -> Self {
        Self::WithMask(Box::new(value))
    }
}
impl From<Opacity> for Attr {
    fn from(value: Opacity) -> Self {
        Self::Opacity(Box::new(value))
    }
}
impl From<ViewBox> for Attr {
    fn from(value: ViewBox) -> Self {
        Self::ViewBox(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Element {
    Canvas(Box<Canvas>),
    Mask(Box<Mask>),
    ClipPath(Box<ClipPath>),
    Filter(Box<Filter>),
    FeComponentTransfer(Box<FeComponentTransfer>),
    FeDiffuseLighting(Box<FeDiffuseLighting>),
    FeMerge(Box<FeMerge>),
    FeSpecularLighting(Box<FeSpecularLighting>),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    Group(Box<Group>),
    Pattern(Box<Pattern>),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
    TextPath(Box<TextPath>),
}
impl From<Canvas> for Element {
    fn from(value: Canvas) -> Self {
        Self::Canvas(Box::new(value))
    }
}
impl From<Mask> for Element {
    fn from(value: Mask) -> Self {
        Self::Mask(Box::new(value))
    }
}
impl From<ClipPath> for Element {
    fn from(value: ClipPath) -> Self {
        Self::ClipPath(Box::new(value))
    }
}
impl From<Filter> for Element {
    fn from(value: Filter) -> Self {
        Self::Filter(Box::new(value))
    }
}
impl From<FeComponentTransfer> for Element {
    fn from(value: FeComponentTransfer) -> Self {
        Self::FeComponentTransfer(Box::new(value))
    }
}
impl From<FeDiffuseLighting> for Element {
    fn from(value: FeDiffuseLighting) -> Self {
        Self::FeDiffuseLighting(Box::new(value))
    }
}
impl From<FeMerge> for Element {
    fn from(value: FeMerge) -> Self {
        Self::FeMerge(Box::new(value))
    }
}
impl From<FeSpecularLighting> for Element {
    fn from(value: FeSpecularLighting) -> Self {
        Self::FeSpecularLighting(Box::new(value))
    }
}
impl From<LinearGradient> for Element {
    fn from(value: LinearGradient) -> Self {
        Self::LinearGradient(Box::new(value))
    }
}
impl From<RadialGradient> for Element {
    fn from(value: RadialGradient) -> Self {
        Self::RadialGradient(Box::new(value))
    }
}
impl From<Group> for Element {
    fn from(value: Group) -> Self {
        Self::Group(Box::new(value))
    }
}
impl From<Pattern> for Element {
    fn from(value: Pattern) -> Self {
        Self::Pattern(Box::new(value))
    }
}
impl From<Text> for Element {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}
impl From<TextSpan> for Element {
    fn from(value: TextSpan) -> Self {
        Self::TextSpan(Box::new(value))
    }
}
impl From<TextPath> for Element {
    fn from(value: TextPath) -> Self {
        Self::TextPath(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Leaf {
    FeDistantLight(Box<FeDistantLight>),
    FePointLight(Box<FePointLight>),
    FeSpotLight(Box<FeSpotLight>),
    FeBlend(Box<FeBlend>),
    FeColorMatrix(Box<FeColorMatrix>),
    FeFuncA(Box<FeFuncA>),
    FeFuncR(Box<FeFuncR>),
    FeFuncG(Box<FeFuncG>),
    FeFuncB(Box<FeFuncB>),
    FeComposite(Box<FeComposite>),
    FeConvolveMatrix(Box<FeConvolveMatrix>),
    FeDisplacementMap(Box<FeDisplacementMap>),
    FeFlood(Box<FeFlood>),
    FeGaussianBlur(Box<FeGaussianBlur>),
    FeMergeNode(Box<FeMergeNode>),
    FeImage(Box<FeImage>),
    FeMorphology(Box<FeMorphology>),
    FeOffset(Box<FeOffset>),
    FeTile(Box<FeTile>),
    FeTurbulence(Box<FeTurbulence>),
    GradientStop(Box<GradientStop>),
    Path(Box<Path>),
    Use(Box<Use>),
    Rect(Box<Rect>),
    Circle(Box<Circle>),
    Ellipse(Box<Ellipse>),
    Line(Box<Line>),
    Polyline(Box<Polyline>),
    Polygon(Box<Polygon>),
    Characters(Box<Characters>),
}
impl From<FeDistantLight> for Leaf {
    fn from(value: FeDistantLight) -> Self {
        Self::FeDistantLight(Box::new(value))
    }
}
impl From<FePointLight> for Leaf {
    fn from(value: FePointLight) -> Self {
        Self::FePointLight(Box::new(value))
    }
}
impl From<FeSpotLight> for Leaf {
    fn from(value: FeSpotLight) -> Self {
        Self::FeSpotLight(Box::new(value))
    }
}
impl From<FeBlend> for Leaf {
    fn from(value: FeBlend) -> Self {
        Self::FeBlend(Box::new(value))
    }
}
impl From<FeColorMatrix> for Leaf {
    fn from(value: FeColorMatrix) -> Self {
        Self::FeColorMatrix(Box::new(value))
    }
}
impl From<FeFuncA> for Leaf {
    fn from(value: FeFuncA) -> Self {
        Self::FeFuncA(Box::new(value))
    }
}
impl From<FeFuncR> for Leaf {
    fn from(value: FeFuncR) -> Self {
        Self::FeFuncR(Box::new(value))
    }
}
impl From<FeFuncG> for Leaf {
    fn from(value: FeFuncG) -> Self {
        Self::FeFuncG(Box::new(value))
    }
}
impl From<FeFuncB> for Leaf {
    fn from(value: FeFuncB) -> Self {
        Self::FeFuncB(Box::new(value))
    }
}
impl From<FeComposite> for Leaf {
    fn from(value: FeComposite) -> Self {
        Self::FeComposite(Box::new(value))
    }
}
impl From<FeConvolveMatrix> for Leaf {
    fn from(value: FeConvolveMatrix) -> Self {
        Self::FeConvolveMatrix(Box::new(value))
    }
}
impl From<FeDisplacementMap> for Leaf {
    fn from(value: FeDisplacementMap) -> Self {
        Self::FeDisplacementMap(Box::new(value))
    }
}
impl From<FeFlood> for Leaf {
    fn from(value: FeFlood) -> Self {
        Self::FeFlood(Box::new(value))
    }
}
impl From<FeGaussianBlur> for Leaf {
    fn from(value: FeGaussianBlur) -> Self {
        Self::FeGaussianBlur(Box::new(value))
    }
}
impl From<FeMergeNode> for Leaf {
    fn from(value: FeMergeNode) -> Self {
        Self::FeMergeNode(Box::new(value))
    }
}
impl From<FeImage> for Leaf {
    fn from(value: FeImage) -> Self {
        Self::FeImage(Box::new(value))
    }
}
impl From<FeMorphology> for Leaf {
    fn from(value: FeMorphology) -> Self {
        Self::FeMorphology(Box::new(value))
    }
}
impl From<FeOffset> for Leaf {
    fn from(value: FeOffset) -> Self {
        Self::FeOffset(Box::new(value))
    }
}
impl From<FeTile> for Leaf {
    fn from(value: FeTile) -> Self {
        Self::FeTile(Box::new(value))
    }
}
impl From<FeTurbulence> for Leaf {
    fn from(value: FeTurbulence) -> Self {
        Self::FeTurbulence(Box::new(value))
    }
}
impl From<GradientStop> for Leaf {
    fn from(value: GradientStop) -> Self {
        Self::GradientStop(Box::new(value))
    }
}
impl From<Path> for Leaf {
    fn from(value: Path) -> Self {
        Self::Path(Box::new(value))
    }
}
impl From<Use> for Leaf {
    fn from(value: Use) -> Self {
        Self::Use(Box::new(value))
    }
}
impl From<Rect> for Leaf {
    fn from(value: Rect) -> Self {
        Self::Rect(Box::new(value))
    }
}
impl From<Circle> for Leaf {
    fn from(value: Circle) -> Self {
        Self::Circle(Box::new(value))
    }
}
impl From<Ellipse> for Leaf {
    fn from(value: Ellipse) -> Self {
        Self::Ellipse(Box::new(value))
    }
}
impl From<Line> for Leaf {
    fn from(value: Line) -> Self {
        Self::Line(Box::new(value))
    }
}
impl From<Polyline> for Leaf {
    fn from(value: Polyline) -> Self {
        Self::Polyline(Box::new(value))
    }
}
impl From<Polygon> for Leaf {
    fn from(value: Polygon) -> Self {
        Self::Polygon(Box::new(value))
    }
}
impl From<Characters> for Leaf {
    fn from(value: Characters) -> Self {
        Self::Characters(Box::new(value))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Apply(Attr),
    Element(Element),
    Pop,
    Leaf(Leaf),
}
impl From<Attr> for Opcode {
    fn from(value: Attr) -> Self {
        Self::Apply(value)
    }
}
impl From<Element> for Opcode {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}
impl From<Leaf> for Opcode {
    fn from(value: Leaf) -> Self {
        Self::Leaf(value)
    }
}
mod variable {
    #[doc = r" The path used by [`Variable`] is used to point to [`Target`]."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Path {
        #[doc = r" Reference by name."]
        Named(String),
        #[doc = r" Reference by optimized position."]
        Index(usize),
    }
    impl From<String> for Path {
        fn from(value: String) -> Self {
            Self::Named(value)
        }
    }
    impl From<&str> for Path {
        fn from(value: &str) -> Self {
            Self::Named(value.to_owned())
        }
    }
    impl From<usize> for Path {
        fn from(value: usize) -> Self {
            Self::Index(value)
        }
    }
    #[doc = r" The type of variable pointed to by [`Path`]."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Target {
        #[doc = r" Target is animation register."]
        Register,
        #[doc = r" Target is `item` variable returns by `foreach` iterator."]
        ForeachItem,
        #[doc = r" Target is `index` variable returns by `foreach` iterator."]
        ForeachIndex,
        #[doc = r" Target is `index` variable returns by `for range` iterator."]
        Range,
    }
    #[doc = r" Variable used by property fields."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Variable<T>
    where
        super::Data: From<T>,
    {
        #[doc = r" A literal constant value."]
        Constant(T),
        Reference {
            path: Path,
            target: Target,
        },
    }
    impl<T> From<T> for Variable<T>
    where
        super::Data: From<T>,
    {
        fn from(value: T) -> Self {
            Self::Constant(value)
        }
    }
    impl<T> Default for Variable<T>
    where
        T: Default,
        super::Data: From<T>,
    {
        fn default() -> Self {
            Self::Constant(T::default())
        }
    }
    impl<P, T> From<(P, Target)> for Variable<T>
    where
        super::Data: From<T>,
        Path: From<P>,
    {
        fn from(value: (P, Target)) -> Self {
            Self::Reference {
                path: value.0.into(),
                target: value.1,
            }
        }
    }
}
