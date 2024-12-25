use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, ItemStruct};

syn::custom_keyword!(boxed);
pub(super) struct Graphics {
    pub boxed: bool,
    pub targets: Vec<syn::Ident>,
}

impl Parse for Graphics {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let boxed = if input.parse::<Option<boxed>>()?.is_some() {
            input.parse::<Option<syn::Token![,]>>()?;
            true
        } else {
            false
        };

        let mut content_of = vec![];

        while let Some(target) = input.parse()? {
            content_of.push(target);

            if input.parse::<Option<syn::Token![,]>>()?.is_none() {
                break;
            }
        }

        Ok(Self {
            targets: content_of,
            boxed,
        })
    }
}

pub(super) fn derive_content_of(targets: &[syn::Ident], item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let content_of = targets
        .iter()
        .map(|parent| {
            quote! {
                impl #impl_generics crate::sexpr::ContentOf<#parent> for #ident #ty_generics #where_clause {}
            }
        })
        .collect::<Vec<_>>();

    quote! {

        impl crate::sexpr::Element for #ident {}

        #(#content_of)*
    }
}

pub(super) fn derive_apply_to(content_of: &[syn::Ident], item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let apply_to = content_of
        .iter()
        .map(|parent| {
            quote! {
                impl #impl_generics crate::sexpr::ApplyTo<#parent> for #ident #ty_generics #where_clause{}
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #(#apply_to)*
    }
}
