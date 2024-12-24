#[cfg(feature = "sexpr")]
mod sexpr;

#[cfg(feature = "sexpr")]
#[proc_macro_derive(Shape)]
pub fn derive_shape(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sexpr::derive_shape(item)
}

#[cfg(feature = "sexpr")]
#[proc_macro_derive(Data)]
pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sexpr::derive_data(item)
}
