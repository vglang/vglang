use super::Value;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBy {
    /// Reference by name.
    Named(String),
    /// Reference by index.
    Index(usize),
}

impl From<String> for RefBy {
    fn from(value: String) -> Self {
        Self::Named(value)
    }
}

impl From<&str> for RefBy {
    fn from(value: &str) -> Self {
        Self::Named(value.to_owned())
    }
}

impl From<usize> for RefBy {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
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
    for<'a> &'a T: TryFrom<&'a Value, Error = &'a Value>,
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
    for<'a> &'a T: TryFrom<&'a Value, Error = &'a Value>,
    T: Default,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}
