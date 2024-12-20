use std::fmt::Display;

use super::{Angle, Fill, Font, Length, Stroke, Variable};

/// See [`length_adjust`](Text::length_adjust)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextLengthAdjust {
    /// 'spacing' indicates that only the advance values are adjusted. The glyphs themselves are not stretched or compressed.
    Spacing,

    /// 'spacingAndGlyphs' indicates that the advance values are adjusted and the glyphs themselves stretched or compressed
    /// in one axis (i.e., a direction parallel to the inline-progression-direction).
    SpacingAndGlyphs,
}

impl Default for TextLengthAdjust {
    fn default() -> Self {
        Self::Spacing
    }
}

/// The ‘text’ element defines a graphics element consisting of text.
///
/// See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    /// If a single `coordinate` is provided, then the value represents the new absolute X coordinate for
    /// the current text position for rendering the glyphs that correspond to the first character within
    /// this element or any of its descendants.
    ///
    /// If a comma- or space-separated list of n `coordinate`s is provided, then the values represent new
    /// absolute X coordinates for the current text position for rendering the glyphs corresponding to
    /// each of the first n characters within this element or any of its descendants.
    ///
    /// For additional processing rules, refer to the description of the ‘x’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub x: Variable<Vec<Length>>,

    /// The corresponding list of absolute Y coordinates for the glyphs corresponding to the characters within this element.
    /// The processing rules for the ‘y’ attribute parallel the processing rules for the ‘x’ attribute.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub y: Variable<Vec<Length>>,

    /// Shifts in the current text position along the x-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dx’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the x-axis will occur.
    pub dx: Variable<Vec<Length>>,

    /// Shifts in the current text position along the y-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dy’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the y-axis will occur.
    pub dy: Variable<Vec<Length>>,

    /// The supplemental rotation about the current text position that will be applied to all of the glyphs corresponding
    /// to each character within this element.
    ///
    /// Refer to the description of the ‘rotate’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental rotations will occur.
    pub rotate: Variable<Vec<Angle>>,

    /// The author's computation of the total sum of all of the advance values that correspond to character data within
    /// this element, including the advance value on the glyph (horizontal or vertical), the effect of properties ‘kerning’,
    /// ‘letter-spacing’ and ‘word-spacing’ and adjustments due to attributes ‘dx’ and ‘dy’ on ‘tspan’ elements. This value
    /// is used to calibrate the user agent's own calculations with that of the author.
    ///
    /// The purpose of this attribute is to allow the author to achieve exact alignment, in visual rendering order after any
    /// bidirectional reordering, for the first and last rendered glyphs that correspond to this element; thus, for the last
    /// rendered character (in visual rendering order after any bidirectional reordering), any supplemental inter-character
    /// spacing beyond normal glyph advances are ignored (in most cases) when the user agent determines the appropriate amount
    /// to expand/compress the text string to fit within a length of ‘textLength’.
    ///
    /// A negative value is an error (see Error processing).
    ///
    /// If the attribute is not specified, the effect is as if the author's computation exactly matched the value calculated
    /// by the user agent; thus, no advance adjustments are made.
    pub text_length: Variable<Length>,

    /// Indicates the type of adjustments which the user agent shall make to make the rendered length of the text match the
    /// value specified on the ‘textLength’ attribute.
    ///
    /// The user agent is required to achieve correct start and end positions for the text strings, but the locations of
    /// intermediate glyphs are not predictable because user agents might employ advanced algorithms to stretch or compress
    /// text strings in order to balance correct start and end positioning with optimal typography.
    ///
    /// Note that, for a text string that contains n characters, the adjustments to the advance values often occur only for n−1
    /// characters (see description of attribute ‘textLength’), whereas stretching or compressing of the glyphs will be applied
    /// to all n characters.
    ///
    /// If the attribute is not specified, the effect is as a value of 'spacing' were specified.
    pub length_adjust: Variable<TextLengthAdjust>,
}

/// The ‘text’ element defines a graphics element consisting of text.
///
/// See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpan {
    /// If a single `coordinate` is provided, then the value represents the new absolute X coordinate for
    /// the current text position for rendering the glyphs that correspond to the first character within
    /// this element or any of its descendants.
    ///
    /// If a comma- or space-separated list of n `coordinate`s is provided, then the values represent new
    /// absolute X coordinates for the current text position for rendering the glyphs corresponding to
    /// each of the first n characters within this element or any of its descendants.
    ///
    /// For additional processing rules, refer to the description of the ‘x’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub x: Variable<Vec<Length>>,

    /// The corresponding list of absolute Y coordinates for the glyphs corresponding to the characters within this element.
    /// The processing rules for the ‘y’ attribute parallel the processing rules for the ‘x’ attribute.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub y: Variable<Vec<Length>>,

    /// Shifts in the current text position along the x-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dx’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the x-axis will occur.
    pub dx: Variable<Vec<Length>>,

    /// Shifts in the current text position along the y-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dy’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the y-axis will occur.
    pub dy: Variable<Vec<Length>>,

    /// The supplemental rotation about the current text position that will be applied to all of the glyphs corresponding
    /// to each character within this element.
    ///
    /// Refer to the description of the ‘rotate’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental rotations will occur.
    pub rotate: Variable<Vec<Angle>>,

    /// The author's computation of the total sum of all of the advance values that correspond to character data within
    /// this element, including the advance value on the glyph (horizontal or vertical), the effect of properties ‘kerning’,
    /// ‘letter-spacing’ and ‘word-spacing’ and adjustments due to attributes ‘dx’ and ‘dy’ on ‘tspan’ elements. This value
    /// is used to calibrate the user agent's own calculations with that of the author.
    ///
    /// The purpose of this attribute is to allow the author to achieve exact alignment, in visual rendering order after any
    /// bidirectional reordering, for the first and last rendered glyphs that correspond to this element; thus, for the last
    /// rendered character (in visual rendering order after any bidirectional reordering), any supplemental inter-character
    /// spacing beyond normal glyph advances are ignored (in most cases) when the user agent determines the appropriate amount
    /// to expand/compress the text string to fit within a length of ‘textLength’.
    ///
    /// A negative value is an error (see Error processing).
    ///
    /// If the attribute is not specified, the effect is as if the author's computation exactly matched the value calculated
    /// by the user agent; thus, no advance adjustments are made.
    pub text_length: Variable<Length>,

    /// Indicates the type of adjustments which the user agent shall make to make the rendered length of the text match the
    /// value specified on the ‘textLength’ attribute.
    ///
    /// The user agent is required to achieve correct start and end positions for the text strings, but the locations of
    /// intermediate glyphs are not predictable because user agents might employ advanced algorithms to stretch or compress
    /// text strings in order to balance correct start and end positioning with optimal typography.
    ///
    /// Note that, for a text string that contains n characters, the adjustments to the advance values often occur only for n−1
    /// characters (see description of attribute ‘textLength’), whereas stretching or compressing of the glyphs will be applied
    /// to all n characters.
    ///
    /// If the attribute is not specified, the effect is as a value of 'spacing' were specified.
    pub length_adjust: Variable<TextLengthAdjust>,

    /// font properties.
    ///
    /// See [`Font`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub font: Option<Font>,

    /// text layout properties.
    ///
    /// See [`TextLayout`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layout: Option<TextLayout>,

    /// fill properties.
    ///
    /// See [`Fill`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fill: Option<Fill>,

    /// Stroke properties.
    ///
    /// See [`Stroke`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stroke: Option<Stroke>,
}

/// The ‘writing-mode’ property specifies whether the initial inline-progression-direction for a ‘text’ element shall be
/// left-to-right, right-to-left, or top-to-bottom. The ‘writing-mode’ property applies only to ‘text’ elements;
/// the property is ignored for ‘tspan’, ‘tref’, ‘altGlyph’ and ‘textPath’ sub-elements. (Note that the inline-progression-direction
/// can change within a ‘text’ element due to the Unicode bidirectional algorithm and properties ‘direction’ and ‘unicode-bidi’.
/// For more on bidirectional text, see Relationship with bidirectionality.)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WritingMode {
    /// Sets the initial inline-progression-direction to left-to-right, as is common in most Latin-based documents.
    /// For most characters, the current text position is advanced from left to right after each glyph is rendered.
    /// (When the character data includes characters which are subject to the Unicode bidirectional algorithm, the text
    /// advance rules are more complex. See Relationship with bidirectionality).
    LrTb,
    /// Sets the initial inline-progression-direction to right-to-left, as is common in Arabic or Hebrew scripts.
    /// (See Relationship with bidirectionality.)
    RlTb,
    /// Sets the initial inline-progression-direction to top-to-bottom, as is common in some Asian scripts,
    /// such as Chinese and Japanese. Though hardly as frequent as horizontal, this type of vertical layout also occurs
    /// in Latin based documents, particularly in table column or row labels. In most cases, the vertical baselines
    /// running through the middle of each glyph are aligned.
    TbRl,
    /// See [`LrTb`](WritingMode::LrTb)
    Lr,
    /// See [`RlTb`](WritingMode::RlTb)
    Rl,
    /// See [`TbRl`](WritingMode::TbRl)
    Tb,
}

impl Default for WritingMode {
    fn default() -> Self {
        Self::LrTb
    }
}

/// Within text content elements, the alignment of text with regards to the ‘text-anchor’ property is determined by
/// the value of the ‘direction’ property. For example, given a ‘text’ element with a ‘text-anchor’ value of "end",
/// for a ‘direction’ value of "ltr", the text will extend to the left of the position of the ‘text’ element's ‘x’
/// attribute value, while for ‘direction’ value of "rtl", the text will extend to the right of the position of the
/// ‘text’ element's ‘x’ attribute value.
///
/// A more complete discussion of bidirectionality can be found in the Text direction section of
/// [`CSS 2`](https://www.w3.org/TR/CSS2/visuren.html#direction).
///
/// See [`direction`](https://www.w3.org/TR/SVG11/text.html#DirectionProperty)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    Ltr,
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// Except for any additional information provided in this specification, the normative definition of the
/// [`unicode-bidi`] property is in CSS2
///
/// [`unicode-bidi`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/visuren.html#propdef-unicode-bidi
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeBidi {
    Normal,
    Embed,
    BidiOverride,
}

impl Default for UnicodeBidi {
    fn default() -> Self {
        Self::Normal
    }
}

/// The ‘text-anchor’ property is used to align (start-, middle- or end-alignment) a string of text relative to a given point.
///
/// The ‘text-anchor’ property is applied to each individual text chunk within a given ‘text’ element. Each text chunk has an
/// initial current text position, which represents the point in the user coordinate system resulting from (depending on context)
/// application of the ‘x’ and ‘y’ attributes on the ‘text’ element, any ‘x’ or ‘y’ attribute values on a ‘tspan’, ‘tref’ or
/// ‘altGlyph’ element assigned explicitly to the first rendered character in a text chunk, or determination of the initial current
/// text position for a ‘textPath’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAnchor {
    /// The rendered characters are aligned such that the start of the resulting rendered text is at the initial current text position.
    /// For an element with a ‘direction’ property value of "ltr" (typical for most European languages), the left side of the text is
    /// rendered at the initial text position. For an element with a ‘direction’ property value of "rtl" (typical for Arabic and Hebrew),
    /// the right side of the text is rendered at the initial text position. For an element with a vertical primary text direction
    /// (often typical for Asian text), the top side of the text is rendered at the initial text position.
    Start,
    /// The rendered characters are aligned such that the geometric middle of the resulting rendered text is at the initial
    /// current text position.
    Middle,
    /// The rendered characters are aligned such that the end of the resulting rendered text is at the initial current text position.
    /// For an element with a ‘direction’ property value of "ltr" (typical for most European languages), the right side of the text is
    /// rendered at the initial text position. For an element with a ‘direction’ property value of "rtl" (typical for Arabic and Hebrew),
    /// the left side of the text is rendered at the initial text position. For an element with a vertical primary text direction (often
    /// typical for Asian text), the bottom of the text is rendered at the initial text position.
    End,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Start
    }
}

/// See [`baseline`](https://www.w3.org/TR/SVG11/text.html#BaselineAlignmentProperties)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DominantBaseline {
    /// If this property occurs on a ‘text’ element, then the computed value depends on the value of the ‘writing-mode’ property.
    /// If the 'writing-mode' is horizontal, then the value of the dominant-baseline component is 'alphabetic', else if the
    /// 'writing-mode' is vertical, then the value of the dominant-baseline component is 'central'.
    ///
    /// If this property occurs on a ‘tspan’, ‘tref’, ‘altGlyph’ or ‘textPath’ element, then the dominant-baseline and the
    /// baseline-table components remain the same as those of the parent text content element. If the computed ‘baseline-shift’
    /// value actually shifts the baseline, then the baseline-table font-size component is set to the value of the ‘font-size’
    /// property on the element on which the ‘dominant-baseline’ property occurs, otherwise the baseline-table font-size remains
    /// the same as that of the element. If there is no parent text content element, the scaled-baseline-table value is
    /// constructed as above for ‘text’ elements.
    Auto,
    /// The dominant-baseline and the baseline-table components are set by determining the predominant script of the character
    /// data content. The ‘writing-mode’, whether horizontal or vertical, is used to select the appropriate set of baseline-tables
    /// and the dominant baseline is used to select the baseline-table that corresponds to that baseline. The baseline-table
    /// font-size component is set to the value of the ‘font-size’ property on the element on which the ‘dominant-baseline’ property
    /// occurs.
    UseScript,
    /// The dominant-baseline, the baseline-table, and the baseline-table font-size remain the same as that of the parent text
    /// content element.
    NoChange,
    /// The dominant-baseline and the baseline-table remain the same, but the baseline-table font-size is changed to the value
    /// of the ‘font-size’ property on this element. This re-scales the baseline-table for the current ‘font-size’.
    ResetSize,
    /// The baseline-identifier for the dominant-baseline is set to be 'ideographic', the derived baseline-table is constructed
    /// using the 'ideographic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Ideographic,
    /// The baseline-identifier for the dominant-baseline is set to be 'alphabetic', the derived baseline-table is constructed
    /// using the 'alphabetic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Alphabetic,
    /// The baseline-identifier for the dominant-baseline is set to be 'hanging', the derived baseline-table is constructed using
    /// the 'hanging' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    Hanging,
    /// The baseline-identifier for the dominant-baseline is set to be 'mathematical', the derived baseline-table is constructed
    /// using the 'mathematical' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Mathematical,
    /// The baseline-identifier for the dominant-baseline is set to be 'central'. The derived baseline-table is constructed from
    /// the defined baselines in a baseline-table in the nominal font. That font baseline-table is chosen using the following
    /// priority order of baseline-table names: 'ideographic', 'alphabetic', 'hanging', 'mathematical'. The baseline-table font-size
    /// is changed to the value of the ‘font-size’ property on this element.
    Central,
    /// The baseline-identifier for the dominant-baseline is set to be 'middle'. The derived baseline-table is constructed from
    /// the defined baselines in a baseline-table in the nominal font. That font baseline -table is chosen using the following
    /// priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'. The baseline-table
    /// font-size is changed to the value of the ‘font-size’ property on this element.
    Middle,
    /// The baseline-identifier for the dominant-baseline is set to be 'text-after-edge'. The derived baseline-table is constructed
    /// from the defined baselines in a baseline-table in the nominal font. The choice of which font baseline-table to use from the
    /// baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    ///
    /// NOTE: using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'
    /// is probably a reasonable strategy for determining which font baseline-table to use.
    TextAfterEdge,
    /// The baseline-identifier for the dominant-baseline is set to be 'text-before-edge'. The derived baseline-table is constructed
    /// from the defined baselines in a baseline-table in the nominal font. The choice of which baseline-table to use from the
    /// baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    ///
    /// NOTE: Using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'
    /// is probably a reasonable strategy for determining which font baseline-table to use.
    TextBeforeEdge,
}

impl Default for DominantBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignmentBaseline {
    /// The value is the dominant-baseline of the script to which the character belongs - i.e.,
    /// use the dominant-baseline of the parent.
    Auto,
    /// The alignment-point of the object being aligned is aligned with the dominant-baseline of
    /// the parent text content element.
    Baseline,
    /// The alignment-point of the object being aligned is aligned with the "before-edge" baseline of
    /// the parent text content element.
    BeforeEdge,
    /// The alignment-point of the object being aligned is aligned with the "text-before-edge" baseline of
    /// the parent text content element.
    TextBeforeEdge,
    /// The alignment-point of the object being aligned is aligned with the "middle" baseline of the parent text content element.
    Middle,
    /// The alignment-point of the object being aligned is aligned with the "central" baseline of the parent text content element.
    Central,
    /// The alignment-point of the object being aligned is aligned with the "after-edge" baseline of the parent text content element.
    AfterEdge,
    /// The alignment-point of the object being aligned is aligned with the "text-after-edge" baseline of the parent text content element.
    TextAfterEdge,
    /// The alignment-point of the object being aligned is aligned with the "ideographic" baseline of the parent text content element.
    Ideographic,
    /// The alignment-point of the object being aligned is aligned with the "alphabetic" baseline of the parent text content element.
    Alphabetic,
    /// The alignment-point of the object being aligned is aligned with the "hanging" baseline of the parent text content element.
    Hanging,
    /// The alignment-point of the object being aligned is aligned with the "mathematical" baseline of the parent text content element.
    Mathematical,
}

impl Default for AlignmentBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// The ‘baseline-shift’ property allows repositioning of the dominant-baseline relative to the dominant-baseline of
/// the parent text content element. The shifted object might be a sub- or superscript. Within the shifted object,
/// the whole baseline-table is offset; not just a single baseline. The amount of the shift is determined from information
/// from the parent text content element, the sub- or superscript offset from the nominal font of the parent text content
/// element, percent of the "line-height" of the parent text content element or an absolute value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaselineShift {
    /// There is no baseline shift; the dominant-baseline remains in its original position.
    Baseline,
    /// The dominant-baseline is shifted to the default position for subscripts. The offset to this position
    /// is determined using the font data for the nominal font. Because in most fonts the subscript position
    /// is normally given relative to the "alphabetic" baseline, the user agent may compute the effective
    /// position for subscripts for superscripts when some other baseline is dominant. The suggested computation
    /// is to subtract the difference between the position of the dominant baseline and the position of the
    /// "alphabetic" baseline from the position of the subscript. The resulting offset is determined by multiplying
    /// the effective subscript position by the dominant baseline-table font-size. If there is no applicable font
    /// data the user agent may use heuristics to determine the offset.
    Sub,
    /// The dominant-baseline is shifted to the default position for superscripts. The offset to this position is
    /// determined using the font data for the nominal font. Because in most fonts the superscript position is normally
    /// given relative to the "alphabetic" baseline, the user agent may compute the effective position for superscripts
    /// when some other baseline is dominant. The suggested computation is to subtract the difference between the
    /// position of the dominant baseline and the position of the "alphabetic" baseline from the position of the
    /// superscript. The resulting offset is determined by multiplying the effective superscript position by the dominant
    /// baseline-table font-size. If there is no applicable font data the user agent may use heuristics to determine the
    /// offset.
    Super,
    /// The computed value of the property is this percentage multiplied by the computed "line-height" of the ‘text’ element.
    /// The dominant-baseline is shifted in the shift direction (positive value) or opposite to the shift direction
    /// (negative value) of the parent text content element by the computed value. A value of "0" is equivalent to "baseline".
    Value(Length),
}

impl Default for BaselineShift {
    fn default() -> Self {
        Self::Baseline
    }
}

/// support for various international writing directions, such as left-to-right (e.g., Latin scripts) and
/// bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts).
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLayout {
    /// See [`WritingMode`]
    pub write_mode: Option<WritingMode>,
    /// See [`GlyphOrientationHorizontal`]
    pub direction: Option<TextDirection>,

    /// See [`UnicodeBidi`]
    pub unicode_bidi: Option<UnicodeBidi>,

    /// See [`TextAnchor`]
    pub anchor: Option<Variable<TextAnchor>>,

    /// See [`DominantBaseline`]
    pub dominant_baseline: Option<Variable<DominantBaseline>>,

    /// See [`AlignmentBaseline`]
    pub alignment_baseline: Option<Variable<AlignmentBaseline>>,

    /// See [`BaselineShift`]
    pub baseline_shift: Option<Variable<BaselineShift>>,
}

impl From<WritingMode> for TextLayout {
    fn from(value: WritingMode) -> Self {
        Self {
            write_mode: Some(value),
            ..Default::default()
        }
    }
}

impl From<TextDirection> for TextLayout {
    fn from(value: TextDirection) -> Self {
        Self {
            direction: Some(value),
            ..Default::default()
        }
    }
}

impl From<UnicodeBidi> for TextLayout {
    fn from(value: UnicodeBidi) -> Self {
        Self {
            unicode_bidi: Some(value),
            ..Default::default()
        }
    }
}

impl From<TextAnchor> for TextLayout {
    fn from(value: TextAnchor) -> Self {
        Self {
            anchor: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<DominantBaseline> for TextLayout {
    fn from(value: DominantBaseline) -> Self {
        Self {
            dominant_baseline: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<AlignmentBaseline> for TextLayout {
    fn from(value: AlignmentBaseline) -> Self {
        Self {
            alignment_baseline: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<BaselineShift> for TextLayout {
    fn from(value: BaselineShift) -> Self {
        Self {
            baseline_shift: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

/// Indicates the method by which text should be rendered along the path.
///
/// A value of align indicates that the glyphs should be rendered using simple 2x3 transformations such
/// that there is no stretching/warping of the glyphs. Typically, supplemental rotation, scaling and
/// translation transformations are done for each glyph to be rendered. As a result, with align, fonts
/// where the glyphs are designed to be connected (e.g., cursive fonts), the connections may not align
/// properly when text is rendered along a path.
///
/// A value of stretch indicates that the glyph outlines will be converted into paths, and then all end
/// points and control points will be adjusted to be along the perpendicular vectors from the path,
/// thereby stretching and possibly warping the glyphs. With this approach, connected glyphs, such as in
/// cursive scripts, will maintain their connections.
///
/// If the attribute is not specified, the effect is as if a value of align were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathMethod {
    Align,
    Stretch,
}

impl Display for TextPathMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextPathMethod::Align => write!(f, "align"),
            TextPathMethod::Stretch => write!(f, "stretch"),
        }
    }
}

impl Default for TextPathMethod {
    fn default() -> Self {
        Self::Align
    }
}

/// Indicates how the user agent should determine the spacing between glyphs that are to be rendered along a path.
///
/// A value of exact indicates that the glyphs should be rendered exactly according to the spacing rules as specified
/// in Text on a path layout rules.
///
///
/// A value of auto indicates that the user agent should use text-on-a-path layout algorithms to adjust the spacing
/// between glyphs in order to achieve visually appealing results.
///
/// If the attribute is not specified, the effect is as if a value of exact were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathSpacing {
    Auto,
    Exact,
}

impl Display for TextPathSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextPathSpacing::Auto => write!(f, "auto"),
            TextPathSpacing::Exact => write!(f, "exact"),
        }
    }
}

impl Default for TextPathSpacing {
    fn default() -> Self {
        Self::Exact
    }
}

/// In addition to text drawn in a straight line, SVG also includes the ability to place text along the
/// shape of a ‘path’ element. To specify that a block of text is to be rendered along the shape of a ‘path’,
/// include the given text within a ‘textPath’ element which includes an href’ attribute with an IRI
/// reference to a ‘path’ element.

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPath {
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
    pub start_offset: Variable<Length>,

    /// See [`TextPathMethod`]
    pub method: Variable<TextPathMethod>,

    /// See [`TextPathSpacing`]
    pub spacing: Variable<TextPathSpacing>,

    /// An IRI reference to the ‘path’ element onto which the glyphs will be rendered.
    /// If `iri` is an invalid reference (e.g., no such element exists, or the referenced element is not a ‘path’),
    /// then the ‘textPath’ element is in error and its entire contents shall not be rendered by the user agent.
    pub href: Variable<String>,
}

impl From<String> for TextPath {
    fn from(value: String) -> Self {
        Self {
            href: Variable::Constant(value),
            ..Default::default()
        }
    }
}

impl From<&str> for TextPath {
    fn from(value: &str) -> Self {
        Self {
            href: Variable::Constant(value.to_owned()),
            ..Default::default()
        }
    }
}

#[cfg(feature = "sexpr")]
mod sexpr {
    use super::*;
    use crate::MapCollect;

    impl<X, Y> From<(X, Y)> for Text
    where
        X: MapCollect<Length>,
        Y: MapCollect<Length>,
    {
        fn from(value: (X, Y)) -> Self {
            Self {
                x: Variable::Constant(value.0.map_collect()),
                y: Variable::Constant(value.1.map_collect()),
                ..Default::default()
            }
        }
    }

    impl<X, Y> From<(X, Y)> for TextSpan
    where
        X: MapCollect<Length>,
        Y: MapCollect<Length>,
    {
        fn from(value: (X, Y)) -> Self {
            Self {
                x: Variable::Constant(value.0.map_collect()),
                y: Variable::Constant(value.1.map_collect()),
                ..Default::default()
            }
        }
    }
}
