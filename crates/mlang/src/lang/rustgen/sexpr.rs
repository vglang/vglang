use proc_macro2::TokenStream;
use quote::quote;

use crate::lang::ir::Stat;

/// Generate sexpr module from [`stats`](Stat).
pub fn gen_sexpr_mod(_stats: impl AsRef<[Stat]>) -> TokenStream {
    quote! {}
}
