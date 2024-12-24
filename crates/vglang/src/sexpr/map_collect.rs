use crate::opcode::{
    data::{Angle, FontFamily, Length, Number, Point},
    el::{Text, TextSpan},
    variable::Variable,
};

/// Map item via iterator and collect them into vec.
pub trait MapCollect<Item> {
    fn map_collect(self) -> Vec<Item>;
}

impl<F, T> MapCollect<T> for F
where
    T: From<F>,
{
    fn map_collect(self) -> Vec<T> {
        vec![self.into()]
    }
}

impl<X, Y> From<(X, Y)> for Text
where
    X: MapCollect<Length>,
    Y: MapCollect<Length>,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: Variable::Constant(value.0.map_collect()),
            y: Variable::Constant(value.1.map_collect()),
            ..Default::default()
        }
    }
}

impl<X, Y> From<(X, Y)> for TextSpan
where
    X: MapCollect<Length>,
    Y: MapCollect<Length>,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: Variable::Constant(value.0.map_collect()),
            y: Variable::Constant(value.1.map_collect()),
            ..Default::default()
        }
    }
}

macro_rules! tuple_map_collect {
    ($item: ident, $header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+> $crate::sexpr::MapCollect<$item> for ($header, $($tail),+)
        where
            $header: Into<$item>,
            $($tail: Into<$item>),+,
        {
            #[allow(non_snake_case)]
            fn map_collect(self) -> Vec<$item> {
                let ($header, $($tail),+) = self;
                vec![$header.into(),$($tail.into()),+]
            }
        }

        tuple_map_collect!($item, $($tail),+);
    };
    ($item: ident, $header: ident) => {}
}

tuple_map_collect!(
    Length, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
    A19, A20
);

tuple_map_collect!(
    FontFamily, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17,
    A18, A19, A20
);

tuple_map_collect!(
    Angle, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
    A19, A20
);

macro_rules! point_map_collect {
    ($item: ident, $header_x: ident, $header_y: ident, $($tail_x: ident, $tail_y: ident),+) => {

        impl<$header_x, $header_y, $($tail_x, $tail_y),+> MapCollect<$item> for ($header_x, $header_y, $($tail_x, $tail_y),+)
        where
            Number: From<$header_x> + From<$header_y> $(+ From<$tail_x> + From<$tail_y>)+,
        {
            #[allow(non_snake_case)]
            fn map_collect(self) -> Vec<$item> {
                let ($header_x, $header_y, $($tail_x, $tail_y),+) = self;
                vec![Point {x: Number::from($header_x).0,y:  Number::from($header_y).0},
                $(Point {x: Number::from($tail_x).0,y: Number::from($tail_y).0}),+]
            }
        }

        point_map_collect!($item, $($tail_x,$tail_y),+);
    };
    ($item: ident, $header_x: ident, $header_y: ident) => {}
    }

point_map_collect!(
    Point, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
    A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37,
    A38, A39, A40, A41, A42, A43, A44, A45, A46, A47, A48, A49, A50, A51, A52, A53, A54, A55, A56,
    A57, A58, A59
);
