/// A length is a distance Length, given as a number along with a unit which may be optional.
///
/// See [`length`](https://www.w3.org/TR/SVG11/types.html#DataTypeLength)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Length {
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

impl Default for Length {
    fn default() -> Self {
        Self::px(0.0)
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Self::px(value)
    }
}

impl From<i32> for Length {
    fn from(value: i32) -> Self {
        Self::px(value as f32)
    }
}

#[cfg(feature = "sexpr")]
mod sexpr {
    use crate::{tuple_map_collect, MapCollect};

    use super::*;

    impl MapCollect<Length> for Vec<f32> {
        fn map_collect(self) -> Vec<Length> {
            self.into_iter().map(|v| v.into()).collect()
        }
    }

    impl MapCollect<Length> for Vec<i32> {
        fn map_collect(self) -> Vec<Length> {
            self.into_iter().map(|v| v.into()).collect()
        }
    }

    tuple_map_collect!(
        Length, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17,
        A18, A19, A20
    );
}
