use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::ext::{FieldGen, NodeGen},
    ir::{Enum, Node, Stat},
};

trait SvgCodeGen {
    fn gen_node_writer(&self, opcode_mod: &TokenStream) -> TokenStream;

    fn gen_attrs_writer(&self, opcode_mod: &TokenStream) -> TokenStream;

    fn gen_attr_value_writer(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SvgCodeGen for Node {
    fn gen_attr_value_writer(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
    fn gen_attrs_writer(&self, opcode_mod: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();

        quote! {
            impl SvgAttrsWriter for #opcode_mod #ident {
                fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(),Node::Error>
                where
                    C: SvgContext<Error = E>,
                    Node: SvgNode<Error = E>,
                {
                    Ok(())
                }
            }
        }
    }
    fn gen_node_writer(&self, opcode_mod: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();

        let xml_name = self
            .xml_name()
            .map(|name| name.to_string())
            .unwrap_or(self.ident.1.to_lower_camel_case());

        let mut stats = vec![];

        for (idx, field) in self.fields.iter().enumerate() {
            let name = match field.gen_xml_attr_name() {
                Some(name) => name,
                _ => continue,
            };

            let value = if field.is_variable() {
                quote! {
                    let value = ctx.valueof(&value)?.to_svg_attr_value();
                }
            } else {
                quote! {
                    let value = value.to_svg_attr_value();
                }
            };

            let field_name = if let Some(field_name) = field.gen_ident() {
                quote! {
                    self.#field_name
                }
            } else {
                format!("self.{}", idx).parse().unwrap()
            };

            if field.is_option() {
                stats.push(quote! {
                    if let Some(value) = &#field_name {
                        #value
                        node.set_svg_attr(#name,&value)?;
                    }
                });
            } else {
                stats.push(quote! {
                    let value = &#field_name;
                    #value
                    node.set_svg_attr(#name,&value)?;
                });
            }
        }

        quote! {
            impl SvgAttrsWriter for #opcode_mod #ident {
                fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(),Node::Error>
                where
                    C: SvgContext<Error = E>,
                    Node: SvgNode<Error = E>,
                {
                    #(#stats)*
                    Ok(())
                }
            }

            impl SvgNodeWriter for #opcode_mod #ident {
                fn to_svg_node_name(&self) -> &str {
                    #xml_name
                }
            }
        }
    }
}

impl SvgCodeGen for Enum {
    fn gen_attr_value_writer(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }

    fn gen_node_writer(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }

    fn gen_attrs_writer(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

/// A generator that create `sexpr` mod for `mlang`.
#[allow(unused)]
pub struct SvgModGen {
    opcode_mod: TokenStream,
}

impl SvgModGen {
    /// Create new sexpr mode generator
    pub fn new<S>(opcode_mod: S) -> Self
    where
        String: From<S>,
    {
        Self {
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
        }
    }

    /// Generate svg mod
    pub fn gen(self, stats: &[Stat]) -> TokenStream {
        let mut token_streams = vec![];

        token_streams.push(self.gen_base_codes());
        token_streams.push(self.gen_builtin_types_traits());

        for state in stats {
            match state {
                Stat::Element(node) | Stat::Leaf(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_node_writer(&self.opcode_mod));
                    }
                }
                Stat::Attr(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attrs_writer(&self.opcode_mod));
                    }
                }
                Stat::Data(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attr_value_writer(&self.opcode_mod));
                    }
                }
                Stat::Enum(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attr_value_writer(&self.opcode_mod));
                    }
                }
                _ => {}
            }
        }

        quote! {
            #(#token_streams)*
        }
    }

    fn gen_builtin_types_traits(&self) -> TokenStream {
        let builtin_types = vec![
            "bool", "String", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "f32", "f64",
        ];

        let mut impls = vec![];

        for ty in builtin_types {
            let ident: TokenStream = ty.parse().unwrap();

            impls.push(quote! {
                impl SvgAttrValueWriter for #ident {
                    fn to_svg_attr_value(&self) -> String {
                        format!("{}",self)
                    }
                }
            });
        }

        quote! { #(#impls)* }
    }

    fn gen_base_codes(&self) -> TokenStream {
        let opcode_mod = &self.opcode_mod;

        quote! {
            /// The trait to access context data.
            pub trait SvgContext {
                /// Error type returns by this trait.
                type Error;

                fn valueof<'a, T>(&'a self, variable: &'a #opcode_mod variable::Variable<T>) -> Result<&'a T,Self::Error>
                where
                    #opcode_mod Data: From<T>,
                    for<'c> &'c T: TryFrom<&'c #opcode_mod Data, Error = ()>;
            }

            /// The abstract of xml `node`.
            #[allow(unused)]
            pub trait SvgNode {
                /// Error type returns by this trait.
                type Error;

                /// set a new attribute/value pair.
                fn set_svg_attr(&mut self, name: &str, value: &str) -> Result<(),Self::Error>;
            }

            /// All attr node must implement this trait.
            pub trait SvgAttrsWriter {
                /// Write node attribute/value pairs.
                fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(),Node::Error>
                where
                    C: SvgContext<Error = E>,
                    Node: SvgNode<Error = E>;
            }

            /// elements/leaves should implement this trait.
            pub trait SvgNodeWriter: SvgAttrsWriter {
                /// Returns the name of creating svg node.
                fn to_svg_node_name(&self) -> &str;
            }

            /// All data types should implement this trait.
            pub trait SvgAttrValueWriter {
                /// Create a attribute value from data.
                fn to_svg_attr_value(&self) -> String;
            }
        }
    }
}
