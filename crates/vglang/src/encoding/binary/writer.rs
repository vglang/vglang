//! A memory-optimised vglang intermediate code format

use std::io::{Cursor, Write};

use ml::rt::serde::ser::{Serialize, SerializeNode, SerializeSeq, Serializer};
use varint_rs::VarintWriter;

use crate::opcode::Opcode;

#[derive(Debug, thiserror::Error)]
pub enum BinaryWriteError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[repr(u8)]
pub enum Tag {
    None,
    Pop,
    Bool,
    String,
    Byte,
    Ubyte,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    List,
    Variable,
}

#[derive(Default)]
struct BinaryWriter {
    write_buf: Cursor<Vec<u8>>,
}

impl BinaryWriter {
    fn end(self) -> Result<Vec<u8>, BinaryWriteError> {
        Ok(self.write_buf.into_inner())
    }
}

impl<'a> Serializer for &'a mut BinaryWriter {
    type Error = BinaryWriteError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        type_id: usize,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.write_buf.write_u8_varint(type_id as u8)?;
        Ok(self)
    }

    fn serialize_leaf(
        self,
        type_id: usize,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.write_buf.write_u8_varint(type_id as u8)?;
        Ok(self)
    }

    fn serialize_attr(
        self,
        type_id: usize,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.write_buf.write_u8_varint(type_id as u8)?;
        Ok(self)
    }

    fn serialize_data(
        self,
        type_id: usize,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.write_buf.write_u8_varint(type_id as u8)?;
        Ok(self)
    }

    fn serialize_enum(
        self,
        type_id: usize,
        _: &str,
        _: &str,
        variant_index: usize,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.write_buf.write_u8_varint(type_id as u8)?;
        self.write_buf.write_u8_varint(variant_index as u8)?;
        Ok(self)
    }

    fn serialize_seq(self, len: usize) -> Result<Self::SerializeSeq, Self::Error> {
        self.write_buf.write_u8_varint(Tag::List as u8)?;
        self.write_buf.write_u32_varint(len as u32)?;
        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Bool as u8)?;
        self.write_buf.write_u8_varint(if value { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_string(self, value: &str) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::String as u8)?;
        self.write_buf.write_u16_varint(value.len() as u16)?;
        self.write_buf.write_all(value.as_bytes())?;
        Ok(())
    }

    fn serialize_byte(self, value: i8) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Byte as u8)?;
        self.write_buf.write_u8_varint(value as u8)?;
        Ok(())
    }

    fn serialize_ubyte(self, value: u8) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Ubyte as u8)?;
        self.write_buf.write_u8_varint(value as u8)?;
        Ok(())
    }

    fn serialize_short(self, value: i16) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Short as u8)?;
        self.write_buf.write_u16_varint(value as u16)?;
        Ok(())
    }

    fn serialize_ushort(self, value: u16) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Ushort as u8)?;
        self.write_buf.write_u16_varint(value as u16)?;
        Ok(())
    }

    fn serialize_int(self, value: i32) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Int as u8)?;
        self.write_buf.write_u32_varint(value as u32)?;
        Ok(())
    }

    fn serialize_uint(self, value: u32) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Uint as u8)?;
        self.write_buf.write_u32_varint(value as u32)?;
        Ok(())
    }

    fn serialize_long(self, value: i64) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Long as u8)?;
        self.write_buf.write_u64_varint(value as u64)?;
        Ok(())
    }

    fn serialize_ulong(self, value: u64) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Ulong as u8)?;
        self.write_buf.write_u64_varint(value as u64)?;
        Ok(())
    }

    fn serialize_float(self, value: f32) -> Result<(), Self::Error> {
        if value.trunc() == value {
            self.write_buf.write_u8_varint(Tag::Int as u8)?;
            self.write_buf.write_u32_varint(value as u32)?;
        } else {
            self.write_buf.write_u8_varint(Tag::Float as u8)?;
            self.write_buf.write_all(&value.to_be_bytes())?;
        }

        Ok(())
    }

    fn serialize_double(self, value: f64) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Double as u8)?;
        self.write_buf.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::None as u8)?;
        Ok(())
    }

    fn serialize_variable(
        self,
        path: &ml::rt::opcode::variable::Path,
        target: &ml::rt::opcode::variable::Target,
    ) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Variable as u8)?;
        match path {
            ml::rt::opcode::variable::Path::Named(v) => {
                self.serialize_string(v)?;
            }
            ml::rt::opcode::variable::Path::Index(v) => {
                self.serialize_uint(*v as u32)?;
            }
        }

        self.write_buf.write_u8_varint((*target) as u8)?;

        Ok(())
    }

    fn serialize_pop(self) -> Result<(), Self::Error> {
        self.write_buf.write_u8_varint(Tag::Pop as u8)?;

        Ok(())
    }
}

impl<'a> SerializeNode for &'a mut BinaryWriter {
    type Error = BinaryWriteError;

    fn serialize_field<T>(
        &mut self,
        index: usize,
        _: Option<&str>,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + ml::rt::serde::ser::Serialize,
    {
        self.write_buf.write_u8_varint(index as u8)?;
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut BinaryWriter {
    type Error = BinaryWriteError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ml::rt::serde::ser::Serialize,
    {
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Convert opcodes into binary format.
pub fn to_binary<O: AsRef<[Opcode]>>(opcodes: O) -> Result<Vec<u8>, BinaryWriteError> {
    let mut writer = BinaryWriter::default();

    for opcode in opcodes.as_ref() {
        opcode.serialize(&mut writer)?;
    }

    writer.end()
}
