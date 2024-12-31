use std::{collections::HashMap, future::Future, pin::Pin};

use vglang::{
    opcode::{data::Data, variable::Variable, Opcode},
    surface::Program,
};
use xml_builder::{XMLBuilder, XMLElement, XMLVersion, XML};

use crate::{Context, Error, SvgAttr, SvgElement};

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

    type Error = Error;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(
        &'a self,
        registers: &'a std::collections::HashMap<String, vglang::opcode::data::Data>,
    ) -> Self::Run<'a> {
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

impl<'a> Context for SvgRendering<'a> {
    fn valueof<'b, T>(&'b self, variable: &'b Variable<T>) -> Option<&'b T>
    where
        Data: From<T>,
        for<'c> &'c T: TryFrom<&'c Data, Error = ()>,
    {
        match variable {
            vglang::opcode::variable::Variable::Constant(v) => Some(v),
            vglang::opcode::variable::Variable::Reference { path, target } => match target {
                vglang::opcode::variable::Target::Register => match path {
                    vglang::opcode::variable::Path::Named(name) => {
                        if let Some(v) = self.registers.get(name) {
                            TryFrom::try_from(v).ok()
                        } else {
                            None
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

    fn render(mut self) -> Result<Vec<u8>, Error> {
        for (idx, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Apply(_) => self.attrs += 1,
                Opcode::Container(container) => {
                    match container {
                        vglang::opcode::el::Container::Group(el) => self.render_element(el),
                        vglang::opcode::el::Container::Text(el) => self.render_element(el),
                        vglang::opcode::el::Container::TextSpan(el) => self.render_element(el),
                        vglang::opcode::el::Container::Canvas(el) => self.render_element(el),
                        vglang::opcode::el::Container::LinearGradient(el) => {
                            self.render_element(el)
                        }
                        vglang::opcode::el::Container::RadialGradient(el) => {
                            self.render_element(el)
                        }
                        vglang::opcode::el::Container::Pattern(el) => self.render_element(el),
                        vglang::opcode::el::Container::Filter(el) => self.render_element(el),
                        vglang::opcode::el::Container::FeComponentTransfer(el) => {
                            self.render_element(el)
                        }
                        vglang::opcode::el::Container::FeDiffuseLighting(el) => {
                            self.render_element(el)
                        }
                        vglang::opcode::el::Container::FeMerge(el) => self.render_element(el),
                        vglang::opcode::el::Container::FeSpecularLighting(el) => {
                            self.render_element(el)
                        }
                        vglang::opcode::el::Container::Mask(el) => self.render_element(el),
                        vglang::opcode::el::Container::ClipPath(el) => self.render_element(el),
                        vglang::opcode::el::Container::TextPath(el) => self.render_element(el),
                        _ => {
                            // skip control-flow opcodes.
                            continue;
                        }
                    }

                    self.apply_attrs(idx);
                }
                Opcode::Pop => self.pop()?,
                Opcode::Draw(shape) => {
                    match shape {
                        vglang::opcode::el::Shape::Use(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Path(el) => self.render_element(el),
                        vglang::opcode::el::Shape::GradientStop(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Rect(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Circle(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Ellipse(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Line(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Polyline(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Polygon(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeDistantLight(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeSpotLight(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FePointLight(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeBlend(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeColorMatrix(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeFuncA(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeFuncR(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeFuncG(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeFuncB(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeComposite(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeDisplacementMap(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeConvolveMatrix(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeGaussianBlur(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeMergeNode(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeImage(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeFlood(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeOffset(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeMorphology(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeTile(el) => self.render_element(el),
                        vglang::opcode::el::Shape::FeTurbulence(el) => self.render_element(el),
                        vglang::opcode::el::Shape::Characters(el) => {
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

    fn render_element<E: SvgElement>(&mut self, element: &E) {
        self.els.push(element.create_xml_element(self));
    }

    fn apply_attrs(&mut self, offset: usize) {
        let mut parent = self.els.last_mut().expect("Apply attrs, parent");

        let mut defs = false;

        for idx in (offset - self.attrs)..offset {
            match &self.opcodes[idx] {
                Opcode::Apply(attr) => match attr {
                    vglang::opcode::attrs::Attr::Fill(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::Stroke(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::Id(attr) => {
                        attr.attach(&mut parent);
                        defs = true;
                    }
                    vglang::opcode::attrs::Attr::Font(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::ViewBox(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::WithMask(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::Opacity(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::WithClipPath(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::TextLayout(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::WithTransform(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::EnableBackground(attr) => attr.attach(&mut parent),
                    vglang::opcode::attrs::Attr::WithFilter(attr) => attr.attach(&mut parent),
                },
                opcode => {
                    panic!("Apply attrs: {}={:?}", idx, opcode);
                }
            }
        }
        self.attrs = 0;
        self.defs.push(defs);
    }

    fn pop(&mut self) -> Result<(), Error> {
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
