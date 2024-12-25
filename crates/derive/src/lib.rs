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
#[proc_macro_attribute]
pub fn attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    sexpr::derive_attr(attr, item)
}

#[cfg(feature = "sexpr")]
#[proc_macro_derive(Data)]
pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sexpr::derive_data(item)
}

#[cfg(feature = "sexpr")]
#[proc_macro]
pub fn make_tuple_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{parse_macro_input, LitInt};

    let max_length = parse_macro_input!(item as LitInt);

    sexpr::derive_tuple_traits(max_length.base10_parse().unwrap())
}
