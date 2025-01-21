#[doc = r" The trait to access context data."]
pub trait SvgContext {
    #[doc = r" Error type returns by this trait."]
    type Error;
    fn valueof<'a, T>(
        &'a self,
        variable: &'a super::opcode::variable::Variable<T>,
    ) -> Result<&'a T, Self::Error>
    where
        super::opcode::Data: From<T>,
        for<'c> &'c T: TryFrom<&'c super::opcode::Data, Error = ()>;
}
#[doc = r" The abstract of xml `node`."]
#[allow(unused)]
pub trait SvgNode {
    #[doc = r" Error type returns by this trait."]
    type Error;
    #[doc = r" set a new attribute/value pair."]
    fn set_svg_attr(&mut self, name: &str, value: &str) -> Result<(), Self::Error>;
}
#[doc = r" All attr node must implement this trait."]
pub trait SvgAttrsWriter {
    #[doc = r" Write node attribute/value pairs."]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>;
}
#[doc = r" elements/leaves should implement this trait."]
pub trait SvgNodeWriter: SvgAttrsWriter {
    #[doc = r" Returns the name of creating svg node."]
    fn to_svg_node_name(&self) -> &str;
}
#[doc = r" All data types should implement this trait."]
pub trait SvgAttrValueWriter {
    #[doc = r" Create a attribute value from data."]
    fn to_svg_attr_value(&self) -> String;
}
impl<T> SvgAttrValueWriter for Vec<T>
where
    T: SvgAttrValueWriter,
{
    fn to_svg_attr_value(&self) -> String {
        self.iter()
            .map(|v| v.to_svg_attr_value())
            .collect::<Vec<_>>()
            .join(",")
    }
}
impl<T, const N: usize> SvgAttrValueWriter for [T; N]
where
    T: SvgAttrValueWriter,
{
    fn to_svg_attr_value(&self) -> String {
        self.iter()
            .map(|v| v.to_svg_attr_value())
            .collect::<Vec<_>>()
            .join(",")
    }
}
impl SvgAttrValueWriter for bool {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for String {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for i8 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for u8 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for i16 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for u16 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for i32 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for u32 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for i64 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for u64 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for f32 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for f64 {
    fn to_svg_attr_value(&self) -> String {
        format!("{}", self)
    }
}
impl SvgAttrValueWriter for super::opcode::Color {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Aliceblue => "aliceblue".to_string(),
            Self::Antiquewhite => "antiquewhite".to_string(),
            Self::Aqua => "aqua".to_string(),
            Self::Aquamarine => "aquamarine".to_string(),
            Self::Azure => "azure".to_string(),
            Self::Beige => "beige".to_string(),
            Self::Bisque => "bisque".to_string(),
            Self::Black => "black".to_string(),
            Self::Blanchedalmond => "blanchedalmond".to_string(),
            Self::Blue => "blue".to_string(),
            Self::Blueviolet => "blueviolet".to_string(),
            Self::Brown => "brown".to_string(),
            Self::Burlywood => "burlywood".to_string(),
            Self::Cadetblue => "cadetblue".to_string(),
            Self::Chartreuse => "chartreuse".to_string(),
            Self::Chocolate => "chocolate".to_string(),
            Self::Coral => "coral".to_string(),
            Self::Cornflowerblue => "cornflowerblue".to_string(),
            Self::Cornsilk => "cornsilk".to_string(),
            Self::Crimson => "crimson".to_string(),
            Self::Cyan => "cyan".to_string(),
            Self::Darkblue => "darkblue".to_string(),
            Self::Darkcyan => "darkcyan".to_string(),
            Self::Darkgoldenrod => "darkgoldenrod".to_string(),
            Self::Darkgray => "darkgray".to_string(),
            Self::Darkgreen => "darkgreen".to_string(),
            Self::Darkgrey => "darkgrey".to_string(),
            Self::Darkkhaki => "darkkhaki".to_string(),
            Self::Darkmagenta => "darkmagenta".to_string(),
            Self::Darkolivegreen => "darkolivegreen".to_string(),
            Self::Darkorange => "darkorange".to_string(),
            Self::Darkorchid => "darkorchid".to_string(),
            Self::Darkred => "darkred".to_string(),
            Self::Darksalmon => "darksalmon".to_string(),
            Self::Darkseagreen => "darkseagreen".to_string(),
            Self::Darkslateblue => "darkslateblue".to_string(),
            Self::Darkslategray => "darkslategray".to_string(),
            Self::Darkslategrey => "darkslategrey".to_string(),
            Self::Darkturquoise => "darkturquoise".to_string(),
            Self::Darkviolet => "darkviolet".to_string(),
            Self::Deeppink => "deeppink".to_string(),
            Self::Deepskyblue => "deepskyblue".to_string(),
            Self::Dimgray => "dimgray".to_string(),
            Self::Dimgrey => "dimgrey".to_string(),
            Self::Dodgerblue => "dodgerblue".to_string(),
            Self::Firebrick => "firebrick".to_string(),
            Self::Floralwhite => "floralwhite".to_string(),
            Self::Forestgreen => "forestgreen".to_string(),
            Self::Fuchsia => "fuchsia".to_string(),
            Self::Gainsboro => "gainsboro".to_string(),
            Self::Ghostwhite => "ghostwhite".to_string(),
            Self::Gold => "gold".to_string(),
            Self::Goldenrod => "goldenrod".to_string(),
            Self::Gray => "gray".to_string(),
            Self::Grey => "grey".to_string(),
            Self::Green => "green".to_string(),
            Self::Greenyellow => "greenyellow".to_string(),
            Self::Honeydew => "honeydew".to_string(),
            Self::Hotpink => "hotpink".to_string(),
            Self::Indianred => "indianred".to_string(),
            Self::Indigo => "indigo".to_string(),
            Self::Ivory => "ivory".to_string(),
            Self::Khaki => "khaki".to_string(),
            Self::Lavender => "lavender".to_string(),
            Self::Lavenderblush => "lavenderblush".to_string(),
            Self::Lawngreen => "lawngreen".to_string(),
            Self::Lemonchiffon => "lemonchiffon".to_string(),
            Self::Lightblue => "lightblue".to_string(),
            Self::Lightcoral => "lightcoral".to_string(),
            Self::Lightcyan => "lightcyan".to_string(),
            Self::Lightgoldenrodyellow => "lightgoldenrodyellow".to_string(),
            Self::Lightgray => "lightgray".to_string(),
            Self::Lightgreen => "lightgreen".to_string(),
            Self::Lightgrey => "lightgrey".to_string(),
            Self::Lightpink => "lightpink".to_string(),
            Self::Lightsalmon => "lightsalmon".to_string(),
            Self::Lightseagreen => "lightseagreen".to_string(),
            Self::Lightskyblue => "lightskyblue".to_string(),
            Self::Lightslategray => "lightslategray".to_string(),
            Self::Lightslategrey => "lightslategrey".to_string(),
            Self::Lightsteelblue => "lightsteelblue".to_string(),
            Self::Lightyellow => "lightyellow".to_string(),
            Self::Lime => "lime".to_string(),
            Self::Limegreen => "limegreen".to_string(),
            Self::Linen => "linen".to_string(),
            Self::Magenta => "magenta".to_string(),
            Self::Maroon => "maroon".to_string(),
            Self::Mediumaquamarine => "mediumaquamarine".to_string(),
            Self::Mediumblue => "mediumblue".to_string(),
            Self::Mediumorchid => "mediumorchid".to_string(),
            Self::Mediumpurple => "mediumpurple".to_string(),
            Self::Mediumseagreen => "mediumseagreen".to_string(),
            Self::Mediumslateblue => "mediumslateblue".to_string(),
            Self::Mediumspringgreen => "mediumspringgreen".to_string(),
            Self::Mediumturquoise => "mediumturquoise".to_string(),
            Self::Mediumvioletred => "mediumvioletred".to_string(),
            Self::Midnightblue => "midnightblue".to_string(),
            Self::Mintcream => "mintcream".to_string(),
            Self::Mistyrose => "mistyrose".to_string(),
            Self::Moccasin => "moccasin".to_string(),
            Self::Navajowhite => "navajowhite".to_string(),
            Self::Navy => "navy".to_string(),
            Self::Oldlace => "oldlace".to_string(),
            Self::Olive => "olive".to_string(),
            Self::Olivedrab => "olivedrab".to_string(),
            Self::Orange => "orange".to_string(),
            Self::Orangered => "orangered".to_string(),
            Self::Orchid => "orchid".to_string(),
            Self::Palegoldenrod => "palegoldenrod".to_string(),
            Self::Palegreen => "palegreen".to_string(),
            Self::Paleturquoise => "paleturquoise".to_string(),
            Self::Palevioletred => "palevioletred".to_string(),
            Self::Papayawhip => "papayawhip".to_string(),
            Self::Peachpuff => "peachpuff".to_string(),
            Self::Peru => "peru".to_string(),
            Self::Pink => "pink".to_string(),
            Self::Plum => "plum".to_string(),
            Self::Powderblue => "powderblue".to_string(),
            Self::Purple => "purple".to_string(),
            Self::Red => "red".to_string(),
            Self::Rosybrown => "rosybrown".to_string(),
            Self::Royalblue => "royalblue".to_string(),
            Self::Saddlebrown => "saddlebrown".to_string(),
            Self::Salmon => "salmon".to_string(),
            Self::Sandybrown => "sandybrown".to_string(),
            Self::Seagreen => "seagreen".to_string(),
            Self::Seashell => "seashell".to_string(),
            Self::Sienna => "sienna".to_string(),
            Self::Silver => "silver".to_string(),
            Self::Skyblue => "skyblue".to_string(),
            Self::Slateblue => "slateblue".to_string(),
            Self::Slategray => "slategray".to_string(),
            Self::Slategrey => "slategrey".to_string(),
            Self::Snow => "snow".to_string(),
            Self::Springgreen => "springgreen".to_string(),
            Self::Steelblue => "steelblue".to_string(),
            Self::Tan => "tan".to_string(),
            Self::Teal => "teal".to_string(),
            Self::Thistle => "thistle".to_string(),
            Self::Tomato => "tomato".to_string(),
            Self::Turquoise => "turquoise".to_string(),
            Self::Violet => "violet".to_string(),
            Self::Wheat => "wheat".to_string(),
            Self::White => "white".to_string(),
            Self::Whitesmoke => "whitesmoke".to_string(),
            Self::Yellow => "yellow".to_string(),
            Self::Yellowgreen => "yellowgreen".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::Rgb {
    fn to_svg_attr_value(&self) -> String {
        let mut values = vec![];
        let value = &self.0;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        let value = &self.1;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        let value = &self.2;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        format!("{}({})", "rgb", values.join(","))
    }
}
impl SvgAttrValueWriter for super::opcode::Point {
    fn to_svg_attr_value(&self) -> String {
        let mut values = vec![];
        let value = &self.0;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        let value = &self.1;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        values.join(",")
    }
}
impl SvgAttrValueWriter for super::opcode::Percent {
    fn to_svg_attr_value(&self) -> String {
        self.0.to_svg_attr_value()
    }
}
impl SvgAttrValueWriter for super::opcode::Paint {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Color(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Server(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::NumberOptNumber {
    fn to_svg_attr_value(&self) -> String {
        let mut values = vec![];
        let value = &self.0;
        let value = value.to_svg_attr_value();
        values.push(value.to_svg_attr_value());
        if let Some(value) = &self.1 {
            let value = value.to_svg_attr_value();
            values.push(value.to_svg_attr_value());
        }
        values.join(",")
    }
}
impl SvgAttrValueWriter for super::opcode::Coords {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::UserSpaceOnUse => "userSpaceOnUse".to_string(),
            Self::ObjectBoundingBox => "objectBoundingBox".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::Transform {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Translate(p0, p1) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.push(p1.to_svg_attr_value());
                values.join(",")
            }
            Self::Matrix(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Scale(p0, p1) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                if let Some(value) = &p1 {
                    values.push(value.to_svg_attr_value());
                }
                values.join(",")
            }
            Self::Rotate { angle, cx, cy } => {
                let mut values = vec![];
                values.push(angle.to_svg_attr_value());
                values.push(cx.to_svg_attr_value());
                values.push(cy.to_svg_attr_value());
                values.join(",")
            }
            Self::SkewX(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::SkewY(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::Channel {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::R => "r".to_string(),
            Self::G => "g".to_string(),
            Self::B => "b".to_string(),
            Self::A => "a".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::ClipRule {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Nonzero => "nonzero".to_string(),
            Self::EvenOdd => "evenOdd".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FillRule {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Nonzero => "nonzero".to_string(),
            Self::EvenOdd => "evenOdd".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::StrokeLineCap {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Butt => "butt".to_string(),
            Self::Round => "round".to_string(),
            Self::Square => "square".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::StrokeLineJoin {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Miter(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Round => "round".to_string(),
            Self::Bevel => "bevel".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::SpreadMethod {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Pad => "pad".to_string(),
            Self::Reflect => "reflect".to_string(),
            Self::Repeat => "repeat".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FontStyle {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Italic => "italic".to_string(),
            Self::Oblique => "oblique".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FontVariant {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::SmallCaps => "smallCaps".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FontWeight {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Bold => "bold".to_string(),
            Self::Bolder => "bolder".to_string(),
            Self::Lighter => "lighter".to_string(),
            Self::W100 => "w100".to_string(),
            Self::W200 => "w200".to_string(),
            Self::W300 => "w300".to_string(),
            Self::W400 => "w400".to_string(),
            Self::W500 => "w500".to_string(),
            Self::W600 => "w600".to_string(),
            Self::W700 => "w700".to_string(),
            Self::W800 => "w800".to_string(),
            Self::W900 => "w900".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FontFamily {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Serif => "serif".to_string(),
            Self::SansSerif => "sansSerif".to_string(),
            Self::Cursive => "cursive".to_string(),
            Self::Fantasy => "fantasy".to_string(),
            Self::Monospace => "monospace".to_string(),
            Self::Generic(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FontStretch {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Wider => "wider".to_string(),
            Self::Narrower => "narrower".to_string(),
            Self::UltraCondensed => "ultraCondensed".to_string(),
            Self::ExtraCondensed => "extraCondensed".to_string(),
            Self::Condensed => "condensed".to_string(),
            Self::SemiCondensed => "semiCondensed".to_string(),
            Self::SemiExpanded => "semiExpanded".to_string(),
            Self::Expanded => "expanded".to_string(),
            Self::ExtraExpanded => "extraExpanded".to_string(),
            Self::UltraExpanded => "ultraExpanded".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::Background {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Accumulate => "accumulate".to_string(),
            Self::New {
                x,
                y,
                width,
                height,
            } => {
                let mut values = vec![];
                if let Some(value) = &x {
                    values.push(value.to_svg_attr_value());
                }
                if let Some(value) = &y {
                    values.push(value.to_svg_attr_value());
                }
                if let Some(value) = &width {
                    values.push(value.to_svg_attr_value());
                }
                if let Some(value) = &height {
                    values.push(value.to_svg_attr_value());
                }
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeIn {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::SourceGraphic => "SourceGraphic".to_string(),
            Self::SourceAlpha => "SourceAlpha".to_string(),
            Self::BackgroundImage => "BackgroundImage".to_string(),
            Self::BackgroundAlpha => "BackgroundAlpha".to_string(),
            Self::FillPaint => "FillPaint".to_string(),
            Self::StrokePaint => "StrokePaint".to_string(),
            Self::Result(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeOut {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Position => "position".to_string(),
            Self::Named(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeBlendMode {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Multiply => "multiply".to_string(),
            Self::Screen => "screen".to_string(),
            Self::Darken => "darken".to_string(),
            Self::Lighten => "lighten".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextLengthAdjust {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Spacing => "spacing".to_string(),
            Self::SpacingAndGlyphs => "spacingAndGlyphs".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::WritingMode {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::LrTb => "lrTb".to_string(),
            Self::RlTb => "rlTb".to_string(),
            Self::TbRl => "tbRl".to_string(),
            Self::Lr => "lr".to_string(),
            Self::Rl => "rl".to_string(),
            Self::Tb => "tb".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextDirection {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Ltr => "ltr".to_string(),
            Self::Rtl => "rtl".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::UnicodeBidi {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Embed => "embed".to_string(),
            Self::BidiOverride => "bidiOverride".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextAnchor {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Start => "start".to_string(),
            Self::Middle => "middle".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::DominantBaseline {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::UseScript => "useScript".to_string(),
            Self::NoChange => "noChange".to_string(),
            Self::ResetSize => "resetSize".to_string(),
            Self::Ideographic => "ideographic".to_string(),
            Self::Alphabetic => "alphabetic".to_string(),
            Self::Hanging => "hanging".to_string(),
            Self::Mathematical => "mathematical".to_string(),
            Self::Central => "central".to_string(),
            Self::Middle => "middle".to_string(),
            Self::TextAfterEdge => "textAfterEdge".to_string(),
            Self::TextBeforeEdge => "textBeforeEdge".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::AlignmentBaseline {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::Baseline => "baseline".to_string(),
            Self::BeforeEdge => "beforeEdge".to_string(),
            Self::TextBeforeEdge => "textBeforeEdge".to_string(),
            Self::Middle => "middle".to_string(),
            Self::Central => "central".to_string(),
            Self::AfterEdge => "afterEdge".to_string(),
            Self::TextAfterEdge => "textAfterEdge".to_string(),
            Self::Ideographic => "ideographic".to_string(),
            Self::Alphabetic => "alphabetic".to_string(),
            Self::Hanging => "hanging".to_string(),
            Self::Mathematical => "mathematical".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::BaselineShift {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Baseline => "baseline".to_string(),
            Self::Sub => "sub".to_string(),
            Self::Super => "super".to_string(),
            Self::Value(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextDecoration {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Underline => "underline".to_string(),
            Self::Overline => "overline".to_string(),
            Self::LineThrough => "lineThrough".to_string(),
            Self::Blink => "blink".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextPathMethod {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Align => "align".to_string(),
            Self::Stretch => "stretch".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::TextPathSpacing {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::Exact => "exact".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::LetterSpacing {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Length(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::WordSpacing {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Normal => "normal".to_string(),
            Self::Length(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::MeetOrSlice {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Meet => "meet".to_string(),
            Self::Slice => "slice".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::PreserveAspectRatio {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::XMinYMin(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMidYMin(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMaxYMin(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMinYMid(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMidYMid(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMaxYMid(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMinYMax(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMidYMax(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::XMaxYMax(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrsWriter for super::opcode::TextLayout {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.write_mode {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("writeMode", &value)?;
        }
        if let Some(value) = &self.direction {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("direction", &value)?;
        }
        if let Some(value) = &self.unicode_bidi {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("unicodeBidi", &value)?;
        }
        if let Some(value) = &self.anchor {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("text-anchor", &value)?;
        }
        if let Some(value) = &self.dominant_baseline {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dominantBaseline", &value)?;
        }
        if let Some(value) = &self.alignment_baseline {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("alignmentBaseline", &value)?;
        }
        if let Some(value) = &self.baseline_shift {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("baselineShift", &value)?;
        }
        if let Some(value) = &self.decoration {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("text-decoration", &value)?;
        }
        if let Some(value) = &self.letter_spacing {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("letterSpacing", &value)?;
        }
        if let Some(value) = &self.word_spacing {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("wordSpacing", &value)?;
        }
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithTransform {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.0;
        let value = value.to_svg_attr_value();
        node.set_svg_attr("transform", &value)?;
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Id {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.0;
        let value = value.to_svg_attr_value();
        node.set_svg_attr("id", &value)?;
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Fill {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.paint {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("fill", &value)?;
        }
        if let Some(value) = &self.rule {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("fill-rule", &value)?;
        }
        if let Some(value) = &self.opacity {
            let value = value.to_svg_attr_value();
            node.set_svg_attr("fill-opacity", &value)?;
        }
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Stroke {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.paint {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-width", &value)?;
        }
        if let Some(value) = &self.linecap {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-linecap", &value)?;
        }
        if let Some(value) = &self.linejoin {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-linejoin", &value)?;
        }
        if let Some(value) = &self.dasharray {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-dasharray", &value)?;
        }
        if let Some(value) = &self.dashoffset {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-dashoffset", &value)?;
        }
        if let Some(value) = &self.opacity {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stroke-opacity", &value)?;
        }
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Font {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.family {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-family", &value)?;
        }
        if let Some(value) = &self.style {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-style", &value)?;
        }
        if let Some(value) = &self.variant {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-variant", &value)?;
        }
        if let Some(value) = &self.weight {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-weight", &value)?;
        }
        if let Some(value) = &self.size {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-size", &value)?;
        }
        if let Some(value) = &self.stretch {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("font-stretch", &value)?;
        }
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::EnableBackground {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithFilter {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithClipPath {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithMask {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.0;
        let value = value.to_svg_attr_value();
        node.set_svg_attr("mask", &value)?;
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Opacity {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.0;
        let value = value.to_svg_attr_value();
        node.set_svg_attr("opacity", &value)?;
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Canvas {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.width;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("width", &value)?;
        let value = &self.height;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("height", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Canvas {
    fn to_svg_node_name(&self) -> &str {
        "svg"
    }
}
impl SvgAttrsWriter for super::opcode::Mask {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("maskUnits", &value)?;
        }
        if let Some(value) = &self.content_units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("contentUnits", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Mask {
    fn to_svg_node_name(&self) -> &str {
        "mask"
    }
}
impl SvgAttrsWriter for super::opcode::ClipPath {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::ClipPath {
    fn to_svg_node_name(&self) -> &str {
        "clipPath"
    }
}
impl SvgAttrsWriter for super::opcode::Filter {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("units", &value)?;
        }
        if let Some(value) = &self.primitive_units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("primitiveUnits", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.res {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("res", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Filter {
    fn to_svg_node_name(&self) -> &str {
        "filter"
    }
}
impl SvgAttrsWriter for super::opcode::FeDistantLight {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.azimuth {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("azimuth", &value)?;
        }
        if let Some(value) = &self.elevation {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("elevation", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeDistantLight {
    fn to_svg_node_name(&self) -> &str {
        "feDistantLight"
    }
}
impl SvgAttrsWriter for super::opcode::FePointLight {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.z {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("z", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FePointLight {
    fn to_svg_node_name(&self) -> &str {
        "fePointLight"
    }
}
impl SvgAttrsWriter for super::opcode::FeSpotLight {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.z {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("z", &value)?;
        }
        if let Some(value) = &self.point_at_x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("pointAtX", &value)?;
        }
        if let Some(value) = &self.point_at_y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("pointAtY", &value)?;
        }
        if let Some(value) = &self.point_at_z {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("pointAtZ", &value)?;
        }
        if let Some(value) = &self.specular_exponent {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("specularExponent", &value)?;
        }
        if let Some(value) = &self.limiting_cone_angle {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("limitingConeAngle", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeSpotLight {
    fn to_svg_node_name(&self) -> &str {
        "feSpotLight"
    }
}
impl SvgAttrsWriter for super::opcode::FeBlend {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.mode {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("mode", &value)?;
        }
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.in2 {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in2", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeBlend {
    fn to_svg_node_name(&self) -> &str {
        "feBlend"
    }
}
impl SvgAttrValueWriter for super::opcode::FeColorMatrixValues {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Matrix(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Saturate(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::HueRotate(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::LuminanceToAlpha => "luminanceToAlpha".to_string(),
        }
    }
}
impl SvgAttrsWriter for super::opcode::FeColorMatrix {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.r#in;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("in", &value)?;
        let value = &self.values;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("values", &value)?;
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeColorMatrix {
    fn to_svg_node_name(&self) -> &str {
        "feColorMatrix"
    }
}
impl SvgAttrValueWriter for super::opcode::FeFunc {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Identity => "identity".to_string(),
            Self::Table(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Discrete(p0) => {
                let mut values = vec![];
                values.push(p0.to_svg_attr_value());
                values.join(",")
            }
            Self::Linear { slope, intercept } => {
                let mut values = vec![];
                values.push(slope.to_svg_attr_value());
                values.push(intercept.to_svg_attr_value());
                values.join(",")
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                let mut values = vec![];
                values.push(amplitude.to_svg_attr_value());
                values.push(exponent.to_svg_attr_value());
                values.push(offset.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeCompositeOperator {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Over => "over".to_string(),
            Self::In => "in".to_string(),
            Self::Out => "out".to_string(),
            Self::Atop => "atop".to_string(),
            Self::Xor => "xor".to_string(),
            Self::Arithmetic { k1, k2, k3, k4 } => {
                let mut values = vec![];
                values.push(k1.to_svg_attr_value());
                values.push(k2.to_svg_attr_value());
                values.push(k3.to_svg_attr_value());
                values.push(k4.to_svg_attr_value());
                values.join(",")
            }
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeConvolveMatrixEdgeMode {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Duplicate => "duplicate".to_string(),
            Self::Wrap => "wrap".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeMorphologyOperator {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Erode => "erode".to_string(),
            Self::Dilate => "dilate".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeStitchTiles {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::Stitch => "stitch".to_string(),
            Self::NoStitch => "noStitch".to_string(),
        }
    }
}
impl SvgAttrValueWriter for super::opcode::FeTurbulenceType {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Self::FractalNoise => "fractalNoise".to_string(),
            Self::Turbulence => "turbulence".to_string(),
        }
    }
}
impl SvgAttrsWriter for super::opcode::FeComponentTransfer {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeComponentTransfer {
    fn to_svg_node_name(&self) -> &str {
        "feComponentTransfer"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncA {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeFuncA {
    fn to_svg_node_name(&self) -> &str {
        "feFuncA"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncR {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeFuncR {
    fn to_svg_node_name(&self) -> &str {
        "feFuncR"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncG {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeFuncG {
    fn to_svg_node_name(&self) -> &str {
        "feFuncG"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncB {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeFuncB {
    fn to_svg_node_name(&self) -> &str {
        "feFuncB"
    }
}
impl SvgAttrsWriter for super::opcode::FeComposite {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        let value = &self.in2;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("in2", &value)?;
        if let Some(value) = &self.operator {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("operator", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeComposite {
    fn to_svg_node_name(&self) -> &str {
        "feComposite"
    }
}
impl SvgAttrsWriter for super::opcode::FeConvolveMatrix {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.order {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("order", &value)?;
        }
        let value = &self.kernel;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("kernel", &value)?;
        if let Some(value) = &self.divisor {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("divisor", &value)?;
        }
        if let Some(value) = &self.bias {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("bias", &value)?;
        }
        if let Some(value) = &self.target_x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("targetX", &value)?;
        }
        if let Some(value) = &self.target_y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("targetY", &value)?;
        }
        let value = &self.edge_mode;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("edgeMode", &value)?;
        if let Some(value) = &self.kernel_unit_len {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("kernelUnitLen", &value)?;
        }
        let value = &self.preserve_alpha;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("preserveAlpha", &value)?;
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeConvolveMatrix {
    fn to_svg_node_name(&self) -> &str {
        "feConvolveMatrix"
    }
}
impl SvgAttrsWriter for super::opcode::FeDiffuseLighting {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.r#in;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("in", &value)?;
        if let Some(value) = &self.surface_scale {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("surfaceScale", &value)?;
        }
        if let Some(value) = &self.diffuse_constant {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("diffuseConstant", &value)?;
        }
        if let Some(value) = &self.kernel_unit_len {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("kernelUnitLen", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeDiffuseLighting {
    fn to_svg_node_name(&self) -> &str {
        "feDiffuseLighting"
    }
}
impl SvgAttrsWriter for super::opcode::FeDisplacementMap {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        let value = &self.in2;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("in2", &value)?;
        if let Some(value) = &self.scale {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("scale", &value)?;
        }
        if let Some(value) = &self.x_channel_selector {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("xChannelSelector", &value)?;
        }
        if let Some(value) = &self.y_channel_selector {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("yChannelSelector", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeDisplacementMap {
    fn to_svg_node_name(&self) -> &str {
        "feDisplacementMap"
    }
}
impl SvgAttrsWriter for super::opcode::FeFlood {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.color {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("color", &value)?;
        }
        if let Some(value) = &self.opacity {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("opacity", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeFlood {
    fn to_svg_node_name(&self) -> &str {
        "feFlood"
    }
}
impl SvgAttrsWriter for super::opcode::FeGaussianBlur {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.std_deviation {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stdDeviation", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeGaussianBlur {
    fn to_svg_node_name(&self) -> &str {
        "feGaussianBlur"
    }
}
impl SvgAttrsWriter for super::opcode::FeMerge {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeMerge {
    fn to_svg_node_name(&self) -> &str {
        "feMerge"
    }
}
impl SvgAttrsWriter for super::opcode::FeMergeNode {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeMergeNode {
    fn to_svg_node_name(&self) -> &str {
        "feMergeNode"
    }
}
impl SvgAttrsWriter for super::opcode::FeImage {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.href;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("href", &value)?;
        if let Some(value) = &self.aspect {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("aspect", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeImage {
    fn to_svg_node_name(&self) -> &str {
        "feImage"
    }
}
impl SvgAttrsWriter for super::opcode::FeMorphology {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.mode {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("mode", &value)?;
        }
        if let Some(value) = &self.radius {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("radius", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeMorphology {
    fn to_svg_node_name(&self) -> &str {
        "feMorphology"
    }
}
impl SvgAttrsWriter for super::opcode::FeOffset {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.dx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dx", &value)?;
        }
        if let Some(value) = &self.dy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dy", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeOffset {
    fn to_svg_node_name(&self) -> &str {
        "feOffset"
    }
}
impl SvgAttrsWriter for super::opcode::FeSpecularLighting {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.surface_scale {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("surfaceScale", &value)?;
        }
        if let Some(value) = &self.specular_constant {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("specularConstant", &value)?;
        }
        if let Some(value) = &self.specular_exponent {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("specularExponent", &value)?;
        }
        if let Some(value) = &self.kernel_unit_len {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("kernelUnitLen", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeSpecularLighting {
    fn to_svg_node_name(&self) -> &str {
        "feSpecularLighting"
    }
}
impl SvgAttrsWriter for super::opcode::FeTile {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.r#in {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("in", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeTile {
    fn to_svg_node_name(&self) -> &str {
        "feTile"
    }
}
impl SvgAttrsWriter for super::opcode::FeTurbulence {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.base_frequency {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("baseFrequency", &value)?;
        }
        if let Some(value) = &self.num_octaves {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("numOctaves", &value)?;
        }
        if let Some(value) = &self.seed {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("seed", &value)?;
        }
        if let Some(value) = &self.stitch_tiles {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stitchTiles", &value)?;
        }
        if let Some(value) = &self.r#type {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("type", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        if let Some(value) = &self.result {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("result", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::FeTurbulence {
    fn to_svg_node_name(&self) -> &str {
        "feTurbulence"
    }
}
impl SvgAttrsWriter for super::opcode::LinearGradient {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("gradientUnits", &value)?;
        }
        if let Some(value) = &self.transform {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("transform", &value)?;
        }
        if let Some(value) = &self.x1 {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x1", &value)?;
        }
        if let Some(value) = &self.y1 {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y1", &value)?;
        }
        if let Some(value) = &self.x2 {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x2", &value)?;
        }
        if let Some(value) = &self.y2 {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y2", &value)?;
        }
        if let Some(value) = &self.spread {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("spread", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::LinearGradient {
    fn to_svg_node_name(&self) -> &str {
        "linearGradient"
    }
}
impl SvgAttrsWriter for super::opcode::RadialGradient {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.unit {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("unit", &value)?;
        }
        if let Some(value) = &self.transform {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("transform", &value)?;
        }
        if let Some(value) = &self.cx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("cx", &value)?;
        }
        if let Some(value) = &self.cy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("cy", &value)?;
        }
        if let Some(value) = &self.r {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("r", &value)?;
        }
        if let Some(value) = &self.fx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("fx", &value)?;
        }
        if let Some(value) = &self.fy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("fy", &value)?;
        }
        if let Some(value) = &self.spread {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("spread", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::RadialGradient {
    fn to_svg_node_name(&self) -> &str {
        "radialGradient"
    }
}
impl SvgAttrsWriter for super::opcode::GradientStop {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.offset;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("offset", &value)?;
        if let Some(value) = &self.color {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stop-color", &value)?;
        }
        if let Some(value) = &self.opacity {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("stop-opacity", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::GradientStop {
    fn to_svg_node_name(&self) -> &str {
        "stop"
    }
}
impl SvgAttrsWriter for super::opcode::Group {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Group {
    fn to_svg_node_name(&self) -> &str {
        "g"
    }
}
impl SvgAttrsWriter for super::opcode::Pattern {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("units", &value)?;
        }
        if let Some(value) = &self.content_units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("contentUnits", &value)?;
        }
        if let Some(value) = &self.transform {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("transform", &value)?;
        }
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.width {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("width", &value)?;
        }
        if let Some(value) = &self.height {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("height", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Pattern {
    fn to_svg_node_name(&self) -> &str {
        "pattern"
    }
}
impl SvgAttrsWriter for super::opcode::Use {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.0;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("xlink:href", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Use {
    fn to_svg_node_name(&self) -> &str {
        "use"
    }
}
impl SvgAttrsWriter for super::opcode::Rect {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.x;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("x", &value)?;
        let value = &self.y;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("y", &value)?;
        let value = &self.width;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("width", &value)?;
        let value = &self.height;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("height", &value)?;
        if let Some(value) = &self.rx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("rx", &value)?;
        }
        if let Some(value) = &self.ry {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("ry", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Rect {
    fn to_svg_node_name(&self) -> &str {
        "rect"
    }
}
impl SvgAttrsWriter for super::opcode::Circle {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.cx;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("cx", &value)?;
        let value = &self.cy;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("cy", &value)?;
        let value = &self.r;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("r", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Circle {
    fn to_svg_node_name(&self) -> &str {
        "circle"
    }
}
impl SvgAttrsWriter for super::opcode::Ellipse {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.cx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("cx", &value)?;
        }
        if let Some(value) = &self.cy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("cy", &value)?;
        }
        let value = &self.rx;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("rx", &value)?;
        let value = &self.ry;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("ry", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Ellipse {
    fn to_svg_node_name(&self) -> &str {
        "ellipse"
    }
}
impl SvgAttrsWriter for super::opcode::Line {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        let value = &self.x1;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("x1", &value)?;
        let value = &self.y1;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("y1", &value)?;
        let value = &self.x2;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("x2", &value)?;
        let value = &self.y2;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("y2", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Line {
    fn to_svg_node_name(&self) -> &str {
        "line"
    }
}
impl SvgAttrsWriter for super::opcode::Polyline {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Polyline {
    fn to_svg_node_name(&self) -> &str {
        "polyline"
    }
}
impl SvgAttrsWriter for super::opcode::Polygon {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Polygon {
    fn to_svg_node_name(&self) -> &str {
        "polygon"
    }
}
impl SvgAttrsWriter for super::opcode::Text {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.dx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dx", &value)?;
        }
        if let Some(value) = &self.dy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dy", &value)?;
        }
        if let Some(value) = &self.rotate {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("rotate", &value)?;
        }
        if let Some(value) = &self.text_length {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("textLength", &value)?;
        }
        if let Some(value) = &self.length_adjust {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("lengthAdjust", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Text {
    fn to_svg_node_name(&self) -> &str {
        "text"
    }
}
impl SvgAttrsWriter for super::opcode::TextSpan {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.x {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("x", &value)?;
        }
        if let Some(value) = &self.y {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("y", &value)?;
        }
        if let Some(value) = &self.dx {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dx", &value)?;
        }
        if let Some(value) = &self.dy {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("dy", &value)?;
        }
        if let Some(value) = &self.rotate {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("rotate", &value)?;
        }
        if let Some(value) = &self.text_length {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("textLength", &value)?;
        }
        if let Some(value) = &self.length_adjust {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("lengthAdjust", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::TextSpan {
    fn to_svg_node_name(&self) -> &str {
        "tspan"
    }
}
impl SvgAttrsWriter for super::opcode::Characters {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Characters {
    fn to_svg_node_name(&self) -> &str {
        "characters"
    }
}
impl SvgAttrsWriter for super::opcode::TextPath {
    #[allow(unused)]
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.start_offset {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("startOffset", &value)?;
        }
        if let Some(value) = &self.method {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("method", &value)?;
        }
        if let Some(value) = &self.spacing {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("spacing", &value)?;
        }
        let value = &self.href;
        let value = ctx.valueof(&value)?.to_svg_attr_value();
        node.set_svg_attr("href", &value)?;
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::TextPath {
    fn to_svg_node_name(&self) -> &str {
        "textPath"
    }
}
