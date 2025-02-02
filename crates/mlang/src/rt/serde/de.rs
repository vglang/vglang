use std::path::Path;

use crate::rt::opcode::Target;

/// Error used by [`Visitor`]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unexpect(Kind),
}

/// Unexpect kind .
#[derive(Debug, thiserror::Error)]
pub enum Kind {
    #[error("unexpect type kind: bool.")]
    Bool,
    #[error("unexpect type kind: string.")]
    String,
    #[error("unexpect type kind: byte.")]
    Byte,
    #[error("unexpect type kind: utype.")]
    Ubyte,
    #[error("unexpect type kind: short.")]
    Short,
    #[error("unexpect type kind: ushort.")]
    Ushort,
    #[error("unexpect type kind: int.")]
    Int,
    #[error("unexpect type kind: uint.")]
    Uint,
    #[error("unexpect type kind: long.")]
    Long,
    #[error("unexpect type kind: ulong.")]
    Ulong,
    #[error("unexpect type kind: float.")]
    Float,
    #[error("unexpect type kind: double.")]
    Double,
    #[error("unexpect type kind: enum.")]
    Enum,
    #[error("unexpect type kind: node.")]
    Node,
    #[error("unexpect type kind: seq.")]
    Seq,
    #[error("unexpect type kind: none.")]
    None,
    #[error("unexpect type kind: variable.")]
    Variable,

    #[error("unexpect type kind: pop.")]
    Pop,

    #[error("unexpect type kind: field.")]
    Field,
}

/// This trait represents a visitor that walks through a deserializer.
pub trait Visitor<'de>: Sized {
    type Value;

    /// visit bool value
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Bool).into())
    }

    /// Visit vglang `string`.
    fn visit_string<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::String).into())
    }

    /// Visit vglang `byte`.
    fn visit_byte<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Byte).into())
    }

    /// Visit vglang `ubyte`.
    fn visit_ubyte<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Ubyte).into())
    }

    /// Visit vglang `byte`.
    fn visit_short<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Short).into())
    }

    /// Visit vglang `ubyte`.
    fn visit_ushort<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Ushort).into())
    }

    /// Visit vglang `int`.
    fn visit_int<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Int).into())
    }

    /// Visit vglang `uint`.
    fn visit_uint<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Uint).into())
    }

    /// Visit vglang `long`.
    fn visit_long<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Long).into())
    }

    /// Visit vglang `ulong`.
    fn visit_ulong<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Ulong).into())
    }

    /// Visit vglang `long`.
    fn visit_float<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Float).into())
    }

    /// Visit vglang `double`.
    fn visit_double<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = value;

        Err(Error::Unexpect(Kind::Double).into())
    }

    /// Visit a none value.
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        Err(Error::Unexpect(Kind::None).into())
    }

    /// Visit a none value.
    fn visit_variable<E>(self, path: &Path, target: &Target) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        let _ = path;
        let _ = target;

        Err(Error::Unexpect(Kind::Variable).into())
    }

    /// Visit pop directive.
    fn visit_pop<E>(self) -> Result<Self::Value, E>
    where
        E: From<Error>,
    {
        Err(Error::Unexpect(Kind::Pop).into())
    }

    /// Visit node fields.
    fn visit_node<A>(self, type_id: usize, node: A) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = type_id;
        let _ = node;
        Err(Error::Unexpect(Kind::Node).into())
    }

    /// Visit node fields.
    fn visit_node_by_name<A>(self, name: &str, node: A) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = name;
        let _ = node;
        Err(Error::Unexpect(Kind::Node).into())
    }

    /// Visit node fields.
    fn visit_enum<A>(
        self,
        type_id: usize,
        variant: Option<&str>,
        variant_index: Option<usize>,
        node: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = type_id;
        let _ = variant;
        let _ = variant_index;
        let _ = node;

        Err(Error::Unexpect(Kind::Enum).into())
    }

    /// Visit node fields.
    fn visit_enum_by_name<A>(
        self,
        name: &str,
        variant: Option<&str>,
        variant_index: Option<usize>,
        node: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = name;
        let _ = variant;
        let _ = variant_index;
        let _ = node;
        Err(Error::Unexpect(Kind::Enum).into())
    }

    /// Visit node field.
    fn visit_node_field<A>(self, index: usize, node: A) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = index;
        let _ = node;
        Err(Error::Unexpect(Kind::Node).into())
    }

    /// Visit node field.
    fn visit_node_field_by_name<A>(self, name: &str, node: A) -> Result<Self::Value, A::Error>
    where
        A: NodeAccess<'de>,
    {
        let _ = name;
        let _ = node;
        Err(Error::Unexpect(Kind::Node).into())
    }

    /// Visit sequence type.
    fn visit_seq<S>(self, name: &str, seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let _ = name;
        let _ = seq;
        Err(Error::Unexpect(Kind::Node).into())
    }
}

/// Provides a Visitor access to sequence items.
pub trait SeqAccess<'de>: Deserializer<'de> {
    /// deserialize node fields.
    fn next_item<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

/// Provides a Visitor access to each field of a node in the input.
pub trait NodeAccess<'de>: Deserializer<'de> {
    /// deserialize node fields.
    fn next_field<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

/// A deserializer support deserialize data from any format.
pub trait Deserializer<'de>: Sized {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// Require the Deserializer to figure out how to drive the visitor based on what data type is in the input.
    fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the Deserialize type is expecting a sequence of values.
    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the Deserialize type is expecting a sequence of values.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the Deserialize type is expecting a node value.
    fn deserialize_node<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `string`.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `string`.
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `byte`.
    fn deserialize_byte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `ubyte`.
    fn deserialize_ubyte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `byte`.
    fn deserialize_short<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `ubyte`.
    fn deserialize_ushort<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `int`.
    fn deserialize_int<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `uint`.
    fn deserialize_uint<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `long`.
    fn deserialize_long<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `ulong`.
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `long`.
    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize vglang `double`.
    fn deserialize_double<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
