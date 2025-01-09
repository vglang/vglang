use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

use crate::opcode::{Comment, Enum, Ident, Node, Opcode, Property, Type};

/// A code generator must implement this trait.
pub trait CodeGen {
    /// An el type code gen.
    type Node: NodeCodeGen;
    /// A enum type code generator.
    type Enum: EnumCodeGen;

    /// Create a `el` node
    fn create_node(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Node;

    /// Create a `enum data` node
    fn create_enum(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Enum;
    fn push_mixin(&mut self, el: Self::Node);
    fn push_el(&mut self, el: Self::Node);
    fn push_leaf(&mut self, el: Self::Node);
    fn push_attr(&mut self, el: Self::Node);
    fn push_data(&mut self, el: Self::Node);
    fn push_enum(&mut self, node: Self::Enum);

    /// Generate rust codes.
    fn gen(self) -> TokenStream;
}

/// Recognized field attrs.
#[derive(Default, Clone, Copy)]
pub struct FieldAttrs {
    pub option: bool,
    pub variable: bool,
}

impl From<&[Property]> for FieldAttrs {
    fn from(value: &[Property]) -> Self {
        let mut attrs = FieldAttrs::default();

        for p in value {
            for callexpr in &p.params {
                match callexpr.ident.0.as_str() {
                    "option" => attrs.option = true,
                    "variable" => attrs.variable = true,
                    _ => {}
                }
            }
        }

        attrs
    }
}

/// An `node` code generator must implement this trait.
///
/// Generally, call [`create_el`](CodeGen::create_el) to get a new instance.
pub trait EnumCodeGen {
    type Node: NodeCodeGen;

    /// Create a `el` node
    fn create_field(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Node;

    fn push_field(&mut self, el: Self::Node);
}

/// Field type used by [`NodeCodeGen`]
#[derive(Clone)]
pub enum FieldType {
    Noraml(TokenStream),
    List(TokenStream),
    Array(TokenStream, usize),
}

impl From<FieldType> for TokenStream {
    fn from(value: FieldType) -> Self {
        match value {
            FieldType::Noraml(token_stream) => token_stream,
            FieldType::List(token_stream) => {
                quote! {
                    Vec<#token_stream>
                }
            }
            FieldType::Array(token_stream, num) => {
                quote! {
                    [#token_stream;#num]
                }
            }
        }
    }
}

/// An `node` code generator must implement this trait.
///
/// Generally, call [`create_el`](CodeGen::create_el) to get a new instance.
pub trait NodeCodeGen {
    fn mixin(&mut self, target: String);
    /// Create a new field generator.
    fn push_field(
        &mut self,
        comments: TokenStream,
        ident: Option<TokenStream>,
        attrs: FieldAttrs,
        ty: FieldType,
    );
}

/// Conver comments to `TokenStream`
fn comments(comments: &[Comment]) -> TokenStream {
    comments
        .iter()
        .map(|c| format!("///{}", c.0))
        .collect::<Vec<_>>()
        .join("\n")
        .parse::<TokenStream>()
        .unwrap()
}

fn type_to_token_stream(ty: &Type) -> FieldType {
    match ty {
        Type::Bool(_) => FieldType::Noraml(quote! {bool}),
        Type::String(_) => FieldType::Noraml(quote! {String}),
        Type::Byte(_) => FieldType::Noraml(quote! {i8}),
        Type::Ubyte(_) => FieldType::Noraml(quote! {u8}),
        Type::Short(_) => FieldType::Noraml(quote! {i16}),
        Type::Ushort(_) => FieldType::Noraml(quote! {u16}),
        Type::Int(_) => FieldType::Noraml(quote! {i32}),
        Type::Uint(_) => FieldType::Noraml(quote! {u32}),
        Type::Long(_) => FieldType::Noraml(quote! {i64}),
        Type::Ulong(_) => FieldType::Noraml(quote! {u64}),
        Type::Float(_) => FieldType::Noraml(quote! {f32}),
        Type::Double(_) => FieldType::Noraml(quote! {f64}),
        Type::Data(ident) => FieldType::Noraml(ident.0.parse().unwrap()),
        Type::ListOf(component, _) => FieldType::List(type_to_token_stream(component).into()),
        Type::ArrayOf(component, lit_num, _) => {
            FieldType::Array(type_to_token_stream(component).into(), lit_num.0)
        }
    }
}

fn field_name(ident: &Ident) -> TokenStream {
    match ident.0.as_str() {
        "in" | "type" => format!("r#{}", ident.0).to_snake_case().parse().unwrap(),
        _ => ident.0.to_snake_case().parse().unwrap(),
    }
}

fn node_codegen<G: NodeCodeGen>(node: &Node, g: &mut G) {
    for field in &node.fields {
        let ident = field.ident.as_ref().map(field_name);

        let attrs = FieldAttrs::from(field.properties.as_slice());

        let comments = comments(&field.comments);

        let ty = type_to_token_stream(&field.ty);

        g.push_field(comments, ident, attrs, ty);
    }
}

fn enum_codegen<G: EnumCodeGen>(node: &Enum, g: &mut G) {
    for field in &node.fields {
        let comments = comments(&field.comments);
        let ident = field.ident.0.to_upper_camel_case().parse().unwrap();

        let mut cg = g.create_field(comments, ident);

        node_codegen(field, &mut cg);

        g.push_field(cg);
    }
}

/// Generate rust code from `opcode stream`.
pub fn codegen<G: CodeGen>(opcodes: &[Opcode], mut g: G) -> TokenStream {
    for opcode in opcodes {
        match opcode {
            Opcode::Element(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_node(comments, ident);
                node_codegen(node, &mut cg);
                g.push_el(cg);
            }
            Opcode::Leaf(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_node(comments, ident);
                node_codegen(node, &mut cg);
                g.push_leaf(cg);
            }
            Opcode::Attr(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_node(comments, ident);
                node_codegen(node, &mut cg);
                g.push_attr(cg);
            }
            Opcode::Mixin(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_node(comments, ident);
                node_codegen(node, &mut cg);
                g.push_mixin(cg);
            }
            Opcode::Data(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_node(comments, ident);
                node_codegen(node, &mut cg);
                g.push_data(cg);
            }
            Opcode::Enum(node) => {
                let ident: TokenStream = node.ident.0.parse().unwrap();
                let comments = comments(&node.comments);
                let mut cg = g.create_enum(comments, ident);
                enum_codegen(node, &mut cg);
                g.push_enum(cg);
            }
            Opcode::Group(_) => {}
            Opcode::ApplyTo(_) => {}
            Opcode::ChildrenOf(_) => {}
        }
    }

    g.gen()
}
