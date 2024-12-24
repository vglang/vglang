use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::sexpr::stream::derive_stream_api;

use super::element::{derive_content_of, Element};

pub fn derive_container(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let Element { boxed, content_of } = parse_macro_input!(attr as Element);

    let item = parse_macro_input!(item as ItemStruct);

    let stream_apis = derive_stream_api(&item);

    let graphics = derive_graphics(boxed, &item);

    let content_of = derive_content_of(&content_of, &item);

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
            impl<B> crate::sexpr::Graphics<B> for #ident
                where B: crate::surface::Builder,
            {
                fn build(self, builder: &mut B) {
                    builder.push(crate::opcode::el::Container::#ident(Box::new(self)).into())
                }
            }
        }
    } else {
        quote! {
            impl<B> crate::sexpr::Graphics<B> for #ident
                where B: crate::surface::Builder,
            {
                fn build(self, builder: &mut B) {
                    builder.push(crate::opcode::el::Container::#ident(self).into())
                }
            }
        }
    }
}
