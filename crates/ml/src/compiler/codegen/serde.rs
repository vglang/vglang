use proc_macro2::TokenStream;
use quote::quote;

use crate::compiler::{
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
        let name = ident.to_string();

        let mut stats = vec![];

        for (idx, field) in self.fields.iter().enumerate() {
            if let Some(ident) = field.gen_ident() {
                let name = ident.to_string();
                stats.push(quote! {
                    serializer.serialize_field(#idx, Some(#name), &self.#ident)?
                });
            } else {
                let ident = format!("self.{}", idx).parse::<TokenStream>().unwrap();
                stats.push(quote! {
                    serializer.serialize_field(#idx, None, &#ident)?
                });
            }
        }

        let fields = stats.len();

        quote! {
            impl ml::rt::serde::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::Serializer
                {
                    use ml::rt::serde::SerializeNode;
                    let mut serializer = serializer.#serialize_fn(#idx, #name, #fields)?;
                    #(#stats;)*

                    Ok(())
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
        let enum_name = self.gen_ident().to_string();

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
                    Ok(())
                }
            };

            stats.push(stat);
        }

        let ident = self.gen_ident();

        quote! {
            impl ml::rt::serde::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::Serializer
                {
                    use ml::rt::serde::SerializeNode;
                    match self {
                        #(#stats),*
                    }
                }
            }
        }
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

        let mut els = vec![];
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

                    let ident = node.gen_ident();

                    els.push(quote! {
                        #opcode_mod Element::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_leaf },
                        idx,
                    ));

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
                }
                Stat::Enum(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_enum },
                        idx,
                    ));
                }
                _ => {}
            }
        }
        quote! {
            #(#impls)*

            impl ml::rt::serde::Serialize for #opcode_mod Opcode {
                fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::Serializer
                {
                    match self {
                        Self::Apply(v) => {
                            match v {
                                #(#attrs),*
                            }
                        },
                        Self::Element(v) => {
                            match v {
                                #(#els),*
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
