#[cfg(feature = "sexpr")]
mod sexpr;

#[cfg(feature = "sexpr")]
#[proc_macro_attribute]
pub fn shape_element(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    sexpr::derive_shape(attr, item)
}

#[cfg(feature = "sexpr")]
#[proc_macro_attribute]
pub fn container_element(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    sexpr::derive_container(attr, item)
}

#[cfg(feature = "sexpr")]
#[proc_macro_derive(Data)]
pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sexpr::derive_data(item)
}
