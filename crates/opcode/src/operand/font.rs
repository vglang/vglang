use std::fmt::Display;

use super::{Length, Variable};

/// See [`css2`](https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-font-style)
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

/// Same syntax and semantics as the ‘font-weight’ descriptor within an @font-face rule.
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

impl From<&str> for FontFamily {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
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

/// Shorthand property for setting ‘font-style’, ‘font-variant’, ‘font-weight’, ‘font-size’, ‘line-height’ and ‘font-family’.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    /// See [`FontFamily`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub family: Option<Variable<FontFamily>>,
    /// See [`FontStyle`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style: Option<Variable<FontStyle>>,
    /// See [`FontVariant`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub variant: Option<Variable<FontVariant>>,
    /// See [`FontWeight`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub weight: Option<Variable<FontWeight>>,
    /// This property refers to the size of the font from baseline to baseline when multiple lines of
    /// text are set solid in a multiline layout environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<Variable<Length>>,
    /// See [`FontStretch`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stretch: Option<Variable<FontStretch>>,
}

impl From<FontFamily> for Font {
    fn from(value: FontFamily) -> Self {
        Self {
            family: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<String> for Font {
    fn from(value: String) -> Self {
        Self {
            family: Some(Variable::Constant(FontFamily::Custom(value))),
            ..Default::default()
        }
    }
}

impl From<&str> for Font {
    fn from(value: &str) -> Self {
        Self {
            family: Some(Variable::Constant(FontFamily::Custom(value.to_owned()))),
            ..Default::default()
        }
    }
}

impl From<FontStyle> for Font {
    fn from(value: FontStyle) -> Self {
        Self {
            style: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontVariant> for Font {
    fn from(value: FontVariant) -> Self {
        Self {
            variant: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontWeight> for Font {
    fn from(value: FontWeight) -> Self {
        Self {
            weight: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<FontStretch> for Font {
    fn from(value: FontStretch) -> Self {
        Self {
            stretch: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl<T> From<T> for Font
where
    Length: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            size: Some(Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
