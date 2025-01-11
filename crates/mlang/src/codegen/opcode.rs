use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::opcode::{Enum, Field, Node, Opcode, Type};

/// To generate a opcode type.
pub trait Definition {
    fn gen_ident(&self) -> TokenStream;
    fn gen_keyword(&self) -> TokenStream;
    fn gen_body(&self, fields: Vec<TokenStream>) -> TokenStream;
    fn gen_opcode_end(&self) -> TokenStream;
    fn gen_fields(&self, vis: &TokenStream) -> Vec<TokenStream>;
    fn gen_comments(&self) -> Vec<TokenStream>;
    /// Generate one opcode's definition codes.
    fn gen_definition(&self) -> TokenStream {
        let keyword = self.gen_keyword();
        let ident = self.gen_ident();
        let comments = self.gen_comments();

        let body = self.gen_body(self.gen_fields(&quote! {pub}));

        let opcode_end = self.gen_opcode_end();

        quote! {
            #(#comments)*
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub #keyword #ident #body #opcode_end
        }
    }
}

impl Definition for Node {
    fn gen_comments(&self) -> Vec<TokenStream> {
        self.comments
            .iter()
            .map(|comment| format!("/// {0}", comment.0).parse().unwrap())
            .collect()
    }
    fn gen_ident(&self) -> TokenStream {
        self.ident.to_upper_camel_case().parse().unwrap()
    }

    fn gen_keyword(&self) -> TokenStream {
        "struct".parse().unwrap()
    }

    fn gen_body(&self, fields: Vec<TokenStream>) -> TokenStream {
        if fields.is_empty() {
            return quote! {};
        }

        if self.is_tuple() {
            quote! {(#(#fields),*)}
        } else {
            quote! {{#(#fields),*}}
        }
    }

    fn gen_opcode_end(&self) -> TokenStream {
        if self.is_tuple() {
            quote! {;}
        } else {
            quote! {}
        }
    }

    fn gen_fields(&self, vis: &TokenStream) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| field.gen_field(&vis))
            .collect()
    }
}

pub trait FieldDefinition {
    fn gen_field(&self, vis: &TokenStream) -> TokenStream;
}

pub trait FieldDefinitionExt: FieldDefinition {
    fn gen_ident(&self) -> Option<TokenStream>;
    fn is_option(&self) -> bool;
    fn is_variable(&self) -> bool;
}

impl FieldDefinition for Node {
    fn gen_field(&self, vis: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();

        let body = self.gen_body(self.gen_fields(vis));

        quote! {
            #ident #body
        }
    }
}

impl<'a> FieldDefinition for Field<'a> {
    fn gen_field(&self, vis: &TokenStream) -> TokenStream {
        let ident = if let Some(ident) = self.gen_ident() {
            format!("{}:", ident).parse().unwrap()
        } else {
            quote! {}
        };

        let mut ty = self.ty().gen_type();

        if self.is_variable() {
            ty = quote! { variable::Variable<#ty> };
        }

        if self.is_option() {
            ty = quote! { Option<#ty> };
        }

        quote! {
            #vis #ident #ty
        }
    }
}

impl<'a> FieldDefinitionExt for Field<'a> {
    fn is_option(&self) -> bool {
        for property in self.properties() {
            for callexpr in &property.params {
                if callexpr.ident.0 == "option" {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_variable(&self) -> bool {
        for property in self.properties() {
            for callexpr in &property.params {
                if callexpr.ident.0 == "variable" {
                    return true;
                }
            }
        }

        return false;
    }

    fn gen_ident(&self) -> Option<TokenStream> {
        if let Some(ident) = self.ident() {
            let ident = match ident.0.as_str() {
                "in" => "r#in",
                "type" => "r#type",
                ident => ident,
            };
            Some(ident.parse().unwrap())
        } else {
            None
        }
    }
}

impl Definition for Enum {
    fn gen_comments(&self) -> Vec<TokenStream> {
        self.comments
            .iter()
            .map(|comment| format!("/// {0}", comment.0).parse().unwrap())
            .collect()
    }
    fn gen_ident(&self) -> TokenStream {
        self.ident.parse().unwrap()
    }

    fn gen_keyword(&self) -> TokenStream {
        "enum".parse().unwrap()
    }

    fn gen_body(&self, fields: Vec<TokenStream>) -> TokenStream {
        quote! {
            { #(#fields),* }
        }
    }

    fn gen_opcode_end(&self) -> TokenStream {
        quote! {}
    }

    fn gen_fields(&self, _vis: &TokenStream) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| field.gen_field(&quote! {}))
            .collect::<Vec<_>>()
    }
}

pub trait TypeDefinition {
    fn gen_type(&self) -> TokenStream;
}

impl TypeDefinition for Type {
    fn gen_type(&self) -> TokenStream {
        match self {
            Type::Bool(_) => quote! {bool},
            Type::String(_) => quote! {String},
            Type::Byte(_) => quote! {i8},
            Type::Ubyte(_) => quote! {u8},
            Type::Short(_) => quote! {i16},
            Type::Ushort(_) => quote! {u16},
            Type::Int(_) => quote! {i32},
            Type::Uint(_) => quote! {u32},
            Type::Long(_) => quote! {i64},
            Type::Ulong(_) => quote! {u64},
            Type::Float(_) => quote! {f32},
            Type::Double(_) => quote! {f64},
            Type::Data(ident) => ident.0.to_upper_camel_case().parse().unwrap(),
            Type::ListOf(component, _) => {
                let component = component.gen_type();

                quote! { Vec<#component> }
            }
            Type::ArrayOf(component, lit_num, _) => {
                let component = component.gen_type();
                let num = lit_num.0;

                quote! { [#component;#num] }
            }
        }
    }
}

/// A generator for `opcode` mod.
#[derive(Default)]
pub struct OpcodeModGen {
    /// collection of data types.
    data_types: Vec<TokenStream>,
    /// collection of attr types
    attr_types: Vec<TokenStream>,
    /// collection of el types
    el_types: Vec<TokenStream>,
    /// collection of leaf node types.
    leaf_types: Vec<TokenStream>,
}

impl OpcodeModGen {
    /// Consume self and generate rust codes.
    pub fn gen(mut self, opcodes: &[Opcode]) -> TokenStream {
        let mut token_streams = vec![];

        for opcode in opcodes {
            match opcode {
                Opcode::Element(node) => {
                    token_streams.push(node.gen_definition());
                    self.el_types.push(node.gen_ident());
                }
                Opcode::Leaf(node) => {
                    token_streams.push(node.gen_definition());
                    self.leaf_types.push(node.gen_ident());
                }
                Opcode::Attr(node) => {
                    token_streams.push(node.gen_definition());
                    self.attr_types.push(node.gen_ident());
                }
                Opcode::Data(node) => {
                    token_streams.push(node.gen_definition());
                    self.data_types.push(node.gen_ident());
                }
                Opcode::Enum(node) => {
                    token_streams.push(node.gen_definition());
                    self.data_types.push(node.gen_ident());
                }
                _ => {}
            }
        }

        token_streams.push(self.gen_data_definition());
        token_streams.push(self.gen_attr_definition());
        token_streams.push(self.gen_el_definition());
        token_streams.push(self.gen_leaf_definition());
        token_streams.push(self.gen_opcode_definition());
        token_streams.push(self.gen_variable_definition());

        quote! {
            #(#token_streams)*
        }
    }

    fn gen_variable_definition(&self) -> TokenStream {
        quote! {
            mod variable {
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
                    super::Data: From<T>,
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
                    super::Data: From<T>,
                {
                    fn from(value: T) -> Self {
                        Self::Constant(value)
                    }
                }

                impl<T> Default for Variable<T>
                where
                    T: Default,
                    super::Data: From<T>,
                {
                    fn default() -> Self {
                        Self::Constant(T::default())
                    }
                }

                impl<P, T> From<(P, Target)> for Variable<T>
                where
                    super::Data: From<T>,
                    Path: From<P>,
                {
                    fn from(value: (P, Target)) -> Self {
                        Self::Reference {
                            path: value.0.into(),
                            target: value.1,
                        }
                    }
                }
            }
        }
    }

    fn gen_data_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        let builtin_types = vec![
            ("bool", "bool"),
            ("string", "String"),
            ("byte", "i8"),
            ("ubyte", "u8"),
            ("short", "i16"),
            ("ushort", "u16"),
            ("int", "i32"),
            ("uint", "u32"),
            ("long", "i64"),
            ("ulong", "u64"),
            ("float", "f32"),
            ("double", "f64"),
        ];

        for (ident, ty) in builtin_types {
            let name = ident.to_upper_camel_case();
            let ty: TokenStream = ty.parse().unwrap();

            let ident: TokenStream = name.parse().unwrap();
            let list_ident = format_ident!("ListOf{}", name);

            fields.push(quote! {
                #ident(#ty),#list_ident(Box<Vec<#ty>>)
            });

            froms.push(quote! {
                impl From<#ty> for Data {
                    fn from(value: #ty) -> Self {
                        Data::#ident(value)
                    }
                }

                impl From<Vec<#ty>> for Data {
                    fn from(value: Vec<#ty>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }
            });
        }

        for ident in &self.data_types {
            let list_ident = format_ident!("ListOf{}", ident.to_string());

            fields.push(quote! {
                #ident(Box<#ident>),#list_ident(Box<Vec<#ident>>)
            });

            froms.push(quote! {
                impl From<#ident> for Data {
                    fn from(value: #ident) -> Self {
                        Data::#ident(Box::new(value))
                    }
                }

                impl From<Vec<#ident>> for Data {
                    fn from(value: Vec<#ident>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }
            });
        }

        quote! {
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Data {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_el_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.el_types {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Element {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Element {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_leaf_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.leaf_types {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Leaf {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Leaf {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    pub(super) fn gen_attr_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.attr_types {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Attr {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Attr {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_opcode_definition(&self) -> TokenStream {
        quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Opcode {
                Apply(Attr),
                Element(Element),
                Pop,
                Leaf(Leaf),
            }

            impl From<Attr> for Opcode {
                fn from(value: Attr) -> Self {
                    Self::Apply(value)
                }
            }

            impl From<Element> for Opcode {
                fn from(value: Element) -> Self {
                    Self::Element(value)
                }
            }

            impl From<Leaf> for Opcode {
                fn from(value: Leaf) -> Self {
                    Self::Leaf(value)
                }
            }
        }
    }
}
