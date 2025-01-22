use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::Stat;

/// `serde.rs` mod generator.
pub struct BinaryModGen {
    #[allow(unused)]
    opcode_mod: TokenStream,
}

impl BinaryModGen {
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
        quote! {}
    }
}
