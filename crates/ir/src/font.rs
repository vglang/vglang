use std::fmt::Display;

use crate::{Animatable, FrameVariable, Measurement};

/// See [`style`](FontFace::style)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameVariable for FontStyle {}

/// Same syntax and semantics as the ‘font-variant’ descriptor within an @font-face rule.
/// Indication of whether this face is the small-caps variant of a font. Takes on the same values
/// as the ‘font-variant’ property, except that a comma-separated list is permitted.
///
/// If the attribute is not specified, the effect is as if a value of 'normal' were specified.

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    Normal,
    SmallCaps,
}

impl Default for FontVariant {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameVariable for FontVariant {}

/// Same syntax and semantics as the ‘font-weight’ descriptor within an @font-face rule.
///
/// See [`FontFace`]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameVariable for FontWeight {}

/// This property specifies a prioritized font family names and/or generic family names.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Custom(String),
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::Serif
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontFamily::Serif => write!(f, "serif"),
            FontFamily::SansSerif => write!(f, "sans-serif"),
            FontFamily::Cursive => write!(f, "cursive"),
            FontFamily::Fantasy => write!(f, "fantasy"),
            FontFamily::Monospace => write!(f, "monospace"),
            FontFamily::Custom(v) => write!(f, "{}", v),
        }
    }
}
impl FrameVariable for FontFamily {}

/// Same syntax and semantics as the ‘font-stretch’ descriptor within an @font-face rule.
/// Indication of the condensed or expanded nature of the face relative to others in the same font family.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Normal,
    Wider,
    Narrower,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Default for FontStretch {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameVariable for FontStretch {}

/// Shorthand property for setting ‘font-style’, ‘font-variant’, ‘font-weight’, ‘font-size’, ‘line-height’ and ‘font-family’.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "dsl", derive(cotati_derive::Dsl))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    /// See [`FontFamily`]
    pub family: Option<Animatable<FontFamily>>,
    /// See [`FontStyle`]
    pub style: Option<Animatable<FontStyle>>,
    /// See [`FontVariant`]
    pub variant: Option<Animatable<FontVariant>>,
    /// See [`FontWeight`]
    pub weight: Option<Animatable<FontWeight>>,
    /// This property refers to the size of the font from baseline to baseline when multiple lines of
    /// text are set solid in a multiline layout environment.
    pub size: Option<Animatable<Measurement>>,
    /// See [`FontStretch`]
    pub stretch: Option<Animatable<FontStretch>>,
}

impl From<FontFamily> for Font {
    fn from(value: FontFamily) -> Self {
        Self {
            family: Some(Animatable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontStyle> for Font {
    fn from(value: FontStyle) -> Self {
        Self {
            style: Some(Animatable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontVariant> for Font {
    fn from(value: FontVariant) -> Self {
        Self {
            variant: Some(Animatable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontWeight> for Font {
    fn from(value: FontWeight) -> Self {
        Self {
            weight: Some(Animatable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontStretch> for Font {
    fn from(value: FontStretch) -> Self {
        Self {
            stretch: Some(Animatable::Constant(value)),
            ..Default::default()
        }
    }
}

impl<T> From<T> for Font
where
    Measurement: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            size: Some(Animatable::Constant(value.into())),
            ..Default::default()
        }
    }
}
