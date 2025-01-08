//! The intermediate representation of the `mlang`.
//!

use parserc::Span;

/// Defines a `identifier` of one node or one attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident(pub String, pub Span);

impl Ident {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), span)
    }
}

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitNum(pub usize, pub Span);

impl LitNum {
    /// Create a new `Ident` with span.
    pub fn from_span(name: usize, span: Span) -> Self {
        Self(name.into(), span)
    }
}

/// Defines literal numeric.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LitStr(pub String, pub Span);

impl LitStr {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), span)
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
pub struct Comment(pub String, pub Span);

impl Comment {
    /// Create a new `Ident` with span.
    pub fn from_span<T>(name: T, span: Span) -> Self
    where
        String: From<T>,
    {
        Self(name.into(), span)
    }
}

/// A property call-expr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallExpr {
    pub span: Span,
    /// The identifier name of this element.
    pub ident: Ident,
    /// Optional call input parameter list.
    pub params: Vec<LitExpr>,
}

impl CallExpr {
    /// Create a new `CallExpr` instance with `span`.
    pub fn from_span(ident: Ident, span: Span) -> Self {
        Self {
            span,
            ident,
            params: vec![],
        }
    }
    /// Add a new param.
    pub fn param<P>(mut self, param: P) -> Self
    where
        LitExpr: From<P>,
    {
        self.params.push(param.into());

        self
    }
}

/// Defines custom property list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// the whole span of this `property`.
    pub span: Span,
    /// call-expr list.
    pub params: Vec<CallExpr>,
}

impl Property {
    /// Create a new `CallExpr` instance with `span`.
    pub fn from_span(span: Span) -> Self {
        Self {
            span,
            params: vec![],
        }
    }
    /// Add a new param.
    pub fn param<P>(mut self, param: P) -> Self
    where
        CallExpr: From<P>,
    {
        self.params.push(param.into());

        self
    }
}

/// Defines a node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node {
    /// comment of this node
    pub comments: Vec<Comment>,
    /// The ident of the mixin data.
    pub mixin: Option<Ident>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The identifier name of this node
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines an enum data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// comment of this node
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Node>,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// filed name,
    pub ident: Option<Ident>,
    /// The type of this field.
    pub ty: Type,
}

/// Defines apply to link
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyTo {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The span of group.
    pub span: Span,
    /// From target.
    pub from: Vec<Ident>,
    /// To target
    pub to: Vec<Ident>,
}

/// Defines childof link
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildrenOf {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The span of group.
    pub span: Span,
    /// children group.
    pub from: Vec<Ident>,
    /// parent group.
    pub to: Vec<Ident>,
}

/// Defines a group.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The span of group.
    pub span: Span,
    /// The name of the group.
    pub ident: Ident,
    /// The content of the group.
    pub children: Vec<Ident>,
}

/// Defines a `type` of one field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Bool(Span),
    String(Span),
    Byte(Span),
    Ubyte(Span),
    Short(Span),
    Ushort(Span),
    Int(Span),
    Uint(Span),
    Long(Span),
    Ulong(Span),
    Float(Span),
    Double(Span),
    /// A data/enum reference.
    Data(Ident),
    /// This type is `vec[T]`.
    ListOf(Box<Type>, Span),
    /// Array type [T;30]
    ArrayOf(Box<Type>, LitNum, Span),
}

impl Type {
    /// Return span of the type.
    pub fn span(&self) -> &Span {
        match self {
            Type::Bool(span) => span,
            Type::String(span) => span,
            Type::Byte(span) => span,
            Type::Ubyte(span) => span,
            Type::Short(span) => span,
            Type::Ushort(span) => span,
            Type::Int(span) => span,
            Type::Uint(span) => span,
            Type::Long(span) => span,
            Type::Ulong(span) => span,
            Type::Float(span) => span,
            Type::Double(span) => span,
            Type::Data(ident) => &ident.1,
            Type::ListOf(_, span) => span,
            Type::ArrayOf(_, _, span) => span,
        }
    }
}

/// Defines `mlang`'s opcode.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Element(Box<Node>),
    Leaf(Box<Node>),
    Attr(Box<Node>),
    Mixin(Box<Node>),
    Data(Box<Node>),
    Enum(Box<Enum>),
    Group(Box<Group>),
    ApplyTo(Box<ApplyTo>),
    ChildrenOf(Box<ChildrenOf>),
}
