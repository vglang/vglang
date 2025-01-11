use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{codegen::opcode::NodeGen, opcode::Opcode};

/// Rust codes generator for `mlang`.
#[derive(Default)]
pub struct CodeGen {
    /// collection of data types.
    data_types: Vec<TokenStream>,
    /// collection of attr types
    attr_types: Vec<TokenStream>,
    /// collection of el types
    el_types: Vec<TokenStream>,
    /// collection of leaf node types.
    leaf_types: Vec<TokenStream>,
}

impl CodeGen {
    /// Consume self and generate rust codes.
    pub fn generate(mut self, opcodes: &[Opcode]) -> TokenStream {
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

    pub(super) fn gen_el_definition(&self) -> TokenStream {
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
