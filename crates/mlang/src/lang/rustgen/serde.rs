use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::{
    ir::{Enum, Field, Node, Stat},
    rustgen::mapping::{ComplexTypeMapping, FieldMapping},
};

trait SerdeDisplayName {
    fn display_name(&self) -> Option<&str>;
}

impl SerdeDisplayName for Node {
    fn display_name(&self) -> Option<&str> {
        if let Some(v) = self.rename() {
            Some(v)
        } else {
            Some(self.ident.1.as_str())
        }
    }
}

impl SerdeDisplayName for Enum {
    fn display_name(&self) -> Option<&str> {
        if let Some(v) = self.rename() {
            Some(v)
        } else {
            Some(self.ident.1.as_str())
        }
    }
}

impl<'a> SerdeDisplayName for Field<'a> {
    fn display_name(&self) -> Option<&str> {
        if let Some(v) = self.rename() {
            Some(v)
        } else {
            self.ident().map(|v| v.1.as_str())
        }
    }
}

trait SerializeCodeGen {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        idx: usize,
    ) -> TokenStream;
}

impl SerializeCodeGen for Node {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        idx: usize,
    ) -> TokenStream {
        let ident = self.to_ident();
        let name = self.display_name().unwrap();

        let mut stats = vec![];

        for (idx, field) in self.fields.iter().enumerate() {
            let value = if let Some(ident) = field.to_ident() {
                quote! {self.#ident}
            } else {
                format!("self.{}", idx).parse::<TokenStream>().unwrap()
            };

            let name = if let Some(name) = field.display_name() {
                quote! { Some(#name) }
            } else {
                quote! { None }
            };

            stats.push(quote! {
                serializer.serialize_field(#idx, #name, &#value)?
            });
        }

        let fields = stats.len();

        let mut_token = if stats.is_empty() {
            quote! {}
        } else {
            quote! {mut}
        };

        quote! {
            impl mlang::rt::serde::ser::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: mlang::rt::serde::ser::Serializer
                {
                    use mlang::rt::serde::ser::SerializeNode;
                    let #mut_token serializer = serializer.#serialize_fn(#idx, #name, #fields)?;
                    #(#stats;)*

                    serializer.finish()
                }
            }
        }
    }
}

impl SerializeCodeGen for Enum {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        type_id: usize,
    ) -> TokenStream {
        let mut stats = vec![];
        let enum_name = self.display_name().unwrap();

        for (idx, node) in self.fields.iter().enumerate() {
            let ident = node.to_ident();

            let mut node_stats = vec![];
            let mut fields = vec![];

            for (idx, field) in node.fields.iter().enumerate() {
                if let Some(ident) = field.to_ident() {
                    let name = ident.to_string();
                    node_stats.push(quote! {
                        serializer.serialize_field(#idx, Some(#name), #ident)?;
                    });
                    fields.push(ident);
                } else {
                    let ident = format!("p{}", idx).parse::<TokenStream>().unwrap();
                    node_stats.push(quote! {
                        serializer.serialize_field(#idx, None, #ident)?;
                    });
                    fields.push(ident);
                }
            }

            let field_count = fields.len();

            let body = node.to_struct_body(fields);

            let variant = ident.to_string();

            let mut_token = if node_stats.is_empty() {
                quote! {}
            } else {
                quote! {mut}
            };

            let stat = quote! {
                Self::#ident #body => {
                    let #mut_token serializer = serializer.#serialize_fn(#type_id, #enum_name, #variant, #idx, #field_count)?;
                    #(#node_stats)*
                    serializer.finish()
                }
            };

            stats.push(stat);
        }

        let ident = self.to_ident();

        quote! {
            impl mlang::rt::serde::ser::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: mlang::rt::serde::ser::Serializer
                {
                    use mlang::rt::serde::ser::SerializeNode;
                    match self {
                        #(#stats),*
                    }
                }
            }
        }
    }
}

trait DeserializeElCodeGen {
    fn gen_deserialize_el(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream;
}

impl DeserializeElCodeGen for Node {
    fn gen_deserialize_el(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream {
        let _ = opcode_mod;
        let _ = type_id;

        let ident = self.to_ident();

        quote! {
            impl<'de> mlang::rt::serde::de::DeserializeElement<'de> for #opcode_mod #ident {

                type Value = Self;

                fn deserialize<D>(deserializer: D) -> Result<<Self as mlang::rt::serde::de::DeserializeElement<'de>>::Value, D::Error>
                where
                    D: mlang::rt::serde::de::ElementAccess<'de>
                {
                    let _ = deserializer;

                    todo!()
                }
            }
        }
    }
}

trait DeserializeLeafCodeGen {
    fn gen_deserialize_leaf(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream;
}

impl DeserializeLeafCodeGen for Node {
    fn gen_deserialize_leaf(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream {
        let _ = opcode_mod;
        let _ = type_id;

        let ident = self.to_ident();

        quote! {
            impl<'de> mlang::rt::serde::de::DeserializeLeaf<'de> for #opcode_mod #ident {

                type Value = Self;

                fn deserialize<D>(deserializer: D) -> Result<<Self as mlang::rt::serde::de::DeserializeLeaf<'de>>::Value, D::Error>
                where
                    D: mlang::rt::serde::de::LeafAccess<'de>
                {
                    let _ = deserializer;

                    todo!()
                }
            }
        }
    }
}

trait DeserializeAttrCodeGen {
    fn gen_deserialize_attr(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream;
}

impl DeserializeAttrCodeGen for Node {
    fn gen_deserialize_attr(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream {
        let _ = opcode_mod;
        let _ = type_id;

        let ident = self.to_ident();

        let mut fields = vec![];

        for field in self.fields.iter() {
            let param = field.to_type_definition(opcode_mod);

            fields.push(field.to_definition_clause(&quote! {}, &quote! { Option<#param> }));
        }

        let body = self.to_struct_body(fields);
        let semi_token = self.to_semi_token();

        quote! {
            impl<'de> mlang::rt::serde::de::DeserializeAttr<'de> for #opcode_mod #ident {

                type Value = Self;

                fn deserialize<D>(deserializer: D) -> Result<<Self as mlang::rt::serde::de::DeserializeAttr<'de>>::Value, D::Error>
                where
                    D: mlang::rt::serde::de::AttrAccess<'de>
                {
                    let _ = deserializer;

                    #[allow(unused)]
                    struct Visitor #body #semi_token

                    todo!()
                }
            }
        }
    }
}

trait DeserializeDataCodeGen {
    fn gen_deserialize_data(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream;
}

impl DeserializeDataCodeGen for Node {
    fn gen_deserialize_data(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream {
        let _ = opcode_mod;
        let _ = type_id;

        let ident = self.to_ident();

        quote! {
            impl<'de> mlang::rt::serde::de::DeserializeData<'de> for #opcode_mod #ident {

                type Value = Self;

                fn deserialize<D>(deserializer: D) -> Result<<Self as mlang::rt::serde::de::DeserializeData<'de>>::Value, D::Error>
                where
                    D: mlang::rt::serde::de::DataAccess<'de>
                {
                    let _ = deserializer;

                    todo!()
                }
            }
        }
    }
}

impl DeserializeDataCodeGen for Enum {
    fn gen_deserialize_data(&self, opcode_mod: &TokenStream, type_id: usize) -> TokenStream {
        let _ = opcode_mod;
        let _ = type_id;

        let ident = self.to_ident();

        quote! {
            impl<'de> mlang::rt::serde::de::DeserializeData<'de> for #opcode_mod #ident {

                type Value = Self;

                fn deserialize<D>(deserializer: D) -> Result<<Self as mlang::rt::serde::de::DeserializeData<'de>>::Value, D::Error>
                where
                    D: mlang::rt::serde::de::DataAccess<'de>
                {
                    let _ = deserializer;

                    todo!()
                }
            }
        }
    }
}

struct CodeGen(TokenStream);

impl CodeGen {
    /// Create new sexpr mode generator
    pub fn new(opcode_mod: impl AsRef<str>) -> Self {
        Self(opcode_mod.as_ref().parse().unwrap())
    }

    /// Generate sexpr mod
    pub fn gen(self, stats: &[Stat]) -> TokenStream {
        let opcode_mod = &self.0;

        let mut impls: Vec<TokenStream> = vec![];

        let mut ser_els = vec![];
        let mut ser_leaves = vec![];
        let mut ser_attrs = vec![];

        for (idx, stat) in stats.iter().enumerate() {
            match stat {
                Stat::Element(node) => {
                    impls.push(node.gen_serialize_trait(opcode_mod, quote! { serialize_el }, idx));
                    impls.push(node.gen_deserialize_el(opcode_mod, idx));

                    let ident = node.to_ident();

                    ser_els.push(quote! {
                        #opcode_mod Element::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_leaf },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_leaf(opcode_mod, idx));

                    let ident = node.to_ident();

                    ser_leaves.push(quote! {
                        #opcode_mod Leaf::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Attr(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_attr },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_attr(opcode_mod, idx));

                    let ident = node.to_ident();

                    ser_attrs.push(quote! {
                        #opcode_mod Attr::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Data(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_data },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_data(opcode_mod, idx));
                }
                Stat::Enum(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_enum },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_data(opcode_mod, idx));
                }
                _ => {}
            }
        }
        quote! {
            #(#impls)*

            impl mlang::rt::serde::ser::Serialize for #opcode_mod Opcode {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: mlang::rt::serde::ser::Serializer
                {
                    match self {
                        Self::Apply(v) => {
                            match v {
                                #(#ser_attrs),*
                            }
                        },
                        Self::Element(v) => {
                            match v {
                                #(#ser_els),*
                            }
                        },
                        Self::Leaf(v) => {
                            match v {
                                #(#ser_leaves),*
                            }
                        }
                        Self::Pop => {
                            serializer.serialize_pop()
                        },

                    }
                }
            }
        }
    }
}

/// Generate serde module from [`stats`](Stat).
pub fn gen_serde_mod(stats: impl AsRef<[Stat]>, opcode_mod: impl AsRef<str>) -> TokenStream {
    CodeGen::new(opcode_mod).gen(stats.as_ref())
}
