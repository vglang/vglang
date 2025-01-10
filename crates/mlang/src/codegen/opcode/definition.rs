use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{OpcodeEnumCodeGen, OpcodeFieldGen, OpcodeNodeCodeGen};

pub(super) trait TypeDefinition {
    /// Generate type definition codes.
    fn gen_type_definition(&self) -> TokenStream;
}

impl TypeDefinition for OpcodeNodeCodeGen {
    fn gen_type_definition(&self) -> TokenStream {
        let ident = &self.ident;
        let comments = &self.comments;

        if self.fields.is_empty() {
            quote! {
                #comments
                #[derive(Debug, Clone, PartialEq, PartialOrd)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct #ident;
            }
        } else {
            let fields = self
                .fields
                .iter()
                .map(|field| field.gen_type_definition())
                .collect::<Vec<_>>();

            if self.tuple {
                quote! {
                    #comments
                    #[derive(Debug, Clone, PartialEq, PartialOrd)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #ident(#(#fields),*);
                }
            } else {
                quote! {
                    #comments
                    #[derive(Debug, Clone, PartialEq, PartialOrd)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #ident { #(#fields),* }
                }
            }
        }
    }
}

impl OpcodeNodeCodeGen {
    fn gen_enum_field_definition(&self) -> TokenStream {
        let ident = &self.ident;
        let comments = &self.comments;

        if self.fields.is_empty() {
            quote! {
                #comments
                #ident
            }
        } else {
            let fields = self
                .fields
                .iter()
                .map(|field| field.gen_type_definition())
                .collect::<Vec<_>>();

            if self.tuple {
                quote! {
                    #comments
                    #ident(#(#fields),*)
                }
            } else {
                quote! {
                    #comments
                    #ident { #(#fields),* }
                }
            }
        }
    }
}

impl TypeDefinition for OpcodeFieldGen {
    fn gen_type_definition(&self) -> TokenStream {
        let ident = if let Some(ident) = &self.ident {
            quote! { #ident: }
        } else {
            quote! {}
        };

        let mut ty = TokenStream::from(self.ty.clone());

        if self.attrs.variable {
            ty = quote! { super::variable::Variable<#ty> };
        }

        if self.attrs.option {
            ty = quote! { Option<#ty> };
        }

        quote! { #ident #ty }
    }
}

impl TypeDefinition for OpcodeEnumCodeGen {
    fn gen_type_definition(&self) -> TokenStream {
        let ident = &self.ident;
        let comments = &self.comments;

        let fields = self
            .fields
            .iter()
            .map(|field| field.gen_enum_field_definition())
            .collect::<Vec<_>>();

        quote! {
            #comments
            #[derive(Debug, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #ident {
                #(#fields),*
            }
        }
    }
}

pub(super) fn gen_data_definition(data_fields: &[TokenStream]) -> TokenStream {
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

    for ident in data_fields {
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

pub(super) fn gen_el_definition(el_fileds: &[TokenStream]) -> TokenStream {
    let mut fields = vec![];
    let mut froms = vec![];

    for el in el_fileds {
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

pub(super) fn gen_leaf_definition(leaf_fields: &[TokenStream]) -> TokenStream {
    let mut fields = vec![];
    let mut froms = vec![];

    for el in leaf_fields {
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

pub(super) fn gen_attr_definition(attr_fileds: &[TokenStream]) -> TokenStream {
    let mut fields = vec![];
    let mut froms = vec![];

    for el in attr_fileds {
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

pub(super) fn gen_opcode_definition() -> TokenStream {
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
