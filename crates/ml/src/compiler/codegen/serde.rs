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

        quote! {
            impl ml::rt::serde::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::Serializer
                {
                    use ml::rt::serde::SerializeNode;
                    let mut serializer = serializer.#serialize_fn(#idx,#name)?;
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
        idx: usize,
    ) -> TokenStream {
        let mut stats = vec![];

        for node in self.fields.iter() {
            let ident = node.gen_ident();
            let name = ident.to_string();

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

            let body = node.gen_body_expr(fields);

            let stat = quote! {
                Self::#ident #body => {
                    let mut serializer = serializer.serialize_field(#name)?;
                    #(#node_stats)*
                    Ok(())
                }
            };

            stats.push(stat);
        }

        let ident = self.gen_ident();
        let name = ident.to_string();

        quote! {
            impl ml::rt::serde::Serialize for #opcode_mod #ident {
                fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where
                    S: ml::rt::serde::Serializer
                {
                    use ml::rt::serde::SerializeNode;
                    use ml::rt::serde::SerializeEnum;
                    let mut serializer = serializer.#serialize_fn(#idx,#name)?;
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
        let mut impls: Vec<TokenStream> = vec![];

        for (idx, stat) in stats.iter().enumerate() {
            match stat {
                Stat::Element(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_el },
                        idx,
                    ));
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_leaf },
                        idx,
                    ));
                }
                Stat::Attr(node) => {
                    impls.push(node.gen_serialize_trait(
                        &self.opcode_mod,
                        quote! { serialize_attr },
                        idx,
                    ));
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
        }
    }
}
