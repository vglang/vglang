use super::Svalue;

/// A length is a distance measurement, given as a number along with a unit which may be optional.
///
/// See [`length`](https://www.w3.org/TR/SVG11/types.html#DataTypeLength)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Slength {
    /// Represents the calculated font-size of the element. If used on the font-size property itself,
    /// it represents the inherited font-size of the element.
    em(f32),
    /// Represents the x-height of the element's font. In fonts with the x letter, this is generally
    /// the height of lowercase letters in the font; 1ex ≈ 0.5em in many fonts.
    ex(f32),
    /// Pixels
    px(f32),
    /// Inches
    r#in(f32),
    /// Centimeters
    cm(f32),
    /// Millimeters
    mm(f32),
    /// Points, 1pt = 1/72nd of 1in
    pt(f32),
    /// Picas, 	1pc = 1/6th of 1in
    pc(f32),
    /// A percentage value
    percent(f32),
}

impl Default for Slength {
    fn default() -> Self {
        Self::px(0.0)
    }
}

impl From<Slength> for Svalue {
    fn from(value: Slength) -> Self {
        Self::Length(value)
    }
}

impl TryFrom<Svalue> for Slength {
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::Length(v) => Ok(v),
            _ => Err(value),
        }
    }
}
