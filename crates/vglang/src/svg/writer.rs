use heck::ToLowerCamelCase;
use ml::rt::{
    opcode::variable,
    serde::{Serialize, SerializeEnum, SerializeNode, SerializeSeq, Serializer},
};

use crate::opcode::Opcode;

use xml_builder::{XMLBuilder, XMLElement, XMLError, XMLVersion, XML};

#[derive(Debug, thiserror::Error)]
pub enum SvgWriteError {
    #[error(transparent)]
    XmlError(#[from] XMLError),
    #[error("Unsafisfied variable path={0:?}, target={1:?}")]
    Unsatisfied(variable::Path, variable::Target),
}

enum Processing {
    El(usize),
    Leaf(usize),
    Attr(usize),
    Data(usize),
    Enum(String, usize),
    Seq(usize),
}

struct SvgWriter {
    /// rendering xml element stack.
    els: Vec<XMLElement>,
    /// xml document.
    document: XML,
    /// processing status stack.
    processing: Vec<Processing>,
    /// Value cached stack.
    vals: Vec<Option<String>>,
    /// defs stack.
    defs: Vec<bool>,
    /// the caching of defs node.
    defs_node_cache: Vec<XMLElement>,
}

impl SvgWriter {
    fn new() -> Self {
        Self {
            els: Default::default(),
            processing: Default::default(),
            vals: Default::default(),
            defs: Default::default(),
            defs_node_cache: Default::default(),
            document: XMLBuilder::new()
                .version(XMLVersion::XML1_0)
                .encoding("UTF-8".into())
                .build(),
        }
    }
    fn end(self) -> Result<Vec<u8>, SvgWriteError> {
        assert_eq!(self.els.len(), 0);
        let mut writer: Vec<u8> = Vec::new();
        self.document.generate(&mut writer)?;

        Ok(writer)
    }

    fn normalize_tag(&self, tag: &str) -> String {
        match tag {
            "Canvas" => "svg".to_string(),
            _ => tag.to_lower_camel_case(),
        }
    }

    fn add_attribute(&mut self, name: &str, value: String) {
        self.els.last_mut().unwrap().add_attribute(name, &value);
    }
}

impl<'a> SerializeNode for &'a mut SvgWriter {
    type Error = SvgWriteError;

    fn serialize_field<T>(
        &mut self,
        index: usize,
        name: Option<&str>,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let processing = self.processing.pop().unwrap();

        match processing {
            Processing::El(mut fields) => {
                if let Some(name) = name {
                    value.serialize(&mut **self)?;
                    if let Some(value) = self
                        .vals
                        .pop()
                        .expect(&format!("process el attr({})", name))
                    {
                        self.add_attribute(name, value);
                    }
                }

                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::El(fields));
                }
            }
            Processing::Leaf(mut fields) => {
                if let Some(name) = name {
                    value.serialize(&mut **self)?;
                    if let Some(value) = self
                        .vals
                        .pop()
                        .expect(&format!("process leaf attr({})", name))
                    {
                        self.add_attribute(name, value);
                    }
                }

                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::Leaf(fields));
                } else {
                    self.serialize_pop()?;
                }
            }
            Processing::Attr(mut fields) => {
                if let Some(name) = name {
                    value.serialize(&mut **self)?;
                    if let Some(value) = self
                        .vals
                        .pop()
                        .expect(&format!("process leaf attr({})", name))
                    {}
                }
                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::Attr(fields));
                }
            }
            Processing::Data(mut fields) => {
                value.serialize(&mut **self)?;
                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::Data(fields));
                }
            }
            Processing::Enum(v, mut fields) => {
                value.serialize(&mut **self)?;

                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::Enum(v, fields));
                }
            }
            Processing::Seq(mut fields) => {
                value.serialize(&mut **self)?;
                fields -= 1;

                if fields > 0 {
                    self.processing.push(Processing::Seq(fields));
                }
            }
        }

        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut SvgWriter {
    type Error = SvgWriteError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> Serializer for &'a mut SvgWriter {
    type Error = SvgWriteError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.els.push(XMLElement::new(&self.normalize_tag(name)));
        self.defs.push(false);

        if fields > 0 {
            self.processing.push(Processing::El(fields));
        }

        Ok(self)
    }

    fn serialize_leaf(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.defs.push(false);
        self.els.push(XMLElement::new(&self.normalize_tag(name)));

        if fields > 0 {
            self.processing.push(Processing::Leaf(fields));
        }

        Ok(self)
    }

    fn serialize_attr(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(fields > 0, "zero fields attr.");
        self.processing.push(Processing::Attr(fields));
        Ok(self)
    }

    fn serialize_data(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(fields > 0, "zero fields data.");
        self.processing.push(Processing::Data(fields));
        Ok(self)
    }

    fn serialize_enum(
        self,
        type_id: usize,
        name: &str,
        variant: &str,
        variant_index: usize,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        if fields == 0 {
            self.vals.push(Some(variant.to_lower_camel_case()));
        } else {
            self.processing
                .push(Processing::Enum(variant.to_string(), fields));
        }

        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_string(self, value: &str) -> Result<(), Self::Error> {
        self.vals.push(Some(value.to_string()));

        Ok(())
    }

    fn serialize_byte(self, value: i8) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_ubyte(self, value: u8) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_short(self, value: i16) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_ushort(self, value: u16) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_int(self, value: i32) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_uint(self, value: u32) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_long(self, value: i64) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_ulong(self, value: u64) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_float(self, value: f32) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_double(self, value: f64) -> Result<(), Self::Error> {
        self.vals.push(Some(format!("{}", value)));

        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.vals.push(None);

        Ok(())
    }

    fn serialize_variable(
        self,
        path: &ml::rt::opcode::variable::Path,
        target: &ml::rt::opcode::variable::Target,
    ) -> Result<(), Self::Error> {
        Err(SvgWriteError::Unsatisfied(path.clone(), target.clone()))
    }

    fn serialize_seq(self, len: usize) -> Result<Self::SerializeSeq, Self::Error> {
        self.processing.push(Processing::Seq(len));
        Ok(self)
    }

    fn serialize_pop(self) -> Result<(), Self::Error> {
        let mut element = self.els.pop().expect("el stack.");
        let defs = self.defs.pop().expect("defs pop.");

        if defs {
            self.defs_node_cache.push(element);
            return Ok(());
        }

        if let Some(last) = self.els.last_mut() {
            last.add_child(element)?;
        } else {
            if !self.defs_node_cache.is_empty() {
                let mut defs = XMLElement::new("defs");

                for node in self.defs_node_cache.drain(..) {
                    defs.add_child(node)?;
                }

                element.add_child(defs)?;
            }

            element.add_attribute("xmlns", "http://www.w3.org/2000/svg");
            element.add_attribute("version", "1.1");
            element.add_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");

            self.document.set_root_element(element);
        }

        Ok(())
    }
}

/// Convert vglang opcodes into svg image.
pub fn to_svg<O: AsRef<[Opcode]>>(opcodes: O) -> Result<Vec<u8>, SvgWriteError> {
    let mut writer = SvgWriter::new();

    for opcode in opcodes.as_ref() {
        opcode.serialize(&mut writer)?;
    }

    writer.end()
}
