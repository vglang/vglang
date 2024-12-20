use std::f32::consts::PI;

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

impl From<i32> for Angle {
    fn from(value: i32) -> Self {
        Self::deg(value as f32)
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Self::deg(value)
    }
}

impl Angle {
    pub fn as_deg(&self) -> f32 {
        match self {
            Angle::deg(v) => *v,
            Angle::grad(v) => *v * 360.0 / 400.0,
            Angle::rad(v) => *v * 180.0 / PI,
        }
    }
}

#[cfg(feature = "sexpr")]
mod sexpr {
    use crate::{tuple_map_collect, MapCollect};

    use super::*;

    impl MapCollect<Angle> for Vec<f32> {
        fn map_collect(self) -> Vec<Angle> {
            self.into_iter().map(|v| Angle::deg(v)).collect()
        }
    }

    impl MapCollect<Angle> for Vec<i32> {
        fn map_collect(self) -> Vec<Angle> {
            self.into_iter().map(|v| Angle::deg(v as f32)).collect()
        }
    }

    tuple_map_collect!(
        Angle, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
        A19, A20
    );
}
