use crate::opcode::{
    data::{Angle, Length, TextLengthAdjust},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use super::*;

/// The ‘text’ element defines a graphics element consisting of text.
///
/// See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, Group, If, For, Foreach)
)]
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
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, Group, Text, TextSpan, If, For, Foreach)
)]
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
}

/// Text content chars.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::shape_element(boxed, Text, TextSpan, If, For, Foreach)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Characters(pub String);

impl<T> From<T> for Characters
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
