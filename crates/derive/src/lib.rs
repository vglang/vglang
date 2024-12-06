#[cfg(feature = "dsl")]
mod dsl;

use proc_macro::TokenStream;

#[cfg(feature = "dsl")]
#[proc_macro_derive(Dsl)]
pub fn derive_dsl_api(item: TokenStream) -> TokenStream {
    dsl::derive_api(item)
}
