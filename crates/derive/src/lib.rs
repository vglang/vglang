use proc_macro::TokenStream;

mod dsl;

#[proc_macro_derive(Dsl)]
pub fn derive_dsl_api(item: TokenStream) -> TokenStream {
    dsl::derive_api(item)
}
