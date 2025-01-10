use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::FieldType;

use super::{OpcodeEnumCodeGen, OpcodeFieldGen, OpcodeNodeCodeGen};

/// Sexpr trait for nodes.
pub(super) trait Sexpr {
    fn gen_sexpr_init_fns(&self) -> TokenStream;
}

impl Sexpr for OpcodeNodeCodeGen {
    fn gen_sexpr_init_fns(&self) -> TokenStream {
        if self.fields.is_empty() {
            return quote! {};
        }

        let ident = &self.ident;

        let non_option_fields = self
            .fields
            .iter()
            .filter(|field| !field.attrs.option)
            .count();

        if non_option_fields == 0 {
            return quote! {};
        }

        let from_tuple = non_option_fields > 1;

        let mut non_option_index = 0;

        let fields = self
            .fields
            .iter()
            .map(|field| field.gen_sexpr_init_assign(&mut non_option_index, from_tuple))
            .collect::<Vec<_>>();

        let where_clause = self
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.option)
            .map(|(index, field)| field.gen_sexpr_where_clause(index))
            .collect::<Vec<_>>();

        let generics = self
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.option)
            .map(|(index, _)| format!("P{}", index).parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();

        let impl_generics = if generics.len() > 0 {
            quote! { <#(#generics),*> }
        } else {
            quote! {}
        };

        let type_generics = if generics.len() > 1 {
            quote! { (#(#generics),*) }
        } else {
            quote! { #(#generics),* }
        };

        if self.tuple {
            quote! {
                impl #impl_generics From<#type_generics> for #ident
                where
                    #(#where_clause),*
                {
                    fn from(value: #type_generics) -> Self {
                        Self ( #(#fields),* )
                    }
                }
            }
        } else {
            quote! {
                impl #impl_generics From<#type_generics> for #ident
                where
                    #(#where_clause),*
                {
                    fn from(value: #type_generics) -> Self {
                        Self { #(#fields),* }
                    }

                }
            }
        }
    }
}

impl OpcodeFieldGen {
    fn gen_sexpr_where_clause(&self, index: usize) -> TokenStream {
        self.ty.gen_sexpr_where_clause(index)
    }
    fn gen_sexpr_init_assign(&self, index: &mut usize, from_tuple: bool) -> TokenStream {
        if self.attrs.option {
            if let Some(ident) = &self.ident {
                return quote! { #ident: None };
            } else {
                return quote! { None };
            }
        }

        let index_token_stream = if from_tuple {
            format!("value.{}", index).parse().unwrap()
        } else {
            quote! { value }
        };

        *index += 1;

        let mut assign = self.ty.gen_sexpr_init_assign(index_token_stream);

        if self.attrs.variable {
            assign = quote! {
                super::variable::Variable::Constant(#assign)
            };
        }

        if self.attrs.option {
            assign = quote! {
                Some(#assign)
            };
        }

        if let Some(ident) = &self.ident {
            quote! {
                #ident: #assign
            }
        } else {
            quote! {
                #assign
            }
        }
    }
}

impl FieldType {
    fn gen_sexpr_where_clause(&self, index: usize) -> TokenStream {
        let param_type = format!("P{}", index).parse().unwrap();
        match self {
            FieldType::Noraml(token_stream) => {
                Self::gen_sexpr_where_clause_filter(token_stream, param_type)
            }
            FieldType::List(token_stream) => {
                quote! {
                    #param_type: super::sexpr::MapCollect<#token_stream>
                }
            }
            FieldType::Array(token_stream, num) => {
                quote! {
                    [#token_stream;#num]: From<#param_type>
                }
            }
        }
    }

    fn gen_sexpr_where_clause_filter(
        token_stream: &TokenStream,
        index: TokenStream,
    ) -> TokenStream {
        match token_stream.to_string().as_str() {
            "f32" => quote! {
                super::sexpr::Number: From<#index>
            },
            _ => {
                quote! {
                    #token_stream: From<#index>
                }
            }
        }
    }

    fn gen_sexpr_init_assign(&self, index: TokenStream) -> TokenStream {
        match self {
            FieldType::Noraml(token_stream) => {
                Self::gen_sexpr_init_assign_filter(token_stream, index)
            }
            FieldType::List(_) => {
                quote! { #index.map_collect() }
            }
            FieldType::Array(_, _) => {
                quote! { #index.into() }
            }
        }
    }

    fn gen_sexpr_init_assign_filter(token_stream: &TokenStream, index: TokenStream) -> TokenStream {
        match token_stream.to_string().as_str() {
            "f32" => {
                quote! { super::sexpr::Number::from(#index).0 }
            }
            _ => {
                quote! {
                    #index.into()
                }
            }
        }
    }
}

impl Sexpr for OpcodeEnumCodeGen {
    fn gen_sexpr_init_fns(&self) -> TokenStream {
        quote! {}
    }
}
