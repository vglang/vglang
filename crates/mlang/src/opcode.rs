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

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitNum(pub usize);

impl<T> From<T> for LitNum
where
    usize: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitStr(pub String);

impl<T> From<T> for LitStr
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Defines literal expr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LitExpr {
    String(LitStr),
    Numeric(LitNum),
}

impl From<LitStr> for LitExpr {
    fn from(value: LitStr) -> Self {
        Self::String(value)
    }
}

impl From<LitNum> for LitExpr {
    fn from(value: LitNum) -> Self {
        Self::Numeric(value)
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

/// A property call-expr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallExpr {
    /// The identifier name of this element.
    pub ident: Ident,
    /// Optional call input parameter list.
    pub params: Vec<LitExpr>,
}

impl CallExpr {
    /// Add a new param.
    pub fn param<P>(mut self, param: P) -> Self
    where
        LitExpr: From<P>,
    {
        self.params.push(param.into());

        self
    }
}

impl<I> From<I> for CallExpr
where
    Ident: From<I>,
{
    fn from(value: I) -> Self {
        Self {
            ident: value.into(),
            params: vec![],
        }
    }
}

/// Defines custom property list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property(pub Vec<CallExpr>);

impl<I> From<I> for Property
where
    I: IntoIterator,
    CallExpr: From<I::Item>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().map(|v| v.into()).collect())
    }
}

/// Defines a non-leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
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
    /// custom propert list.
    pub property: Option<Vec<Property>>,
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
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}
/// Defines an data.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The identifier name of this element.
    pub ident: Option<Ident>,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines an enum data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<(Ident, Data)>,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
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

    ArrayOf(Box<Type>, LitNum),
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
