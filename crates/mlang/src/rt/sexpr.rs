/// Map item via iterator and collect them into vec.
pub trait MapCollect<Item> {
    fn map_collect(self) -> Vec<Item>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(
    /// The wrapped [`f32`] value.
    pub f32,
);

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(value as f32)
    }
}
