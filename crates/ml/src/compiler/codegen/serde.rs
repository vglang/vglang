use proc_macro2::TokenStream;
use quote::quote;

use crate::compiler::ir::Stat;

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
    pub fn gen(self, _: &[Stat]) -> TokenStream {
        let impls = vec![self.gen_skeleton_codes()];
        quote! {
            #(#impls)*
        }
    }

    fn gen_skeleton_codes(&self) -> TokenStream {
        quote! {}
    }
}
