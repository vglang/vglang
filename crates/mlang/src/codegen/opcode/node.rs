use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::opcode::{Enum, Node};

use super::{CommentGen, FieldGen};

/// A rust enum/struct code generator for `mlang`.
pub trait NodeGen {
    /// Generate field token streams.
    fn gen_fields_definition(&self, vis: TokenStream) -> Vec<TokenStream>;

    /// Generate type name [`TokenStream`].
    fn gen_ident(&self) -> TokenStream;

    /// Generate type definition codes.
    fn gen_definition(&self) -> TokenStream;
}

impl NodeGen for Node {
    fn gen_fields_definition(&self, vis: TokenStream) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| field.gen_definition(&vis))
            .collect()
    }

    fn gen_ident(&self) -> TokenStream {
        self.ident.0.to_upper_camel_case().parse().unwrap()
    }

    fn gen_definition(&self) -> TokenStream {
        let ident = self.gen_ident();
        let fields = self.gen_fields_definition(quote! {pub});

        let comments = self
            .comments
            .iter()
            .map(|comment| comment.gen_definition())
            .collect::<Vec<_>>();

        if self.is_tuple() {
            quote! {
                #(#comments)*
                #[derive(Debug, Clone, PartialEq, PartialOrd)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct #ident(#(#fields),*);
            }
        } else {
            quote! {
                #(#comments)*
                #[derive(Debug, Clone, PartialEq, PartialOrd)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct #ident {#(#fields),*}
            }
        }
    }
}

impl NodeGen for Enum {
    fn gen_fields_definition(&self, _vis: TokenStream) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|node| {
                let ident = node.gen_ident();
                let fields = node.gen_fields_definition(quote! {});

                let comments = node
                    .comments
                    .iter()
                    .map(|comment| comment.gen_definition())
                    .collect::<Vec<_>>();

                if node.is_tuple() {
                    quote! {
                        #(#comments)*
                        #ident(#(#fields),*)
                    }
                } else {
                    quote! {
                        #(#comments)*
                        #ident {#(#fields),*}
                    }
                }
            })
            .collect()
    }

    fn gen_ident(&self) -> TokenStream {
        self.ident.0.to_upper_camel_case().parse().unwrap()
    }

    fn gen_definition(&self) -> TokenStream {
        let fields = self.gen_fields_definition(quote! {});

        let ident = self.gen_ident();

        let comments = self
            .comments
            .iter()
            .map(|comment| comment.gen_definition())
            .collect::<Vec<_>>();

        quote! {
            #(#comments)*
            #[derive(Debug, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #ident {#(#fields),*}
        }
    }
}
