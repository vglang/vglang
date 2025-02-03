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

        let mut sers = vec![];
        let mut leaves = vec![];
        let mut attrs = vec![];

        for (idx, stat) in stats.iter().enumerate() {
            match stat {
                Stat::Element(node) => {
                    impls.push(node.gen_serialize_trait(opcode_mod, quote! { serialize_el }, idx));

                    let ident = node.to_ident();

                    sers.push(quote! {
                        #opcode_mod Element::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_leaf },
                        idx,
                    ));

                    let ident = node.to_ident();

                    leaves.push(quote! {
                        #opcode_mod Leaf::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Attr(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_attr },
                        idx,
                    ));

                    let ident = node.to_ident();

                    attrs.push(quote! {
                        #opcode_mod Attr::#ident(value) => value.serialize(serializer)
                    });
                }
                Stat::Data(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_data },
                        idx,
                    ));
                }
                Stat::Enum(node) => {
                    impls.push(node.gen_serialize_trait(
                        opcode_mod,
                        quote! { serialize_enum },
                        idx,
                    ));
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

/// Generate serde module from [`stats`](Stat).
pub fn gen_serde_mod(stats: impl AsRef<[Stat]>, opcode_mod: impl AsRef<str>) -> TokenStream {
    CodeGen::new(opcode_mod).gen(stats.as_ref())
}
