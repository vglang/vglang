#[cfg(feature = "sexpr")]
mod sexpr;

#[cfg(feature = "sexpr")]
#[proc_macro_derive(Sexpr)]
pub fn derive_dsl_api(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sexpr::derive_api(item)
}
