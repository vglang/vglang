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
    Generic(String),
}

impl From<String> for FontFamily {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::Serif
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
