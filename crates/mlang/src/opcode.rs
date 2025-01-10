//! The intermediate representation of the `mlang`.
//!

use std::{
    ops::Deref,
    slice::{Iter, IterMut},
};

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

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
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
    pub span: Span,
    /// call-expr list.
    pub params: Vec<CallExpr>,
}

impl Property {
    /// Add a new param.
    pub fn param<P>(mut self, param: P) -> Self
    where
        CallExpr: From<P>,
    {
        self.params.push(param.into());

        self
    }
}

/// Named field for nodes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamedField {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// filed name,
    pub ident: Ident,
    /// The type of this field.
    pub ty: Type,
}

/// Unnamed field for tuple body.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnnamedField {
    /// comment of this field.
    pub comments: Vec<Comment>,
    /// custom propert list.
    pub properties: Vec<Property>,
    /// The type of this field.
    pub ty: Type,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fields {
    None,
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

impl Fields {
    /// Check if this node is a tuple node.
    pub fn is_tuple(&self) -> bool {
        match &self {
            Fields::Named(_) => false,
            _ => true,
        }
    }
}

impl Fields {
    pub fn append(self, append: Fields) -> Result<Fields, Fields> {
        match self {
            Self::None => Ok(append),
            Self::Named(mut vec) => match append {
                Self::None => Ok(Fields::Named(vec)),
                Self::Named(mut append) => {
                    vec.append(&mut append);

                    Ok(Fields::Named(vec))
                }
                Self::Unnamed(vec) => Err(Self::Unnamed(vec)),
            },
            Self::Unnamed(mut vec) => match append {
                Self::None => Ok(Fields::Unnamed(vec)),
                Self::Unnamed(mut append) => {
                    vec.append(&mut append);

                    Ok(Fields::Unnamed(vec))
                }
                Self::Named(vec) => Err(Self::Named(vec)),
            },
        }
    }
    /// Create a immutable iterator over fields.
    pub fn iter(&self) -> FieldIter<'_> {
        match self {
            Fields::None => FieldIter::None,
            Fields::Named(vec) => FieldIter::Named(vec.iter()),
            Fields::Unnamed(vec) => FieldIter::Unnamed(vec.iter()),
        }
    }

    /// Create a mutable iterator over fields.
    pub fn iter_mut(&mut self) -> FieldIterMut<'_> {
        match self {
            Fields::None => FieldIterMut::None,
            Fields::Named(vec) => FieldIterMut::Named(vec.iter_mut()),
            Fields::Unnamed(vec) => FieldIterMut::Unnamed(vec.iter_mut()),
        }
    }
}

pub enum FieldIterMut<'a> {
    None,
    Named(IterMut<'a, NamedField>),
    Unnamed(IterMut<'a, UnnamedField>),
}

/// The item type used by FieldIter.
pub enum FieldMut<'a> {
    Named(&'a mut NamedField),
    Unnamed(&'a mut UnnamedField),
}

impl<'a> FieldMut<'a> {
    /// Get comments
    pub fn comments(&mut self) -> &mut [Comment] {
        match self {
            FieldMut::Named(named_field) => &mut named_field.comments,
            FieldMut::Unnamed(unnamed_field) => &mut unnamed_field.comments,
        }
    }

    /// Get properties
    pub fn properties(&mut self) -> &mut [Property] {
        match self {
            FieldMut::Named(named_field) => &mut named_field.properties,
            FieldMut::Unnamed(unnamed_field) => &mut unnamed_field.properties,
        }
    }

    /// Get type.
    pub fn ty(&mut self) -> &mut Type {
        match self {
            FieldMut::Named(named_field) => &mut named_field.ty,
            FieldMut::Unnamed(unnamed_field) => &mut unnamed_field.ty,
        }
    }

    /// Get ident.
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            FieldMut::Named(named_field) => Some(&named_field.ident),
            FieldMut::Unnamed(_) => None,
        }
    }
}

impl<'a> Iterator for FieldIterMut<'a> {
    type Item = FieldMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FieldIterMut::None => None,
            FieldIterMut::Named(iter) => iter.next().map(|next| FieldMut::Named(next)),
            FieldIterMut::Unnamed(iter) => iter.next().map(|next| FieldMut::Unnamed(next)),
        }
    }
}

/// The field immutable iterator.
pub enum FieldIter<'a> {
    None,
    Named(Iter<'a, NamedField>),
    Unnamed(Iter<'a, UnnamedField>),
}

/// The item type used by FieldIter.
pub enum Field<'a> {
    Named(&'a NamedField),
    Unnamed(&'a UnnamedField),
}

impl<'a> Field<'a> {
    /// Get comments
    pub fn comments(&self) -> &[Comment] {
        match self {
            Field::Named(named_field) => &named_field.comments,
            Field::Unnamed(unnamed_field) => &unnamed_field.comments,
        }
    }

    /// Get properties
    pub fn properties(&self) -> &[Property] {
        match self {
            Field::Named(named_field) => &named_field.properties,
            Field::Unnamed(unnamed_field) => &unnamed_field.properties,
        }
    }

    /// Get type.
    pub fn ty(&self) -> &Type {
        match self {
            Field::Named(named_field) => &named_field.ty,
            Field::Unnamed(unnamed_field) => &unnamed_field.ty,
        }
    }

    /// Get ident.
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            Field::Named(named_field) => Some(&named_field.ident),
            Field::Unnamed(_) => None,
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FieldIter::None => None,
            FieldIter::Named(iter) => iter.next().map(|next| Field::Named(next)),
            FieldIter::Unnamed(iter) => iter.next().map(|next| Field::Unnamed(next)),
        }
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
    pub fields: Fields,
}

impl Node {
    /// Check if this node is a tuple node.
    pub fn is_tuple(&self) -> bool {
        self.fields.is_tuple()
    }
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
    /// Array type `[T;30]`
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
