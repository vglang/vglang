use std::slice::Iter;

pub use cotati_device::{Device, VGLProgram};
use cotati_ir::{Animatable, FrameVariable, Text, IR};
use futures::future::BoxFuture;
use xml_dom::level2::{
    ext::{DocumentDecl, XmlDecl},
    get_implementation, Document, Element, Node, RefNode,
};

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    XmlError(#[from] xml_dom::level2::Error),

    #[error("Root viewport is missing.")]
    RootViewPort,

    #[error("Animated variable `{0}` not found.")]
    AnimatedNotFound(String),
}

/// A svg rendering target implementation.
#[derive(Default)]
pub struct SvgDevice {}

impl Device for SvgDevice {
    type Program = SvgGenerator;

    type Error = Error;

    type Compile<'a>
        = BoxFuture<'a, Result<SvgGenerator, Error>>
    where
        Self: 'a;

    fn compile(&self, codes: Vec<cotati_ir::IR>) -> Self::Compile<'_> {
        Box::pin(async move { Ok(SvgGenerator(codes)) })
    }
}

/// `VGLProgram` implementation for svg generator.
#[allow(unused)]
pub struct SvgGenerator(Vec<IR>);

impl VGLProgram for SvgGenerator {
    type Output = String;

    type Error = Error;

    type Execute<'a>
        = BoxFuture<'a, Result<String, Error>>
    where
        Self: 'a;

    fn execute<'a>(
        &'a self,
        animatable: &'a std::collections::HashMap<String, cotati_ir::AnimatableValue>,
    ) -> Self::Execute<'a> {
        Box::pin(async move { SvgGenerating::new(self.0.iter(), animatable)?.generate() })
    }
}

struct SvgGenerating<'a> {
    codes: Iter<'a, IR>,
    animatable: &'a std::collections::HashMap<String, cotati_ir::AnimatableValue>,
    document: RefNode,
    els: Vec<RefNode>,
}

impl<'a> SvgGenerating<'a> {
    fn new(
        codes: Iter<'a, IR>,
        animatable: &'a std::collections::HashMap<String, cotati_ir::AnimatableValue>,
    ) -> Result<Self, Error> {
        let doc_type = get_implementation().create_document_type(
            "svg",
            Some("-//W3C//DTD SVG 1.1//EN"),
            Some("http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"),
        )?;

        let mut document = get_implementation().create_document(
            Some("http://www.w3.org/2000/svg"),
            Some("svg"),
            Some(doc_type),
        )?;

        let xml_decl = XmlDecl::new(
            xml_dom::level2::ext::XmlVersion::V11,
            Some("utf-8".to_owned()),
            Some(true),
        );

        document.set_xml_declaration(xml_decl)?;

        let mut root_element = document.document_element().unwrap();

        root_element.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
        root_element.set_attribute("version", "1.1")?;

        Ok(Self {
            document,
            els: vec![root_element],
            codes,
            animatable,
        })
    }

    fn generate(&mut self) -> Result<String, Error> {
        self.generate_root_viewport()?;

        Ok(self.document.to_string())
    }

    fn get_value<'b, T>(&'b self, value: &'b Animatable<T>) -> Result<&'b T, Error>
    where
        T: FrameVariable,
    {
        value
            .get(&self.animatable)
            .map_err(|err| Error::AnimatedNotFound(err.to_string()))
    }

    fn current_element_mut(&mut self) -> &mut RefNode {
        self.els.last_mut().unwrap()
    }

    fn generate_root_viewport(&mut self) -> Result<(), Error> {
        match self.codes.next() {
            Some(IR::Layer(layer)) => {
                let width = self.get_value(&layer.width)?.to_string();

                self.current_element_mut()
                    .set_attribute("width", width.as_str())?;

                let height = self.get_value(&layer.height)?.to_string();

                self.current_element_mut()
                    .set_attribute("height", height.as_str())?;
            }
            _ => return Err(Error::RootViewPort),
        }

        self.process_next()?;

        Ok(())
    }

    fn process_next(&mut self) -> Result<Option<usize>, Error> {
        if let Some(ir) = self.codes.next() {
            match ir {
                IR::Text(text) => {
                    return self.process_text(text).map(Some);
                }
                IR::String(literal) => {
                    let text_node = self.document.create_text_node(&literal);
                    self.current_element_mut().append_child(text_node)?;
                    return Ok(Some(0));
                }
                IR::Pop(n) => {
                    return Ok(Some(*n));
                }
                _ => todo!(),
            }
        }

        Ok(None)
    }

    fn process_text(&mut self, text: &Text) -> Result<usize, Error> {
        let mut el = self.document.create_element("text")?;

        let x = self
            .get_value(&text.x)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        el.set_attribute("x", &x)?;

        let y = self
            .get_value(&text.y)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        el.set_attribute("y", &y)?;

        self.els.push(el);

        let mut pop_n = 0;

        while let Some(n) = self.process_next()? {
            if n > 0 {
                pop_n = n - 1;
                break;
            }
        }

        let el = self.els.pop().unwrap();

        self.current_element_mut().append_child(el)?;

        Ok(pop_n)
    }
}
