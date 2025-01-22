use crate::{
    codegen::svg::SvgAttrValueWriter,
    opcode::{Angle, Background, FuncIri, Iri, Length},
};

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

impl SvgAttrValueWriter for Iri {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Iri::Local(v) => format!("#{}", v),
            Iri::Path(v) => v.clone(),
        }
    }
}

impl SvgAttrValueWriter for FuncIri {
    fn to_svg_attr_value(&self) -> String {
        format!("url(#{})", self.0)
    }
}

impl SvgAttrValueWriter for Angle {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Angle::Deg(v) => format!("{}deg", v),
            Angle::Grad(v) => format!("{}grad", v),
            Angle::Rad(v) => format!("{}rad", v),
        }
    }
}

impl SvgAttrValueWriter for Background {
    fn to_svg_attr_value(&self) -> String {
        match self {
            Background::Accumulate => "accumulate".to_string(),
            Background::New {
                x,
                y,
                width,
                height,
            } => format!(
                "new {} {} {} {}",
                x.map(|v| v.to_string()).unwrap_or("".to_string()),
                y.map(|v| v.to_string()).unwrap_or("".to_string()),
                width.map(|v| v.to_string()).unwrap_or("".to_string()),
                height.map(|v| v.to_string()).unwrap_or("".to_string()),
            )
            .trim()
            .to_string(),
        }
    }
}
