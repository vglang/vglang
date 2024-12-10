use std::slice::Iter;

pub use cotati_device::{Device, VGLProgram};
use cotati_ir::{
    Animatable, Fill, FontFamily, FontSize, FontStyle, FrameVariable, Layer, PreserveAspectRatio,
    Rect, Stroke, Text, IR,
};
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
                self.process_layer(layer, true)?;
            }
            _ => return Err(Error::RootViewPort),
        }

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
                IR::Stroke(stroke) => {
                    return self.process_stroke(stroke).map(Some);
                }
                IR::Rect(rect) => return self.process_rect(rect).map(Some),
                IR::Fill(fill) => {
                    return self.process_fill(fill).map(Some);
                }
                IR::FontFamily(value) => {
                    return self.process_font_family(value).map(Some);
                }
                IR::FontSize(value) => {
                    return self.process_font_size(value).map(Some);
                }
                IR::FontStyle(value) => {
                    return self.process_font_style(value).map(Some);
                }
                IR::FontFace(_) => return Ok(Some(0)),
                _ => todo!(),
            }
        }

        Ok(None)
    }

    fn process_child(&mut self, is_root: bool) -> Result<usize, Error> {
        let mut pop_n = 0;

        while let Some(n) = self.process_next()? {
            if n > 0 {
                pop_n = n - 1;
                break;
            }
        }

        if !is_root {
            let el = self.els.pop().unwrap();
            self.current_element_mut().append_child(el)?;
        }

        Ok(pop_n)
    }

    fn process_rect(&mut self, rect: &Rect) -> Result<usize, Error> {
        let mut node = self.document.create_element("rect")?;

        node.set_attribute("x", self.get_value(&rect.x)?.to_string().as_str())?;

        node.set_attribute("y", self.get_value(&rect.y)?.to_string().as_str())?;

        node.set_attribute("rx", self.get_value(&rect.rx)?.to_string().as_str())?;

        if let Some(ry) = &rect.ry {
            node.set_attribute("ry", self.get_value(ry)?.to_string().as_str())?;
        }

        node.set_attribute("width", self.get_value(&rect.width)?.to_string().as_str())?;

        node.set_attribute("height", self.get_value(&rect.height)?.to_string().as_str())?;

        self.current_element_mut().append_child(node)?;

        return Ok(0);
    }

    fn process_layer(&mut self, layer: &Layer, is_root: bool) -> Result<usize, Error> {
        let mut el = if is_root {
            self.current_element_mut().clone()
        } else {
            self.document.create_element("svg")?
        };

        let width = self.get_value(&layer.width)?.to_string();

        el.set_attribute("width", width.as_str())?;

        let height = self.get_value(&layer.height)?.to_string();

        el.set_attribute("height", height.as_str())?;

        if let Some(viewbox) = &layer.viewbox {
            let viewbox = self.get_value(viewbox)?;

            el.set_attribute(
                "viewbox",
                format!(
                    "{} {} {} {}",
                    self.get_value(&viewbox.minx)?,
                    self.get_value(&viewbox.miny)?,
                    self.get_value(&viewbox.width)?,
                    self.get_value(&viewbox.height)?
                )
                .as_str(),
            )?;

            if let Some(aspect) = &viewbox.aspect {
                let v = match self.get_value(aspect)? {
                    PreserveAspectRatio::xMinYMin(meet_or_slice) => {
                        format!("xMinYMin {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMidYMin(meet_or_slice) => {
                        format!("xMidYMin {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMaxYMin(meet_or_slice) => {
                        format!("xMaxYMin {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMinYMid(meet_or_slice) => {
                        format!("xMinYMid {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMidYMid(meet_or_slice) => {
                        format!("xMidYMid {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMaxYMid(meet_or_slice) => {
                        format!("xMaxYMid {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMinYMax(meet_or_slice) => {
                        format!("xMinYMax {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMidYMax(meet_or_slice) => {
                        format!("xMidYMax {}", meet_or_slice)
                    }
                    PreserveAspectRatio::xMaxYMax(meet_or_slice) => {
                        format!("xMaxYMax {}", meet_or_slice)
                    }
                };

                el.set_attribute("preserveAspectRatio", v.as_str())?;
            }
        }

        if !is_root {
            self.els.push(el);
        }

        self.process_child(is_root)
    }

    fn process_stroke(&mut self, stroke: &Stroke) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        if let Some(paint) = &stroke.paint {
            match self.get_value(paint)? {
                cotati_ir::Paint::Color(rgba) => el.set_attribute(
                    "stroke",
                    format!(
                        "rgb({},{},{})",
                        (rgba.0 * 255.0) as u8,
                        (rgba.1 * 255.0) as u8,
                        (rgba.2 * 255.0) as u8
                    )
                    .as_str(),
                )?,
                cotati_ir::Paint::Gradient(uri) => {
                    el.set_attribute("stroke", format!("url(#{})", uri).as_str())?
                }
                cotati_ir::Paint::Pattern(uri) => {
                    el.set_attribute("stroke", format!("url(#{})", uri).as_str())?
                }
            }
        }

        self.els.push(el);

        self.process_child(false)
    }

    fn process_fill(&mut self, fill: &Fill) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        if let Some(paint) = &fill.paint {
            match self.get_value(paint)? {
                cotati_ir::Paint::Color(rgba) => el.set_attribute(
                    "fill",
                    format!(
                        "rgb({},{},{})",
                        (rgba.0 * 255.0) as u8,
                        (rgba.1 * 255.0) as u8,
                        (rgba.2 * 255.0) as u8
                    )
                    .as_str(),
                )?,
                cotati_ir::Paint::Gradient(uri) => {
                    el.set_attribute("fill", format!("url(#{})", uri).as_str())?
                }
                cotati_ir::Paint::Pattern(uri) => {
                    el.set_attribute("fill", format!("url(#{})", uri).as_str())?
                }
            }
        } else {
            el.set_attribute("fill", "none")?
        }

        self.els.push(el);

        self.process_child(false)
    }

    fn process_font_family(&mut self, value: &FontFamily) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute("font-family", value.to_string().as_str())?;

        self.els.push(el);

        self.process_child(false)
    }

    fn process_font_size(&mut self, value: &FontSize) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute("font-size", value.0.to_string().as_str())?;

        self.els.push(el);

        self.process_child(false)
    }

    fn process_font_style(&mut self, value: &FontStyle) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        if value.contains(FontStyle::Normal) {
            el.set_attribute("font-style", "normal")?;
        } else if value.contains(FontStyle::Italic) {
            el.set_attribute("font-style", "italic")?;
        } else if value.contains(FontStyle::Oblique) {
            el.set_attribute("font-style", "oblique")?;
        }

        self.els.push(el);

        self.process_child(false)
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

        let rotate = self
            .get_value(&text.rotate)?
            .into_iter()
            .map(|v| v.as_deg().to_string())
            .collect::<Vec<_>>()
            .join(",");

        el.set_attribute("rotate", &rotate)?;

        self.els.push(el);

        self.process_child(false)
    }
}
