#[doc = r" Name type used by [`Serializer`]"]
pub struct Name<'a> {
    pub origin: &'a str,
    pub rename: Option<&'a str>,
}
#[doc = r" Serializer for Opcodes."]
pub trait Serializer {
    type Error;
    #[doc = r" Type returns by [`serialize_el`](Serializer::serialize_el),"]
    #[doc = r" [`serialize_leaf`](Serializer::serialize_leaf) or [`serialize_attr`](Serializer::serialize_attr)"]
    #[doc = r" for serializing the content of node."]
    type SerializeNode: SerializeNode<Error = Self::Error>;
    #[doc = r" Type returns by [`serialize_enum`](Serializer::serialize_enum) for serializing the content of a enum data."]
    type SerializeEnum: SerializeEnum<Error = Self::Error>;
    #[doc = r" Serialize a element node."]
    fn serialize_el(
        &mut self,
        type_id: usize,
        name: Name<'_>,
    ) -> Result<Self::SerializeNode, Self::Error>;
    #[doc = r" Serialize a leaf node."]
    fn serialize_leaf(
        &mut self,
        type_id: usize,
        name: Name<'_>,
    ) -> Result<Self::SerializeNode, Self::Error>;
    #[doc = r" Serialize a attr node."]
    fn serialize_attr(
        &mut self,
        type_id: usize,
        name: Name<'_>,
    ) -> Result<Self::SerializeNode, Self::Error>;
    #[doc = r" Serialize a data."]
    fn serialize_data(
        &mut self,
        type_id: usize,
        name: Name<'_>,
    ) -> Result<Self::SerializeNode, Self::Error>;
    #[doc = r" Serialize a enum data."]
    fn serialize_enum(
        &mut self,
        type_id: usize,
        name: Name<'_>,
    ) -> Result<Self::SerializeEnum, Self::Error>;
}
#[doc = r" A trait to help serialzing a node."]
pub trait SerializeNode {
    #[doc = r" Error type returns by this trait."]
    type Error;
    #[doc = r" Returns by [`serialize_field`](SerializeNode::serialize_field) to help serializing field type."]
    type SerializeType: SerializeType<Error = Self::Error>;
    #[doc = r" Serialize a field."]
    fn serialize_field(
        &mut self,
        index: usize,
        name: Option<Name<'_>>,
    ) -> Result<Self::SerializeType, Self::Error>;
}
#[doc = r" A trait to help serialzing a enum data."]
pub trait SerializeEnum {
    #[doc = r" Error type returns by this trait."]
    type Error;
    #[doc = r" Returns by [`serialize_field`](SerializeEnum::serialize_field) to help serializing enum field."]
    type SerializeNode: SerializeNode<Error = Self::Error>;
    #[doc = r" Serialize a enum field node."]
    fn serialize_field(&mut self, name: Name<'_>) -> Result<Self::SerializeNode, Self::Error>;
}
#[doc = r" A trait to help serializing vglang types."]
pub trait SerializeType {
    #[doc = r" Error type returns by this trait."]
    type Error;
    #[doc = r" Returns by [`serialize_seq`](SerializeType::serialize_seq) to help serializing array of vector."]
    type SerializeSeq: SerializeSeq<Error = Self::Error>;
    #[doc = r" Serialize vglang `string`."]
    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `byte`."]
    fn serialize_byte(&mut self, value: i8) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `ubyte`."]
    fn serialize_ubyte(&mut self, value: u8) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `byte`."]
    fn serialize_short(&mut self, value: i16) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `ubyte`."]
    fn serialize_ushort(&mut self, value: u16) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `int`."]
    fn serialize_int(&mut self, value: i32) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `uint`."]
    fn serialize_uint(&mut self, value: u32) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `long`."]
    fn serialize_long(&mut self, value: i64) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `ulong`."]
    fn serialize_ulong(&mut self, value: u64) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `long`."]
    fn serialize_float(&mut self, value: f32) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `double`."]
    fn serialize_double(&mut self, value: f64) -> Result<(), Self::Error>;
    #[doc = r" Serialize vglang `vec[T]` or `[T;N]`"]
    fn serialize_seq(&mut self, len: usize) -> Result<Self::SerializeSeq, Self::Error>;
}
#[doc = r" A trait to help serializing vglang `vec[T]` or `[T;N]`."]
pub trait SerializeSeq {
    #[doc = r" Error returns by this trait."]
    type Error;
    #[doc = r" Type returns by [`serialize_data`](SerializeSeq::serialize_data), for serializing the data item."]
    type SerializeNode: SerializeNode<Error = Self::Error>;
    #[doc = r" Type returns by [`serialize_enum`](SerializeSeq::serialize_data), for serializing the enum data item."]
    type SerializeEnum: SerializeEnum<Error = Self::Error>;
    #[doc = r" Returns by [`serialize_type`](SerializeSeq::serialize_type) , for serializing the other type item."]
    type SerializeType: SerializeType<Error = Self::Error>;
}
