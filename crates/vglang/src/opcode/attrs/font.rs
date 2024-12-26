use crate::opcode::{
    data::{FontFamily, FontStretch, FontStyle, FontVariant, FontWeight, Length},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use crate::opcode::el::*;

/// Shorthand property for setting ‘font-style’, ‘font-variant’, ‘font-weight’, ‘font-size’, ‘line-height’ and ‘font-family’.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(boxed, Group, Text, TextSpan, Characters)
)]
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
            family: Some(Variable::Constant(FontFamily::Generic(value))),
            ..Default::default()
        }
    }
}

impl From<&str> for Font {
    fn from(value: &str) -> Self {
        Self {
            family: Some(Variable::Constant(FontFamily::Generic(value.to_owned()))),
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
