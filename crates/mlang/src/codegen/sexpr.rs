//! The `sexpr` mod generator.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::opcode::{Definition, FieldDefinitionExt},
    opcode::{ApplyTo, ChildrenOf, Enum, Node, Opcode},
};

pub trait SexprGen {
    fn gen_sexpr_fns(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SexprGen for Node {
    fn gen_sexpr_fns(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut token_streams = vec![];

        token_streams.push(self.gen_sexpr_default_fn(opcode_mod));

        token_streams.push(self.gen_sexpr_from_init_fn(opcode_mod));

        token_streams.push(self.gen_sexpr_from_fns(opcode_mod));

        quote! {
            #(#token_streams)*
        }
    }
}

impl Node {
    fn gen_sexpr_default_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let no_options = self.fields.iter().fold(0usize, |non_option, field| {
            if field.is_option() {
                non_option
            } else {
                non_option + 1
            }
        });

        if no_options > 0 {
            return quote! {};
        }

        let fields = self
            .fields
            .iter()
            .map(|field| {
                if let Some(ident) = field.gen_ident() {
                    quote! { #ident: None}
                } else {
                    quote! { None }
                }
            })
            .collect::<Vec<_>>();

        let ident = self.gen_ident();

        let ident = quote! {#opcode_mod #ident};

        let body = self.gen_body(fields);

        quote! {
            impl Default for #ident {
                fn default() -> Self {
                    Self #body
                }
            }
        }
    }

    fn gen_sexpr_from_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }

    fn gen_sexpr_from_init_fn(&self, _: &TokenStream) -> TokenStream {
        let no_options = self.fields.iter().fold(0usize, |non_option, field| {
            if field.is_option() {
                non_option
            } else {
                non_option + 1
            }
        });

        if no_options == 0 {
            return quote! {};
        }

        quote! {}
    }
}

impl SexprGen for Enum {
    fn gen_sexpr_fns(&self, _opcode_mod: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ApplyTo {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ChildrenOf {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

/// A generator that create `sexpr` mod for `mlang`.
pub struct SexprModGen {
    opcode_mod: TokenStream,
}

impl SexprModGen {
    /// Create new sexpr mode generator
    pub fn new<S>(opcode_mod: S) -> Self
    where
        String: From<S>,
    {
        Self {
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
        }
    }
    /// Generate sexpr mod
    pub fn gen(self, opcodes: &[Opcode]) -> TokenStream {
        let mut token_streams = vec![];

        for opcode in opcodes {
            match opcode {
                Opcode::Element(node)
                | Opcode::Leaf(node)
                | Opcode::Attr(node)
                | Opcode::Data(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                Opcode::Enum(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }

                Opcode::ApplyTo(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                Opcode::ChildrenOf(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                }
                _ => {}
            }
        }

        quote! {
            #(#token_streams)*
        }
    }
}
