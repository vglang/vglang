//! Defines reference types that can be used as properties and attributes values.

use crate::opcode::data::Data;

/// The path used by [`Variable`] is used to point to [`Target`].
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Path {
    /// Reference by name.
    Named(String),
    /// Reference by optimized position.
    Index(usize),
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::Named(value)
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::Named(value.to_owned())
    }
}

impl From<usize> for Path {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

/// The type of variable pointed to by [`Path`].
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Target {
    /// Target is animation register.
    Register,
    /// Target is `item` variable returns by `foreach` iterator.
    ForeachItem,
    /// Target is `index` variable returns by `foreach` iterator.
    ForeachIndex,
    /// Target is `index` variable returns by `for range` iterator.
    Range,
}

/// Variable used by property fields.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variable<T>
where
    Data: From<T>,
{
    /// A literal constant value.
    Constant(T),

    Reference {
        path: Path,
        target: Target,
    },
}

impl<T> From<T> for Variable<T>
where
    Data: From<T>,
{
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T> Default for Variable<T>
where
    Data: From<T>,
    T: Default,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}

impl<P, T> From<(P, Target)> for Variable<T>
where
    Data: From<T>,
    Path: From<P>,
{
    fn from(value: (P, Target)) -> Self {
        Self::Reference {
            path: value.0.into(),
            target: value.1,
        }
    }
}
