use proc_macro2::TokenStream;
use quote::quote;

use crate::{codegen::opcode::TypeGen, opcode::Field};

pub(super) trait FieldGen {
    fn gen_definition(&self, vis: &TokenStream) -> TokenStream;

    fn gen_indent(&self) -> Option<TokenStream>;
}

impl<'a> FieldGen for Field<'a> {
    fn gen_definition(&self, vis: &TokenStream) -> TokenStream {
        let ident = if let Some(ident) = self.gen_indent() {
            format!("{}:", ident).parse().unwrap()
        } else {
            quote! {}
        };

        let ty = self.ty().gen_definition();

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
