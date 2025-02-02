use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::{
    codegen::ext::{EnumGen, FieldGen, NodeGen},
    ir::{Enum, Node, Stat},
};

trait SerializeGen {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        idx: usize,
    ) -> TokenStream;
}

impl SerializeGen for Node {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        idx: usize,
    ) -> TokenStream {
        let ident = self.gen_ident();
        let name = self.gen_display_name();

        let mut stats = vec![];

        for (idx, field) in self.fields.iter().enumerate() {
            let value = if let Some(ident) = field.gen_ident() {
                quote! {self.#ident}
            } else {
                format!("self.{}", idx).parse::<TokenStream>().unwrap()
            };

            let name = if let Some(name) = field.gen_display_name() {
                quote! { Some(#name) }
            } else {
                quote! { None }
            };

            stats.push(quote! {
                serializer.serialize_field(#idx, #name, &#value)?
            });
        }

        let fields = stats.len();

        quote! {
            impl ml::rt::serde::ser::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::ser::Serializer
                {
                    use ml::rt::serde::ser::SerializeNode;
                    let mut serializer = serializer.#serialize_fn(#idx, #name, #fields)?;
                    #(#stats;)*

                    serializer.finish()
                }
            }
        }
    }
}

impl SerializeGen for Enum {
    fn gen_serialize_trait(
        &self,
        opcode_mod: &TokenStream,
        serialize_fn: TokenStream,
        type_id: usize,
    ) -> TokenStream {
        let mut stats = vec![];
        let enum_name = self.gen_display_name();

        for (idx, node) in self.fields.iter().enumerate() {
            let ident = node.gen_ident();

            let mut node_stats = vec![];
            let mut fields = vec![];

            for (idx, field) in node.fields.iter().enumerate() {
                if let Some(ident) = field.gen_ident() {
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

            let body = node.gen_body_expr(fields);

            let variant = ident.to_string();

            let stat = quote! {
                Self::#ident #body => {
                    let mut serializer = serializer.#serialize_fn(#type_id, #enum_name, #variant, #idx, #field_count)?;
                    #(#node_stats)*
                    serializer.finish()
                }
            };

            stats.push(stat);
        }

        let ident = self.gen_ident();

        quote! {
            impl ml::rt::serde::ser::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::ser::Serializer
                {
                    use ml::rt::serde::ser::SerializeNode;
                    match self {
                        #(#stats),*
                    }
                }
            }
        }
    }
}

trait DeserializeGen {
    fn gen_deserialize_trait(&self, opcode_mod: &TokenStream, idx: usize) -> TokenStream;
}

#[allow(unused)]
impl DeserializeGen for Node {
    fn gen_deserialize_trait(&self, opcode_mod: &TokenStream, idx: usize) -> TokenStream {
        let ident = self.gen_ident();

        let visitor_ident = format!("{}Visitor", self.ident.1)
            .parse::<TokenStream>()
            .unwrap();

        let mut fields = vec![];

        for field in self.fields.iter() {
            let ty = field.gen_type_definition(opcode_mod);

            fields.push(field.gen_definition(quote! {}, Some(quote! { Option<#ty> })));
        }

        let body = self.gen_body_expr(fields);

        quote! {
            impl<'de> ml::rt::serde::de::Deserialize<'de> for #opcode_mod #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ml::rt::serde::de::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
                        type Value = #opcode_mod #ident;
                    }

                    todo!()
                }
            }
        }
    }
}

#[allow(unused)]
impl DeserializeGen for Enum {
    fn gen_deserialize_trait(&self, opcode_mod: &TokenStream, idx: usize) -> TokenStream {
        quote! {}
    }
}

/// `serde.rs` mod generator.
pub struct SerdeModGen {
    #[allow(unused)]
    opcode_mod: TokenStream,
}

impl SerdeModGen {
    /// Create new sexpr mode generator
    pub fn new<S>(opcode_mod: S) -> Self
    where
        String: From<S>,
    {
        Self {
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
        }
    }

    /// Generate sexpr mod
    pub fn gen(self, stats: &[Stat]) -> TokenStream {
        let opcode_mod = &self.opcode_mod;

        let mut impls: Vec<TokenStream> = vec![];

        let mut sers = vec![];
        let mut leaves = vec![];
        let mut attrs = vec![];

        for (idx, stat) in stats.iter().enumerate() {
            match stat {
                Stat::Element(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_el },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_trait(&self.opcode_mod, idx));

                    let ident = node.gen_ident();

                    sers.push(quote! {
                        #opcode_mod Element::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_leaf },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_trait(&self.opcode_mod, idx));

                    let ident = node.gen_ident();

                    leaves.push(quote! {
                        #opcode_mod Leaf::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Attr(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_attr },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_trait(&self.opcode_mod, idx));

                    let ident = node.gen_ident();

                    attrs.push(quote! {
                        #opcode_mod Attr::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Data(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_data },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_trait(&self.opcode_mod, idx));
                }
                Stat::Enum(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_enum },
                        idx,
                    ));

                    impls.push(node.gen_deserialize_trait(&self.opcode_mod, idx));
                }
                _ => {}
            }
        }
        quote! {
            #(#impls)*

            impl ml::rt::serde::ser::Serialize for #opcode_mod Opcode {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::ser::Serializer
                {
                    match self {
                        Self::Apply(v) => {
                            match v {
                                #(#attrs),*
                            }
                        },
                        Self::Element(v) => {
                            match v {
                                #(#sers),*
                            }
                        },
                        Self::Leaf(v) => {
                            match v {
                                #(#leaves),*
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
