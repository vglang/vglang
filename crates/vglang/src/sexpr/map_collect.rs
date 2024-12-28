use crate::opcode::{
    attrs::Transformed,
    data::{Length, Transform},
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
            x: Some(Variable::Constant(value.0.map_collect())),
            y: Some(Variable::Constant(value.1.map_collect())),
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
            x: Some(Variable::Constant(value.0.map_collect())),
            y: Some(Variable::Constant(value.1.map_collect())),
            ..Default::default()
        }
    }
}

impl<T> From<T> for Transformed
where
    T: MapCollect<Transform>,
{
    fn from(value: T) -> Self {
        Self(value.map_collect())
    }
}
