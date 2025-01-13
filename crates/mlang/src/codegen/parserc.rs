//! metadata generated for building parser.
//!

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::ext::{EnumGen, IdentGen},
    opcode::{Enum, Opcode},
};

pub trait ParsecGen {
    fn gen_parserc_codes(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl ParsecGen for Enum {
    fn gen_parserc_codes(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut fields = vec![];

        for field in self.fields.iter() {
            fields.push(field.ident.field_ident().to_string());
        }

        let ident = self.gen_ident();

        quote! {
            impl #opcode_mod #ident {
                pub fn is_valid_field_name(name: &str) -> bool {
                    match name {
                        #(#fields)|* => true,
                        _ => false
                    }
                }
            }
        }
    }
}

/// A generator for `opcode` mod.
pub struct ParsercModGen {
    /// opcode mod path prefix.
    opcode_mod: TokenStream,
}

impl ParsercModGen {
    /// Create new parserc mod generator.
    pub fn new<S>(opcode_mod: S) -> Self
    where
        S: AsRef<str>,
    {
        ParsercModGen {
            opcode_mod: opcode_mod.as_ref().parse().unwrap(),
        }
    }

    /// Generate mod codes.
    pub fn gen(self, opcodes: &[Opcode]) -> TokenStream {
        let mut token_streams = vec![];
        for opcode in opcodes {
            match opcode {
                Opcode::Enum(node) => {
                    token_streams.push(node.gen_parserc_codes(&self.opcode_mod));
                }
                _ => {}
            }
        }
        quote! {
            #(#token_streams)*
        }
    }
}
