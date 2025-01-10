use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::FieldType;

impl FieldType {
    pub(super) fn gen_sexpr_from_assign(&self, this: TokenStream) -> TokenStream {
        match self {
            FieldType::Noraml(token_stream) => match token_stream.to_string().as_str() {
                "f32" => {
                    quote! { super::sexpr::Number::from(#this).0 }
                }
                _ => quote! {
                    #this.into()
                },
            },
            FieldType::List(_) => {
                quote! {
                    #this.map_collect()
                }
            }
            FieldType::Array(_, _) => {
                quote! {
                    #this.into()
                }
            }
        }
    }

    pub(super) fn gen_sexpr_from_where_clause(&self, this: TokenStream) -> TokenStream {
        match self {
            FieldType::Noraml(token_stream) => match token_stream.to_string().as_str() {
                "f32" => {
                    quote! { super::sexpr::Number: From<#this> }
                }
                _ => quote! {
                    #token_stream: From<#this>
                },
            },
            FieldType::List(token_stream) => {
                quote! {
                    #this: super::sexpr::MapCollect<#token_stream>
                }
            }
            FieldType::Array(token_stream, num) => {
                quote! {
                    [#token_stream;#num]: From<#this>
                }
            }
        }
    }
}
