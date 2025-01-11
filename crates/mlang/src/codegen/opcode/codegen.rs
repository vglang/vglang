use proc_macro2::TokenStream;
use quote::quote;

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
        quote! {
            #(#token_streams)*
        }
    }
}
