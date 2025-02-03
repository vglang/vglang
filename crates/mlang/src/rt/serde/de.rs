use crate::rt::opcode::{Path, Target};

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
    #[error("unexpect type kind: data.")]
    Data,
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

pub trait OpcodeVisitor<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// The value produced by this visitor.
    type Opcode;

    /// visit element by type_id.
    fn visit_el<A>(self, type_id: usize, el: A) -> Result<Vec<Self::Opcode>, Self::Error>
    where
        A: ElementAccess<'de, Error = Self::Error>;

    /// visit element by display name.
    fn visit_el_with<A>(self, name: &str, el: A) -> Result<Vec<Self::Opcode>, Self::Error>
    where
        A: ElementAccess<'de, Error = Self::Error>;

    /// visit leaf by type_id.
    fn visit_leaf<A>(self, type_id: usize, el: A) -> Result<Vec<Self::Opcode>, Self::Error>
    where
        A: LeafAccess<'de, Error = Self::Error>;

    /// visit leaf by display name.
    fn visit_leaf_with<A>(self, name: &str, el: A) -> Result<Vec<Self::Opcode>, Self::Error>
    where
        A: LeafAccess<'de, Error = Self::Error>;

    fn visit_pop(self) -> Result<Self::Opcode, Self::Error>;
}

pub trait DataVisitor<'de>: Sized {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// The value produced by this visitor.
    type Value;

    /// visit complex data value.
    fn visit_data<A>(self, data: A) -> Result<Self::Value, Self::Error>
    where
        A: DataAccess<'de, Error = Self::Error>,
    {
        let _ = data;

        Err(Error::Unexpect(Kind::Data).into())
    }

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

    /// Visit node fields.
    fn visit_enum<A>(
        self,
        type_id: usize,
        variant: Option<&str>,
        variant_index: Option<usize>,
        node: A,
    ) -> Result<Self::Value, Self::Error>
    where
        A: EnumAccess<'de, Error = Self::Error>,
    {
        let _ = type_id;
        let _ = variant;
        let _ = variant_index;
        let _ = node;

        Err(Error::Unexpect(Kind::Enum).into())
    }

    /// Visit node fields.
    fn visit_enum_with<A>(
        self,
        name: &str,
        variant: Option<&str>,
        variant_index: Option<usize>,
        node: A,
    ) -> Result<Self::Value, Self::Error>
    where
        A: EnumAccess<'de, Error = Self::Error>,
    {
        let _ = name;
        let _ = variant;
        let _ = variant_index;
        let _ = node;
        Err(Error::Unexpect(Kind::Enum).into())
    }

    /// Visit sequence type.
    fn visit_seq<S>(self, name: &str, seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de, Error = Self::Error>,
    {
        let _ = name;
        let _ = seq;
        Err(Error::Unexpect(Kind::Node).into())
    }
}

/// Provides a Visitor access to sequence items.
pub trait SeqAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;
    /// deserialize node fields.
    fn next_item<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;
}

/// Provides a Visitor access to each entry of an attribute node in the input.
pub trait DataAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// deserialize a field.
    fn deserialize_field<V>(
        &mut self,
        index: usize,
        name: Option<&str>,
        visitor: V,
    ) -> Result<Option<V::Value>, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;
}

/// Provides a Visitor access to each entry of an attribute node in the input.
pub trait AttrAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// deserialize a field.
    fn deserialize_field<V>(
        &mut self,
        index: usize,
        name: Option<&str>,
        visitor: V,
    ) -> Result<Option<V::Value>, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;
}

/// Provides a Visitor access to each entry of an enum field node in the input.
pub trait EnumAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// deserialize a field.
    fn deserialize_field<V>(
        &mut self,
        index: usize,
        name: Option<&str>,
        visitor: V,
    ) -> Result<Option<V::Value>, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;
}

/// Provides a Visitor access to each entry of an element node in the input.
pub trait ElementAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// [`AttrAccess`] type returns by [`deserialize_attr`](ElementAccess::deserialize_attr)
    type AttrAccess: AttrAccess<'de, Error = Self::Error>;

    /// deserialize a field.
    fn deserialize_field<V>(
        &mut self,
        index: usize,
        name: Option<&str>,
        visitor: V,
    ) -> Result<Option<V::Value>, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;

    /// Deserialize a attr node.
    fn deserialize_attr(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::AttrAccess, Self::Error>;
}

/// Provides a Visitor access to each entry of a leaf node in the input.
pub trait LeafAccess<'de> {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// [`AttrAccess`] type returns by [`deserialize_attr`](ElementAccess::deserialize_attr)
    type AttrAccess: AttrAccess<'de, Error = Self::Error>;

    /// deserialize a field.
    fn deserialize_field<V>(
        &mut self,
        index: usize,
        name: Option<&str>,
        visitor: V,
    ) -> Result<Option<V::Value>, Self::Error>
    where
        V: DataVisitor<'de, Error = Self::Error>;

    /// Deserialize a attr node.
    fn deserialize_attr(
        &mut self,
        type_id: usize,
        name: &str,
    ) -> Result<Self::AttrAccess, Self::Error>;
}

/// A deserializer support deserialize data from any format.
pub trait Deserializer<'de>: Sized {
    /// Error type used by this trait.
    type Error: From<Error>;

    /// Deserialize opcodes.
    fn deserialize<V>(self, visitor: V) -> Result<Option<Vec<V::Opcode>>, Self::Error>
    where
        V: OpcodeVisitor<'de>;
}

/// Element node must implement this trait to support deserializing.
pub trait DeserializeElement<'de>: Sized {
    type Value;
    fn deserialize<A>(deserializer: A) -> Result<Self::Value, A::Error>
    where
        A: ElementAccess<'de>;
}

/// Leaf node must implement this trait to support deserializing.
pub trait DeserializeLeaf<'de>: Sized {
    type Value;
    fn deserialize<A>(deserializer: A) -> Result<Self::Value, A::Error>
    where
        A: LeafAccess<'de>;
}

/// Attr node must implement this trait to support deserializing.
pub trait DeserializeAttr<'de>: Sized {
    type Value;
    fn deserialize<A>(deserializer: A) -> Result<Self::Value, A::Error>
    where
        A: AttrAccess<'de>;
}

/// Attr node must implement this trait to support deserializing.
pub trait DeserializeData<'de>: Sized {
    type Value;
    fn deserialize<A>(deserializer: A) -> Result<Self::Value, A::Error>
    where
        A: DataAccess<'de>;
}

pub trait Deserialize<'de>: Sized {
    type Value;
    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
