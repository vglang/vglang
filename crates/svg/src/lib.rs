use std::{fmt::Debug, future::Future, pin::Pin, slice::Iter};

use vglang_opcode::{
    operand::{
        AlignmentBaseline, BaselineShift, Canvas, Circle, ClipPath, ClipPathed, ClipRule,
        DominantBaseline, Ellipse, Fill, Font, FontStretch, FontStyle, FontVariant, FontWeight,
        GradientStop, LinearGradient, Mask, Masked, Opacity, Paint, Path, Pattern, Polygon,
        Polyline, RadialGradient, Rect, RefBy, Stroke, Text, TextAnchor, TextDirection, TextLayout,
        TextLengthAdjust, TextPath, TextSpan, Transform, UnicodeBidi, Use, Value, Variable,
        WritingMode,
    },
    Opcode,
};
pub use vglang_targets::*;
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
    #[error("Pop up the wrong {0} elements.")]
    Pop(usize),

    #[error("Unsatisfied register name")]
    Register(String),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}

/// A target output rendering result to svg image.
#[derive(Default, Debug)]
pub struct SvgTarget {}

impl Target for SvgTarget {
    type Builder = SvgBuilder;
    fn build(&self) -> Self::Builder {
        SvgBuilder::default()
    }
}

/// A [`SvgProgram`] builder.
#[derive(Default, Debug)]
pub struct SvgBuilder(Vec<Opcode>);

impl Builder for SvgBuilder {
    type Error = Error;
    type Program = SvgProgram;

    type Create = Pin<Box<dyn Future<Output = Result<Self::Program, Self::Error>> + 'static>>;

    fn push<O>(&mut self, opcode: O)
    where
        Opcode: From<O>,
    {
        self.0.push(opcode.into());
    }

    fn create(self) -> Self::Create {
        Box::pin(async move { Ok(SvgProgram(self.0)) })
    }
}

/// A svg [`Program`] builder.
#[derive(Default, Debug)]
pub struct SvgProgram(pub Vec<Opcode>);

impl Program for SvgProgram {
    type Output = String;

    type Error = Error;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(&'a self, registers: &'a std::collections::HashMap<String, Value>) -> Self::Run<'a> {
        Box::pin(async move {
            let mut document = get_implementation().create_document(
                Some("http://www.w3.org/2000/svg"),
                Some("svg"),
                None,
            )?;

            let xml_decl = XmlDecl::new(xml_dom::level2::ext::XmlVersion::V11, None, Some(true));

            document.set_xml_declaration(xml_decl)?;

            let mut root_element = document.document_element().unwrap();

            root_element.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
            root_element.set_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink")?;
            root_element.set_attribute("version", "1.1")?;

            SvgCreator {
                document,
                el_stack: vec![root_element],
                opcodes: self.0.iter(),
                registers,
                id: Default::default(),
            }
            .create()
        })
    }
}

#[allow(unused)]
struct SvgCreator<'a> {
    opcodes: Iter<'a, Opcode>,
    registers: &'a std::collections::HashMap<String, Value>,
    document: RefNode,
    el_stack: Vec<RefNode>,
    id: Option<String>,
}

impl<'a> SvgCreator<'a> {
    fn create(&mut self) -> Result<String, Error> {
        match self.opcodes.next() {
            Some(Opcode::Canvas(canvs)) => {
                let mut el = self.el_stack.last_mut().unwrap().clone();
                self.handle_canvas(&mut el, canvs)?;
            }
            _ => return Err(Error::RootViewPort),
        }

        while let Some(opcode) = self.opcodes.next() {
            match opcode {
                Opcode::Id(id) => {
                    self.id = Some(id.0.clone());
                    let el = self.document.create_element("defs")?;
                    self.el_stack.push(el);
                }
                Opcode::Pop(n) => {
                    for _ in 0..*n {
                        let el = self.el_stack.pop().ok_or_else(|| {
                            return Error::Pop(*n);
                        })?;

                        if let Some(parent) = self.el_stack.last_mut() {
                            parent.append_child(el)?;
                        }
                    }
                }
                Opcode::Canvas(canvs) => {
                    let mut el = self.document.create_element("svg")?;
                    self.append_id(&mut el)?;
                    self.handle_canvas(&mut el, canvs)?;
                    self.el_stack.push(el);
                }
                Opcode::Fill(operand) => {
                    let mut el = self.document.create_element("g")?;
                    self.append_id(&mut el)?;
                    self.handle_fill(&mut el, operand)?;
                    self.el_stack.push(el);
                }
                Opcode::Stroke(operand) => {
                    let mut el = self.document.create_element("g")?;
                    self.append_id(&mut el)?;
                    self.handle_stroke(&mut el, operand)?;
                    self.el_stack.push(el);
                }
                Opcode::Font(operand) => {
                    let mut el = self.document.create_element("g")?;
                    self.append_id(&mut el)?;
                    self.handle_font(&mut el, operand)?;
                    self.el_stack.push(el);
                }
                Opcode::TextLayout(operand) => {
                    let mut el = self.document.create_element("g")?;
                    self.append_id(&mut el)?;
                    self.handle_text_layout(&mut el, operand)?;
                    self.el_stack.push(el);
                }

                Opcode::Characters(operand) => {
                    let text = self.get_value(operand)?;
                    let mut text_node = self.document.create_text_node(text);
                    self.append_id(&mut text_node)?;
                    self.append_child(text_node)?;
                }
                Opcode::Transform(transform) => {
                    let transform = self.get_value(transform)?.clone();

                    self.handle_transform(&transform)?;
                }
                Opcode::Text(operand) => self.handle_text(operand)?,
                Opcode::TextSpan(operand) => self.handle_text_span(operand)?,
                Opcode::TextPath(operand) => self.handle_text_path(operand)?,
                Opcode::Rect(operand) => self.handle_rect(operand)?,
                Opcode::Circle(circle) => self.handle_circle(circle)?,
                Opcode::Ellipse(operand) => self.handle_ellipse(operand)?,
                Opcode::Path(path) => self.handle_path(path)?,
                Opcode::Polyline(polyline) => self.handle_polyline(polyline)?,
                Opcode::Polygon(operand) => self.handle_polygon(operand)?,
                Opcode::Use(v) => self.handle_use(v)?,
                Opcode::LinearGradient(operand) => self.handle_linear_gradient(operand)?,
                Opcode::RadialGradient(operand) => self.handle_radial_gradient(operand)?,
                Opcode::GradientStop(operand) => self.handle_gradient_stop(operand)?,
                Opcode::Pattern(operand) => self.handle_pattern(operand)?,
                Opcode::Mask(operand) => self.handle_mask(operand)?,
                Opcode::Masked(operand) => self.handle_masked(operand)?,
                Opcode::Opacity(operand) => self.handle_opacity(operand)?,
                Opcode::ClipPath(operand) => self.handle_clip_path(operand)?,
                Opcode::ClipPathed(operand) => self.handle_clip_pathed(operand)?,
                Opcode::ClipRule(operand) => self.handle_clip_rule(operand)?,
            }
        }

        Ok(self.document.to_string())
    }

    fn append_id(&mut self, el: &mut RefNode) -> Result<(), Error> {
        if let Some(id) = self.id.take() {
            el.set_attribute("id", &id)?;
        }

        Ok(())
    }

    /// get variable referenced value.
    fn get_value<'b, T>(&'b self, value: &'b Variable<T>) -> Result<&'b T, Error>
    where
        for<'c> &'c T: TryFrom<&'c Value, Error = &'c Value>,
        Value: From<T>,
        T: Debug,
    {
        match value {
            Variable::Constant(v) => Ok(v),
            Variable::Animated(RefBy::Named(name)) => {
                let value = self
                    .registers
                    .get(name)
                    .ok_or_else(|| Error::Register(name.to_owned()))?;

                value
                    .try_into()
                    .map_err(|_| Error::Register(name.to_owned()))
            }
            _ => todo!("unimplement variable lookup: {:?}", value),
        }
    }

    fn append_child(&mut self, el: RefNode) -> Result<(), Error> {
        self.el_stack
            .last_mut()
            .ok_or_else(|| {
                return Error::Pop(1);
            })?
            .append_child(el)?;

        Ok(())
    }

    fn handle_use(&mut self, v: &Use) -> Result<(), Error> {
        let mut node = self.document.create_element("use")?;

        self.append_id(&mut node)?;

        node.set_attribute("xlink:href", &format!("#{}", v.0))?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_polygon(&mut self, polygon: &Polygon) -> Result<(), Error> {
        let mut node = self.document.create_element("polygon")?;

        self.append_id(&mut node)?;

        let data = self
            .get_value(&polygon.0)?
            .iter()
            .map(|v| format!("{},{}", v.x, v.y))
            .collect::<Vec<_>>()
            .join(" ");

        node.set_attribute("points", &data)?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_polyline(&mut self, polyline: &Polyline) -> Result<(), Error> {
        let mut node = self.document.create_element("polyline")?;

        self.append_id(&mut node)?;

        let data = self
            .get_value(&polyline.0)?
            .iter()
            .map(|v| format!("{},{}", v.x, v.y))
            .collect::<Vec<_>>()
            .join(" ");

        node.set_attribute("points", &data)?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_path(&mut self, path: &Path) -> Result<(), Error> {
        let mut node = self.document.create_element("path")?;

        self.append_id(&mut node)?;

        let data = path
            .data
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        node.set_attribute("d", &data)?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_rect(&mut self, rect: &Rect) -> Result<(), Error> {
        let mut node = self.document.create_element("rect")?;

        self.append_id(&mut node)?;

        node.set_attribute("x", self.get_value(&rect.x)?.to_string().as_str())?;

        node.set_attribute("y", self.get_value(&rect.y)?.to_string().as_str())?;

        node.set_attribute("rx", self.get_value(&rect.rx)?.to_string().as_str())?;

        if let Some(ry) = &rect.ry {
            node.set_attribute("ry", self.get_value(ry)?.to_string().as_str())?;
        }

        node.set_attribute("width", self.get_value(&rect.width)?.to_string().as_str())?;

        node.set_attribute("height", self.get_value(&rect.height)?.to_string().as_str())?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_circle(&mut self, circle: &Circle) -> Result<(), Error> {
        let mut node = self.document.create_element("circle")?;

        self.append_id(&mut node)?;

        node.set_attribute("cx", self.get_value(&circle.cx)?.to_string().as_str())?;

        node.set_attribute("cy", self.get_value(&circle.cy)?.to_string().as_str())?;

        node.set_attribute("r", self.get_value(&circle.r)?.to_string().as_str())?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_ellipse(&mut self, ellipse: &Ellipse) -> Result<(), Error> {
        let mut node = self.document.create_element("ellipse")?;

        self.append_id(&mut node)?;

        node.set_attribute("cx", self.get_value(&ellipse.cx)?.to_string().as_str())?;

        node.set_attribute("cy", self.get_value(&ellipse.cy)?.to_string().as_str())?;

        node.set_attribute("rx", self.get_value(&ellipse.rx)?.to_string().as_str())?;
        node.set_attribute("ry", self.get_value(&ellipse.ry)?.to_string().as_str())?;

        self.append_child(node)?;

        Ok(())
    }

    fn handle_transform(&mut self, transform: &Transform) -> Result<(), Error> {
        let mut el = self.document.create_element("g")?;

        self.append_id(&mut el)?;

        el.set_attribute("transform", transform.to_string().as_str())?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_linear_gradient(&mut self, gradient: &LinearGradient) -> Result<(), Error> {
        let mut el = self.document.create_element("linearGradient")?;

        self.append_id(&mut el)?;

        if let Some(value) = &gradient.units {
            el.set_attribute("gradientUnits", &self.get_value(value)?.to_string())?;
        }

        if let Some(transform) = &gradient.transform {
            el.set_attribute("gradientTransform", &self.get_value(transform)?.to_string())?;
        }

        if let Some(value) = &gradient.x1 {
            el.set_attribute("x1", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.y1 {
            el.set_attribute("y1", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.x2 {
            el.set_attribute("x2", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.y2 {
            el.set_attribute("y2", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.spread {
            el.set_attribute("spreadMethod", &self.get_value(value)?.to_string())?;
        }

        self.append_id(&mut el)?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_radial_gradient(&mut self, gradient: &RadialGradient) -> Result<(), Error> {
        let mut el = self.document.create_element("radialGradient")?;

        self.append_id(&mut el)?;

        if let Some(value) = &gradient.unit {
            el.set_attribute("gradientUnits", &self.get_value(value)?.to_string())?;
        }

        if let Some(transform) = &gradient.transform {
            el.set_attribute("gradientTransform", &self.get_value(transform)?.to_string())?;
        }

        if let Some(value) = &gradient.fx {
            el.set_attribute("fx", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.fy {
            el.set_attribute("fy", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.cx {
            el.set_attribute("cx", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.cy {
            el.set_attribute("cy", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.r {
            el.set_attribute("r", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &gradient.spread {
            el.set_attribute("spreadMethod", &self.get_value(value)?.to_string())?;
        }

        self.append_id(&mut el)?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_gradient_stop(&mut self, stop: &GradientStop) -> Result<(), Error> {
        let mut el = self.document.create_element("stop")?;

        el.set_attribute("offset", &self.get_value(&stop.offset)?.to_string())?;

        if let Some(value) = &stop.color {
            el.set_attribute("stop-color", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &stop.opacity {
            el.set_attribute("stop-opacity", &self.get_value(value)?.to_string())?;
        }

        self.append_child(el)?;

        Ok(())
    }

    fn handle_pattern(&mut self, pattern: &Pattern) -> Result<(), Error> {
        let mut el = self.document.create_element("pattern")?;

        self.append_id(&mut el)?;

        if let Some(value) = &pattern.x {
            el.set_attribute("x", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.y {
            el.set_attribute("y", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.width {
            el.set_attribute("width", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.height {
            el.set_attribute("height", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.units {
            el.set_attribute("patternUnits", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.content_units {
            el.set_attribute("patternContentUnits", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.transform {
            el.set_attribute("patternTransform", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &pattern.viewbox {
            el.set_attribute(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    self.get_value(&value.minx)?,
                    self.get_value(&value.miny)?,
                    self.get_value(&value.width)?,
                    self.get_value(&value.height)?
                )
                .as_str(),
            )?;

            if let Some(aspect) = &value.aspect {
                el.set_attribute("preserveAspectRatio", &self.get_value(aspect)?.to_string())?;
            }
        }

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_opacity(&mut self, operand: &Opacity) -> Result<(), Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute(
            "opacity",
            self.get_value(&operand.0)?.0.to_string().as_str(),
        )?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_clip_rule(&mut self, operand: &Variable<ClipRule>) -> Result<(), Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute("clip-rule", self.get_value(operand)?.to_string().as_str())?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_clip_pathed(&mut self, operand: &ClipPathed) -> Result<(), Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute(
            "clip-path",
            self.get_value(&operand.0)?.to_string().as_str(),
        )?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_masked(&mut self, operand: &Masked) -> Result<(), Error> {
        let mut el = self.document.create_element("g")?;

        el.set_attribute("mask", self.get_value(&operand.0)?.to_string().as_str())?;

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_clip_path(&mut self, operand: &ClipPath) -> Result<(), Error> {
        let mut el = self.document.create_element("clipPath")?;

        self.append_id(&mut el)?;

        if let Some(value) = &operand.0 {
            el.set_attribute("clipPathUnits", &self.get_value(value)?.to_string())?;
        }

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_mask(&mut self, operand: &Mask) -> Result<(), Error> {
        let mut el = self.document.create_element("mask")?;

        self.append_id(&mut el)?;

        if let Some(value) = &operand.x {
            el.set_attribute("x", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &operand.y {
            el.set_attribute("y", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &operand.width {
            el.set_attribute("width", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &operand.height {
            el.set_attribute("height", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &operand.units {
            el.set_attribute("patternUnits", &self.get_value(value)?.to_string())?;
        }

        if let Some(value) = &operand.content_units {
            el.set_attribute("patternContentUnits", &self.get_value(value)?.to_string())?;
        }

        self.el_stack.push(el);

        Ok(())
    }

    fn handle_text(&mut self, text: &Text) -> Result<(), Error> {
        let mut el = self.document.create_element("text")?;

        self.append_id(&mut el)?;

        let x = self
            .get_value(&text.x)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if x.len() > 0 {
            el.set_attribute("x", &x)?;
        }

        let y = self
            .get_value(&text.y)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if y.len() > 0 {
            el.set_attribute("y", &y)?;
        }

        let dx = self
            .get_value(&text.dx)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if dx.len() > 0 {
            el.set_attribute("dx", &dx)?;
        }

        let dy = self
            .get_value(&text.dy)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if dy.len() > 0 {
            el.set_attribute("dy", &dy)?;
        }

        let rotate = self
            .get_value(&text.rotate)?
            .into_iter()
            .map(|v| v.as_deg().to_string())
            .collect::<Vec<_>>()
            .join(",");

        if rotate.len() > 0 {
            el.set_attribute("rotate", &rotate)?;
        }

        let text_length = self.get_value(&text.text_length)?;

        el.set_attribute("textLength", text_length.to_string().as_str())?;

        let length_adjust = self.get_value(&text.length_adjust)?;

        match length_adjust {
            TextLengthAdjust::Spacing => el.set_attribute("lengthAdjust", "spacing")?,
            TextLengthAdjust::SpacingAndGlyphs => {
                el.set_attribute("lengthAdjust", "spacingAndGlyphs")?
            }
        }

        if let Some(value) = &text.font {
            self.handle_font(&mut el, value)?;
        }

        if let Some(value) = &text.layout {
            self.handle_text_layout(&mut el, value)?;
        }

        if let Some(value) = &text.fill {
            self.handle_fill(&mut el, value)?;
        }

        if let Some(value) = &text.stroke {
            self.handle_stroke(&mut el, value)?;
        }

        self.el_stack.push(el);
        Ok(())
    }

    fn handle_text_path(&mut self, text_path: &TextPath) -> Result<(), Error> {
        let mut el = self.document.create_element("textPath")?;

        el.set_attribute(
            "startOffset",
            &self.get_value(&text_path.start_offset)?.to_string(),
        )?;

        el.set_attribute("method", &self.get_value(&text_path.method)?.to_string())?;
        el.set_attribute("spacing", &self.get_value(&text_path.spacing)?.to_string())?;

        el.set_attribute(
            "xlink:href",
            &format!("#{}", &self.get_value(&text_path.href)?),
        )?;

        self.append_id(&mut el)?;

        self.el_stack.push(el);
        Ok(())
    }

    fn handle_text_span(&mut self, text: &TextSpan) -> Result<(), Error> {
        let mut el = self.document.create_element("tspan")?;

        self.append_id(&mut el)?;

        let x = self
            .get_value(&text.x)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if x.len() > 0 {
            el.set_attribute("x", &x)?;
        }

        let y = self
            .get_value(&text.y)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if y.len() > 0 {
            el.set_attribute("y", &y)?;
        }

        let dx = self
            .get_value(&text.dx)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if dx.len() > 0 {
            el.set_attribute("dx", &dx)?;
        }

        let dy = self
            .get_value(&text.dy)?
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if dy.len() > 0 {
            el.set_attribute("dy", &dy)?;
        }

        let rotate = self
            .get_value(&text.rotate)?
            .into_iter()
            .map(|v| v.as_deg().to_string())
            .collect::<Vec<_>>()
            .join(",");

        if rotate.len() > 0 {
            el.set_attribute("rotate", &rotate)?;
        }

        let text_length = self.get_value(&text.text_length)?;

        el.set_attribute("textLength", text_length.to_string().as_str())?;

        let length_adjust = self.get_value(&text.length_adjust)?;

        match length_adjust {
            TextLengthAdjust::Spacing => el.set_attribute("lengthAdjust", "spacing")?,
            TextLengthAdjust::SpacingAndGlyphs => {
                el.set_attribute("lengthAdjust", "spacingAndGlyphs")?
            }
        }

        if let Some(value) = &text.font {
            self.handle_font(&mut el, value)?;
        }

        if let Some(value) = &text.layout {
            self.handle_text_layout(&mut el, value)?;
        }

        if let Some(value) = &text.fill {
            self.handle_fill(&mut el, value)?;
        }

        if let Some(value) = &text.stroke {
            self.handle_stroke(&mut el, value)?;
        }

        self.el_stack.push(el);
        Ok(())
    }

    fn handle_stroke(&self, el: &mut RefNode, value: &Stroke) -> Result<(), Error> {
        if let Some(paint) = &value.paint {
            match self.get_value(paint)? {
                Paint::Color(rgba) => el.set_attribute("stroke", rgba.to_string().as_str())?,
                Paint::Gradient(uri) => el.set_attribute("stroke", uri.to_string().as_str())?,
                Paint::Pattern(uri) => el.set_attribute("stroke", uri.to_string().as_str())?,
            }
        }

        if let Some(value) = &value.width {
            el.set_attribute("stroke-width", self.get_value(value)?.to_string().as_str())?
        }

        Ok(())
    }

    fn handle_fill(&self, el: &mut RefNode, value: &Fill) -> Result<(), Error> {
        if let Some(paint) = &value.paint {
            match self.get_value(paint)? {
                Paint::Color(rgba) => el.set_attribute("fill", rgba.to_string().as_str())?,
                Paint::Gradient(uri) => el.set_attribute("fill", uri.to_string().as_str())?,
                Paint::Pattern(uri) => el.set_attribute("fill", uri.to_string().as_str())?,
            }
        } else {
            el.set_attribute("fill", "none")?
        }

        Ok(())
    }

    fn handle_text_layout(&self, el: &mut RefNode, value: &TextLayout) -> Result<(), Error> {
        if let Some(property) = &value.write_mode {
            match property {
                WritingMode::LrTb => el.set_attribute("writing-mode", "lr-tb")?,
                WritingMode::RlTb => el.set_attribute("writing-mode", "rl-tb")?,
                WritingMode::TbRl => el.set_attribute("writing-mode", "tb-rl")?,
                WritingMode::Lr => el.set_attribute("writing-mode", "lr")?,
                WritingMode::Rl => el.set_attribute("writing-mode", "rl")?,
                WritingMode::Tb => el.set_attribute("writing-mode", "tb")?,
            }
        }

        if let Some(property) = &value.direction {
            match property {
                TextDirection::Ltr => el.set_attribute("direction", "ltr")?,
                TextDirection::Rtl => el.set_attribute("direction", "rtl")?,
            }
        }

        if let Some(property) = &value.unicode_bidi {
            match property {
                UnicodeBidi::Normal => el.set_attribute("unicode-bidi", "normal")?,
                UnicodeBidi::Embed => el.set_attribute("unicode-bidi", "embed")?,
                UnicodeBidi::BidiOverride => el.set_attribute("unicode-bidi", "bidi-override")?,
            }
        }

        if let Some(property) = &value.anchor {
            match self.get_value(property)? {
                TextAnchor::Start => el.set_attribute("text-anchor", "start")?,
                TextAnchor::Middle => el.set_attribute("text-anchor", "middle")?,
                TextAnchor::End => el.set_attribute("text-anchor", "end")?,
            }
        }

        if let Some(property) = &value.dominant_baseline {
            match self.get_value(property)? {
                DominantBaseline::Auto => el.set_attribute("dominant-baseline", "auto")?,
                DominantBaseline::UseScript => {
                    el.set_attribute("dominant-baseline", "use-script")?
                }
                DominantBaseline::NoChange => el.set_attribute("dominant-baseline", "no-change")?,
                DominantBaseline::ResetSize => {
                    el.set_attribute("dominant-baseline", "reset-size")?
                }
                DominantBaseline::Ideographic => {
                    el.set_attribute("dominant-baseline", "ideographic")?
                }
                DominantBaseline::Alphabetic => {
                    el.set_attribute("dominant-baseline", "alphabetic")?
                }
                DominantBaseline::Hanging => el.set_attribute("dominant-baseline", "hanging")?,
                DominantBaseline::Mathematical => {
                    el.set_attribute("dominant-baseline", "mathematical")?
                }
                DominantBaseline::Central => el.set_attribute("dominant-baseline", "central")?,
                DominantBaseline::Middle => el.set_attribute("dominant-baseline", "middle")?,
                DominantBaseline::TextAfterEdge => {
                    el.set_attribute("dominant-baseline", "text-after-edge")?
                }
                DominantBaseline::TextBeforeEdge => {
                    el.set_attribute("dominant-baseline", "text-before-edge")?
                }
            }
        }

        if let Some(property) = &value.alignment_baseline {
            match self.get_value(property)? {
                AlignmentBaseline::Auto => el.set_attribute("alignment-baseline", "auto")?,
                AlignmentBaseline::Baseline => {
                    el.set_attribute("alignment-baseline", "baseline")?
                }
                AlignmentBaseline::BeforeEdge => {
                    el.set_attribute("alignment-baseline", "before-edge")?
                }
                AlignmentBaseline::TextBeforeEdge => {
                    el.set_attribute("alignment-baseline", "text-before-edge")?
                }
                AlignmentBaseline::Middle => el.set_attribute("alignment-baseline", "middle")?,
                AlignmentBaseline::Central => el.set_attribute("alignment-baseline", "central")?,
                AlignmentBaseline::AfterEdge => {
                    el.set_attribute("alignment-baseline", "after-edge")?
                }
                AlignmentBaseline::TextAfterEdge => {
                    el.set_attribute("alignment-baseline", "text-after-edge")?
                }
                AlignmentBaseline::Ideographic => {
                    el.set_attribute("alignment-baseline", "ideographic")?
                }
                AlignmentBaseline::Alphabetic => {
                    el.set_attribute("alignment-baseline", "alphabetic")?
                }
                AlignmentBaseline::Hanging => el.set_attribute("alignment-baseline", "hanging")?,
                AlignmentBaseline::Mathematical => {
                    el.set_attribute("alignment-baseline", "mathematical")?
                }
            }
        }

        if let Some(property) = &value.baseline_shift {
            match self.get_value(property)? {
                BaselineShift::Baseline => el.set_attribute("baseline-shift", "baseline")?,
                BaselineShift::Sub => el.set_attribute("baseline-shift", "sub")?,
                BaselineShift::Super => el.set_attribute("baseline-shift", "super")?,
                BaselineShift::Value(measurement) => {
                    el.set_attribute("baseline-shift", measurement.to_string().as_str())?
                }
            }
        }

        Ok(())
    }

    fn handle_font(&self, el: &mut RefNode, value: &Font) -> Result<(), Error> {
        if let Some(size) = &value.size {
            el.set_attribute("font-size", self.get_value(size)?.to_string().as_str())?;
        }

        if let Some(value) = &value.family {
            el.set_attribute("font-family", self.get_value(value)?.to_string().as_str())?;
        }

        if let Some(value) = &value.style {
            match self.get_value(value)? {
                FontStyle::Normal => el.set_attribute("font-style", "normal")?,
                FontStyle::Italic => el.set_attribute("font-style", "italic")?,
                FontStyle::Oblique => el.set_attribute("font-style", "oblique")?,
            }
        }

        if let Some(value) = &value.variant {
            match self.get_value(value)? {
                FontVariant::Normal => el.set_attribute("font-variant", "normal")?,
                FontVariant::SmallCaps => el.set_attribute("font-variant", "small-caps")?,
            }
        }

        if let Some(value) = &value.weight {
            match self.get_value(value)? {
                FontWeight::Normal => el.set_attribute("font-weight", "normal")?,
                FontWeight::Bold => el.set_attribute("font-weight", "bold")?,
                FontWeight::Bolder => el.set_attribute("font-weight", "bolder")?,
                FontWeight::Lighter => el.set_attribute("font-weight", "lighter")?,
                FontWeight::W100 => el.set_attribute("font-weight", "100")?,
                FontWeight::W200 => el.set_attribute("font-weight", "200")?,
                FontWeight::W300 => el.set_attribute("font-weight", "300")?,
                FontWeight::W400 => el.set_attribute("font-weight", "400")?,
                FontWeight::W500 => el.set_attribute("font-weight", "500")?,
                FontWeight::W600 => el.set_attribute("font-weight", "600")?,
                FontWeight::W700 => el.set_attribute("font-weight", "700")?,
                FontWeight::W800 => el.set_attribute("font-weight", "800")?,
                FontWeight::W900 => el.set_attribute("font-weight", "900")?,
            }
        }

        if let Some(value) = &value.stretch {
            match self.get_value(value)? {
                FontStretch::Normal => el.set_attribute("font-stretch", "normal")?,
                FontStretch::Wider => el.set_attribute("font-stretch", "wider")?,
                FontStretch::Narrower => el.set_attribute("font-stretch", "narrower")?,
                FontStretch::UltraCondensed => {
                    el.set_attribute("font-stretch", "ultra-condensed")?
                }
                FontStretch::ExtraCondensed => {
                    el.set_attribute("font-stretch", "extra-condensed")?
                }
                FontStretch::Condensed => el.set_attribute("font-stretch", "condensed")?,
                FontStretch::SemiCondensed => el.set_attribute("font-stretch", "semi-condensed")?,
                FontStretch::SemiExpanded => el.set_attribute("font-stretch", "semi-expanded")?,
                FontStretch::Expanded => el.set_attribute("font-stretch", "expanded")?,
                FontStretch::ExtraExpanded => el.set_attribute("font-stretch", "extra-expanded")?,
                FontStretch::UltraExpanded => el.set_attribute("font-stretch", "ultra-expanded")?,
            }
        }

        Ok(())
    }

    fn handle_canvas(&mut self, el: &mut RefNode, canvas: &Canvas) -> Result<(), Error> {
        let width = self.get_value(&canvas.width)?.to_string();

        el.set_attribute("width", width.as_str())?;

        let height = self.get_value(&canvas.height)?.to_string();

        el.set_attribute("height", height.as_str())?;

        if let Some(viewbox) = &canvas.viewbox {
            el.set_attribute(
                "viewBox",
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
                el.set_attribute(
                    "preserveAspectRatio",
                    self.get_value(aspect)?.to_string().as_str(),
                )?;
            } else {
                el.set_attribute("preserveAspectRatio", "none")?;
            }
        }

        Ok(())
    }
}
