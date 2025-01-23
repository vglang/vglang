use proc_macro2::TokenStream;
use quote::quote;

use crate::compiler::ir::{Enum, Node, Stat};

trait SerializeGen {
    fn gen_serialize_trait(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SerializeGen for Node {
    fn gen_serialize_trait(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SerializeGen for Enum {
    fn gen_serialize_trait(&self, _: &TokenStream) -> TokenStream {
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
        let mut impls: Vec<TokenStream> = vec![];

        for stat in stats {
            match stat {
                Stat::Element(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                Stat::Leaf(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                Stat::Attr(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                Stat::Mixin(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                Stat::Data(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                Stat::Enum(node) => {
                    impls.push(node.gen_serialize_trait(&self.opcode_mod));
                }
                _ => {}
            }
        }
        quote! {
            #(#impls)*
        }
    }
}
