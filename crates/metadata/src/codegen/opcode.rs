use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    codegen::ext::{EnumGen, FieldGen, NodeGen},
    ir::{Enum, Node, Stat},
};

/// An extension to generate rust structure.
pub trait DefinitionGen {
    fn gen_definition(&self) -> TokenStream;
}

impl DefinitionGen for Node {
    fn gen_definition(&self) -> TokenStream {
        let comments = self.gen_comments();
        let ident = self.gen_ident();

        let fields = self
            .fields
            .iter()
            .map(|field| field.gen_definition(quote! {pub}))
            .collect::<Vec<_>>();

        let body = self.gen_body_expr(fields);

        let end_token = if self.is_tuple() {
            quote! {;}
        } else {
            quote! {}
        };

        quote! {
            #(#comments)*
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #ident #body #end_token
        }
    }
}

impl DefinitionGen for Enum {
    fn gen_definition(&self) -> TokenStream {
        let comments = self.gen_comments();
        let ident = self.gen_ident();

        let fields = self
            .fields
            .iter()
            .map(|node| {
                let comments = node.gen_comments();
                let ident = node.gen_ident();

                let fields = node
                    .fields
                    .iter()
                    .map(|field| field.gen_definition(quote! {}))
                    .collect::<Vec<_>>();

                let body = node.gen_body_expr(fields);

                quote! {
                    #(#comments)*
                    #ident #body
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#comments)*
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #ident { #(#fields),* }
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
    pub fn gen(mut self, opcodes: &[Stat]) -> TokenStream {
        let mut token_streams = vec![];

        for opcode in opcodes {
            match opcode {
                Stat::Element(node) => {
                    token_streams.push(node.gen_definition());
                    self.el_types.push(node.gen_ident());
                }
                Stat::Leaf(node) => {
                    token_streams.push(node.gen_definition());
                    self.leaf_types.push(node.gen_ident());
                }
                Stat::Attr(node) => {
                    token_streams.push(node.gen_definition());
                    self.attr_types.push(node.gen_ident());
                }
                Stat::Data(node) => {
                    token_streams.push(node.gen_definition());
                    self.data_types.push(node.gen_ident());
                }
                Stat::Enum(node) => {
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
            pub mod variable {
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

                impl<'a> TryFrom<&'a Data> for &'a #ty {
                    type Error = ();

                    fn try_from(value: &'a Data) -> Result<Self,Self::Error> {
                        match value {
                            Data::#ident(v) => Ok(v),
                            _ => Err(())
                        }
                    }
                }

                impl From<Vec<#ty>> for Data {
                    fn from(value: Vec<#ty>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }

                impl<'a> TryFrom<&'a Data> for &'a Vec<#ty> {
                    type Error = ();

                    fn try_from(value: &'a Data) -> Result<Self,Self::Error> {
                        match value {
                            Data::#list_ident(v) => Ok(v),
                            _ => Err(())
                        }
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

                impl<'a> TryFrom<&'a Data> for &'a #ident {
                    type Error = ();

                    fn try_from(value: &'a Data) -> Result<Self,Self::Error> {
                        match value {
                            Data::#ident(v) => Ok(v),
                            _ => Err(())
                        }
                    }
                }

                impl From<Vec<#ident>> for Data {
                    fn from(value: Vec<#ident>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }

                impl<'a> TryFrom<&'a Data> for &'a Vec<#ident> {
                    type Error = ();

                    fn try_from(value: &'a Data) -> Result<Self,Self::Error> {
                        match value {
                            Data::#list_ident(v) => Ok(v),
                            _ => Err(())
                        }
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
