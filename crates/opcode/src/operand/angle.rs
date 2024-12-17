/// Angles are specified in one of two ways depending upon
/// whether they are used in CSS property syntax or SVG
/// presentation attribute syntax:
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    deg(f32),
    grad(f32),
    rad(f32),
}

impl Default for Angle {
    fn default() -> Self {
        Self::deg(0.0)
    }
}
