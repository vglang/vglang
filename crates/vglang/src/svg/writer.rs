use heck::ToLowerCamelCase;
use ml::rt::{
    opcode::variable::{self, Path, Target},
    serde::{Serialize, SerializeNode, SerializeSeq, Serializer},
};
use xml_dom::level2::{
    ext::{get_implementation_ext, DocumentDecl, XmlDecl},
    Document, Element, Node,
};

use crate::opcode::Opcode;

#[derive(Debug, thiserror::Error)]
pub enum SvgWriteError {
    #[error(transparent)]
    XmlError(#[from] xml_dom::level2::Error),
    #[error("Unsafisfied variable path={0:?}, target={1:?}")]
    Unsatisfied(variable::Path, variable::Target),
}

/// Processing state of the writer.
enum SvgWriterState {
    El(String),
    Leaf(String),
    Attr(String),
    ViewBox,
    PreserveAspectRatio(String),
    Rgb,
    FuncIri,
    Point,
    Percent,
    NumberOptNumber,
    Characters,
    BackgroundNew,
    Enum(usize, String),
    /// serialize length enum.
    Length(String),
    /// serialize path event enum.
    #[allow(unused)]
    PathEvent(String),
    /// serialize transform enum.
    Transform(String),
    Seq(usize),
    IriLocal,
    IriPath,
}

struct SvgWriter {
    /// rendering xml element stack.
    els: Vec<xml_dom::level2::RefNode>,
    /// xml document.
    document: xml_dom::level2::RefNode,
    /// event processing stack.
    state_stack: Vec<SvgWriterState>,
    /// Applying attribute/value pairs.
    attrs: Vec<(String, String)>,
    /// Processing value stack.
    values: Vec<Option<String>>,
    /// defs stack.
    defs: Vec<bool>,
    /// the caching of defs node.
    defs_node_cache: Vec<xml_dom::level2::RefNode>,
}

impl SvgWriter {
    fn new() -> Self {
        let implementation = get_implementation_ext();

        let mut document = implementation
            .create_document(None, Some("svg"), None)
            .unwrap();

        let decl = XmlDecl::new(
            xml_dom::level2::ext::XmlVersion::V10,
            Some("utf-8".to_string()),
            Some(true),
        );

        document.set_xml_declaration(decl).unwrap();

        Self {
            els: Default::default(),
            document,
            state_stack: Default::default(),
            attrs: Default::default(),
            values: Default::default(),
            defs: Default::default(),
            defs_node_cache: Default::default(),
        }
    }
    fn end(self) -> Result<Vec<u8>, SvgWriteError> {
        assert_eq!(self.els.len(), 0);
        assert_eq!(self.defs_node_cache.len(), 0);

        Ok(self.document.to_string().as_bytes().to_vec())
    }

    fn new_xml_element(&mut self, name: &str) {
        let mut el = if self.els.is_empty() {
            assert_eq!(name, "svg", "root element must `svg`");
            self.document.document_element().unwrap()
        } else {
            self.document.create_element(name).unwrap()
        };

        let mut defs = false;

        for (name, value) in self.attrs.drain(..) {
            if name == "id" {
                defs = true;
            }

            el.set_attribute(&name, &value).unwrap();
        }

        self.defs.push(defs);

        self.els.push(el);
    }
}

impl<'a> Serializer for &'a mut SvgWriter {
    type Error = SvgWriteError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        let name = name.to_lower_camel_case();
        self.new_xml_element(&name);
        self.state_stack.push(SvgWriterState::El(name));

        Ok(self)
    }

    fn serialize_leaf(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        match name {
            "Characters" => {
                self.state_stack.push(SvgWriterState::Characters);
            }
            _ => {
                let name = name.to_lower_camel_case();
                self.new_xml_element(&name);
                self.state_stack.push(SvgWriterState::Leaf(name));
            }
        }

        Ok(self)
    }

    fn serialize_attr(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        match name {
            "ViewBox" => {
                self.state_stack.push(SvgWriterState::ViewBox);
            }
            _ => {
                self.state_stack
                    .push(SvgWriterState::Attr(name.to_lower_camel_case()));
            }
        }

        Ok(self)
    }

    fn serialize_data(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        match name {
            "Rgb" => {
                self.state_stack.push(SvgWriterState::Rgb);
            }
            "FuncIri" => {
                self.state_stack.push(SvgWriterState::FuncIri);
            }
            "Point" => {
                self.state_stack.push(SvgWriterState::Point);
            }
            "Percent" => {
                self.state_stack.push(SvgWriterState::Percent);
            }
            "NumberOptNumber" => {
                self.state_stack.push(SvgWriterState::NumberOptNumber);
            }
            "BackgroundNew" => {
                self.state_stack.push(SvgWriterState::BackgroundNew);
            }
            _ => {
                panic!("Unsupport data `{}`", name);
            }
        }

        Ok(self)
    }

    fn serialize_enum(
        self,
        _: usize,
        name: &str,
        variant: &str,
        _: usize,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        match name {
            "PathEvent" => {
                self.state_stack
                    .push(SvgWriterState::PathEvent(variant.to_string()));
            }
            "Transform" => {
                self.state_stack.push(SvgWriterState::Transform(
                    variant.to_lower_camel_case().to_string(),
                ));
            }
            "Length" => {
                self.state_stack
                    .push(SvgWriterState::Length(variant.to_lowercase().to_string()));
            }
            "Iri" => match variant {
                "Path" => {
                    self.state_stack.push(SvgWriterState::IriPath);
                }
                "Local" => {
                    self.state_stack.push(SvgWriterState::IriLocal);
                }
                _ => {
                    panic!("unsupport iri(`{}`)", variant);
                }
            },
            "PreserveAspectRatio" => {
                self.state_stack
                    .push(SvgWriterState::PreserveAspectRatio(variant.to_string()));
            }
            _ => {
                if fields > 1 {
                    panic!("Unsupport enum {}::{}, fields == {}", name, variant, fields);
                }

                self.state_stack
                    .push(SvgWriterState::Enum(fields, variant.to_lower_camel_case()));
            }
        }
        Ok(self)
    }

    fn serialize_seq(self, len: usize) -> Result<Self::SerializeSeq, Self::Error> {
        self.state_stack.push(SvgWriterState::Seq(len));
        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> Result<(), Self::Error> {
        self.values
            .push(Some(format!("{}", if value { 1 } else { 0 })));
        Ok(())
    }

    fn serialize_string(self, value: &str) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_byte(self, value: i8) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_ubyte(self, value: u8) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_short(self, value: i16) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_ushort(self, value: u16) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_int(self, value: i32) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_uint(self, value: u32) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_long(self, value: i64) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_ulong(self, value: u64) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_float(self, value: f32) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_double(self, value: f64) -> Result<(), Self::Error> {
        self.values.push(Some(format!("{}", value)));
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.values.push(None);
        Ok(())
    }

    fn serialize_variable(self, path: &Path, target: &Target) -> Result<(), Self::Error> {
        return Err(SvgWriteError::Unsatisfied(path.clone(), target.clone()));
    }

    fn serialize_pop(self) -> Result<(), Self::Error> {
        let mut element = self.els.pop().expect("el stack.");
        let defs = self.defs.pop().expect("defs pop.");

        if defs {
            self.defs_node_cache.push(element);
            return Ok(());
        }

        if let Some(last) = self.els.last_mut() {
            last.append_child(element)?;
        } else {
            if !self.defs_node_cache.is_empty() {
                let mut defs = self.document.create_element("defs")?;

                for node in self.defs_node_cache.drain(..) {
                    defs.append_child(node)?;
                }

                element.append_child(defs)?;
            }

            element.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
            element.set_attribute("version", "1.1")?;
            element.set_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink")?;
        }

        Ok(())
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
        let state = self.state_stack.pop().expect("inner error.");

        match &state {
            SvgWriterState::El(ty) | SvgWriterState::Leaf(ty) => {
                if let Some(name) = name {
                    value.serialize(&mut **self)?;
                    if let Some(value) = self.values.pop().expect("serialize value,inner error.") {
                        self.els.last_mut().unwrap().set_attribute(name, &value)?;
                    } else {
                        log::info!("skip serialize {}:{}, value is none.", ty, name);
                    }
                } else {
                    log::warn!(
                        "skip serialize {}:attr({}), display name is none.",
                        ty,
                        index
                    );
                }
            }
            SvgWriterState::Attr(ty) => {
                if let Some(name) = name {
                    value.serialize(&mut **self)?;
                    if let Some(value) = self.values.pop().expect("serialize value,inner error.") {
                        self.attrs.push((name.to_string(), value));
                    } else {
                        log::info!("skip serialize {}:{}, value is none.", ty, name);
                    }
                } else {
                    log::warn!(
                        "skip serialize {}:attr({}), display name is none.",
                        ty,
                        index
                    );
                }
            }
            SvgWriterState::Length(variant) => {
                value.serialize(&mut **self)?;

                if variant != "px" {
                    let value = self
                        .values
                        .pop()
                        .expect("serialize length,inner error.")
                        .expect("length field can not be none.");

                    if variant == "percent" {
                        self.values.push(Some(format!("{}%", value)));
                    } else {
                        self.values.push(Some(format!("{}{}", value, variant)));
                    }
                }
            }
            _ => {
                value.serialize(&mut **self)?;
            }
        }

        self.state_stack.push(state);

        Ok(())
    }

    fn finish(self) -> Result<(), Self::Error> {
        match self.state_stack.pop().expect("node finish pop up state.") {
            SvgWriterState::Enum(fields, variant) => {
                if fields == 0 {
                    self.values.push(Some(variant));
                }
            }
            SvgWriterState::BackgroundNew => {
                assert!(self.values.len() >= 4, "background new fields == 4");
                let values = self.values.split_off(self.values.len() - 4);

                self.values.push(Some(format!(
                    "new {} {} {} {}",
                    values[0].as_ref().expect("transform value 0 is none"),
                    values[1].as_ref().expect("transform value 1 is none"),
                    values[2].as_ref().expect("transform value 2 is none"),
                    values[3].as_ref().expect("transform value 3 is none"),
                )));
            }
            SvgWriterState::ViewBox => {
                assert!(self.values.len() >= 5, "ViewBox fields == 5");
                let mut values = self.values.split_off(self.values.len() - 5);

                let viewbox = format!(
                    "{} {} {} {}",
                    values[0].as_ref().expect("transform value 0 is none"),
                    values[1].as_ref().expect("transform value 1 is none"),
                    values[2].as_ref().expect("transform value 2 is none"),
                    values[3].as_ref().expect("transform value 3 is none"),
                );

                self.attrs.push(("viewBox".to_string(), viewbox));

                if let Some(aspect) = values.pop().unwrap() {
                    self.attrs
                        .push(("preserveAspectRatio".to_string(), aspect.to_string()));
                }
            }
            SvgWriterState::PreserveAspectRatio(variant) => match variant.as_str() {
                "none" => {
                    self.values.push(Some("none".to_string()));
                }
                _ => {
                    let value = self
                        .values
                        .pop()
                        .expect("PreserveAspectRatio fields == 1.")
                        .expect("PreserveAspectRatio field is none");

                    self.values.push(Some(format!("{} {}", variant, value)));
                }
            },
            SvgWriterState::PathEvent(variant) => match variant.as_str() {
                "Close" => {
                    self.values.push(Some("Z".to_string()));
                }
                "MoveTo" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(moveto) fields == 1.")
                        .expect("path(moveto) field 0 is none");

                    self.values.push(Some(format!("M {}", value)));
                }
                "MoveToRelative" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(moveto relative) fields == 1.")
                        .expect("path(moveto relative) field 0 is none");

                    self.values.push(Some(format!("m {}", value)));
                }
                "LineTo" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(lineto) fields == 1.")
                        .expect("path(lineto) field 0 is none");

                    self.values.push(Some(format!("L {}", value)));
                }
                "LineToRelative" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(lineto relative) fields == 1.")
                        .expect("path(lineto relative) field 0 is none");

                    self.values.push(Some(format!("l {}", value)));
                }
                "Polyline" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(polyline) fields == 1.")
                        .expect("path(polyline) field 0 is none");

                    self.values.push(Some(format!("L {}", value)));
                }
                "PolylineRelative" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(polyline relative) fields == 1.")
                        .expect("path(polyline relative) field 0 is none");

                    self.values.push(Some(format!("l {}", value)));
                }
                "CubicBezier" => {
                    assert!(self.values.len() >= 3, "path(cubic) fields == 3");
                    let values = self.values.split_off(self.values.len() - 3);

                    self.values.push(Some(format!(
                        "C {} {} {}",
                        values[0].as_ref().expect("path(cubic) value 0 is none"),
                        values[1].as_ref().expect("path(cubic) value 1 is none"),
                        values[2].as_ref().expect("path(cubic) value 2 is none"),
                    )));
                }
                "CubicBezierRelative" => {
                    assert!(self.values.len() >= 3, "path(cubic,relative) fields == 3");
                    let values = self.values.split_off(self.values.len() - 3);

                    self.values.push(Some(format!(
                        "c {} {} {}",
                        values[0]
                            .as_ref()
                            .expect("path(cubic,relative) value 0 is none"),
                        values[1]
                            .as_ref()
                            .expect("path(cubic,relative) value 1 is none"),
                        values[2]
                            .as_ref()
                            .expect("path(cubic,relative) value 2 is none"),
                    )));
                }
                "CubicBezierSmooth" => {
                    assert!(self.values.len() >= 2, "path(cubic,smooth) fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    self.values.push(Some(format!(
                        "S {} {}",
                        values[0]
                            .as_ref()
                            .expect("path(cubic,smooth) value 0 is none"),
                        values[1]
                            .as_ref()
                            .expect("path(cubic,smooth) value 1 is none"),
                    )));
                }
                "CubicBezierSmoothRelative" => {
                    assert!(
                        self.values.len() >= 2,
                        "path(cubic,smooth,relative) fields == 2"
                    );
                    let values = self.values.split_off(self.values.len() - 2);

                    self.values.push(Some(format!(
                        "s {} {}",
                        values[0]
                            .as_ref()
                            .expect("path(cubic,smooth,relative) value 0 is none"),
                        values[1]
                            .as_ref()
                            .expect("path(cubic,smooth,relative) value 1 is none"),
                    )));
                }
                "QuadraticBezier" => {
                    assert!(self.values.len() >= 2, "path(quad) fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    self.values.push(Some(format!(
                        "Q {} {}",
                        values[0].as_ref().expect("path(quad) value 0 is none"),
                        values[1].as_ref().expect("path(quad) value 1 is none"),
                    )));
                }
                "QuadraticBezierRelative" => {
                    assert!(self.values.len() >= 2, "path(quad,relative) fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    self.values.push(Some(format!(
                        "q {} {}",
                        values[0]
                            .as_ref()
                            .expect("path(quad,relative) value 0 is none"),
                        values[1]
                            .as_ref()
                            .expect("path(quad,relative) value 1 is none"),
                    )));
                }
                "QuadraticBezierSmooth" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(quad,smooth) fields == 1.")
                        .expect("path(quad,smooth) field 0 is none");

                    self.values.push(Some(format!("T {}", value)));
                }
                "QuadraticBezierSmoothRelative" => {
                    let value = self
                        .values
                        .pop()
                        .expect("path(quad,smooth,releative) fields == 1.")
                        .expect("path(quad,smooth,releative) field 0 is none");

                    self.values.push(Some(format!("t {}", value)));
                }
                "Arc" => {
                    assert!(self.values.len() >= 6, "path(arc) fields == 6");
                    let values = self.values.split_off(self.values.len() - 6);

                    self.values.push(Some(format!(
                        "A {} {} {} {} {} {}",
                        values[0].as_ref().expect("path(arc) value 0 is none"),
                        values[1].as_ref().expect("path(arc) value 1 is none"),
                        values[2].as_ref().expect("path(arc) value 2 is none"),
                        values[3].as_ref().expect("path(arc) value 3 is none"),
                        values[4].as_ref().expect("path(arc) value 4 is none"),
                        values[5].as_ref().expect("path(arc) value 5 is none"),
                    )));
                }
                "ArcRelative" => {
                    assert!(self.values.len() >= 6, "path(arc,relative) fields == 6");
                    let values = self.values.split_off(self.values.len() - 6);

                    self.values.push(Some(format!(
                        "a {} {} {} {} {} {}",
                        values[0]
                            .as_ref()
                            .expect("path(arc,relative) value 0 is none"),
                        values[1]
                            .as_ref()
                            .expect("path(arc,relative) value 1 is none"),
                        values[2]
                            .as_ref()
                            .expect("path(arc,relative) value 2 is none"),
                        values[3]
                            .as_ref()
                            .expect("path(arc,relative) value 3 is none"),
                        values[4]
                            .as_ref()
                            .expect("path(arc,relative) value 4 is none"),
                        values[5]
                            .as_ref()
                            .expect("path(arc,relative) value 5 is none"),
                    )));
                }
                _ => {
                    panic!("Unsupport path event `{}`", variant);
                }
            },
            SvgWriterState::Transform(variant) => match variant.as_str() {
                "matrix" => {
                    assert!(self.values.len() >= 6, "matrix fields == 6");
                    let values = self.values.split_off(self.values.len() - 6);

                    self.values.push(Some(format!(
                        "matrix({},{},{},{},{},{})",
                        values[0].as_ref().expect("transform value 0 is none"),
                        values[1].as_ref().expect("transform value 1 is none"),
                        values[2].as_ref().expect("transform value 2 is none"),
                        values[3].as_ref().expect("transform value 3 is none"),
                        values[4].as_ref().expect("transform value 4 is none"),
                        values[5].as_ref().expect("transform value 5 is none"),
                    )));
                }
                "translate" => {
                    assert!(self.values.len() >= 2, "translate fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    if let Some(v2) = &values[1] {
                        self.values.push(Some(format!(
                            "translate({},{})",
                            values[0].as_ref().expect("translate value 0 is none"),
                            v2,
                        )));
                    } else {
                        self.values.push(Some(format!(
                            "translate({})",
                            values[0].as_ref().expect("translate value 0 is none"),
                        )));
                    }
                }
                "scale" => {
                    assert!(self.values.len() >= 2, "scale fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    if let Some(v2) = &values[1] {
                        self.values.push(Some(format!(
                            "scale({},{})",
                            values[0].as_ref().expect("scale value 0 is none"),
                            v2,
                        )));
                    } else {
                        self.values.push(Some(format!(
                            "scale({})",
                            values[0].as_ref().expect("scale value 0 is none"),
                        )));
                    }
                }
                "rotate" => {
                    assert!(self.values.len() >= 2, "rotate fields == 2");
                    let values = self.values.split_off(self.values.len() - 2);

                    if let Some(v2) = &values[1] {
                        self.values.push(Some(format!(
                            "rotate({},{})",
                            values[0].as_ref().expect("rotate value 0 is none"),
                            v2,
                        )));
                    } else {
                        self.values.push(Some(format!(
                            "rotate({})",
                            values[0].as_ref().expect("rotate value 0 is none"),
                        )));
                    }
                }
                "skewX" => {
                    assert!(self.values.len() >= 1, "skewX fields == 1");
                    let values = self.values.split_off(self.values.len() - 1);
                    self.values.push(Some(format!(
                        "skewX({})",
                        values[0].as_ref().expect("skewX value 0 is none"),
                    )));
                }
                "skewY" => {
                    assert!(self.values.len() >= 1, "skewY fields == 1");
                    let values = self.values.split_off(self.values.len() - 1);
                    self.values.push(Some(format!(
                        "skewY({})",
                        values[0].as_ref().expect("skewY value 0 is none"),
                    )));
                }
                _ => {
                    panic!("Unsupport transform `{}`", variant);
                }
            },
            SvgWriterState::Rgb => {
                assert!(self.values.len() >= 3, "rgb fields == 3");
                let values = self.values.split_off(self.values.len() - 3);
                self.values.push(Some(format!(
                    "rgb({},{},{})",
                    values[0].as_ref().expect("rgb value 0 is none"),
                    values[1].as_ref().expect("rgb value 1 is none"),
                    values[2].as_ref().expect("rgb value 2 is none"),
                )));
            }
            SvgWriterState::FuncIri => {
                let value = self
                    .values
                    .pop()
                    .expect("FuncIri fields == 1.")
                    .expect("FuncIri field is none");

                self.values.push(Some(format!("url(#{})", value)));
            }
            SvgWriterState::Point => {
                assert!(self.values.len() >= 2, "point fields == 2");
                let values = self.values.split_off(self.values.len() - 2);
                self.values.push(Some(format!(
                    "{},{}",
                    values[0].as_ref().expect("point field 0 is none"),
                    values[1].as_ref().expect("point field 1 is none"),
                )));
            }
            SvgWriterState::Percent => {
                let value = self
                    .values
                    .pop()
                    .expect("Percent fields == 1.")
                    .expect("Percent field is none");

                self.values.push(Some(format!("{}%", value)));
            }
            SvgWriterState::NumberOptNumber => {
                assert!(self.values.len() >= 2, "NumberOptNumber fields == 2");

                if let Some(v1) = self.values.pop().unwrap() {
                    let v0 = self
                        .values
                        .pop()
                        .unwrap()
                        .expect("NumberOptNumber field 0 is none.");

                    self.values.push(Some(format!("{},{}", v0, v1)));
                }
            }
            SvgWriterState::IriLocal => {
                let value = self
                    .values
                    .pop()
                    .expect("Iri::Local fields == 1.")
                    .expect("Iri::Local field 0 is none");

                self.values.push(Some(format!("#{}", value)));
            }
            SvgWriterState::Characters => {
                let value = self
                    .values
                    .pop()
                    .expect("Characters fields == 1.")
                    .expect("Characters field 0 is none");

                self.els
                    .last_mut()
                    .expect("Characters is root")
                    .append_child(self.document.create_text_node(&value))?;
            }
            SvgWriterState::Leaf(_) => {
                self.serialize_pop()?;
            }
            _ => {}
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
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn finish(self) -> Result<(), Self::Error> {
        match self.state_stack.pop().expect("Seq push.") {
            SvgWriterState::Seq(len) => {
                assert!(self.values.len() >= len, "seq element N mismatch.");
                let values = self.values.split_off(self.values.len() - len);

                self.values.push(Some(
                    values
                        .into_iter()
                        .filter_map(|v| v)
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
            _ => {
                panic!("Expect push seq state.");
            }
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
