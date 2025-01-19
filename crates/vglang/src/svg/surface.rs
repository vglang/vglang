use crate::{
    codegen::svg::*,
    opcode::{Attr, Path},
    surface::{Source, Surface},
};

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::{
    opcode::{Data, Element, Leaf, Opcode},
    surface::Program,
};
use xml_builder::{XMLBuilder, XMLElement, XMLError, XMLVersion, XML};

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum SvgRenderingError {
    #[error(transparent)]
    XmlError(#[from] XMLError),
    #[error("Root viewport is missing.")]
    RootViewPort,
    #[error("Pop up the wrong {0} elements.")]
    Pop(usize),

    #[error("Unsatisfied register name")]
    Register(String),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}

impl SvgNode for XMLElement {
    type Error = XMLError;
    fn set_svg_attr(&mut self, name: &str, value: &str) -> Result<(), Self::Error> {
        self.add_attribute(name, value);

        Ok(())
    }
}

impl SvgAttrsWriter for Path {
    fn write_svg_attrs<Node: SvgNode>(&self, _: &mut Node) -> Result<(), Node::Error> {
        todo!()
    }
}

impl SvgNodeWriter for Path {
    fn as_svg_node_name(&self) -> &str {
        "path"
    }
}

/// A program render vglang into svg image.
pub struct SvgRenderer(Vec<Opcode>);

impl SvgRenderer {
    /// Create a new [`SvgRenderer`] instance with `opcodes`.
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self(opcodes)
    }
}

impl Program for SvgRenderer {
    type Output = Vec<u8>;

    type Error = SvgRenderingError;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(&'a self, registers: &'a std::collections::HashMap<String, Data>) -> Self::Run<'a> {
        Box::pin(async move { SvgRendering::new(&self.0, registers).render() })
    }
}

#[allow(unused)]
struct SvgRendering<'a> {
    /// rendering opcodes.
    opcodes: &'a [Opcode],
    /// The associated register values.
    registers: &'a HashMap<String, Data>,
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
    fn new(opcodes: &'a [Opcode], registers: &'a HashMap<String, Data>) -> Self {
        Self {
            opcodes,
            registers,
            attrs: 0,
            els: Default::default(),
            defs: Default::default(),
            document: XMLBuilder::new()
                .version(XMLVersion::XML1_1)
                .encoding("UTF-8".into())
                .build(),
        }
    }

    fn render(mut self) -> Result<Vec<u8>, SvgRenderingError> {
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

                    self.apply_attrs(idx);
                }
                Opcode::Pop => self.pop()?,
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

        let mut writer: Vec<u8> = Vec::new();
        self.document.generate(&mut writer)?;

        Ok(writer)
    }

    fn render_element<E: SvgNodeWriter>(&mut self, element: &Box<E>) {
        let mut node = xml_builder::XMLElement::new(element.as_svg_node_name());

        element.write_svg_attrs(&mut node).unwrap();

        self.els.push(node);
    }

    fn apply_attrs(&mut self, offset: usize) {
        let parent = self.els.last_mut().expect("Apply attrs, parent");

        let mut defs = false;

        for idx in (offset - self.attrs)..offset {
            match &self.opcodes[idx] {
                Opcode::Apply(attr) => match attr {
                    Attr::Fill(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::Stroke(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::Id(attr) => {
                        attr.write_svg_attrs(parent).unwrap();
                        defs = true;
                    }
                    Attr::Font(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::ViewBox(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::WithMask(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::Opacity(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::WithClipPath(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::TextLayout(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::WithTransform(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::EnableBackground(attr) => attr.write_svg_attrs(parent).unwrap(),
                    Attr::WithFilter(attr) => attr.write_svg_attrs(parent).unwrap(),
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

        let element = if defs {
            let mut defs = XMLElement::new("defs");
            defs.add_child(element)?;
            defs
        } else {
            element
        };

        if let Some(last) = self.els.last_mut() {
            last.add_child(element)?;
        } else {
            self.document.set_root_element(element);
        }

        Ok(())
    }
}

/// A svg rendering target implementation for vglang.
pub struct Svg;

impl Surface for Svg {
    type Program = SvgRenderer;

    type Error = SvgRenderingError;

    type Build<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Program, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn build(&self, source: Source<'_>) -> Self::Build<'_> {
        match source {
            Source::Opcode(cow) => {
                let render = SvgRenderer::new(cow.to_vec());

                Box::pin(async move { Ok(render) })
            }
            _ => Box::pin(async move { unimplemented!("compile source(Vgl)") }),
        }
    }
}
