use proc_macro2::TokenStream;
use quote::quote;

use super::{OpcodeEnumCodeGen, OpcodeFieldGen, OpcodeNodeCodeGen};

/// Sexpr trait for nodes.
pub(super) trait SexprDefault {
    fn gen_sexpr_default_fns(&self) -> TokenStream;

    fn gen_sexpr_from_fns(&self) -> TokenStream;
}

impl SexprDefault for OpcodeNodeCodeGen {
    fn gen_sexpr_default_fns(&self) -> TokenStream {
        if self.fields.is_empty()
            || self.fields.len()
                != self
                    .fields
                    .iter()
                    .filter(|field| field.attrs.option)
                    .count()
        {
            return quote! {};
        }

        let ident = &self.ident;

        let assign_token_streams = self
            .fields
            .iter()
            .map(|field| field.gen_sexpr_default_assign())
            .collect::<Vec<_>>();

        if self.tuple {
            quote! {
                impl Default for #ident {
                    fn default() -> Self {
                        Self ( #(#assign_token_streams),* )
                    }
                }
            }
        } else {
            quote! {
                impl Default for #ident  {
                    fn default() -> Self {
                        Self { #(#assign_token_streams),* }
                    }
                }
            }
        }
    }

    fn gen_sexpr_from_fns(&self) -> TokenStream {
        if self.tuple
            || self.fields.is_empty()
            || self.fields.len()
                != self
                    .fields
                    .iter()
                    .filter(|field| field.attrs.option)
                    .count()
        {
            return quote! {};
        }

        let ident = &self.ident;

        let mut from_fns = vec![];

        for field in self.fields.iter() {
            let assign = field.gen_sexpr_from_assign();

            let where_clause = field.ty.gen_sexpr_from_where_clause(quote! { T });

            let fn_name: TokenStream = format!("from_{}", field.ident.as_ref().unwrap())
                .parse()
                .unwrap();

            from_fns.push(quote! {
                impl #ident {
                    pub fn #fn_name<T>(value: T) -> Self where #where_clause {
                        Self {
                            #assign,
                            ..Default::default()
                        }
                    }
                }
            });
        }

        quote! {
            #(#from_fns)*
        }
    }
}

impl OpcodeFieldGen {
    fn gen_sexpr_default_assign(&self) -> TokenStream {
        if let Some(ident) = &self.ident {
            quote! { #ident: Default::default() }
        } else {
            quote! { Default::default() }
        }
    }

    fn gen_sexpr_from_assign(&self) -> TokenStream {
        let mut assign = self.ty.gen_sexpr_from_assign(quote! { value });

        if self.attrs.variable {
            assign = quote! {
                super::variable::Variable::Constant(#assign)
            }
        }

        if self.attrs.option {
            assign = quote! {
                Some(#assign)
            }
        }

        if let Some(ident) = &self.ident {
            quote! { #ident: #assign }
        } else {
            quote! { #assign }
        }
    }
}

impl SexprDefault for OpcodeEnumCodeGen {
    fn gen_sexpr_default_fns(&self) -> TokenStream {
        quote! {}
    }

    fn gen_sexpr_from_fns(&self) -> TokenStream {
        quote! {}
    }
}
