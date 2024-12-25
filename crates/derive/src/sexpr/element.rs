use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, ItemStruct};

syn::custom_keyword!(boxed);
pub(super) struct Element {
    pub boxed: bool,
    pub content_of: Vec<syn::Ident>,
}

impl Parse for Element {
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

        Ok(Self { content_of, boxed })
    }
}

pub(super) fn derive_content_of(content_of: &[syn::Ident], item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let content_of = content_of
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
