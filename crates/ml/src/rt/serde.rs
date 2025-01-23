use super::opcode::variable::{Path, Target, Variable};

/// Serializer for Opcodes.
pub trait Serializer {
    type Error;
    /// Type returns by [`serialize_el`](Serializer::serialize_el),
    /// [`serialize_leaf`](Serializer::serialize_leaf) or [`serialize_attr`](Serializer::serialize_attr)
    /// for serializing the content of node.
    type SerializeNode: SerializeNode<Error = Self::Error>;

    /// Type returns by [`serialize_enum`](Serializer::serialize_enum) for serializing the content of a enum data.
    type SerializeEnum: SerializeEnum<Error = Self::Error>;

    /// Returns by [`serialize_seq`](SerializeType::serialize_seq) to help serializing array of vector.
    type SerializeSeq: SerializeSeq<Error = Self::Error>;

    /// Serialize a element node.
    fn serialize_el(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::SerializeNode, Self::Error>;

    /// Serialize a leaf node.
    fn serialize_leaf(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::SerializeNode, Self::Error>;

    /// Serialize a attr node.
    fn serialize_attr(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::SerializeNode, Self::Error>;

    /// Serialize a data.
    fn serialize_data(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::SerializeNode, Self::Error>;

    /// Serialize a enum data.
    fn serialize_enum(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::SerializeEnum, Self::Error>;

    /// Serialize vglang `string`.
    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error>;

    /// Serialize vglang `string`.
    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error>;

    /// Serialize vglang `byte`.
    fn serialize_byte(&mut self, value: i8) -> Result<(), Self::Error>;

    /// Serialize vglang `ubyte`.
    fn serialize_ubyte(&mut self, value: u8) -> Result<(), Self::Error>;

    /// Serialize vglang `byte`.
    fn serialize_short(&mut self, value: i16) -> Result<(), Self::Error>;

    /// Serialize vglang `ubyte`.
    fn serialize_ushort(&mut self, value: u16) -> Result<(), Self::Error>;

    /// Serialize vglang `int`.
    fn serialize_int(&mut self, value: i32) -> Result<(), Self::Error>;

    /// Serialize vglang `uint`.
    fn serialize_uint(&mut self, value: u32) -> Result<(), Self::Error>;

    /// Serialize vglang `long`.
    fn serialize_long(&mut self, value: i64) -> Result<(), Self::Error>;

    /// Serialize vglang `ulong`.
    fn serialize_ulong(&mut self, value: u64) -> Result<(), Self::Error>;

    /// Serialize vglang `long`.
    fn serialize_float(&mut self, value: f32) -> Result<(), Self::Error>;

    /// Serialize vglang `double`.
    fn serialize_double(&mut self, value: f64) -> Result<(), Self::Error>;

    /// Serialize a none value.
    fn serialize_none(&mut self) -> Result<(), Self::Error>;

    /// Serialize a none value.
    fn serialize_variable(&mut self, path: &Path, target: &Target) -> Result<(), Self::Error>;

    /// Serialize vglang `vec[T]` or `[T;N]`
    fn serialize_seq(&mut self, len: usize) -> Result<Self::SerializeSeq, Self::Error>;
}

pub trait SerializeSeq {
    type Error;

    /// Serialize next item.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Finish serializing a sequence.
    fn end(self) -> Result<(), Self::Error>;
}

/// A trait to help serialzing a node.
pub trait SerializeNode {
    /// Error type returns by this trait.
    type Error;

    /// Serialize a field.
    fn serialize_field<T>(
        &mut self,
        index: usize,
        name: Option<&str>,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
}

/// A trait to help serialzing a enum data.
pub trait SerializeEnum {
    /// Error type returns by this trait.
    type Error;

    /// Returns by [`serialize_field`](SerializeEnum::serialize_field) to help serializing enum field.
    type SerializeNode: SerializeNode<Error = Self::Error>;

    /// Serialize a enum field node.
    fn serialize_field(&mut self, name: &str) -> Result<Self::SerializeNode, Self::Error>;
}
/// A node/enum must implement this trait to support serde framework.
pub trait Serialize {
    /// serialize self with `serializer`.
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer;
}

impl Serialize for bool {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(*self)
    }
}

impl Serialize for String {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_string(&self)
    }
}

impl Serialize for i8 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_byte(*self)
    }
}

impl Serialize for u8 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_ubyte(*self)
    }
}

impl Serialize for i16 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_short(*self)
    }
}

impl Serialize for u16 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_ushort(*self)
    }
}

impl Serialize for i32 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_int(*self)
    }
}

impl Serialize for u32 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_uint(*self)
    }
}

impl Serialize for i64 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_long(*self)
    }
}

impl Serialize for u64 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_ulong(*self)
    }
}

impl Serialize for f32 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_float(*self)
    }
}

impl Serialize for f64 {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_double(*self)
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        match self {
            Some(v) => v.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl<T> Serialize for Variable<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        match self {
            Variable::Constant(v) => v.serialize(serializer),
            Variable::Reference { path, target } => serializer.serialize_variable(path, target),
        }
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(self.len())?;

        for item in self.iter() {
            seq.serialize_element(item)?;
        }

        seq.end()
    }
}

impl<T, const N: usize> Serialize for [T; N]
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(self.len())?;

        for item in self.iter() {
            seq.serialize_element(item)?;
        }

        seq.end()
    }
}
