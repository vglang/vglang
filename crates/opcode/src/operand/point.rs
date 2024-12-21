use num::ToPrimitive;

/// A 2d coordinate point.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Create a point from (f32,f32) with default unit `px`.
impl<X, Y> From<(X, Y)> for Point
where
    X: ToPrimitive,
    Y: ToPrimitive,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: value.0.to_f32().unwrap(),
            y: value.1.to_f32().unwrap(),
        }
    }
}

#[cfg(feature = "sexpr")]
mod sexpr {

    use crate::MapCollect;

    use super::*;

    macro_rules! point_map_collect {
    ($item: ident, $header_x: ident, $header_y: ident, $($tail_x: ident, $tail_y: ident),+) => {

        impl<$header_x, $header_y, $($tail_x, $tail_y),+> $crate::MapCollect<$item> for ($header_x, $header_y, $($tail_x, $tail_y),+)
        where
            $header_x: ToPrimitive,
            $header_y: ToPrimitive,
            $($tail_x: ToPrimitive, $tail_y: ToPrimitive),+,
        {
            #[allow(non_snake_case)]
            fn map_collect(self) -> Vec<$item> {
                let ($header_x, $header_y, $($tail_x, $tail_y),+) = self;
                vec![Point {x: $header_x.to_f32().unwrap(),y: $header_y.to_f32().unwrap()},
                $(Point {x: $tail_x.to_f32().unwrap(),y: $tail_y.to_f32().unwrap()}),+]
            }
        }

        point_map_collect!($item, $($tail_x,$tail_y),+);
    };
    ($item: ident, $header_x: ident, $header_y: ident) => {}
    }

    point_map_collect!(
        Point, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
        A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36,
        A37, A38, A39, A40, A41, A42, A43, A44, A45, A46, A47, A48, A49, A50, A51, A52, A53, A54,
        A55, A56, A57, A58, A59
    );

    impl MapCollect<Point> for Vec<Point> {
        fn map_collect(self) -> Vec<Point> {
            self
        }
    }
}
