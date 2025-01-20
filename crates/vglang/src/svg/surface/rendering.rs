use std::collections::HashMap;

use xml_builder::{XMLBuilder, XMLElement, XMLVersion, XML};

use crate::{
    codegen::svg::{SvgAttrsWriter, SvgContext, SvgNode, SvgNodeWriter},
    opcode::{
        variable::{self, Target, Variable},
        Attr, Data, Element, Leaf, Opcode,
    },
};

use super::SvgRenderingError;

impl SvgNode for XMLElement {
    type Error = SvgRenderingError;
    fn set_svg_attr(&mut self, name: &str, value: &str) -> Result<(), Self::Error> {
        self.add_attribute(name, value);

        Ok(())
    }
}

/// A program render vglang into svg image.
pub struct SvgRenderer(pub(super) Vec<Opcode>);

impl SvgRenderer {
    /// Create a new [`SvgRenderer`] instance with `opcodes`.
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self(opcodes)
    }
}

struct SvgRenderingContext<'a>(&'a HashMap<String, Data>);

impl<'a> SvgContext for SvgRenderingContext<'a> {
    type Error = SvgRenderingError;
    fn valueof<'b, T>(&'b self, variable: &'b Variable<T>) -> Result<&'b T, Self::Error>
    where
        Data: From<T>,
        for<'c> &'c T: TryFrom<&'c Data, Error = ()>,
    {
        match variable {
            Variable::Constant(v) => Ok(v),
            Variable::Reference { path, target } => match target {
                Target::Register => match path {
                    variable::Path::Named(name) => {
                        if let Some(v) = self.0.get(name) {
                            TryFrom::try_from(v)
                                .map_err(|_| SvgRenderingError::VariableCast(name.to_string()))
                        } else {
                            Err(SvgRenderingError::Register(name.clone()))
                        }
                    }
                    path => unimplemented!("Unsupport variable path {:?}", path),
                },
                target => {
                    unimplemented!("Unsupport variable target {:?}", target);
                }
            },
        }
    }
}

pub struct SvgRendering<'a> {
    /// rendering opcodes.
    opcodes: &'a [Opcode],
    /// The associated register values.
    context: SvgRenderingContext<'a>,
    /// The count of the applied attributes.
    attrs: usize,
    /// rendering xml element stack.
    els: Vec<XMLElement>,
    /// defs stack.
    defs: Vec<bool>,
    /// xml document.
    document: XML,
}

impl<'a> SvgRendering<'a> {
    /// Create a new svg rendering instance.
    pub fn new(opcodes: &'a [Opcode], registers: &'a HashMap<String, Data>) -> Self {
        Self {
            opcodes,
            context: SvgRenderingContext(registers),
            attrs: 0,
            els: Default::default(),
            defs: Default::default(),
            document: XMLBuilder::new()
                .version(XMLVersion::XML1_0)
                .encoding("UTF-8".into())
                .build(),
        }
    }

    /// Calls the real rendering process.
    pub fn render(mut self) -> Result<Vec<u8>, SvgRenderingError> {
        let mut els = 0;
        let mut pops = 0;
        for (idx, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Apply(_) => self.attrs += 1,
                Opcode::Element(element) => {
                    match element {
                        Element::Group(el) => self.render_element(el),
                        Element::Text(el) => self.render_element(el),
                        Element::TextSpan(el) => self.render_element(el),
                        Element::Canvas(el) => self.render_element(el),
                        Element::LinearGradient(el) => self.render_element(el),
                        Element::RadialGradient(el) => self.render_element(el),
                        Element::Pattern(el) => self.render_element(el),
                        Element::Filter(el) => self.render_element(el),
                        Element::FeComponentTransfer(el) => self.render_element(el),
                        Element::FeDiffuseLighting(el) => self.render_element(el),
                        Element::FeMerge(el) => self.render_element(el),
                        Element::FeSpecularLighting(el) => self.render_element(el),
                        Element::Mask(el) => self.render_element(el),
                        Element::ClipPath(el) => self.render_element(el),
                        Element::TextPath(el) => self.render_element(el),
                    }
                    els += 1;
                    self.apply_attrs(idx);
                }
                Opcode::Pop => {
                    pops += 1;
                    self.pop()?
                }
                Opcode::Leaf(shape) => {
                    match shape {
                        Leaf::Use(el) => self.render_element(el),
                        Leaf::Path(el) => self.render_element(el),
                        Leaf::GradientStop(el) => self.render_element(el),
                        Leaf::Rect(el) => self.render_element(el),
                        Leaf::Circle(el) => self.render_element(el),
                        Leaf::Ellipse(el) => self.render_element(el),
                        Leaf::Line(el) => self.render_element(el),
                        Leaf::Polyline(el) => self.render_element(el),
                        Leaf::Polygon(el) => self.render_element(el),
                        Leaf::FeDistantLight(el) => self.render_element(el),
                        Leaf::FeSpotLight(el) => self.render_element(el),
                        Leaf::FePointLight(el) => self.render_element(el),
                        Leaf::FeBlend(el) => self.render_element(el),
                        Leaf::FeColorMatrix(el) => self.render_element(el),
                        Leaf::FeFuncA(el) => self.render_element(el),
                        Leaf::FeFuncR(el) => self.render_element(el),
                        Leaf::FeFuncG(el) => self.render_element(el),
                        Leaf::FeFuncB(el) => self.render_element(el),
                        Leaf::FeComposite(el) => self.render_element(el),
                        Leaf::FeDisplacementMap(el) => self.render_element(el),
                        Leaf::FeConvolveMatrix(el) => self.render_element(el),
                        Leaf::FeGaussianBlur(el) => self.render_element(el),
                        Leaf::FeMergeNode(el) => self.render_element(el),
                        Leaf::FeImage(el) => self.render_element(el),
                        Leaf::FeFlood(el) => self.render_element(el),
                        Leaf::FeOffset(el) => self.render_element(el),
                        Leaf::FeMorphology(el) => self.render_element(el),
                        Leaf::FeTile(el) => self.render_element(el),
                        Leaf::FeTurbulence(el) => self.render_element(el),
                        Leaf::Characters(el) => {
                            self.els
                                .last_mut()
                                .expect("Append text")
                                .add_text(el.0.clone())?;

                            continue;
                        }
                    }

                    self.apply_attrs(idx);

                    self.pop()?;
                }
            }
        }

        assert_eq!(pops, els);

        let mut writer: Vec<u8> = Vec::new();
        self.document.generate(&mut writer)?;

        Ok(writer)
    }

    fn render_element<E: SvgNodeWriter>(&mut self, element: &Box<E>) {
        let mut node = xml_builder::XMLElement::new(element.to_svg_node_name());

        element.write_svg_attrs(&self.context, &mut node).unwrap();

        self.els.push(node);
    }

    fn apply_attrs(&mut self, offset: usize) {
        let parent = self.els.last_mut().expect("Apply attrs, parent");

        let mut defs = false;

        for idx in (offset - self.attrs)..offset {
            match &self.opcodes[idx] {
                Opcode::Apply(attr) => match attr {
                    Attr::Fill(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::Stroke(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::Id(attr) => {
                        attr.write_svg_attrs(&self.context, parent).unwrap();
                        defs = true;
                    }
                    Attr::Font(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::ViewBox(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::WithMask(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::Opacity(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::WithClipPath(attr) => {
                        attr.write_svg_attrs(&self.context, parent).unwrap()
                    }
                    Attr::TextLayout(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                    Attr::WithTransform(attr) => {
                        attr.write_svg_attrs(&self.context, parent).unwrap()
                    }
                    Attr::EnableBackground(attr) => {
                        attr.write_svg_attrs(&self.context, parent).unwrap()
                    }
                    Attr::WithFilter(attr) => attr.write_svg_attrs(&self.context, parent).unwrap(),
                },
                opcode => {
                    panic!("Apply attrs: {}={:?}", idx, opcode);
                }
            }
        }
        self.attrs = 0;
        self.defs.push(defs);
    }

    fn pop(&mut self) -> Result<(), SvgRenderingError> {
        let element = self.els.pop().expect("el stack.");
        let defs = self.defs.pop().expect("defs pop.");

        let mut element = if defs {
            let mut defs = XMLElement::new("defs");
            defs.add_child(element)?;
            defs
        } else {
            element
        };

        if let Some(last) = self.els.last_mut() {
            last.add_child(element)?;
        } else {
            element.add_attribute("xmlns", "http://www.w3.org/2000/svg");
            element.add_attribute("version", "1.1");
            element.add_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
            self.document.set_root_element(element);
        }

        Ok(())
    }
}
