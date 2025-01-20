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
impl SvgAttrsWriter for super::opcode::TextLayout {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithTransform {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Id {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Fill {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Stroke {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Font {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::EnableBackground {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithFilter {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithClipPath {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::WithMask {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Opacity {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::ViewBox {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgAttrsWriter for super::opcode::Canvas {
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
        "canvas"
    }
}
impl SvgAttrsWriter for super::opcode::Mask {
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
impl SvgAttrsWriter for super::opcode::FeColorMatrix {
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
impl SvgAttrsWriter for super::opcode::FeComponentTransfer {
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
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        if let Some(value) = &self.units {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("units", &value)?;
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
            node.set_svg_attr("color", &value)?;
        }
        if let Some(value) = &self.opacity {
            let value = ctx.valueof(&value)?.to_svg_attr_value();
            node.set_svg_attr("opacity", &value)?;
        }
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::GradientStop {
    fn to_svg_node_name(&self) -> &str {
        "gradientStop"
    }
}
impl SvgAttrsWriter for super::opcode::Group {
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
        "group"
    }
}
impl SvgAttrsWriter for super::opcode::Pattern {
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
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}
impl SvgNodeWriter for super::opcode::Use {
    fn to_svg_node_name(&self) -> &str {
        "use"
    }
}
impl SvgAttrsWriter for super::opcode::Rect {
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
