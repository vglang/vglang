use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::sexpr::stream::derive_stream_api;

use super::graphics::{derive_content_of, Graphics};

pub fn derive_shape(
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

    let content_of = derive_content_of(&content_of, &item);

    let apply = derive_apply(&item);

    quote! {

        #item

        #stream_apis

        #graphics

        #content_of

        #apply
    }
    .into()
}

fn derive_apply(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    quote! {
        impl #ident
        {
            pub fn apply<A>(self, attrs: A) -> crate::sexpr::ApplyContainer<A,#ident>
            where
                A: crate::sexpr::Graphics + crate::sexpr::ApplyTo<#ident>,
            {
                crate::sexpr::ApplyContainer {
                    attrs,
                    container: self,
                }
            }
        }
    }
}

fn derive_graphics(boxed: bool, item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    if boxed {
        quote! {
            impl crate::sexpr::Graphics for #ident
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::el::Shape::#ident(Box::new(self)))
                }
            }
        }
    } else {
        quote! {
            impl crate::sexpr::Graphics for #ident
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::el::Shape::#ident(self))
                }
            }
        }
    }
}
