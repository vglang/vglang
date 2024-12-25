use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::sexpr::stream::derive_stream_api;

use super::graphics::{derive_content_of, Graphics};

pub fn derive_container(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let Graphics {
        boxed,
        targets: content_of,
    } = parse_macro_input!(attr as Graphics);

    let item = parse_macro_input!(item as ItemStruct);

    let stream_apis = derive_stream_api(&item);

    let apply = derive_apply(&item);

    let children = derive_children(&item);

    let graphics = derive_graphics(boxed, &item);

    let content_of = derive_content_of(&content_of, &item);

    quote! {

        #item

        #stream_apis

        #graphics

        #content_of

        #apply

        #children
    }
    .into()
}

fn derive_apply(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause
        {
            pub fn apply<A>(self, attrs: A) -> crate::sexpr::ApplyContainer<A,#ident #ty_generics>
            where
                A: crate::sexpr::Graphics + crate::sexpr::ApplyTo<#ident #ty_generics>,
            {
                crate::sexpr::ApplyContainer {
                    attrs,
                    container: self,
                }
            }
        }
    }
}

fn derive_children(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause
        {
            pub fn children<C>(self, children: C) -> crate::sexpr::ContainerChildren<#ident #ty_generics,C>
            where
                C: crate::sexpr::Graphics,
            {
                crate::sexpr::ContainerChildren {
                    container: self,
                    children
                }
            }
        }
    }
}

fn derive_graphics(boxed: bool, item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    if boxed {
        quote! {
            impl #impl_generics crate::sexpr::Graphics for #ident #ty_generics #where_clause
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::el::Container::#ident(Box::new(self)))
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics crate::sexpr::Graphics for #ident #ty_generics #where_clause
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    builder.push(crate::opcode::el::Container::#ident(self))
                }
            }
        }
    }
}
