use std::slice::Iter;

pub use cotati_device::{Device, VGLProgram};
use cotati_ir::{
    Animatable, Fill, Font, FontStyle, FontVariant, FrameVariable, Layer, PreserveAspectRatio,
    Rect, Stroke, Text, TextLayout, IR,
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
        // let doc_type = get_implementation().create_document_type(
        //     "svg",
        //     Some("-//W3C//DTD SVG 1.1//EN"),
        //     Some("http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"),
        // )?;

        let mut document = get_implementation().create_document(
            Some("http://www.w3.org/2000/svg"),
            Some("svg"),
            None,
        )?;

        let xml_decl = XmlDecl::new(xml_dom::level2::ext::XmlVersion::V11, None, Some(true));

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
                IR::Font(value) => {
                    return self.process_font(value).map(Some);
                }
                IR::TextLayout(value) => {
                    return self.process_text_layout(value).map(Some);
                }
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

    fn process_font(&mut self, value: &Font) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

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
                cotati_ir::FontWeight::Normal => el.set_attribute("font-weight", "normal")?,
                cotati_ir::FontWeight::Bold => el.set_attribute("font-weight", "bold")?,
                cotati_ir::FontWeight::Bolder => el.set_attribute("font-weight", "bolder")?,
                cotati_ir::FontWeight::Lighter => el.set_attribute("font-weight", "lighter")?,
                cotati_ir::FontWeight::W100 => el.set_attribute("font-weight", "100")?,
                cotati_ir::FontWeight::W200 => el.set_attribute("font-weight", "200")?,
                cotati_ir::FontWeight::W300 => el.set_attribute("font-weight", "300")?,
                cotati_ir::FontWeight::W400 => el.set_attribute("font-weight", "400")?,
                cotati_ir::FontWeight::W500 => el.set_attribute("font-weight", "500")?,
                cotati_ir::FontWeight::W600 => el.set_attribute("font-weight", "600")?,
                cotati_ir::FontWeight::W700 => el.set_attribute("font-weight", "700")?,
                cotati_ir::FontWeight::W800 => el.set_attribute("font-weight", "800")?,
                cotati_ir::FontWeight::W900 => el.set_attribute("font-weight", "900")?,
            }
        }

        if let Some(value) = &value.stretch {
            match self.get_value(value)? {
                cotati_ir::FontStretch::Normal => el.set_attribute("font-stretch", "normal")?,
                cotati_ir::FontStretch::Wider => el.set_attribute("font-stretch", "wider")?,
                cotati_ir::FontStretch::Narrower => el.set_attribute("font-stretch", "narrower")?,
                cotati_ir::FontStretch::UltraCondensed => {
                    el.set_attribute("font-stretch", "ultra-condensed")?
                }
                cotati_ir::FontStretch::ExtraCondensed => {
                    el.set_attribute("font-stretch", "extra-condensed")?
                }
                cotati_ir::FontStretch::Condensed => {
                    el.set_attribute("font-stretch", "condensed")?
                }
                cotati_ir::FontStretch::SemiCondensed => {
                    el.set_attribute("font-stretch", "semi-condensed")?
                }
                cotati_ir::FontStretch::SemiExpanded => {
                    el.set_attribute("font-stretch", "semi-expanded")?
                }
                cotati_ir::FontStretch::Expanded => el.set_attribute("font-stretch", "expanded")?,
                cotati_ir::FontStretch::ExtraExpanded => {
                    el.set_attribute("font-stretch", "extra-expanded")?
                }
                cotati_ir::FontStretch::UltraExpanded => {
                    el.set_attribute("font-stretch", "ultra-expanded")?
                }
            }
        }

        self.els.push(el);

        self.process_child(false)
    }

    fn process_text_layout(&mut self, value: &TextLayout) -> Result<usize, Error> {
        let mut el = self.document.create_element("g")?;

        if let Some(property) = &value.write_mode {
            match property {
                cotati_ir::WritingMode::LrTb => el.set_attribute("writing-mode", "lr-tb")?,
                cotati_ir::WritingMode::RlTb => el.set_attribute("writing-mode", "rl-tb")?,
                cotati_ir::WritingMode::TbRl => el.set_attribute("writing-mode", "tb-rl")?,
                cotati_ir::WritingMode::Lr => el.set_attribute("writing-mode", "lr")?,
                cotati_ir::WritingMode::Rl => el.set_attribute("writing-mode", "rl")?,
                cotati_ir::WritingMode::Tb => el.set_attribute("writing-mode", "tb")?,
            }
        }

        if let Some(property) = &value.vertical {
            match property {
                cotati_ir::GlyphOrientationVertical::Auto => {
                    el.set_attribute("glyph-orientation-vertical", "auto")?
                }
                cotati_ir::GlyphOrientationVertical::Angle(angle) => el.set_attribute(
                    "glyph-orientation-vertical",
                    format!("{}", angle.as_deg()).as_str(),
                )?,
            }
        }

        if let Some(property) = &value.horizontal {
            el.set_attribute(
                "glyph-orientation-vertical",
                format!("{}", property.0.as_deg()).as_str(),
            )?
        }

        if let Some(property) = &value.direction {
            match property {
                cotati_ir::TextDirection::Ltr => el.set_attribute("direction", "ltr")?,
                cotati_ir::TextDirection::Rtl => el.set_attribute("direction", "rtl")?,
            }
        }

        if let Some(property) = &value.unicode_bidi {
            match property {
                cotati_ir::UnicodeBidi::Normal => el.set_attribute("unicode-bidi", "normal")?,
                cotati_ir::UnicodeBidi::Embed => el.set_attribute("unicode-bidi", "embed")?,
                cotati_ir::UnicodeBidi::BidiOverride => {
                    el.set_attribute("unicode-bidi", "bidi-override")?
                }
            }
        }

        if let Some(property) = &value.anchor {
            match self.get_value(property)? {
                cotati_ir::TextAnchor::Start => el.set_attribute("text-anchor", "start")?,
                cotati_ir::TextAnchor::Middle => el.set_attribute("text-anchor", "middle")?,
                cotati_ir::TextAnchor::End => el.set_attribute("text-anchor", "end")?,
            }
        }

        if let Some(property) = &value.dominant_baseline {
            match self.get_value(property)? {
                cotati_ir::DominantBaseline::Auto => {
                    el.set_attribute("dominant-baseline", "auto")?
                }
                cotati_ir::DominantBaseline::UseScript => {
                    el.set_attribute("dominant-baseline", "use-script")?
                }
                cotati_ir::DominantBaseline::NoChange => {
                    el.set_attribute("dominant-baseline", "no-change")?
                }
                cotati_ir::DominantBaseline::ResetSize => {
                    el.set_attribute("dominant-baseline", "reset-size")?
                }
                cotati_ir::DominantBaseline::Ideographic => {
                    el.set_attribute("dominant-baseline", "ideographic")?
                }
                cotati_ir::DominantBaseline::Alphabetic => {
                    el.set_attribute("dominant-baseline", "alphabetic")?
                }
                cotati_ir::DominantBaseline::Hanging => {
                    el.set_attribute("dominant-baseline", "hanging")?
                }
                cotati_ir::DominantBaseline::Mathematical => {
                    el.set_attribute("dominant-baseline", "mathematical")?
                }
                cotati_ir::DominantBaseline::Central => {
                    el.set_attribute("dominant-baseline", "central")?
                }
                cotati_ir::DominantBaseline::Middle => {
                    el.set_attribute("dominant-baseline", "middle")?
                }
                cotati_ir::DominantBaseline::TextAfterEdge => {
                    el.set_attribute("dominant-baseline", "text-after-edge")?
                }
                cotati_ir::DominantBaseline::TextBeforeEdge => {
                    el.set_attribute("dominant-baseline", "text-before-edge")?
                }
            }
        }

        if let Some(property) = &value.alignment_baseline {
            match self.get_value(property)? {
                cotati_ir::AlignmentBaseline::Auto => {
                    el.set_attribute("alignment-baseline", "auto")?
                }
                cotati_ir::AlignmentBaseline::Baseline => {
                    el.set_attribute("alignment-baseline", "baseline")?
                }
                cotati_ir::AlignmentBaseline::BeforeEdge => {
                    el.set_attribute("alignment-baseline", "before-edge")?
                }
                cotati_ir::AlignmentBaseline::TextBeforeEdge => {
                    el.set_attribute("alignment-baseline", "text-before-edge")?
                }
                cotati_ir::AlignmentBaseline::Middle => {
                    el.set_attribute("alignment-baseline", "middle")?
                }
                cotati_ir::AlignmentBaseline::Central => {
                    el.set_attribute("alignment-baseline", "central")?
                }
                cotati_ir::AlignmentBaseline::AfterEdge => {
                    el.set_attribute("alignment-baseline", "after-edge")?
                }
                cotati_ir::AlignmentBaseline::TextAfterEdge => {
                    el.set_attribute("alignment-baseline", "text-after-edge")?
                }
                cotati_ir::AlignmentBaseline::Ideographic => {
                    el.set_attribute("alignment-baseline", "ideographic")?
                }
                cotati_ir::AlignmentBaseline::Alphabetic => {
                    el.set_attribute("alignment-baseline", "alphabetic")?
                }
                cotati_ir::AlignmentBaseline::Hanging => {
                    el.set_attribute("alignment-baseline", "hanging")?
                }
                cotati_ir::AlignmentBaseline::Mathematical => {
                    el.set_attribute("alignment-baseline", "mathematical")?
                }
            }
        }

        if let Some(property) = &value.baseline_shift {
            match self.get_value(property)? {
                cotati_ir::BaselineShift::Baseline => {
                    el.set_attribute("baseline-shift", "baseline")?
                }
                cotati_ir::BaselineShift::Sub => el.set_attribute("baseline-shift", "sub")?,
                cotati_ir::BaselineShift::Super => el.set_attribute("baseline-shift", "super")?,
                cotati_ir::BaselineShift::Value(measurement) => {
                    el.set_attribute("baseline-shift", measurement.to_string().as_str())?
                }
            }
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
