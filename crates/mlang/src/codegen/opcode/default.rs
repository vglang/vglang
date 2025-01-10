use proc_macro2::TokenStream;
use quote::quote;

use super::{OpcodeEnumCodeGen, OpcodeFieldGen, OpcodeNodeCodeGen};

/// Sexpr trait for nodes.
pub(super) trait SexprDefault {
    fn gen_sexpr_default_fns(&self) -> TokenStream;
}

impl SexprDefault for OpcodeNodeCodeGen {
    fn gen_sexpr_default_fns(&self) -> TokenStream {
        let ident = &self.ident;

        let assign_token_streams = self
            .fields
            .iter()
            .map(|field| field.gen_sexpr_default_assign())
            .collect::<Vec<_>>();

        let where_clause = self
            .fields
            .iter()
            .filter(|field| !field.attrs.option)
            .map(|field| field.gen_sexpr_default_fns_where_clause())
            .collect::<Vec<_>>();

        if self.tuple {
            quote! {
                impl Default for #ident where #(#where_clause),* {
                    fn default() -> Self {
                        Self ( #(#assign_token_streams),* )
                    }
                }
            }
        } else {
            quote! {
                impl Default for #ident where #(#where_clause),* {
                    fn default() -> Self {
                        Self { #(#assign_token_streams),* }
                    }
                }
            }
        }
    }
}

impl OpcodeFieldGen {
    fn gen_sexpr_default_fns_where_clause(&self) -> TokenStream {
        assert!(
            !self.attrs.option,
            "only non-option field can call this fn."
        );

        let ty = TokenStream::from(self.ty.clone());

        quote! { #ty: Default }
    }

    fn gen_sexpr_default_assign(&self) -> TokenStream {
        if let Some(ident) = &self.ident {
            quote! { #ident: Default::default() }
        } else {
            quote! { Default::default() }
        }
    }
}

impl SexprDefault for OpcodeEnumCodeGen {
    fn gen_sexpr_default_fns(&self) -> TokenStream {
        quote! {}
    }
}
