use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::Stat;

/// `serde.rs` mod generator.
pub struct SerdeModGen {
    #[allow(unused)]
    opcode_mod: TokenStream,
}

impl SerdeModGen {
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
    pub fn gen(self, _: &[Stat]) -> TokenStream {
        let impls = vec![self.gen_skeleton_codes()];
        quote! {
            #(#impls)*
        }
    }

    fn gen_skeleton_codes(&self) -> TokenStream {
        quote! {

            /// Name type used by [`Serializer`]
            pub struct Name<'a> {
                pub origin: &'a str,
                pub rename: Option<&'a str>,
            }

            /// Serializer for Opcodes.
            pub trait Serializer {
                type Error;
                /// Type returns by [`serialize_el`](Serializer::serialize_el),
                /// [`serialize_leaf`](Serializer::serialize_leaf) or [`serialize_attr`](Serializer::serialize_attr)
                /// for serializing the content of node.
                type SerializeNode: SerializeNode<Error = Self::Error>;

                /// Type returns by [`serialize_enum`](Serializer::serialize_enum) for serializing the content of a enum data.
                type SerializeEnum: SerializeEnum<Error = Self::Error>;

                /// Serialize a element node.
                fn serialize_el(&mut self, type_id: usize, name: Name<'_>) -> Result<Self::SerializeNode,Self::Error>;

                /// Serialize a leaf node.
                fn serialize_leaf(&mut self, type_id: usize, name: Name<'_>) -> Result<Self::SerializeNode,Self::Error>;

                /// Serialize a attr node.
                fn serialize_attr(&mut self, type_id: usize, name: Name<'_>) -> Result<Self::SerializeNode,Self::Error>;

                         /// Serialize a data.
                fn serialize_data(&mut self, type_id: usize, name: Name<'_>) -> Result<Self::SerializeNode,Self::Error>;

                /// Serialize a enum data.
                fn serialize_enum(&mut self, type_id: usize, name: Name<'_>) -> Result<Self::SerializeEnum,Self::Error>;
            }

            /// A trait to help serialzing a node.
            pub trait SerializeNode {
                /// Error type returns by this trait.
                type Error;

                /// Returns by [`serialize_field`](SerializeNode::serialize_field) to help serializing field type.
                type SerializeType: SerializeType<Error = Self::Error>;

                /// Serialize a field.
                fn serialize_field(&mut self, index: usize, name: Option<Name<'_>>) -> Result<Self::SerializeType, Self::Error>;
            }

            /// A trait to help serialzing a enum data.
            pub trait SerializeEnum {
                /// Error type returns by this trait.
                type Error;

                /// Returns by [`serialize_field`](SerializeEnum::serialize_field) to help serializing enum field.
                type SerializeNode: SerializeNode<Error = Self::Error>;

                /// Serialize a enum field node.
                fn serialize_field(&mut self, name: Name<'_>) -> Result<Self::SerializeNode, Self::Error>;
            }

            /// A trait to help serializing vglang types.
            pub trait SerializeType {
                /// Error type returns by this trait.
                type Error;

                /// Returns by [`serialize_seq`](SerializeType::serialize_seq) to help serializing array of vector.
                type SerializeSeq: SerializeSeq<Error = Self::Error>;

                /// Serialize vglang `string`.
                fn serialize_string(&mut self, value: &str) -> Result<(),Self::Error>;

                /// Serialize vglang `byte`.
                fn serialize_byte(&mut self, value: i8) -> Result<(),Self::Error>;

                /// Serialize vglang `ubyte`.
                fn serialize_ubyte(&mut self, value: u8) -> Result<(),Self::Error>;

                /// Serialize vglang `byte`.
                fn serialize_short(&mut self, value: i16) -> Result<(),Self::Error>;

                /// Serialize vglang `ubyte`.
                fn serialize_ushort(&mut self, value: u16) -> Result<(),Self::Error>;

                /// Serialize vglang `int`.
                fn serialize_int(&mut self, value: i32) -> Result<(),Self::Error>;

                /// Serialize vglang `uint`.
                fn serialize_uint(&mut self, value: u32) -> Result<(),Self::Error>;

                /// Serialize vglang `long`.
                fn serialize_long(&mut self, value: i64) -> Result<(),Self::Error>;

                /// Serialize vglang `ulong`.
                fn serialize_ulong(&mut self, value: u64) -> Result<(),Self::Error>;

                /// Serialize vglang `long`.
                fn serialize_float(&mut self, value: f32) -> Result<(),Self::Error>;

                /// Serialize vglang `double`.
                fn serialize_double(&mut self, value: f64) -> Result<(),Self::Error>;

                /// Serialize vglang `vec[T]` or `[T;N]`
                fn serialize_seq(&mut self, len: usize) -> Result<Self::SerializeSeq, Self::Error>;
            }

            /// A trait to help serializing vglang `vec[T]` or `[T;N]`.
            pub trait SerializeSeq {
                /// Error returns by this trait.
                type Error;

                /// Type returns by [`serialize_data`](SerializeSeq::serialize_data), for serializing the data item.
                type SerializeNode: SerializeNode<Error = Self::Error>;

                /// Type returns by [`serialize_enum`](SerializeSeq::serialize_data), for serializing the enum data item.
                type SerializeEnum: SerializeEnum<Error = Self::Error>;

                /// Returns by [`serialize_type`](SerializeSeq::serialize_type) , for serializing the other type item.
                type SerializeType: SerializeType<Error = Self::Error>;
            }
        }
    }
}
