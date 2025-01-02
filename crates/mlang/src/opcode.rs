//! The intermediate representation of the `mlang`.
//!

use parserc::Span;

/// Defines a `identifier` of one node or one attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident(pub String, pub Option<Span>);

impl<T> From<T> for Ident
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into(), None)
    }
}

impl Ident {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), Some(span))
    }
}

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitNum(pub usize, pub Option<Span>);

impl<T> From<T> for LitNum
where
    usize: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into(), None)
    }
}

impl LitNum {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        usize: From<T>,
    {
        Self(name.into(), Some(span))
    }
}

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitStr(pub String, pub Option<Span>);

impl<T> From<T> for LitStr
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into(), None)
    }
}

impl LitStr {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), Some(span))
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
pub struct Comment(pub String, pub Option<Span>);

impl<T> From<T> for Comment
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into(), None)
    }
}

impl Comment {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), Some(span))
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

/// Defines a mixin data structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mixin {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines a non-leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The ident of the mixin data.
    pub mixin: Option<Ident>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines a leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Leaf {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The ident of the mixin data.
    pub mixin: Option<Ident>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines an attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attr {
    /// custom propert list.
    pub property: Option<Vec<Property>>,
    /// The ident of the mixin data.
    pub mixin: Option<Ident>,
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
    Bool(Option<Span>),
    String(Option<Span>),
    Byte(Option<Span>),
    Ubyte(Option<Span>),
    Short(Option<Span>),
    Ushort(Option<Span>),
    Int(Option<Span>),
    Uint(Option<Span>),
    Long(Option<Span>),
    Ulong(Option<Span>),
    Float(Option<Span>),
    Double(Option<Span>),
    /// A data/enum reference.
    Data(Ident),
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
    Mixin(Box<Mixin>),
    Data(Box<Data>),
    Property(Box<Property>),
    Enum(Box<Enum>),
}
