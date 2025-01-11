use proc_macro2::TokenStream;
use quote::quote;

use crate::opcode::{ApplyTo, ChildrenOf, Enum, Node, Opcode};

pub trait SexprGen {
    fn gen_sexpr_fns(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SexprGen for Node {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for Enum {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ApplyTo {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ChildrenOf {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

/// A generator that create `sexpr` mod for `mlang`.
pub struct SexprModGen {
    opcode_mod: TokenStream,
}

impl SexprModGen {
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
    pub fn gen(self, opcodes: &[Opcode]) -> TokenStream {
        let mut token_streams = vec![];

        for opcode in opcodes {
            match opcode {
                Opcode::Element(node)
                | Opcode::Leaf(node)
                | Opcode::Attr(node)
                | Opcode::Data(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                Opcode::Enum(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }

                Opcode::ApplyTo(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                Opcode::ChildrenOf(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                _ => {}
            }
        }

        quote! {
            #(#token_streams)*
        }
    }
}
