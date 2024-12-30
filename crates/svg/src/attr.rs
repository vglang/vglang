use vglang::opcode::attrs::*;
use xml_builder::XMLElement;

/// A svg rendering attribute must implement this trait.
pub trait SvgAttr {
    fn attach(&self, element: &mut XMLElement);
}

impl SvgAttr for Fill {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for Stroke {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for Id {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for Font {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for ViewBox {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for Opacity {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for WithMask {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for WithClipPath {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for TextLayout {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for WithTransform {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for EnableBackground {
    fn attach(&self, element: &mut XMLElement) {}
}

impl SvgAttr for WithFilter {
    fn attach(&self, element: &mut XMLElement) {}
}
