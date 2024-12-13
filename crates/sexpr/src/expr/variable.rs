use super::Svalue;

/// A reference to in-program entity.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Reference {
    // reference to stack value by name.
    Name(String),
    // reference to stack value by stack index.
    Index(usize),
}

/// variable reference to register.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Register {
    /// reference to register by name.
    Named(String),
    /// reference to register by index.
    Index(usize),
}

/// a `sexpr` to create an animation variable
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Animatable<T>
where
    Svalue: From<T>,
    T: TryFrom<Svalue, Error = Svalue>,
{
    Constant(T),

    /// variable reference to register.
    Register(Register),
}

impl<T> Default for Animatable<T>
where
    Svalue: From<T>,
    T: TryFrom<Svalue, Error = Svalue>,
    T: Default,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}

/// variable reference to contant value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Constant {
    /// reference to constant table value by index.
    Index(usize),
    /// in-place constant value.
    Value(Svalue),
}
/// Variables
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variable {
    /// variable reference to register.
    Register(Register),
    /// variable reference to constant value.
    Constant(Constant),
    /// variable reference to `foreach` item.
    Item(Reference),
    /// variable reference to `for range` index.
    Index(Reference),
    /// variable reference to code fragment.
    Fragment(Reference),
}
