use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::sexpr::stream::derive_stream_api;

pub fn derive_shape(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let stream_apis = derive_stream_api(&item);

    let graphics = derive_graphics(&item);

    quote! {
        #stream_apis

        #graphics
    }
    .into()
}

fn derive_graphics(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    quote! {
        impl<B> crate::sexpr::Graphics<B> for #ident
            where B: crate::surface::Builder,
        {
            fn build(self, builder: &mut B) {

            }
        }
    }
}
