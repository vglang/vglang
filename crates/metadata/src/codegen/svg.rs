use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::ext::NodeGen,
    ir::{Enum, Node, Stat},
};

trait SvgCodeGen {
    fn gen_node_writer(&self, opcode_mod: &TokenStream) -> TokenStream;

    fn gen_attrs_writer(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SvgCodeGen for Node {
    fn gen_attrs_writer(&self, opcode_mod: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();

        quote! {
            impl SvgAttrsWriter for #opcode_mod #ident {
                #[allow(unused)]
                fn write_svg_attrs<Node: SvgNode>(&self, node: &mut Node) -> Result<(),Node::Error> {
                    todo!()
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

        quote! {
            impl SvgAttrsWriter for #opcode_mod #ident {
                #[allow(unused)]
                fn write_svg_attrs<Node: SvgNode>(&self, node: &mut Node) -> Result<(),Node::Error> {
                    todo!()
                }
            }

            impl SvgNodeWriter for #opcode_mod #ident {
                fn as_svg_node_name(&self) -> &str {
                    #xml_name
                }
            }
        }
    }
}

impl SvgCodeGen for Enum {
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

        for state in stats {
            match state {
                Stat::Element(node) | Stat::Leaf(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_node_writer(&self.opcode_mod));
                    }
                }
                Stat::Data(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attrs_writer(&self.opcode_mod));
                    }
                }
                Stat::Enum(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attrs_writer(&self.opcode_mod));
                    }
                }
                Stat::Attr(node) => {
                    if !node.xml_skip() {
                        token_streams.push(node.gen_attrs_writer(&self.opcode_mod));
                    }
                }
                _ => {}
            }
        }

        quote! {
            #(#token_streams)*
        }
    }

    fn gen_base_codes(&self) -> TokenStream {
        quote! {
            /// The abstract of xml `node`.
            #[allow(unused)]
            pub trait SvgNode {
                /// Error type returns by this trait.
                type Error;

                /// set a new attribute/value pair.
                fn set_svg_attr(&mut self, name: &str, value: &str) -> Result<(),Self::Error>;
            }

            /// Write self as xml attrs.
            pub trait SvgAttrsWriter {
                /// write self as a xml node's attribute/value pairs.
                fn write_svg_attrs<Node: SvgNode>(&self, node: &mut Node) -> Result<(),Node::Error>;
            }

            /// A trait to generate xml node.
            ///
            /// All `vglang` nodes must implement this trait.
            pub trait SvgNodeWriter: SvgAttrsWriter {
                fn as_svg_node_name(&self) -> &str;
            }
        }
    }
}
