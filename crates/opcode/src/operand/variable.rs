use super::Value;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBy {
    /// Reference by name.
    Named(String),
    /// Reference by index.
    Index(usize),
}

/// Variable is reference to context value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Context {
    /// Reference to a `foreach` iterator.
    Each(RefBy),
    /// Reference to a `for range` iterator.
    RangeIndex(RefBy),
}

/// Variable used by property fields.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variable<T>
where
    Value: From<T>,
    T: TryFrom<Value, Error = Value>,
{
    /// Variable is a constant value.
    Constant(T),
    /// reference to animation register.
    Animated(RefBy),
    /// reference to context value.
    Context(Context),
}

impl<T> Default for Variable<T>
where
    Value: From<T>,
    T: TryFrom<Value, Error = Value> + Default,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}
