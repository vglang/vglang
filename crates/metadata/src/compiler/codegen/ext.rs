//! Extension traits to help generate rust code from `ml` opcodes.

use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

use crate::compiler::ir::{Comment, Enum, Field, Ident, Node, Type};

/// An extension trait for [`Ident`] to helper rust codes generating.
pub trait IdentGen {
    fn field_ident(&self) -> TokenStream;
    fn type_ident(&self) -> TokenStream;
    fn xml_attr_name(&self) -> String;

    fn field_ident_with_prefix(&self, prefix: &str) -> TokenStream;
}

impl IdentGen for Ident {
    fn field_ident_with_prefix(&self, prefix: &str) -> TokenStream {
        Ident(self.0, format!("{}{}", prefix, self.1)).field_ident()
    }
    fn field_ident(&self) -> TokenStream {
        match self.1.to_snake_case().as_str() {
            "type" => "r#type".parse().unwrap(),
            "in" => "r#in".parse().unwrap(),
            "for" => "r#for".parse().unwrap(),
            ident => ident.parse().unwrap(),
        }
    }

    fn type_ident(&self) -> TokenStream {
        self.1.to_upper_camel_case().parse().unwrap()
    }

    fn xml_attr_name(&self) -> String {
        self.1.to_lower_camel_case()
    }
}

/// An extension trait that generate rust comment.
pub trait CommentGen {
    /// Generate comment [`TokenStream`]
    fn gen_comment(&self) -> TokenStream;
}

impl CommentGen for Comment {
    fn gen_comment(&self) -> TokenStream {
        format!("/// {0}", self.1).parse().unwrap()
    }
}

/// An extension trait that generate a type definition.
pub trait TypeGen {
    fn gen_type_definition(&self, opcode_mod: &TokenStream) -> TokenStream;

    fn gen_from_where_clause(
        &self,
        opcode_mod: &TokenStream,
        sexpr_mod: &TokenStream,
        param: &TokenStream,
    ) -> TokenStream;

    fn gen_from_expr(&self, sexpr_mod: &TokenStream, param: &TokenStream) -> TokenStream;
}

impl TypeGen for Type {
    fn gen_from_expr(&self, sexpr_mod: &TokenStream, param: &TokenStream) -> TokenStream {
        match self {
            Type::ListOf(_, _) => {
                quote! {
                    #param.map_collect()
                }
            }
            Type::Float(_) => {
                quote! { #sexpr_mod Number::from(#param).0 }
            }
            _ => {
                quote! {
                    #param.into()
                }
            }
        }
    }
    fn gen_from_where_clause(
        &self,
        opcode_mod: &TokenStream,
        sexpr_mod: &TokenStream,
        param: &TokenStream,
    ) -> TokenStream {
        match self {
            Type::Data(ident) => {
                let ty = ident.type_ident();
                quote! {
                   #opcode_mod #ty : From<#param>
                }
            }
            Type::ListOf(component, _) => {
                let ty = component.gen_type_definition(opcode_mod);
                quote! {
                    #param: #sexpr_mod MapCollect<#ty>
                }
            }
            Type::Float(_) => {
                quote! { #sexpr_mod Number: From<#param> }
            }
            _ => {
                let ty = self.gen_type_definition(opcode_mod);
                quote! {
                   #ty : From<#param>
                }
            }
        }
    }

    fn gen_type_definition(&self, opcode_mod: &TokenStream) -> TokenStream {
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
            Type::Data(ident) => {
                let ident = ident.type_ident();

                quote! { #opcode_mod #ident }
            }
            Type::ListOf(component, _) => {
                let component = component.gen_type_definition(opcode_mod);

                quote! { Vec<#component> }
            }
            Type::ArrayOf(component, lit_num, _) => {
                let component = component.gen_type_definition(opcode_mod);
                let num = lit_num.0;

                quote! { [#component;#num] }
            }
        }
    }
}
/// An extension trait for [`Field`] to helpe code generating.
pub trait FieldGen {
    /// Generate field comments.
    fn gen_comments(&self) -> Vec<TokenStream>;

    /// Generate field init expr: such as: `name: Some(1i32)` or `name: 0i32`
    fn gen_init_expr(&self, opcode_mod: &TokenStream, value: TokenStream) -> TokenStream;

    fn gen_assign_expr(&self, opcode_mod: &TokenStream, value: TokenStream) -> TokenStream;

    /// Only option field can call this function.
    fn gen_init_none(&self) -> TokenStream;

    /// Generate filed ident with keywords escape.
    fn gen_ident(&self) -> Option<TokenStream>;

    /// Generate filed ident with keywords escape.
    fn gen_xml_attr_name(&self) -> Option<String>;

    /// Generate field definition code.
    fn gen_definition(&self, vis: TokenStream) -> TokenStream;

    /// Generate the field type definition code.
    fn gen_type_definition(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl<'a> FieldGen for Field<'a> {
    fn gen_comments(&self) -> Vec<TokenStream> {
        self.comments().iter().map(|c| c.gen_comment()).collect()
    }

    fn gen_init_expr(&self, opcode_mod: &TokenStream, mut value: TokenStream) -> TokenStream {
        if self.is_variable() {
            value = quote! { #opcode_mod variable::Variable::Constant(#value) };
        }

        if self.is_option() {
            value = quote! { Some(#value) };
        }

        if let Some(ident) = self.gen_ident() {
            quote! { #ident: #value }
        } else {
            quote! { #value }
        }
    }

    fn gen_assign_expr(&self, opcode_mod: &TokenStream, mut value: TokenStream) -> TokenStream {
        if self.is_variable() {
            value = quote! { #opcode_mod variable::Variable::Constant(#value) };
        }

        if self.is_option() {
            value = quote! { Some(#value) };
        }

        value
    }

    /// Only option field can call this function.
    fn gen_init_none(&self) -> TokenStream {
        if let Some(ident) = self.gen_ident() {
            quote! { #ident: None }
        } else {
            quote! { None }
        }
    }

    fn gen_ident(&self) -> Option<TokenStream> {
        self.ident().map(|ident| ident.field_ident())
    }

    fn gen_xml_attr_name(&self) -> Option<String> {
        if let Some(name) = self.xml_name() {
            Some(name.to_string())
        } else {
            self.ident().map(|ident| ident.xml_attr_name())
        }
    }

    fn gen_type_definition(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut ty = self.ty().gen_type_definition(opcode_mod);

        if self.is_variable() {
            ty = quote! {
                variable::Variable<#ty>
            };
        }

        if self.is_option() {
            ty = quote! { Option<#ty> };
        }

        ty
    }

    fn gen_definition(&self, vis: TokenStream) -> TokenStream {
        let comments = self.gen_comments();

        let ty = self.gen_type_definition(&quote! {});
        if let Some(ident) = self.gen_ident() {
            quote! { #(#comments)* #vis #ident: #ty }
        } else {
            quote! { #(#comments)* #vis #ty }
        }
    }
}

/// An extension trait for [`Node`] to helpe code generating.
pub trait NodeGen {
    /// Generate an body by custom input fields. such as: (field1, field2...) or { field: value.0, field1: value.1,... }
    fn gen_body_expr(&self, fields: Vec<TokenStream>) -> TokenStream;

    /// Generate rust ident of this [`Node`]
    fn gen_ident(&self) -> TokenStream;

    /// Generate node comment list.
    fn gen_comments(&self) -> Vec<TokenStream>;

    fn gen_xml_attr_name(&self) -> String;
}

impl NodeGen for Node {
    fn gen_body_expr(&self, fields: Vec<TokenStream>) -> TokenStream {
        if fields.is_empty() {
            return quote! {};
        }

        if self.is_tuple() {
            quote! { (#(#fields),*) }
        } else {
            quote! { {#(#fields),*} }
        }
    }

    fn gen_ident(&self) -> TokenStream {
        self.ident.1.to_upper_camel_case().parse().unwrap()
    }

    fn gen_comments(&self) -> Vec<TokenStream> {
        self.comments.iter().map(|c| c.gen_comment()).collect()
    }

    fn gen_xml_attr_name(&self) -> String {
        self.xml_name()
            .map(|v| v.to_string())
            .unwrap_or(self.ident.xml_attr_name())
    }
}

pub trait EnumGen {
    /// Generate rust ident of this [`Enum`]
    fn gen_ident(&self) -> TokenStream;
    /// Generate node comment list.
    fn gen_comments(&self) -> Vec<TokenStream>;
}

impl EnumGen for Enum {
    fn gen_ident(&self) -> TokenStream {
        self.ident.1.to_upper_camel_case().parse().unwrap()
    }

    fn gen_comments(&self) -> Vec<TokenStream> {
        self.comments.iter().map(|c| c.gen_comment()).collect()
    }
}
