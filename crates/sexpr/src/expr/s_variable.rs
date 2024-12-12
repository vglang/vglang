use super::Svalue;

/// variable reference to stack value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sref {
    // reference to stack value by name.
    Name(String),
    // reference to stack value by stack index.
    Index(usize),
}

/// variable reference to register.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sregister {
    /// reference to register by name.
    Named(String),
    /// reference to register by index.
    Index(usize),
}

/// a `sexpr` to create an animation variable
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sanimatable<T>
where
    Svalue: From<T>,
    T: TryFrom<Svalue, Error = Svalue>,
{
    Constant(T),

    /// variable reference to register.
    Register(Sregister),
}

impl<T> Default for Sanimatable<T>
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
pub enum Sconstant {
    /// reference to constant table value by index.
    Index(usize),
    /// in-place constant value.
    Value(Svalue),
}
/// Variables
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Svariable {
    /// variable reference to register.
    Register(Sregister),
    /// variable reference to constant value.
    Constant(Sconstant),
    /// variable reference to `foreach` item.
    Item(Sref),
    /// variable reference to `for range` index.
    Index(Sref),
}
