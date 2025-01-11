use proc_macro2::TokenStream;
use quote::quote;

use crate::{codegen::opcode::TypeGen, opcode::Field};

pub trait FieldGen {
    fn gen_definition(&self, vis: &TokenStream) -> TokenStream;

    fn gen_indent(&self) -> Option<TokenStream>;

    fn is_option(&self) -> bool;
    fn is_variable(&self) -> bool;
}

impl<'a> FieldGen for Field<'a> {
    fn is_option(&self) -> bool {
        for property in self.properties() {
            for callexpr in &property.params {
                if callexpr.ident.0 == "option" {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_variable(&self) -> bool {
        for property in self.properties() {
            for callexpr in &property.params {
                if callexpr.ident.0 == "variable" {
                    return true;
                }
            }
        }

        return false;
    }

    fn gen_definition(&self, vis: &TokenStream) -> TokenStream {
        let ident = if let Some(ident) = self.gen_indent() {
            format!("{}:", ident).parse().unwrap()
        } else {
            quote! {}
        };

        let mut ty = self.ty().gen_definition();

        if self.is_variable() {
            ty = quote! { variable::Variable<#ty> };
        }

        if self.is_option() {
            ty = quote! { Option<#ty> };
        }

        quote! {
            #vis #ident #ty
        }
    }

    fn gen_indent(&self) -> Option<TokenStream> {
        if let Some(ident) = self.ident() {
            let ident = match ident.0.as_str() {
                "in" => "r#in",
                "type" => "r#type",
                ident => ident,
            };
            Some(ident.parse().unwrap())
        } else {
            None
        }
    }
}
