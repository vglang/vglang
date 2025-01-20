use crate::{codegen::svg::SvgAttrValueWriter, opcode::Length};

impl SvgAttrValueWriter for Length {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Length::Em(v) => format!("{}em", v),
            Length::Ex(v) => format!("{}ex", v),
            Length::Px(v) => format!("{}px", v),
            Length::Inch(v) => format!("{}in", v),
            Length::Cm(v) => format!("{}cm", v),
            Length::Mm(v) => format!("{}mm", v),
            Length::Pt(v) => format!("{}pt", v),
            Length::Pc(v) => format!("{}pc", v),
            Length::Percent(v) => format!("{}%", v),
        }
    }
}
