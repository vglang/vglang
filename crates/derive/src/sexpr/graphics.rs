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

        loop {
            content_of.push(input.parse()?);

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

    let content_of = targets
        .iter()
        .map(|parent| {
            quote! {
                impl crate::sexpr::ContentOf<#parent> for #ident {}
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

    let apply_to = content_of
        .iter()
        .map(|parent| {
            quote! {
                impl crate::sexpr::ApplyTo<#parent> for #ident {}
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #(#apply_to)*
    }
}
