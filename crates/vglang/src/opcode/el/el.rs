use super::*;

/// the container element types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Container {
    If(Box<If>),
    Else(Else),
    For(Box<For>),
    Foreach(Box<Foreach>),
    Use(Box<Use>),
    Group(Group),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
    Canvas(Box<Canvas>),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    Pattern(Box<Pattern>),
    Filter(Box<Filter>),
    FeComponentTransfer(Box<FeComponentTransfer>),
    FeDiffuseLighting(Box<FeDiffuseLighting>),
    FeMerge(Box<FeMerge>),
    FeSpecularLighting(Box<FeSpecularLighting>),
    Mask(Box<Mask>),
    ClipPath(Box<ClipPath>),
}

/// the graphic types that can be used as `opcode operand`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape {
    Characters(Box<Characters>),
    Path(Box<Path>),
    GradientStop(Box<GradientStop>),
    Rect(Box<Rect>),
    Circle(Box<Circle>),
    Ellipse(Box<Ellipse>),
    Line(Box<Line>),
    Polyline(Box<Polyline>),
    Polygon(Box<Polygon>),
    FeDistantLight(Box<FeDistantLight>),
    FeSpotLight(Box<FeSpotLight>),
    FePointLight(Box<FePointLight>),
    FeBlend(Box<FeBlend>),
    FeColorMatrix(Box<FeColorMatrix>),
    FeFuncA(Box<FeFuncA>),
    FeFuncG(Box<FeFuncG>),
    FeFuncB(Box<FeFuncB>),
    FeComposite(Box<FeComposite>),
    FeDisplacementMap(Box<FeDisplacementMap>),
    FeConvolveMatrix(Box<FeConvolveMatrix>),
    FeGaussianBlur(Box<FeGaussianBlur>),
    FeMergeItem(Box<FeMergeItem>),
    FeImage(Box<FeImage>),
    FeFlood(Box<FeFlood>),
    FeOffset(Box<FeOffset>),
    FeMorphology(Box<FeMorphology>),
    FeTile(Box<FeTile>),
    FeTurbulence(Box<FeTurbulence>),
}
