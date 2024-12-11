use std::slice::Iter;

use futures::future::BoxFuture;
pub use vglang_device::{Device, VGLProgram};
use vglang_ir::{
    Animatable, Fill, Font, FontStyle, FontVariant, FrameVariable, Layer, PreserveAspectRatio,
    Rect, Stroke, Text, TextLayout, IR,
};
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

    fn compile(&self, codes: Vec<vglang_ir::IR>) -> Self::Compile<'_> {
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
        animatable: &'a std::collections::HashMap<String, vglang_ir::AnimatableValue>,
    ) -> Self::Execute<'a> {
        Box::pin(async move { SvgGenerating::new(self.0.iter(), animatable)?.generate() })
    }
}

struct SvgGenerating<'a> {
    codes: Iter<'a, IR>,
    animatable: &'a std::collections::HashMap<String, vglang_ir::AnimatableValue>,
    document: RefNode,
    els: Vec<RefNode>,
}

impl<'a> SvgGenerating<'a> {
    fn new(
        codes: Iter<'a, IR>,
        animatable: &'a std::collections::HashMap<String, vglang_ir::AnimatableValue>,
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
                vglang_ir::Paint::Color(rgba) => el.set_attribute(
                    "stroke",
                    format!(
                        "rgb({},{},{})",
                        (rgba.0 * 255.0) as u8,
                        (rgba.1 * 255.0) as u8,
                        (rgba.2 * 255.0) as u8
                    )
                    .as_str(),
                )?,
                vglang_ir::Paint::Gradient(uri) => {
                    el.set_attribute("stroke", format!("url(#{})", uri).as_str())?
                }
                vglang_ir::Paint::Pattern(uri) => {
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
                vglang_ir::Paint::Color(rgba) => el.set_attribute(
                    "fill",
                    format!(
                        "rgb({},{},{})",
                        (rgba.0 * 255.0) as u8,
                        (rgba.1 * 255.0) as u8,
                        (rgba.2 * 255.0) as u8
                    )
                    .as_str(),
                )?,
                vglang_ir::Paint::Gradient(uri) => {
                    el.set_attribute("fill", format!("url(#{})", uri).as_str())?
                }
                vglang_ir::Paint::Pattern(uri) => {
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
                vglang_ir::FontWeight::Normal => el.set_attribute("font-weight", "normal")?,
                vglang_ir::FontWeight::Bold => el.set_attribute("font-weight", "bold")?,
                vglang_ir::FontWeight::Bolder => el.set_attribute("font-weight", "bolder")?,
                vglang_ir::FontWeight::Lighter => el.set_attribute("font-weight", "lighter")?,
                vglang_ir::FontWeight::W100 => el.set_attribute("font-weight", "100")?,
                vglang_ir::FontWeight::W200 => el.set_attribute("font-weight", "200")?,
                vglang_ir::FontWeight::W300 => el.set_attribute("font-weight", "300")?,
                vglang_ir::FontWeight::W400 => el.set_attribute("font-weight", "400")?,
                vglang_ir::FontWeight::W500 => el.set_attribute("font-weight", "500")?,
                vglang_ir::FontWeight::W600 => el.set_attribute("font-weight", "600")?,
                vglang_ir::FontWeight::W700 => el.set_attribute("font-weight", "700")?,
                vglang_ir::FontWeight::W800 => el.set_attribute("font-weight", "800")?,
                vglang_ir::FontWeight::W900 => el.set_attribute("font-weight", "900")?,
            }
        }

        if let Some(value) = &value.stretch {
            match self.get_value(value)? {
                vglang_ir::FontStretch::Normal => el.set_attribute("font-stretch", "normal")?,
                vglang_ir::FontStretch::Wider => el.set_attribute("font-stretch", "wider")?,
                vglang_ir::FontStretch::Narrower => el.set_attribute("font-stretch", "narrower")?,
                vglang_ir::FontStretch::UltraCondensed => {
                    el.set_attribute("font-stretch", "ultra-condensed")?
                }
                vglang_ir::FontStretch::ExtraCondensed => {
                    el.set_attribute("font-stretch", "extra-condensed")?
                }
                vglang_ir::FontStretch::Condensed => {
                    el.set_attribute("font-stretch", "condensed")?
                }
                vglang_ir::FontStretch::SemiCondensed => {
                    el.set_attribute("font-stretch", "semi-condensed")?
                }
                vglang_ir::FontStretch::SemiExpanded => {
                    el.set_attribute("font-stretch", "semi-expanded")?
                }
                vglang_ir::FontStretch::Expanded => el.set_attribute("font-stretch", "expanded")?,
                vglang_ir::FontStretch::ExtraExpanded => {
                    el.set_attribute("font-stretch", "extra-expanded")?
                }
                vglang_ir::FontStretch::UltraExpanded => {
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
                vglang_ir::WritingMode::LrTb => el.set_attribute("writing-mode", "lr-tb")?,
                vglang_ir::WritingMode::RlTb => el.set_attribute("writing-mode", "rl-tb")?,
                vglang_ir::WritingMode::TbRl => el.set_attribute("writing-mode", "tb-rl")?,
                vglang_ir::WritingMode::Lr => el.set_attribute("writing-mode", "lr")?,
                vglang_ir::WritingMode::Rl => el.set_attribute("writing-mode", "rl")?,
                vglang_ir::WritingMode::Tb => el.set_attribute("writing-mode", "tb")?,
            }
        }

        if let Some(property) = &value.vertical {
            match property {
                vglang_ir::GlyphOrientationVertical::Auto => {
                    el.set_attribute("glyph-orientation-vertical", "auto")?
                }
                vglang_ir::GlyphOrientationVertical::Angle(angle) => el.set_attribute(
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
                vglang_ir::TextDirection::Ltr => el.set_attribute("direction", "ltr")?,
                vglang_ir::TextDirection::Rtl => el.set_attribute("direction", "rtl")?,
            }
        }

        if let Some(property) = &value.unicode_bidi {
            match property {
                vglang_ir::UnicodeBidi::Normal => el.set_attribute("unicode-bidi", "normal")?,
                vglang_ir::UnicodeBidi::Embed => el.set_attribute("unicode-bidi", "embed")?,
                vglang_ir::UnicodeBidi::BidiOverride => {
                    el.set_attribute("unicode-bidi", "bidi-override")?
                }
            }
        }

        if let Some(property) = &value.anchor {
            match self.get_value(property)? {
                vglang_ir::TextAnchor::Start => el.set_attribute("text-anchor", "start")?,
                vglang_ir::TextAnchor::Middle => el.set_attribute("text-anchor", "middle")?,
                vglang_ir::TextAnchor::End => el.set_attribute("text-anchor", "end")?,
            }
        }

        if let Some(property) = &value.dominant_baseline {
            match self.get_value(property)? {
                vglang_ir::DominantBaseline::Auto => {
                    el.set_attribute("dominant-baseline", "auto")?
                }
                vglang_ir::DominantBaseline::UseScript => {
                    el.set_attribute("dominant-baseline", "use-script")?
                }
                vglang_ir::DominantBaseline::NoChange => {
                    el.set_attribute("dominant-baseline", "no-change")?
                }
                vglang_ir::DominantBaseline::ResetSize => {
                    el.set_attribute("dominant-baseline", "reset-size")?
                }
                vglang_ir::DominantBaseline::Ideographic => {
                    el.set_attribute("dominant-baseline", "ideographic")?
                }
                vglang_ir::DominantBaseline::Alphabetic => {
                    el.set_attribute("dominant-baseline", "alphabetic")?
                }
                vglang_ir::DominantBaseline::Hanging => {
                    el.set_attribute("dominant-baseline", "hanging")?
                }
                vglang_ir::DominantBaseline::Mathematical => {
                    el.set_attribute("dominant-baseline", "mathematical")?
                }
                vglang_ir::DominantBaseline::Central => {
                    el.set_attribute("dominant-baseline", "central")?
                }
                vglang_ir::DominantBaseline::Middle => {
                    el.set_attribute("dominant-baseline", "middle")?
                }
                vglang_ir::DominantBaseline::TextAfterEdge => {
                    el.set_attribute("dominant-baseline", "text-after-edge")?
                }
                vglang_ir::DominantBaseline::TextBeforeEdge => {
                    el.set_attribute("dominant-baseline", "text-before-edge")?
                }
            }
        }

        if let Some(property) = &value.alignment_baseline {
            match self.get_value(property)? {
                vglang_ir::AlignmentBaseline::Auto => {
                    el.set_attribute("alignment-baseline", "auto")?
                }
                vglang_ir::AlignmentBaseline::Baseline => {
                    el.set_attribute("alignment-baseline", "baseline")?
                }
                vglang_ir::AlignmentBaseline::BeforeEdge => {
                    el.set_attribute("alignment-baseline", "before-edge")?
                }
                vglang_ir::AlignmentBaseline::TextBeforeEdge => {
                    el.set_attribute("alignment-baseline", "text-before-edge")?
                }
                vglang_ir::AlignmentBaseline::Middle => {
                    el.set_attribute("alignment-baseline", "middle")?
                }
                vglang_ir::AlignmentBaseline::Central => {
                    el.set_attribute("alignment-baseline", "central")?
                }
                vglang_ir::AlignmentBaseline::AfterEdge => {
                    el.set_attribute("alignment-baseline", "after-edge")?
                }
                vglang_ir::AlignmentBaseline::TextAfterEdge => {
                    el.set_attribute("alignment-baseline", "text-after-edge")?
                }
                vglang_ir::AlignmentBaseline::Ideographic => {
                    el.set_attribute("alignment-baseline", "ideographic")?
                }
                vglang_ir::AlignmentBaseline::Alphabetic => {
                    el.set_attribute("alignment-baseline", "alphabetic")?
                }
                vglang_ir::AlignmentBaseline::Hanging => {
                    el.set_attribute("alignment-baseline", "hanging")?
                }
                vglang_ir::AlignmentBaseline::Mathematical => {
                    el.set_attribute("alignment-baseline", "mathematical")?
                }
            }
        }

        if let Some(property) = &value.baseline_shift {
            match self.get_value(property)? {
                vglang_ir::BaselineShift::Baseline => {
                    el.set_attribute("baseline-shift", "baseline")?
                }
                vglang_ir::BaselineShift::Sub => el.set_attribute("baseline-shift", "sub")?,
                vglang_ir::BaselineShift::Super => el.set_attribute("baseline-shift", "super")?,
                vglang_ir::BaselineShift::Value(measurement) => {
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
