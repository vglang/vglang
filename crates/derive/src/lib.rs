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

#[cfg(feature = "literal")]
#[proc_macro]
pub fn rgb(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::quote;

    let lit = item.to_string();

    if !lit.starts_with("#") {
        panic!("hexadecimal rgb value must start with '#'");
    }

    let lit = &lit[1..];

    match lit.len() {
        3 => {
            let r = u8::from_str_radix(&format!("{}{}", &lit[0..1], &lit[0..1]), 16)
                .expect("Invalid rgb hexadecimal value");
            let g = u8::from_str_radix(&format!("{}{}", &lit[1..2], &lit[1..2]), 16)
                .expect("Invalid rgb hexadecimal value");
            let b = u8::from_str_radix(&format!("{}{}", &lit[2..3], &lit[2..3]), 16)
                .expect("Invalid rgb hexadecimal value");

            return quote! {
                vglang::opcode::data::Rgb::rgb(#r,#g,#b)
            }
            .into();
        }
        6 => {
            let r = u8::from_str_radix(&lit[0..2], 16).expect("Invalid rgb hexadecimal value");
            let g = u8::from_str_radix(&lit[2..4], 16).expect("Invalid rgb hexadecimal value");
            let b = u8::from_str_radix(&lit[4..6], 16).expect("Invalid rgb hexadecimal value");

            return quote! {
                vglang::opcode::data::Rgb::rgb(#r,#g,#b)
            }
            .into();
        }
        _ => {
            panic!("only support three-digit RGB notation or six-digit RGB notation.");
        }
    }
}
