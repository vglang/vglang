use super::Svalue;

/// Angles are specified in one of two ways depending upon
/// whether they are used in CSS property syntax or SVG
/// presentation attribute syntax:
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sangle {
    deg(f32),
    grad(f32),
    rad(f32),
}

impl Default for Sangle {
    fn default() -> Self {
        Self::deg(0.0)
    }
}

impl From<Sangle> for Svalue {
    fn from(value: Sangle) -> Self {
        Self::Angle(value)
    }
}

impl TryFrom<Svalue> for Sangle {
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::Angle(v) => Ok(v),
            _ => Err(value),
        }
    }
}
