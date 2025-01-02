//! The intermediate representation of the `mlang`.
//!

/// Defines a `identifier` of one node or one attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident(pub String);

impl<T> From<T> for Ident
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Defines literal signed number.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Numeric(pub usize);

impl<T> From<T> for Numeric
where
    usize: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Defines token of a line of comment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Comment(pub String);

impl<T> From<T> for Comment
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Defines custom property list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property(pub Vec<Ident>);

impl<I> From<I> for Property
where
    I: IntoIterator,
    Ident: From<I::Item>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().map(|v| v.into()).collect())
    }
}

/// Defines a non-leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
    /// allow child nodes.
    pub allow_children: Vec<Ident>,
    /// appliable attrs.
    pub allow_attrs: Vec<Ident>,
}

/// Defines a leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Leaf {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
    /// appliable attrs.
    pub allow_attrs: Vec<Ident>,
}

/// Defines an attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attr {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}
/// Defines an data.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    /// The identifier name of this element.
    pub ident: Option<Ident>,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines an enum data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<(Ident, Data)>,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// filed name,
    pub ident: Option<Ident>,
    /// The type of this field.
    pub ty: Type,
    /// Indicate this field is optional.
    pub optional: bool,
    /// Indicate this field is a variable field.
    pub variable: bool,
}

/// Defines a `type` of one field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Bool,
    String,
    Byte,
    Ubyte,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    /// A type reference.
    Ref(Ident),
    /// This type is  vec[T].
    ListOf(Box<Type>),

    ArrayOf(Box<Type>, Numeric),
}

/// Defines `mlang`'s opcode.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Comment(Box<Comment>),
    Element(Box<Element>),
    Leaf(Box<Leaf>),
    Attr(Box<Attr>),
    Data(Box<Data>),
    Property(Box<Property>),
    Enum(Box<Enum>),
}