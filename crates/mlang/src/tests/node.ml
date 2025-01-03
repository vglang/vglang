/// In addition to text drawn in a straight line, SVG also includes the ability to place text along the
/// shape of a ‘path’ element. To specify that a block of text is to be rendered along the shape of a ‘path’,
/// include the given text within a ‘textPath’ element which includes an href’ attribute with an IRI
/// reference to a ‘path’ element.
leaf TextPath {
    /// An offset from the start of the ‘path’ for the initial current text position,
    /// calculated using the user agent's [`distance along the path`] algorithm.
    ///
    /// If a `length` other than a percentage is given, then the ‘startOffset’ represents a
    /// distance along the path measured in the current user coordinate system.
    ///
    /// If a percentage is given, then the ‘startOffset’ represents a percentage distance along
    /// the entire path. Thus, startOffset="0%" indicates the start point of the ‘path’ and
    /// startOffset="100%" indicates the end point of the ‘path’.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// [`distance along the path`]: https://www.w3.org/TR/SVG11/paths.html#DistanceAlongAPath
    start_offset: Length,

    /// See [`TextPathMethod`]
    method: TextPathMethod,

    /// See [`TextPathSpacing`]
    spacing: TextPathSpacing,

    /// An IRI reference to the ‘path’ element onto which the glyphs will be rendered.
    /// If `iri` is an invalid reference (e.g., no such element exists, or the referenced element is not a ‘path’),
    /// then the ‘textPath’ element is in error and its entire contents shall not be rendered by the user agent.
    href: Iri,
}

/// In addition to text drawn in a straight line, SVG also includes the ability to place text along the
/// shape of a ‘path’ element. To specify that a block of text is to be rendered along the shape of a ‘path’,
/// include the given text within a ‘textPath’ element which includes an href’ attribute with an IRI
/// reference to a ‘path’ element.
#[hello,world("hello world")]

leaf TextPath mixin Hello {
    /// An offset from the start of the ‘path’ for the initial current text position,
    /// calculated using the user agent's [`distance along the path`] algorithm.
    ///
    /// If a `length` other than a percentage is given, then the ‘startOffset’ represents a
    /// distance along the path measured in the current user coordinate system.
    ///
    /// If a percentage is given, then the ‘startOffset’ represents a percentage distance along
    /// the entire path. Thus, startOffset="0%" indicates the start point of the ‘path’ and
    /// startOffset="100%" indicates the end point of the ‘path’.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// [`distance along the path`]: https://www.w3.org/TR/SVG11/paths.html#DistanceAlongAPath
    start_offset: Length,

    /// See [`TextPathMethod`]
    method: TextPathMethod,

    /// See [`TextPathSpacing`]
    spacing: TextPathSpacing,

    /// An IRI reference to the ‘path’ element onto which the glyphs will be rendered.
    /// If `iri` is an invalid reference (e.g., no such element exists, or the referenced element is not a ‘path’),
    /// then the ‘textPath’ element is in error and its entire contents shall not be rendered by the user agent.
    href: Iri
}

/// Text content chars.
leaf TextPath(  String,   World,   



);