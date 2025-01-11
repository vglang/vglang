use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::opcode::{Comment, Type};

pub trait TypeGen {
    fn gen_definition(&self) -> TokenStream;
}

impl TypeGen for Type {
    fn gen_definition(&self) -> TokenStream {
        match self {
            Type::Bool(_) => quote! {bool},
            Type::String(_) => quote! {String},
            Type::Byte(_) => quote! {i8},
            Type::Ubyte(_) => quote! {u8},
            Type::Short(_) => quote! {i16},
            Type::Ushort(_) => quote! {u16},
            Type::Int(_) => quote! {i32},
            Type::Uint(_) => quote! {u32},
            Type::Long(_) => quote! {i64},
            Type::Ulong(_) => quote! {u64},
            Type::Float(_) => quote! {f32},
            Type::Double(_) => quote! {f64},
            Type::Data(ident) => ident.0.to_upper_camel_case().parse().unwrap(),
            Type::ListOf(component, _) => {
                let component = component.gen_definition();

                quote! { Vec<#component> }
            }
            Type::ArrayOf(component, lit_num, _) => {
                let component = component.gen_definition();
                let num = lit_num.0;

                quote! { [#component;#num] }
            }
        }
    }
}

pub(super) trait CommentGen {
    fn gen_definition(&self) -> TokenStream;
}

impl CommentGen for Comment {
    fn gen_definition(&self) -> TokenStream {
        format!("/// {}", self.0).parse().unwrap()
    }
}
