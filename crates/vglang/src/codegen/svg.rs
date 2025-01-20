#[doc = r" The trait to access context data."]
pub trait SvgContext {
    fn valueof<'a, T>(
        &'a self,
        variable: &'a super::opcode::variable::Variable<T>,
    ) -> Option<&'a T>
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
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error>;
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
impl SvgAttrsWriter for super::opcode::TextLayout {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithTransform {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Id {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Fill {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Stroke {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Font {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::EnableBackground {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithFilter {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithClipPath {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithMask {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Opacity {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::ViewBox {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Canvas {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Canvas {
    fn to_svg_node_name(&self) -> &str {
        "canvas"
    }
}
impl SvgAttrsWriter for super::opcode::Mask {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Mask {
    fn to_svg_node_name(&self) -> &str {
        "mask"
    }
}
impl SvgAttrsWriter for super::opcode::ClipPath {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::ClipPath {
    fn to_svg_node_name(&self) -> &str {
        "clipPath"
    }
}
impl SvgAttrsWriter for super::opcode::Filter {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Filter {
    fn to_svg_node_name(&self) -> &str {
        "filter"
    }
}
impl SvgAttrsWriter for super::opcode::FeDistantLight {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeDistantLight {
    fn to_svg_node_name(&self) -> &str {
        "feDistantLight"
    }
}
impl SvgAttrsWriter for super::opcode::FePointLight {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FePointLight {
    fn to_svg_node_name(&self) -> &str {
        "fePointLight"
    }
}
impl SvgAttrsWriter for super::opcode::FeSpotLight {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeSpotLight {
    fn to_svg_node_name(&self) -> &str {
        "feSpotLight"
    }
}
impl SvgAttrsWriter for super::opcode::FeBlend {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeBlend {
    fn to_svg_node_name(&self) -> &str {
        "feBlend"
    }
}
impl SvgAttrsWriter for super::opcode::FeColorMatrix {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeColorMatrix {
    fn to_svg_node_name(&self) -> &str {
        "feColorMatrix"
    }
}
impl SvgAttrsWriter for super::opcode::FeComponentTransfer {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeComponentTransfer {
    fn to_svg_node_name(&self) -> &str {
        "feComponentTransfer"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncA {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeFuncA {
    fn to_svg_node_name(&self) -> &str {
        "feFuncA"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncR {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeFuncR {
    fn to_svg_node_name(&self) -> &str {
        "feFuncR"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncG {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeFuncG {
    fn to_svg_node_name(&self) -> &str {
        "feFuncG"
    }
}
impl SvgAttrsWriter for super::opcode::FeFuncB {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeFuncB {
    fn to_svg_node_name(&self) -> &str {
        "feFuncB"
    }
}
impl SvgAttrsWriter for super::opcode::FeComposite {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeComposite {
    fn to_svg_node_name(&self) -> &str {
        "feComposite"
    }
}
impl SvgAttrsWriter for super::opcode::FeConvolveMatrix {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeConvolveMatrix {
    fn to_svg_node_name(&self) -> &str {
        "feConvolveMatrix"
    }
}
impl SvgAttrsWriter for super::opcode::FeDiffuseLighting {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeDiffuseLighting {
    fn to_svg_node_name(&self) -> &str {
        "feDiffuseLighting"
    }
}
impl SvgAttrsWriter for super::opcode::FeDisplacementMap {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeDisplacementMap {
    fn to_svg_node_name(&self) -> &str {
        "feDisplacementMap"
    }
}
impl SvgAttrsWriter for super::opcode::FeFlood {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeFlood {
    fn to_svg_node_name(&self) -> &str {
        "feFlood"
    }
}
impl SvgAttrsWriter for super::opcode::FeGaussianBlur {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeGaussianBlur {
    fn to_svg_node_name(&self) -> &str {
        "feGaussianBlur"
    }
}
impl SvgAttrsWriter for super::opcode::FeMerge {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeMerge {
    fn to_svg_node_name(&self) -> &str {
        "feMerge"
    }
}
impl SvgAttrsWriter for super::opcode::FeMergeNode {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeMergeNode {
    fn to_svg_node_name(&self) -> &str {
        "feMergeNode"
    }
}
impl SvgAttrsWriter for super::opcode::FeImage {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeImage {
    fn to_svg_node_name(&self) -> &str {
        "feImage"
    }
}
impl SvgAttrsWriter for super::opcode::FeMorphology {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeMorphology {
    fn to_svg_node_name(&self) -> &str {
        "feMorphology"
    }
}
impl SvgAttrsWriter for super::opcode::FeOffset {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeOffset {
    fn to_svg_node_name(&self) -> &str {
        "feOffset"
    }
}
impl SvgAttrsWriter for super::opcode::FeSpecularLighting {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeSpecularLighting {
    fn to_svg_node_name(&self) -> &str {
        "feSpecularLighting"
    }
}
impl SvgAttrsWriter for super::opcode::FeTile {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeTile {
    fn to_svg_node_name(&self) -> &str {
        "feTile"
    }
}
impl SvgAttrsWriter for super::opcode::FeTurbulence {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::FeTurbulence {
    fn to_svg_node_name(&self) -> &str {
        "feTurbulence"
    }
}
impl SvgAttrsWriter for super::opcode::LinearGradient {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::LinearGradient {
    fn to_svg_node_name(&self) -> &str {
        "linearGradient"
    }
}
impl SvgAttrsWriter for super::opcode::RadialGradient {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::RadialGradient {
    fn to_svg_node_name(&self) -> &str {
        "radialGradient"
    }
}
impl SvgAttrsWriter for super::opcode::GradientStop {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::GradientStop {
    fn to_svg_node_name(&self) -> &str {
        "gradientStop"
    }
}
impl SvgAttrsWriter for super::opcode::Group {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Group {
    fn to_svg_node_name(&self) -> &str {
        "group"
    }
}
impl SvgAttrsWriter for super::opcode::Pattern {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Pattern {
    fn to_svg_node_name(&self) -> &str {
        "pattern"
    }
}
impl SvgAttrsWriter for super::opcode::Use {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Use {
    fn to_svg_node_name(&self) -> &str {
        "use"
    }
}
impl SvgAttrsWriter for super::opcode::Rect {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Rect {
    fn to_svg_node_name(&self) -> &str {
        "rect"
    }
}
impl SvgAttrsWriter for super::opcode::Circle {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Circle {
    fn to_svg_node_name(&self) -> &str {
        "circle"
    }
}
impl SvgAttrsWriter for super::opcode::Ellipse {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Ellipse {
    fn to_svg_node_name(&self) -> &str {
        "ellipse"
    }
}
impl SvgAttrsWriter for super::opcode::Line {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Line {
    fn to_svg_node_name(&self) -> &str {
        "line"
    }
}
impl SvgAttrsWriter for super::opcode::Polyline {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Polyline {
    fn to_svg_node_name(&self) -> &str {
        "polyline"
    }
}
impl SvgAttrsWriter for super::opcode::Polygon {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Polygon {
    fn to_svg_node_name(&self) -> &str {
        "polygon"
    }
}
impl SvgAttrsWriter for super::opcode::Text {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Text {
    fn to_svg_node_name(&self) -> &str {
        "text"
    }
}
impl SvgAttrsWriter for super::opcode::TextSpan {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::TextSpan {
    fn to_svg_node_name(&self) -> &str {
        "tspan"
    }
}
impl SvgAttrsWriter for super::opcode::Characters {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::Characters {
    fn to_svg_node_name(&self) -> &str {
        "characters"
    }
}
impl SvgAttrsWriter for super::opcode::TextPath {
    #[allow(unused)]
    fn write_svg_attrs<C: SvgContext, Node: SvgNode>(
        &self,
        ctx: &C,
        node: &mut Node,
    ) -> Result<(), Node::Error> {
        todo!()
    }
}
impl SvgNodeWriter for super::opcode::TextPath {
    fn to_svg_node_name(&self) -> &str {
        "textPath"
    }
}
