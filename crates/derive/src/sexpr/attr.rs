use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::sexpr::{graphics::Graphics, stream::derive_stream_api};

use super::graphics::derive_apply_to;

pub fn derive_attr(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let Graphics {
        boxed,
        targets: content_of,
    } = parse_macro_input!(attr as Graphics);

    let item = parse_macro_input!(item as ItemStruct);

    let stream_apis = derive_stream_api(&item);

    let graphics = derive_graphics(boxed, &item);

    let content_of = derive_apply_to(&content_of, &item);

    quote! {

        #item

        #stream_apis

        #graphics

        #content_of
    }
    .into()
}

fn derive_graphics(boxed: bool, item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    if boxed {
        quote! {
            impl crate::sexpr::Graphics for #ident
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::attrs::Attr::#ident(Box::new(self)))
                }
            }
        }
    } else {
        quote! {
            impl crate::sexpr::Graphics for #ident
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::attrs::Attr::#ident(self))
                }
            }
        }
    }
}
