#[doc = r" An attribute that can used as appliable attribute for one element must implement this trait."]
pub trait ApplyTo<E> {}
#[doc = r" An element that can used as container element's child must implement this trait."]
pub trait ContentOf<E> {}
#[doc = r" s-expr combinator must implement this trait."]
pub trait Graphics {
    #[doc = r" Generate `Stat`s for specific surface."]
    fn build(self, builder: &mut BuildContext);
}
pub trait ElementGraphics: Graphics {
    #[doc = r" Generate `Stat`s for specific surface."]
    fn build_element(self, builder: &mut BuildContext);
}
#[doc = r" build context used by [`Graphics`] trait."]
#[derive(Debug, Default)]
pub struct BuildContext(Vec<super::opcode::Opcode>);
impl From<BuildContext> for Vec<super::opcode::Opcode> {
    fn from(value: BuildContext) -> Self {
        value.0
    }
}
impl AsRef<[super::opcode::Opcode]> for BuildContext {
    fn as_ref(&self) -> &[super::opcode::Opcode] {
        &self.0
    }
}
impl BuildContext {
    #[doc = r" Push a new `Stat`"]
    pub fn push<O>(&mut self, opcode: O)
    where
        super::opcode::Opcode: From<O>,
    {
        self.0.push(opcode.into());
    }
    #[doc = r" Push a `Pop` opcode."]
    pub fn pop(&mut self) {
        self.0.push(super::opcode::Opcode::Pop);
    }
    #[doc = r" Build a [`Graphics`] and return result ase a [`Source`]."]
    #[cfg(feature = "surface")]
    #[cfg_attr(docsrs, doc(cfg(feature = "surface")))]
    pub fn create_source(grapchics: impl Graphics) -> crate::surface::Source<'static> {
        let mut builder = Self::default();
        grapchics.build(&mut builder);
        builder.0.into()
    }
}
#[doc = r" A wrapper [`Graphics`] returns by shape's apply function."]
pub struct ApplyLeaf<Attrs, Node> {
    pub attrs: Attrs,
    pub node: Node,
}
impl<Attrs, Node> Graphics for ApplyLeaf<Attrs, Node>
where
    Attrs: ApplyTo<Node> + Graphics,
    Node: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.attrs.build(builder);
        self.node.build(builder);
    }
}
#[doc = r" A wrapper [`Graphics`] returns by shape's apply function."]
pub struct ApplyElement<Attrs, Node> {
    pub attrs: Attrs,
    pub node: Node,
}
impl<Attrs, Node, E> ContentOf<E> for ApplyElement<Attrs, Node> where Node: ContentOf<E> {}
impl<Attrs, Node> Graphics for ApplyElement<Attrs, Node>
where
    Attrs: ApplyTo<Node> + Graphics,
    Node: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.attrs.build(builder);
        self.node.build(builder);
    }
}
impl<Attrs, Node> ApplyElement<Attrs, Node>
where
    Attrs: ApplyTo<Node> + Graphics,
    Node: Graphics,
{
    #[doc = r" Apply the children node."]
    pub fn children<Children>(
        self,
        children: Children,
    ) -> ApplyElementChildren<Attrs, Node, Children> {
        ApplyElementChildren {
            attrs: self.attrs,
            node: self.node,
            children,
        }
    }
}
#[doc = r" A wrapper [`Graphics`] returns by [`ApplyElement::children`] or container's `children` function."]
pub struct ApplyElementChildren<Attrs, Node, Children> {
    pub attrs: Attrs,
    pub node: Node,
    pub children: Children,
}
impl<Attrs, Node, Children, E> ContentOf<E> for ApplyElementChildren<Attrs, Node, Children> where
    Node: ContentOf<E>
{
}
impl<Attrs, Node, Children> Graphics for ApplyElementChildren<Attrs, Node, Children>
where
    Attrs: ApplyTo<Node> + Graphics,
    Node: ElementGraphics,
    Children: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.attrs.build(builder);
        self.node.build_element(builder);
        self.children.build(builder);
        builder.pop();
    }
}
#[doc = r" A wrapper [`Graphics`] returns by calling container's children function."]
pub struct ElementChildren<Node, Children> {
    pub node: Node,
    pub children: Children,
}
impl<Node, Children, E> ContentOf<E> for ElementChildren<Node, Children> where Node: ContentOf<E> {}
impl<Node, Children> ElementChildren<Node, Children>
where
    Node: Graphics,
{
    #[doc = r" Apply the children node."]
    pub fn apply<Attrs>(self, attrs: Attrs) -> ApplyElementChildren<Attrs, Node, Children>
    where
        Attrs: ApplyTo<Node>,
    {
        ApplyElementChildren {
            attrs,
            node: self.node,
            children: self.children,
        }
    }
}
impl<Node, Children> Graphics for ElementChildren<Node, Children>
where
    Node: ElementGraphics,
    Children: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.node.build_element(builder);
        self.children.build(builder);
        builder.pop();
    }
}
#[doc = r" Map item via iterator and collect them into vec."]
#[allow(unused)]
pub trait MapCollect<Item> {
    fn map_collect(self) -> Vec<Item>;
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(#[doc = r" The wrapped [`f32`] value."] pub f32);
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}
impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(value as f32)
    }
}
pub trait SangleDeg {
    fn deg(self) -> super::opcode::Angle;
}
impl<P0> SangleDeg for P0
where
    Number: From<P0>,
{
    fn deg(self) -> super::opcode::Angle {
        super::opcode::Angle::Deg(Number::from(self).0)
    }
}
pub trait SangleGrad {
    fn grad(self) -> super::opcode::Angle;
}
impl<P0> SangleGrad for P0
where
    Number: From<P0>,
{
    fn grad(self) -> super::opcode::Angle {
        super::opcode::Angle::Grad(Number::from(self).0)
    }
}
pub trait SangleRad {
    fn rad(self) -> super::opcode::Angle;
}
impl<P0> SangleRad for P0
where
    Number: From<P0>,
{
    fn rad(self) -> super::opcode::Angle {
        super::opcode::Angle::Rad(Number::from(self).0)
    }
}
pub trait SlengthEm {
    fn em(self) -> super::opcode::Length;
}
impl<P0> SlengthEm for P0
where
    Number: From<P0>,
{
    fn em(self) -> super::opcode::Length {
        super::opcode::Length::Em(Number::from(self).0)
    }
}
pub trait SlengthEx {
    fn ex(self) -> super::opcode::Length;
}
impl<P0> SlengthEx for P0
where
    Number: From<P0>,
{
    fn ex(self) -> super::opcode::Length {
        super::opcode::Length::Ex(Number::from(self).0)
    }
}
pub trait SlengthPx {
    fn px(self) -> super::opcode::Length;
}
impl<P0> SlengthPx for P0
where
    Number: From<P0>,
{
    fn px(self) -> super::opcode::Length {
        super::opcode::Length::Px(Number::from(self).0)
    }
}
pub trait SlengthInch {
    fn inch(self) -> super::opcode::Length;
}
impl<P0> SlengthInch for P0
where
    Number: From<P0>,
{
    fn inch(self) -> super::opcode::Length {
        super::opcode::Length::Inch(Number::from(self).0)
    }
}
pub trait SlengthCm {
    fn cm(self) -> super::opcode::Length;
}
impl<P0> SlengthCm for P0
where
    Number: From<P0>,
{
    fn cm(self) -> super::opcode::Length {
        super::opcode::Length::Cm(Number::from(self).0)
    }
}
pub trait SlengthMm {
    fn mm(self) -> super::opcode::Length;
}
impl<P0> SlengthMm for P0
where
    Number: From<P0>,
{
    fn mm(self) -> super::opcode::Length {
        super::opcode::Length::Mm(Number::from(self).0)
    }
}
pub trait SlengthPt {
    fn pt(self) -> super::opcode::Length;
}
impl<P0> SlengthPt for P0
where
    Number: From<P0>,
{
    fn pt(self) -> super::opcode::Length {
        super::opcode::Length::Pt(Number::from(self).0)
    }
}
pub trait SlengthPc {
    fn pc(self) -> super::opcode::Length;
}
impl<P0> SlengthPc for P0
where
    Number: From<P0>,
{
    fn pc(self) -> super::opcode::Length {
        super::opcode::Length::Pc(Number::from(self).0)
    }
}
pub trait SlengthPercent {
    fn percent(self) -> super::opcode::Length;
}
impl<P0> SlengthPercent for P0
where
    Number: From<P0>,
{
    fn percent(self) -> super::opcode::Length {
        super::opcode::Length::Percent(Number::from(self).0)
    }
}
pub trait SiriLocal {
    fn local(self) -> super::opcode::Iri;
}
impl<P0> SiriLocal for P0
where
    String: From<P0>,
{
    fn local(self) -> super::opcode::Iri {
        super::opcode::Iri::Local(self.into())
    }
}
pub trait SiriPath {
    fn path(self) -> super::opcode::Iri;
}
impl<P0> SiriPath for P0
where
    String: From<P0>,
{
    fn path(self) -> super::opcode::Iri {
        super::opcode::Iri::Path(self.into())
    }
}
impl<P0> From<P0> for super::opcode::FuncIri
where
    String: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl<P0, P1> From<(P0, P1)> for super::opcode::Point
where
    Number: From<P0>,
    Number: From<P1>,
{
    fn from(value: (P0, P1)) -> Self {
        Self(Number::from(value.0).0, Number::from(value.1).0)
    }
}
impl<P0> From<P0> for super::opcode::Percent
where
    Number: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(Number::from(value).0)
    }
}
pub trait SpaintColor {
    fn color(self) -> super::opcode::Paint;
}
impl<P0> SpaintColor for P0
where
    super::opcode::Rgb: From<P0>,
{
    fn color(self) -> super::opcode::Paint {
        super::opcode::Paint::Color(self.into())
    }
}
pub trait SpaintServer {
    fn server(self) -> super::opcode::Paint;
}
impl<P0> SpaintServer for P0
where
    super::opcode::FuncIri: From<P0>,
{
    fn server(self) -> super::opcode::Paint {
        super::opcode::Paint::Server(self.into())
    }
}
impl<P0> From<P0> for super::opcode::NumberOptNumber
where
    Number: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(Number::from(value).0, None)
    }
}
pub trait StransformTranslate {
    fn translate(self) -> super::opcode::Transform;
}
impl<P0, P1> StransformTranslate for (P0, P1)
where
    Number: From<P0>,
    Number: From<P1>,
{
    fn translate(self) -> super::opcode::Transform {
        super::opcode::Transform::Translate(Number::from(self.0).0, Number::from(self.1).0)
    }
}
pub trait StransformMatrix {
    fn matrix(self) -> super::opcode::Transform;
}
impl<P0> StransformMatrix for P0
where
    [f32; 6usize]: From<P0>,
{
    fn matrix(self) -> super::opcode::Transform {
        super::opcode::Transform::Matrix(self.into())
    }
}
pub trait StransformScale {
    fn scale(self) -> super::opcode::Transform;
}
impl<P0> StransformScale for P0
where
    Number: From<P0>,
{
    fn scale(self) -> super::opcode::Transform {
        super::opcode::Transform::Scale(Number::from(self).0, None)
    }
}
pub trait StransformRotate {
    fn rotate(self) -> super::opcode::Transform;
}
impl<Angle, Cx, Cy> StransformRotate for (Angle, Cx, Cy)
where
    Number: From<Angle>,
    Number: From<Cx>,
    Number: From<Cy>,
{
    fn rotate(self) -> super::opcode::Transform {
        super::opcode::Transform::Rotate {
            angle: Number::from(self.0).0,
            cx: Number::from(self.1).0,
            cy: Number::from(self.2).0,
        }
    }
}
pub trait StransformSkewX {
    fn skew_x(self) -> super::opcode::Transform;
}
impl<P0> StransformSkewX for P0
where
    Number: From<P0>,
{
    fn skew_x(self) -> super::opcode::Transform {
        super::opcode::Transform::SkewX(Number::from(self).0)
    }
}
pub trait StransformSkewY {
    fn skew_y(self) -> super::opcode::Transform;
}
impl<P0> StransformSkewY for P0
where
    Number: From<P0>,
{
    fn skew_y(self) -> super::opcode::Transform {
        super::opcode::Transform::SkewY(Number::from(self).0)
    }
}
pub trait SpathEventMoveTo {
    fn move_to(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventMoveTo for P0
where
    super::opcode::Point: From<P0>,
{
    fn move_to(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::MoveTo(self.into())
    }
}
pub trait SpathEventMoveToRelative {
    fn move_to_relative(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventMoveToRelative for P0
where
    super::opcode::Point: From<P0>,
{
    fn move_to_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::MoveToRelative(self.into())
    }
}
pub trait SpathEventLineTo {
    fn line_to(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventLineTo for P0
where
    super::opcode::Point: From<P0>,
{
    fn line_to(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::LineTo(self.into())
    }
}
pub trait SpathEventLineToRelative {
    fn line_to_relative(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventLineToRelative for P0
where
    super::opcode::Point: From<P0>,
{
    fn line_to_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::LineToRelative(self.into())
    }
}
pub trait SpathEventPolyline {
    fn polyline(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventPolyline for P0
where
    P0: MapCollect<super::opcode::Point>,
{
    fn polyline(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::Polyline(self.map_collect())
    }
}
pub trait SpathEventPolylineRelative {
    fn polyline_relative(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventPolylineRelative for P0
where
    P0: MapCollect<super::opcode::Point>,
{
    fn polyline_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::PolylineRelative(self.map_collect())
    }
}
pub trait SpathEventCubicBezier {
    fn cubic_bezier(self) -> super::opcode::PathEvent;
}
impl<Ctrl1, Ctrl2, ToPoint> SpathEventCubicBezier for (Ctrl1, Ctrl2, ToPoint)
where
    super::opcode::Point: From<Ctrl1>,
    super::opcode::Point: From<Ctrl2>,
    super::opcode::Point: From<ToPoint>,
{
    fn cubic_bezier(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::CubicBezier {
            ctrl1: self.0.into(),
            ctrl2: self.1.into(),
            to_point: self.2.into(),
        }
    }
}
pub trait SpathEventCubicBezierRelative {
    fn cubic_bezier_relative(self) -> super::opcode::PathEvent;
}
impl<Ctrl1, Ctrl2, ToPoint> SpathEventCubicBezierRelative for (Ctrl1, Ctrl2, ToPoint)
where
    super::opcode::Point: From<Ctrl1>,
    super::opcode::Point: From<Ctrl2>,
    super::opcode::Point: From<ToPoint>,
{
    fn cubic_bezier_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::CubicBezierRelative {
            ctrl1: self.0.into(),
            ctrl2: self.1.into(),
            to_point: self.2.into(),
        }
    }
}
pub trait SpathEventCubicBezierSmooth {
    fn cubic_bezier_smooth(self) -> super::opcode::PathEvent;
}
impl<Ctrl2, ToPoint> SpathEventCubicBezierSmooth for (Ctrl2, ToPoint)
where
    super::opcode::Point: From<Ctrl2>,
    super::opcode::Point: From<ToPoint>,
{
    fn cubic_bezier_smooth(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::CubicBezierSmooth {
            ctrl2: self.0.into(),
            to_point: self.1.into(),
        }
    }
}
pub trait SpathEventCubicBezierSmoothRelative {
    fn cubic_bezier_smooth_relative(self) -> super::opcode::PathEvent;
}
impl<Ctrl2, ToPoint> SpathEventCubicBezierSmoothRelative for (Ctrl2, ToPoint)
where
    super::opcode::Point: From<Ctrl2>,
    super::opcode::Point: From<ToPoint>,
{
    fn cubic_bezier_smooth_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::CubicBezierSmoothRelative {
            ctrl2: self.0.into(),
            to_point: self.1.into(),
        }
    }
}
pub trait SpathEventQuadraticBezier {
    fn quadratic_bezier(self) -> super::opcode::PathEvent;
}
impl<Ctrl, ToPoint> SpathEventQuadraticBezier for (Ctrl, ToPoint)
where
    super::opcode::Point: From<Ctrl>,
    super::opcode::Point: From<ToPoint>,
{
    fn quadratic_bezier(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::QuadraticBezier {
            ctrl: self.0.into(),
            to_point: self.1.into(),
        }
    }
}
pub trait SpathEventQuadraticBezierRelative {
    fn quadratic_bezier_relative(self) -> super::opcode::PathEvent;
}
impl<Ctrl, ToPoint> SpathEventQuadraticBezierRelative for (Ctrl, ToPoint)
where
    super::opcode::Point: From<Ctrl>,
    super::opcode::Point: From<ToPoint>,
{
    fn quadratic_bezier_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::QuadraticBezierRelative {
            ctrl: self.0.into(),
            to_point: self.1.into(),
        }
    }
}
pub trait SpathEventQuadraticBezierSmooth {
    fn quadratic_bezier_smooth(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventQuadraticBezierSmooth for P0
where
    super::opcode::Point: From<P0>,
{
    fn quadratic_bezier_smooth(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::QuadraticBezierSmooth(self.into())
    }
}
pub trait SpathEventQuadraticBezierSmoothRelative {
    fn quadratic_bezier_smooth_relative(self) -> super::opcode::PathEvent;
}
impl<P0> SpathEventQuadraticBezierSmoothRelative for P0
where
    super::opcode::Point: From<P0>,
{
    fn quadratic_bezier_smooth_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::QuadraticBezierSmoothRelative(self.into())
    }
}
pub trait SpathEventArc {
    fn arc(self) -> super::opcode::PathEvent;
}
impl<Rx, Ry, XRotation, LargeArc, Sweep, ToPoint> SpathEventArc
    for (Rx, Ry, XRotation, LargeArc, Sweep, ToPoint)
where
    Number: From<Rx>,
    Number: From<Ry>,
    Number: From<XRotation>,
    bool: From<LargeArc>,
    bool: From<Sweep>,
    super::opcode::Point: From<ToPoint>,
{
    fn arc(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::Arc {
            rx: Number::from(self.0).0,
            ry: Number::from(self.1).0,
            x_rotation: Number::from(self.2).0,
            large_arc: self.3.into(),
            sweep: self.4.into(),
            to_point: self.5.into(),
        }
    }
}
pub trait SpathEventArcRelative {
    fn arc_relative(self) -> super::opcode::PathEvent;
}
impl<Rx, Ry, XRotation, LargeArc, Sweep, ToPoint> SpathEventArcRelative
    for (Rx, Ry, XRotation, LargeArc, Sweep, ToPoint)
where
    Number: From<Rx>,
    Number: From<Ry>,
    Number: From<XRotation>,
    bool: From<LargeArc>,
    bool: From<Sweep>,
    super::opcode::Point: From<ToPoint>,
{
    fn arc_relative(self) -> super::opcode::PathEvent {
        super::opcode::PathEvent::ArcRelative {
            rx: Number::from(self.0).0,
            ry: Number::from(self.1).0,
            x_rotation: Number::from(self.2).0,
            large_arc: self.3.into(),
            sweep: self.4.into(),
            to_point: self.5.into(),
        }
    }
}
pub trait SstrokeLineJoinMiter {
    fn miter(self) -> super::opcode::StrokeLineJoin;
}
impl<P0> SstrokeLineJoinMiter for P0
where
    Number: From<P0>,
{
    fn miter(self) -> super::opcode::StrokeLineJoin {
        super::opcode::StrokeLineJoin::Miter(Number::from(self).0)
    }
}
pub trait SfontFamilyGeneric {
    fn generic(self) -> super::opcode::FontFamily;
}
impl<P0> SfontFamilyGeneric for P0
where
    String: From<P0>,
{
    fn generic(self) -> super::opcode::FontFamily {
        super::opcode::FontFamily::Generic(self.into())
    }
}
pub trait SbackgroundNew {
    fn new(self) -> super::opcode::Background;
}
pub trait SfeInResult {
    fn result(self) -> super::opcode::FeIn;
}
impl<P0> SfeInResult for P0
where
    String: From<P0>,
{
    fn result(self) -> super::opcode::FeIn {
        super::opcode::FeIn::Result(self.into())
    }
}
pub trait SfeOutNamed {
    fn named(self) -> super::opcode::FeOut;
}
impl<P0> SfeOutNamed for P0
where
    String: From<P0>,
{
    fn named(self) -> super::opcode::FeOut {
        super::opcode::FeOut::Named(self.into())
    }
}
pub trait SbaselineShiftValue {
    fn value(self) -> super::opcode::BaselineShift;
}
impl<P0> SbaselineShiftValue for P0
where
    super::opcode::Length: From<P0>,
{
    fn value(self) -> super::opcode::BaselineShift {
        super::opcode::BaselineShift::Value(self.into())
    }
}
pub trait SletterSpacingLength {
    fn length(self) -> super::opcode::LetterSpacing;
}
impl<P0> SletterSpacingLength for P0
where
    super::opcode::Length: From<P0>,
{
    fn length(self) -> super::opcode::LetterSpacing {
        super::opcode::LetterSpacing::Length(self.into())
    }
}
pub trait SwordSpacingLength {
    fn length(self) -> super::opcode::WordSpacing;
}
impl<P0> SwordSpacingLength for P0
where
    super::opcode::Length: From<P0>,
{
    fn length(self) -> super::opcode::WordSpacing {
        super::opcode::WordSpacing::Length(self.into())
    }
}
pub trait SpreserveAspectRatioXMinYMin {
    fn x_min_y_min(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMinYMin for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_min_y_min(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMinYMin(self.into())
    }
}
pub trait SpreserveAspectRatioXMidYMin {
    fn x_mid_y_min(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMidYMin for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_mid_y_min(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMidYMin(self.into())
    }
}
pub trait SpreserveAspectRatioXMaxYMin {
    fn x_max_y_min(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMaxYMin for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_max_y_min(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMaxYMin(self.into())
    }
}
pub trait SpreserveAspectRatioXMinYMid {
    fn x_min_y_mid(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMinYMid for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_min_y_mid(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMinYMid(self.into())
    }
}
pub trait SpreserveAspectRatioXMidYMid {
    fn x_mid_y_mid(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMidYMid for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_mid_y_mid(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMidYMid(self.into())
    }
}
pub trait SpreserveAspectRatioXMaxYMid {
    fn x_max_y_mid(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMaxYMid for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_max_y_mid(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMaxYMid(self.into())
    }
}
pub trait SpreserveAspectRatioXMinYMax {
    fn x_min_y_max(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMinYMax for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_min_y_max(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMinYMax(self.into())
    }
}
pub trait SpreserveAspectRatioXMidYMax {
    fn x_mid_y_max(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMidYMax for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_mid_y_max(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMidYMax(self.into())
    }
}
pub trait SpreserveAspectRatioXMaxYMax {
    fn x_max_y_max(self) -> super::opcode::PreserveAspectRatio;
}
impl<P0> SpreserveAspectRatioXMaxYMax for P0
where
    super::opcode::MeetOrSlice: From<P0>,
{
    fn x_max_y_max(self) -> super::opcode::PreserveAspectRatio {
        super::opcode::PreserveAspectRatio::XMaxYMax(self.into())
    }
}
impl Default for super::opcode::TextLayout {
    fn default() -> Self {
        Self {
            write_mode: None,
            direction: None,
            unicode_bidi: None,
            anchor: None,
            dominant_baseline: None,
            alignment_baseline: None,
            baseline_shift: None,
            decoration: None,
            letter_spacing: None,
            word_spacing: None,
        }
    }
}
impl super::opcode::TextLayout {
    pub fn write_mode<T>(mut self, value: T) -> Self
    where
        super::opcode::WritingMode: From<T>,
    {
        self.write_mode = Some(value.into());
        self
    }
}
impl super::opcode::TextLayout {
    pub fn direction<T>(mut self, value: T) -> Self
    where
        super::opcode::TextDirection: From<T>,
    {
        self.direction = Some(value.into());
        self
    }
}
impl super::opcode::TextLayout {
    pub fn unicode_bidi<T>(mut self, value: T) -> Self
    where
        super::opcode::UnicodeBidi: From<T>,
    {
        self.unicode_bidi = Some(value.into());
        self
    }
}
impl super::opcode::TextLayout {
    pub fn anchor<T>(mut self, value: T) -> Self
    where
        super::opcode::TextAnchor: From<T>,
    {
        self.anchor = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn dominant_baseline<T>(mut self, value: T) -> Self
    where
        super::opcode::DominantBaseline: From<T>,
    {
        self.dominant_baseline = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn alignment_baseline<T>(mut self, value: T) -> Self
    where
        super::opcode::AlignmentBaseline: From<T>,
    {
        self.alignment_baseline = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn baseline_shift<T>(mut self, value: T) -> Self
    where
        super::opcode::BaselineShift: From<T>,
    {
        self.baseline_shift = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn decoration<T>(mut self, value: T) -> Self
    where
        super::opcode::TextDecoration: From<T>,
    {
        self.decoration = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn letter_spacing<T>(mut self, value: T) -> Self
    where
        super::opcode::LetterSpacing: From<T>,
    {
        self.letter_spacing = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextLayout {
    pub fn word_spacing<T>(mut self, value: T) -> Self
    where
        super::opcode::WordSpacing: From<T>,
    {
        self.word_spacing = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::TextLayout {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::WithTransform
where
    P0: MapCollect<super::opcode::Transform>,
{
    fn from(value: P0) -> Self {
        Self(value.map_collect())
    }
}
impl Graphics for super::opcode::WithTransform {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::Id
where
    String: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::Id {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<Paint> From<Paint> for super::opcode::Fill
where
    super::opcode::Paint: From<Paint>,
{
    fn from(value: Paint) -> Self {
        Self {
            paint: value.into(),
            rule: None,
            opacity: None,
        }
    }
}
impl super::opcode::Fill {
    pub fn paint<T>(mut self, value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        self.paint = value.into();
        self
    }
}
impl super::opcode::Fill {
    pub fn rule<T>(mut self, value: T) -> Self
    where
        super::opcode::FillRule: From<T>,
    {
        self.rule = Some(value.into());
        self
    }
}
impl super::opcode::Fill {
    pub fn opacity<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.opacity = Some(Number::from(value).0);
        self
    }
}
impl Graphics for super::opcode::Fill {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<Paint> From<Paint> for super::opcode::Stroke
where
    super::opcode::Paint: From<Paint>,
{
    fn from(value: Paint) -> Self {
        Self {
            paint: super::opcode::variable::Variable::Constant(value.into()),
            width: None,
            linecap: None,
            linejoin: None,
            dasharray: None,
            dashoffset: None,
            opacity: None,
        }
    }
}
impl super::opcode::Stroke {
    pub fn paint<T>(mut self, value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        self.paint = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Stroke {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Stroke {
    pub fn linecap<T>(mut self, value: T) -> Self
    where
        super::opcode::StrokeLineCap: From<T>,
    {
        self.linecap = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Stroke {
    pub fn linejoin<T>(mut self, value: T) -> Self
    where
        super::opcode::StrokeLineJoin: From<T>,
    {
        self.linejoin = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Stroke {
    pub fn dasharray<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.dasharray = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Stroke {
    pub fn dashoffset<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.dashoffset = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Stroke {
    pub fn opacity<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.opacity = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl Graphics for super::opcode::Stroke {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Default for super::opcode::Font {
    fn default() -> Self {
        Self {
            family: None,
            style: None,
            variant: None,
            weight: None,
            size: None,
            stretch: None,
        }
    }
}
impl super::opcode::Font {
    pub fn family<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::FontFamily>,
    {
        self.family = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Font {
    pub fn style<T>(mut self, value: T) -> Self
    where
        super::opcode::FontStyle: From<T>,
    {
        self.style = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Font {
    pub fn variant<T>(mut self, value: T) -> Self
    where
        super::opcode::FontVariant: From<T>,
    {
        self.variant = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Font {
    pub fn weight<T>(mut self, value: T) -> Self
    where
        super::opcode::FontWeight: From<T>,
    {
        self.weight = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Font {
    pub fn size<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.size = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Font {
    pub fn stretch<T>(mut self, value: T) -> Self
    where
        super::opcode::FontStretch: From<T>,
    {
        self.stretch = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Font {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::EnableBackground
where
    super::opcode::Background: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::EnableBackground {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::WithFilter
where
    super::opcode::FuncIri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::WithFilter {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::WithClipPath
where
    super::opcode::FuncIri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::WithClipPath {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::WithMask
where
    super::opcode::FuncIri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::WithMask {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<P0> From<P0> for super::opcode::Opacity
where
    Number: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(Number::from(value).0)
    }
}
impl Graphics for super::opcode::Opacity {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<Minx, Miny, Width, Height> From<(Minx, Miny, Width, Height)> for super::opcode::ViewBox
where
    Number: From<Minx>,
    Number: From<Miny>,
    Number: From<Width>,
    Number: From<Height>,
{
    fn from(value: (Minx, Miny, Width, Height)) -> Self {
        Self {
            minx: super::opcode::variable::Variable::Constant(Number::from(value.0).0),
            miny: super::opcode::variable::Variable::Constant(Number::from(value.1).0),
            width: super::opcode::variable::Variable::Constant(Number::from(value.2).0),
            height: super::opcode::variable::Variable::Constant(Number::from(value.3).0),
            aspect: None,
        }
    }
}
impl super::opcode::ViewBox {
    pub fn minx<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.minx = super::opcode::variable::Variable::Constant(Number::from(value).0);
        self
    }
}
impl super::opcode::ViewBox {
    pub fn miny<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.miny = super::opcode::variable::Variable::Constant(Number::from(value).0);
        self
    }
}
impl super::opcode::ViewBox {
    pub fn width<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.width = super::opcode::variable::Variable::Constant(Number::from(value).0);
        self
    }
}
impl super::opcode::ViewBox {
    pub fn height<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.height = super::opcode::variable::Variable::Constant(Number::from(value).0);
        self
    }
}
impl super::opcode::ViewBox {
    pub fn aspect<T>(mut self, value: T) -> Self
    where
        super::opcode::PreserveAspectRatio: From<T>,
    {
        self.aspect = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::ViewBox {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl<Width, Height> From<(Width, Height)> for super::opcode::Canvas
where
    super::opcode::Length: From<Width>,
    super::opcode::Length: From<Height>,
{
    fn from(value: (Width, Height)) -> Self {
        Self {
            width: super::opcode::variable::Variable::Constant(value.0.into()),
            height: super::opcode::variable::Variable::Constant(value.1.into()),
        }
    }
}
impl super::opcode::Canvas {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Canvas {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::Canvas {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Canvas {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Canvas {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Canvas {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<X, Y, Width, Height> From<(X, Y, Width, Height)> for super::opcode::Mask
where
    super::opcode::Length: From<X>,
    super::opcode::Length: From<Y>,
    super::opcode::Length: From<Width>,
    super::opcode::Length: From<Height>,
{
    fn from(value: (X, Y, Width, Height)) -> Self {
        Self {
            units: None,
            content_units: None,
            x: Some(super::opcode::variable::Variable::Constant(value.0.into())),
            y: Some(super::opcode::variable::Variable::Constant(value.1.into())),
            width: Some(super::opcode::variable::Variable::Constant(value.2.into())),
            height: Some(super::opcode::variable::Variable::Constant(value.3.into())),
        }
    }
}
impl Default for super::opcode::Mask {
    fn default() -> Self {
        Self {
            units: None,
            content_units: None,
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }
}
impl super::opcode::Mask {
    pub fn units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Mask {
    pub fn content_units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.content_units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Mask {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Mask {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Mask {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Mask {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Mask {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Mask {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Mask {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Mask {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl Default for super::opcode::ClipPath {
    fn default() -> Self {
        Self(None)
    }
}
impl Graphics for super::opcode::ClipPath {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::ClipPath {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::ClipPath {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::ClipPath {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<X, Y, Width, Height> From<(X, Y, Width, Height)> for super::opcode::Filter
where
    super::opcode::Length: From<X>,
    super::opcode::Length: From<Y>,
    super::opcode::Length: From<Width>,
    super::opcode::Length: From<Height>,
{
    fn from(value: (X, Y, Width, Height)) -> Self {
        Self {
            units: None,
            primitive_units: None,
            x: Some(super::opcode::variable::Variable::Constant(value.0.into())),
            y: Some(super::opcode::variable::Variable::Constant(value.1.into())),
            width: Some(super::opcode::variable::Variable::Constant(value.2.into())),
            height: Some(super::opcode::variable::Variable::Constant(value.3.into())),
            res: None,
        }
    }
}
impl Default for super::opcode::Filter {
    fn default() -> Self {
        Self {
            units: None,
            primitive_units: None,
            x: None,
            y: None,
            width: None,
            height: None,
            res: None,
        }
    }
}
impl super::opcode::Filter {
    pub fn units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn primitive_units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.primitive_units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Filter {
    pub fn res<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.res = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Filter {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Filter {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Filter {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Filter {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl Default for super::opcode::FeDistantLight {
    fn default() -> Self {
        Self {
            azimuth: None,
            elevation: None,
        }
    }
}
impl super::opcode::FeDistantLight {
    pub fn azimuth<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.azimuth = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeDistantLight {
    pub fn elevation<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.elevation = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl Graphics for super::opcode::FeDistantLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeDistantLight {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FePointLight {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }
}
impl super::opcode::FePointLight {
    pub fn x<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FePointLight {
    pub fn y<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FePointLight {
    pub fn z<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.z = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl Graphics for super::opcode::FePointLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FePointLight {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeSpotLight {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            point_at_x: None,
            point_at_y: None,
            point_at_z: None,
            specular_exponent: None,
            limiting_cone_angle: None,
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn x<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn y<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn z<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.z = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn point_at_x<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.point_at_x = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn point_at_y<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.point_at_y = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn point_at_z<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.point_at_z = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn specular_exponent<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.specular_exponent = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpotLight {
    pub fn limiting_cone_angle<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.limiting_cone_angle = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl Graphics for super::opcode::FeSpotLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeSpotLight {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeBlend {
    fn default() -> Self {
        Self {
            mode: None,
            r#in: None,
            in2: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeBlend {
    pub fn mode<T>(mut self, value: T) -> Self
    where
        super::opcode::FeBlendMode: From<T>,
    {
        self.mode = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn in2<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.in2 = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeBlend {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeBlend {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeBlend {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
pub trait SfeColorMatrixValuesMatrix {
    fn matrix(self) -> super::opcode::FeColorMatrixValues;
}
impl<P0> SfeColorMatrixValuesMatrix for P0
where
    [f32; 20usize]: From<P0>,
{
    fn matrix(self) -> super::opcode::FeColorMatrixValues {
        super::opcode::FeColorMatrixValues::Matrix(self.into())
    }
}
pub trait SfeColorMatrixValuesSaturate {
    fn saturate(self) -> super::opcode::FeColorMatrixValues;
}
impl<P0> SfeColorMatrixValuesSaturate for P0
where
    Number: From<P0>,
{
    fn saturate(self) -> super::opcode::FeColorMatrixValues {
        super::opcode::FeColorMatrixValues::Saturate(Number::from(self).0)
    }
}
pub trait SfeColorMatrixValuesHueRotate {
    fn hue_rotate(self) -> super::opcode::FeColorMatrixValues;
}
impl<P0> SfeColorMatrixValuesHueRotate for P0
where
    Number: From<P0>,
{
    fn hue_rotate(self) -> super::opcode::FeColorMatrixValues {
        super::opcode::FeColorMatrixValues::HueRotate(Number::from(self).0)
    }
}
impl<Values> From<Values> for super::opcode::FeColorMatrix
where
    super::opcode::FeColorMatrixValues: From<Values>,
{
    fn from(value: Values) -> Self {
        Self {
            r#in: None,
            values: super::opcode::variable::Variable::Constant(value.into()),
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeColorMatrix {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn values<T>(mut self, value: T) -> Self
    where
        super::opcode::FeColorMatrixValues: From<T>,
    {
        self.values = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeColorMatrix {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeColorMatrix {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeColorMatrix {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
pub trait SfeFuncTable {
    fn table(self) -> super::opcode::FeFunc;
}
impl<P0> SfeFuncTable for P0
where
    P0: MapCollect<f32>,
{
    fn table(self) -> super::opcode::FeFunc {
        super::opcode::FeFunc::Table(self.map_collect())
    }
}
pub trait SfeFuncDiscrete {
    fn discrete(self) -> super::opcode::FeFunc;
}
impl<P0> SfeFuncDiscrete for P0
where
    P0: MapCollect<f32>,
{
    fn discrete(self) -> super::opcode::FeFunc {
        super::opcode::FeFunc::Discrete(self.map_collect())
    }
}
pub trait SfeFuncLinear {
    fn linear(self) -> super::opcode::FeFunc;
}
impl<Slope, Intercept> SfeFuncLinear for (Slope, Intercept)
where
    Number: From<Slope>,
    Number: From<Intercept>,
{
    fn linear(self) -> super::opcode::FeFunc {
        super::opcode::FeFunc::Linear {
            slope: Number::from(self.0).0,
            intercept: Number::from(self.1).0,
        }
    }
}
pub trait SfeFuncGamma {
    fn gamma(self) -> super::opcode::FeFunc;
}
impl<Amplitude, Exponent, Offset> SfeFuncGamma for (Amplitude, Exponent, Offset)
where
    Number: From<Amplitude>,
    Number: From<Exponent>,
    Number: From<Offset>,
{
    fn gamma(self) -> super::opcode::FeFunc {
        super::opcode::FeFunc::Gamma {
            amplitude: Number::from(self.0).0,
            exponent: Number::from(self.1).0,
            offset: Number::from(self.2).0,
        }
    }
}
pub trait SfeCompositeOperatorArithmetic {
    fn arithmetic(self) -> super::opcode::FeCompositeOperator;
}
impl<K1, K2, K3, K4> SfeCompositeOperatorArithmetic for (K1, K2, K3, K4)
where
    Number: From<K1>,
    Number: From<K2>,
    Number: From<K3>,
    Number: From<K4>,
{
    fn arithmetic(self) -> super::opcode::FeCompositeOperator {
        super::opcode::FeCompositeOperator::Arithmetic {
            k1: Number::from(self.0).0,
            k2: Number::from(self.1).0,
            k3: Number::from(self.2).0,
            k4: Number::from(self.3).0,
        }
    }
}
impl Default for super::opcode::FeComponentTransfer {
    fn default() -> Self {
        Self(None)
    }
}
impl Graphics for super::opcode::FeComponentTransfer {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::FeComponentTransfer {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::FeComponentTransfer {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::FeComponentTransfer {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<P0> From<P0> for super::opcode::FeFuncA
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::FeFuncA {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeFuncA {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<P0> From<P0> for super::opcode::FeFuncR
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::FeFuncR {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeFuncR {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<P0> From<P0> for super::opcode::FeFuncG
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::FeFuncG {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeFuncG {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<P0> From<P0> for super::opcode::FeFuncB
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::FeFuncB {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeFuncB {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<In2> From<In2> for super::opcode::FeComposite
where
    super::opcode::FeIn: From<In2>,
{
    fn from(value: In2) -> Self {
        Self {
            r#in: None,
            in2: super::opcode::variable::Variable::Constant(value.into()),
            operator: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeComposite {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn in2<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.in2 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeComposite {
    pub fn operator<T>(mut self, value: T) -> Self
    where
        super::opcode::FeCompositeOperator: From<T>,
    {
        self.operator = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeComposite {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeComposite {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeComposite {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Kernel, EdgeMode, PreserveAlpha> From<(Kernel, EdgeMode, PreserveAlpha)>
    for super::opcode::FeConvolveMatrix
where
    Kernel: MapCollect<f32>,
    super::opcode::FeConvolveMatrixEdgeMode: From<EdgeMode>,
    bool: From<PreserveAlpha>,
{
    fn from(value: (Kernel, EdgeMode, PreserveAlpha)) -> Self {
        Self {
            r#in: None,
            order: None,
            kernel: super::opcode::variable::Variable::Constant(value.0.map_collect()),
            divisor: None,
            bias: None,
            target_x: None,
            target_y: None,
            edge_mode: super::opcode::variable::Variable::Constant(value.1.into()),
            kernel_unit_len: None,
            preserve_alpha: super::opcode::variable::Variable::Constant(value.2.into()),
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn order<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.order = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn kernel<T>(mut self, value: T) -> Self
    where
        T: MapCollect<f32>,
    {
        self.kernel = super::opcode::variable::Variable::Constant(value.map_collect());
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn divisor<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.divisor = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn bias<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.bias = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn target_x<T>(mut self, value: T) -> Self
    where
        i32: From<T>,
    {
        self.target_x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn target_y<T>(mut self, value: T) -> Self
    where
        i32: From<T>,
    {
        self.target_y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn edge_mode<T>(mut self, value: T) -> Self
    where
        super::opcode::FeConvolveMatrixEdgeMode: From<T>,
    {
        self.edge_mode = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn kernel_unit_len<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.kernel_unit_len = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn preserve_alpha<T>(mut self, value: T) -> Self
    where
        bool: From<T>,
    {
        self.preserve_alpha = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeConvolveMatrix {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<In> From<In> for super::opcode::FeDiffuseLighting
where
    super::opcode::FeIn: From<In>,
{
    fn from(value: In) -> Self {
        Self {
            r#in: super::opcode::variable::Variable::Constant(value.into()),
            surface_scale: None,
            diffuse_constant: None,
            kernel_unit_len: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn surface_scale<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.surface_scale = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn diffuse_constant<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.diffuse_constant = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn kernel_unit_len<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.kernel_unit_len = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeDiffuseLighting {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::FeDiffuseLighting {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::FeDiffuseLighting {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<In2> From<In2> for super::opcode::FeDisplacementMap
where
    super::opcode::FeIn: From<In2>,
{
    fn from(value: In2) -> Self {
        Self {
            r#in: None,
            in2: super::opcode::variable::Variable::Constant(value.into()),
            scale: None,
            x_channel_selector: None,
            y_channel_selector: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn in2<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.in2 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn scale<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.scale = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn x_channel_selector<T>(mut self, value: T) -> Self
    where
        super::opcode::Channel: From<T>,
    {
        self.x_channel_selector = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn y_channel_selector<T>(mut self, value: T) -> Self
    where
        super::opcode::Channel: From<T>,
    {
        self.y_channel_selector = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeDisplacementMap {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeFlood {
    fn default() -> Self {
        Self {
            color: None,
            opacity: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeFlood {
    pub fn color<T>(mut self, value: T) -> Self
    where
        super::opcode::Rgb: From<T>,
    {
        self.color = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn opacity<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.opacity = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeFlood {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeFlood {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeFlood {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<StdDeviation> From<StdDeviation> for super::opcode::FeGaussianBlur
where
    super::opcode::NumberOptNumber: From<StdDeviation>,
{
    fn from(value: StdDeviation) -> Self {
        Self {
            r#in: None,
            std_deviation: Some(super::opcode::variable::Variable::Constant(value.into())),
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl Default for super::opcode::FeGaussianBlur {
    fn default() -> Self {
        Self {
            r#in: None,
            std_deviation: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn std_deviation<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.std_deviation = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeGaussianBlur {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeMerge {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeMerge {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMerge {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMerge {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMerge {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMerge {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeMerge {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::FeMerge {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::FeMerge {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::FeMerge {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<P0> From<P0> for super::opcode::FeMergeNode
where
    super::opcode::FeIn: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(value.into()))
    }
}
impl Graphics for super::opcode::FeMergeNode {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeMergeNode {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Href> From<Href> for super::opcode::FeImage
where
    super::opcode::FuncIri: From<Href>,
{
    fn from(value: Href) -> Self {
        Self {
            href: super::opcode::variable::Variable::Constant(value.into()),
            aspect: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeImage {
    pub fn href<T>(mut self, value: T) -> Self
    where
        super::opcode::FuncIri: From<T>,
    {
        self.href = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::FeImage {
    pub fn aspect<T>(mut self, value: T) -> Self
    where
        super::opcode::PreserveAspectRatio: From<T>,
    {
        self.aspect = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeImage {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeImage {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeImage {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeImage {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeImage {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeImage {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeImage {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeMorphology {
    fn default() -> Self {
        Self {
            r#in: None,
            mode: None,
            radius: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn mode<T>(mut self, value: T) -> Self
    where
        super::opcode::FeMorphologyOperator: From<T>,
    {
        self.mode = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn radius<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.radius = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeMorphology {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeMorphology {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeMorphology {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Dx, Dy> From<(Dx, Dy)> for super::opcode::FeOffset
where
    Number: From<Dx>,
    Number: From<Dy>,
{
    fn from(value: (Dx, Dy)) -> Self {
        Self {
            r#in: None,
            dx: Some(super::opcode::variable::Variable::Constant(
                Number::from(value.0).0,
            )),
            dy: Some(super::opcode::variable::Variable::Constant(
                Number::from(value.1).0,
            )),
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl Default for super::opcode::FeOffset {
    fn default() -> Self {
        Self {
            r#in: None,
            dx: None,
            dy: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeOffset {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn dx<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.dx = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn dy<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.dy = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeOffset {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeOffset {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeOffset {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<In> From<In> for super::opcode::FeSpecularLighting
where
    super::opcode::FeIn: From<In>,
{
    fn from(value: In) -> Self {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            surface_scale: None,
            specular_constant: None,
            specular_exponent: None,
            kernel_unit_len: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl Default for super::opcode::FeSpecularLighting {
    fn default() -> Self {
        Self {
            r#in: None,
            surface_scale: None,
            specular_constant: None,
            specular_exponent: None,
            kernel_unit_len: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn surface_scale<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.surface_scale = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn specular_constant<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.specular_constant = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn specular_exponent<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.specular_exponent = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn kernel_unit_len<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.kernel_unit_len = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeSpecularLighting {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::FeSpecularLighting {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl Default for super::opcode::FeTile {
    fn default() -> Self {
        Self {
            r#in: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeTile {
    pub fn r#in<T>(mut self, value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        self.r#in = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTile {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTile {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTile {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTile {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTile {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeTile {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeTile {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::FeTurbulence {
    fn default() -> Self {
        Self {
            base_frequency: None,
            num_octaves: None,
            seed: None,
            stitch_tiles: None,
            r#type: None,
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn base_frequency<T>(mut self, value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        self.base_frequency = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn num_octaves<T>(mut self, value: T) -> Self
    where
        i32: From<T>,
    {
        self.num_octaves = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn seed<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.seed = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn stitch_tiles<T>(mut self, value: T) -> Self
    where
        super::opcode::FeStitchTiles: From<T>,
    {
        self.stitch_tiles = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn r#type<T>(mut self, value: T) -> Self
    where
        super::opcode::FeTurbulenceType: From<T>,
    {
        self.r#type = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::FeTurbulence {
    pub fn result<T>(mut self, value: T) -> Self
    where
        String: From<T>,
    {
        self.result = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::FeTurbulence {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::FeTurbulence {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<X1, Y1, X2, Y2> From<(X1, Y1, X2, Y2)> for super::opcode::LinearGradient
where
    super::opcode::Length: From<X1>,
    super::opcode::Length: From<Y1>,
    super::opcode::Length: From<X2>,
    super::opcode::Length: From<Y2>,
{
    fn from(value: (X1, Y1, X2, Y2)) -> Self {
        Self {
            units: None,
            transform: None,
            x1: Some(super::opcode::variable::Variable::Constant(value.0.into())),
            y1: Some(super::opcode::variable::Variable::Constant(value.1.into())),
            x2: Some(super::opcode::variable::Variable::Constant(value.2.into())),
            y2: Some(super::opcode::variable::Variable::Constant(value.3.into())),
            spread: None,
        }
    }
}
impl Default for super::opcode::LinearGradient {
    fn default() -> Self {
        Self {
            units: None,
            transform: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            spread: None,
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn transform<T>(mut self, value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        self.transform = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn x1<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x1 = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn y1<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y1 = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn x2<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x2 = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn y2<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y2 = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::LinearGradient {
    pub fn spread<T>(mut self, value: T) -> Self
    where
        super::opcode::SpreadMethod: From<T>,
    {
        self.spread = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::LinearGradient {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::LinearGradient {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::LinearGradient {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::LinearGradient {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl Default for super::opcode::RadialGradient {
    fn default() -> Self {
        Self {
            unit: None,
            transform: None,
            cx: None,
            cy: None,
            r: None,
            fx: None,
            fy: None,
            spread: None,
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn unit<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.unit = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn transform<T>(mut self, value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        self.transform = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn cx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cx = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn cy<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cy = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn r<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.r = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn fx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.fx = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn fy<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.fy = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::RadialGradient {
    pub fn spread<T>(mut self, value: T) -> Self
    where
        super::opcode::SpreadMethod: From<T>,
    {
        self.spread = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::RadialGradient {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::RadialGradient {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::RadialGradient {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::RadialGradient {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<Offset> From<Offset> for super::opcode::GradientStop
where
    Number: From<Offset>,
{
    fn from(value: Offset) -> Self {
        Self {
            offset: super::opcode::variable::Variable::Constant(Number::from(value).0),
            color: None,
            opacity: None,
        }
    }
}
impl super::opcode::GradientStop {
    pub fn offset<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.offset = super::opcode::variable::Variable::Constant(Number::from(value).0);
        self
    }
}
impl super::opcode::GradientStop {
    pub fn color<T>(mut self, value: T) -> Self
    where
        super::opcode::Rgb: From<T>,
    {
        self.color = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::GradientStop {
    pub fn opacity<T>(mut self, value: T) -> Self
    where
        Number: From<T>,
    {
        self.opacity = Some(super::opcode::variable::Variable::Constant(
            Number::from(value).0,
        ));
        self
    }
}
impl Graphics for super::opcode::GradientStop {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::GradientStop {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::Group {
    fn default() -> Self {
        Self
    }
}
impl Graphics for super::opcode::Group {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Group {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Group {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Group {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<Events, Length> From<(Events, Length)> for super::opcode::Path
where
    Events: MapCollect<super::opcode::PathEvent>,
    super::opcode::Length: From<Length>,
{
    fn from(value: (Events, Length)) -> Self {
        Self {
            events: super::opcode::variable::Variable::Constant(value.0.map_collect()),
            length: super::opcode::variable::Variable::Constant(value.1.into()),
        }
    }
}
impl super::opcode::Path {
    pub fn events<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::PathEvent>,
    {
        self.events = super::opcode::variable::Variable::Constant(value.map_collect());
        self
    }
}
impl super::opcode::Path {
    pub fn length<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.length = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::Path {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Path {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl Default for super::opcode::Pattern {
    fn default() -> Self {
        Self {
            units: None,
            content_units: None,
            transform: None,
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }
}
impl super::opcode::Pattern {
    pub fn units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn content_units<T>(mut self, value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        self.content_units = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn transform<T>(mut self, value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        self.transform = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Pattern {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Pattern {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Pattern {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Pattern {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Pattern {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<P0> From<P0> for super::opcode::Use
where
    super::opcode::Iri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(value.into()))
    }
}
impl Graphics for super::opcode::Use {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Use {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<X, Y, Width, Height> From<(X, Y, Width, Height)> for super::opcode::Rect
where
    super::opcode::Length: From<X>,
    super::opcode::Length: From<Y>,
    super::opcode::Length: From<Width>,
    super::opcode::Length: From<Height>,
{
    fn from(value: (X, Y, Width, Height)) -> Self {
        Self {
            x: super::opcode::variable::Variable::Constant(value.0.into()),
            y: super::opcode::variable::Variable::Constant(value.1.into()),
            width: super::opcode::variable::Variable::Constant(value.2.into()),
            height: super::opcode::variable::Variable::Constant(value.3.into()),
            rx: None,
            ry: None,
        }
    }
}
impl super::opcode::Rect {
    pub fn x<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Rect {
    pub fn y<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Rect {
    pub fn width<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.width = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Rect {
    pub fn height<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.height = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Rect {
    pub fn rx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.rx = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Rect {
    pub fn ry<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.ry = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Rect {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Rect {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Cx, Cy, R> From<(Cx, Cy, R)> for super::opcode::Circle
where
    super::opcode::Length: From<Cx>,
    super::opcode::Length: From<Cy>,
    super::opcode::Length: From<R>,
{
    fn from(value: (Cx, Cy, R)) -> Self {
        Self {
            cx: super::opcode::variable::Variable::Constant(value.0.into()),
            cy: super::opcode::variable::Variable::Constant(value.1.into()),
            r: super::opcode::variable::Variable::Constant(value.2.into()),
        }
    }
}
impl super::opcode::Circle {
    pub fn cx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cx = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Circle {
    pub fn cy<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cy = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Circle {
    pub fn r<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.r = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::Circle {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Circle {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Rx, Ry> From<(Rx, Ry)> for super::opcode::Ellipse
where
    super::opcode::Length: From<Rx>,
    super::opcode::Length: From<Ry>,
{
    fn from(value: (Rx, Ry)) -> Self {
        Self {
            cx: None,
            cy: None,
            rx: super::opcode::variable::Variable::Constant(value.0.into()),
            ry: super::opcode::variable::Variable::Constant(value.1.into()),
        }
    }
}
impl super::opcode::Ellipse {
    pub fn cx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cx = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Ellipse {
    pub fn cy<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.cy = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::Ellipse {
    pub fn rx<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.rx = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Ellipse {
    pub fn ry<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.ry = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::Ellipse {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Ellipse {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<X1, Y1, X2, Y2> From<(X1, Y1, X2, Y2)> for super::opcode::Line
where
    super::opcode::Length: From<X1>,
    super::opcode::Length: From<Y1>,
    super::opcode::Length: From<X2>,
    super::opcode::Length: From<Y2>,
{
    fn from(value: (X1, Y1, X2, Y2)) -> Self {
        Self {
            x1: super::opcode::variable::Variable::Constant(value.0.into()),
            y1: super::opcode::variable::Variable::Constant(value.1.into()),
            x2: super::opcode::variable::Variable::Constant(value.2.into()),
            y2: super::opcode::variable::Variable::Constant(value.3.into()),
        }
    }
}
impl super::opcode::Line {
    pub fn x1<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x1 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Line {
    pub fn y1<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y1 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Line {
    pub fn x2<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.x2 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl super::opcode::Line {
    pub fn y2<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.y2 = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::Line {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Line {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<P0> From<P0> for super::opcode::Polyline
where
    P0: MapCollect<super::opcode::Point>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ))
    }
}
impl Graphics for super::opcode::Polyline {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Polyline {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<P0> From<P0> for super::opcode::Polygon
where
    P0: MapCollect<super::opcode::Point>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ))
    }
}
impl Graphics for super::opcode::Polygon {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Polygon {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<X, Y> From<(X, Y)> for super::opcode::Text
where
    X: MapCollect<super::opcode::Length>,
    Y: MapCollect<super::opcode::Length>,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                value.0.map_collect(),
            )),
            y: Some(super::opcode::variable::Variable::Constant(
                value.1.map_collect(),
            )),
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl Default for super::opcode::Text {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl super::opcode::Text {
    pub fn x<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn y<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn dx<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.dx = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn dy<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.dy = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn rotate<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Angle>,
    {
        self.rotate = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn text_length<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.text_length = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::Text {
    pub fn length_adjust<T>(mut self, value: T) -> Self
    where
        super::opcode::TextLengthAdjust: From<T>,
    {
        self.length_adjust = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::Text {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::Text {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::Text {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::Text {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<X, Y> From<(X, Y)> for super::opcode::TextSpan
where
    X: MapCollect<super::opcode::Length>,
    Y: MapCollect<super::opcode::Length>,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                value.0.map_collect(),
            )),
            y: Some(super::opcode::variable::Variable::Constant(
                value.1.map_collect(),
            )),
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl Default for super::opcode::TextSpan {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl super::opcode::TextSpan {
    pub fn x<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.x = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn y<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.y = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn dx<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.dx = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn dy<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.dy = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn rotate<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Angle>,
    {
        self.rotate = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn text_length<T>(mut self, value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        self.text_length = Some(super::opcode::variable::Variable::Constant(
            value.map_collect(),
        ));
        self
    }
}
impl super::opcode::TextSpan {
    pub fn length_adjust<T>(mut self, value: T) -> Self
    where
        super::opcode::TextLengthAdjust: From<T>,
    {
        self.length_adjust = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl Graphics for super::opcode::TextSpan {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::TextSpan {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::TextSpan {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::TextSpan {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl<P0> From<P0> for super::opcode::Characters
where
    String: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl Graphics for super::opcode::Characters {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl super::opcode::Characters {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl<Href> From<Href> for super::opcode::TextPath
where
    super::opcode::Iri: From<Href>,
{
    fn from(value: Href) -> Self {
        Self {
            start_offset: None,
            method: None,
            spacing: None,
            href: super::opcode::variable::Variable::Constant(value.into()),
        }
    }
}
impl super::opcode::TextPath {
    pub fn start_offset<T>(mut self, value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        self.start_offset = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextPath {
    pub fn method<T>(mut self, value: T) -> Self
    where
        super::opcode::TextPathMethod: From<T>,
    {
        self.method = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextPath {
    pub fn spacing<T>(mut self, value: T) -> Self
    where
        super::opcode::TextPathSpacing: From<T>,
    {
        self.spacing = Some(super::opcode::variable::Variable::Constant(value.into()));
        self
    }
}
impl super::opcode::TextPath {
    pub fn href<T>(mut self, value: T) -> Self
    where
        super::opcode::Iri: From<T>,
    {
        self.href = super::opcode::variable::Variable::Constant(value.into());
        self
    }
}
impl Graphics for super::opcode::TextPath {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
        builder.pop();
    }
}
impl ElementGraphics for super::opcode::TextPath {
    fn build_element(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Element::from(self));
    }
}
impl super::opcode::TextPath {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
    }
}
impl super::opcode::TextPath {
    pub fn children<C>(self, children: C) -> ElementChildren<Self, C>
    where
        C: ContentOf<Self>,
    {
        ElementChildren {
            node: self,
            children,
        }
    }
}
impl ContentOf<super::opcode::Text> for super::opcode::Text {}
impl ContentOf<super::opcode::Text> for super::opcode::TextSpan {}
impl ContentOf<super::opcode::Text> for super::opcode::TextPath {}
impl ContentOf<super::opcode::Text> for super::opcode::Characters {}
impl ContentOf<super::opcode::TextSpan> for super::opcode::TextSpan {}
impl ContentOf<super::opcode::TextSpan> for super::opcode::Characters {}
impl ContentOf<super::opcode::TextPath> for super::opcode::TextSpan {}
impl ContentOf<super::opcode::TextPath> for super::opcode::Characters {}
impl ContentOf<super::opcode::Group> for super::opcode::Text {}
impl ContentOf<super::opcode::Group> for super::opcode::ClipPath {}
impl ContentOf<super::opcode::Group> for super::opcode::Filter {}
impl ContentOf<super::opcode::Group> for super::opcode::Mask {}
impl ContentOf<super::opcode::Group> for super::opcode::Group {}
impl ContentOf<super::opcode::Group> for super::opcode::Canvas {}
impl ContentOf<super::opcode::Group> for super::opcode::Rect {}
impl ContentOf<super::opcode::Group> for super::opcode::Circle {}
impl ContentOf<super::opcode::Group> for super::opcode::Ellipse {}
impl ContentOf<super::opcode::Group> for super::opcode::Line {}
impl ContentOf<super::opcode::Group> for super::opcode::Polyline {}
impl ContentOf<super::opcode::Group> for super::opcode::Polygon {}
impl ContentOf<super::opcode::Group> for super::opcode::LinearGradient {}
impl ContentOf<super::opcode::Group> for super::opcode::RadialGradient {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Text {}
impl ContentOf<super::opcode::Canvas> for super::opcode::ClipPath {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Filter {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Mask {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Group {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Canvas {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Rect {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Circle {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Ellipse {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Line {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Polyline {}
impl ContentOf<super::opcode::Canvas> for super::opcode::Polygon {}
impl ContentOf<super::opcode::Canvas> for super::opcode::LinearGradient {}
impl ContentOf<super::opcode::Canvas> for super::opcode::RadialGradient {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeBlend {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeColorMatrix {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeComponentTransfer {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeComposite {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeConvolveMatrix {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeDiffuseLighting {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeDisplacementMap {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeFlood {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeGaussianBlur {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeMerge {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeImage {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeMorphology {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeOffset {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeSpecularLighting {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeTile {}
impl ContentOf<super::opcode::Filter> for super::opcode::FeTurbulence {}
impl ContentOf<super::opcode::FeComponentTransfer> for super::opcode::FeFuncR {}
impl ContentOf<super::opcode::FeComponentTransfer> for super::opcode::FeFuncG {}
impl ContentOf<super::opcode::FeComponentTransfer> for super::opcode::FeFuncB {}
impl ContentOf<super::opcode::FeComponentTransfer> for super::opcode::FeFuncA {}
impl ContentOf<super::opcode::FeMerge> for super::opcode::FeMergeNode {}
impl ContentOf<super::opcode::FeDiffuseLighting> for super::opcode::FeDistantLight {}
impl ContentOf<super::opcode::FeDiffuseLighting> for super::opcode::FePointLight {}
impl ContentOf<super::opcode::FeDiffuseLighting> for super::opcode::FeSpotLight {}
impl ContentOf<super::opcode::FeSpecularLighting> for super::opcode::FeDistantLight {}
impl ContentOf<super::opcode::FeSpecularLighting> for super::opcode::FePointLight {}
impl ContentOf<super::opcode::FeSpecularLighting> for super::opcode::FeSpotLight {}
impl ContentOf<super::opcode::LinearGradient> for super::opcode::GradientStop {}
impl ContentOf<super::opcode::RadialGradient> for super::opcode::GradientStop {}
impl ContentOf<super::opcode::Pattern> for super::opcode::ClipPath {}
impl ContentOf<super::opcode::Pattern> for super::opcode::Filter {}
impl ContentOf<super::opcode::Pattern> for super::opcode::Mask {}
impl ContentOf<super::opcode::Pattern> for super::opcode::Text {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Rect {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Circle {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Ellipse {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Line {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Polyline {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Polygon {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Text {}
impl ContentOf<super::opcode::ClipPath> for super::opcode::Use {}
impl ContentOf<super::opcode::Mask> for super::opcode::Text {}
impl ContentOf<super::opcode::Mask> for super::opcode::ClipPath {}
impl ContentOf<super::opcode::Mask> for super::opcode::Filter {}
impl ContentOf<super::opcode::Mask> for super::opcode::Group {}
impl ContentOf<super::opcode::Mask> for super::opcode::Canvas {}
impl ContentOf<super::opcode::Mask> for super::opcode::Use {}
impl ContentOf<super::opcode::Mask> for super::opcode::Rect {}
impl ContentOf<super::opcode::Mask> for super::opcode::Circle {}
impl ContentOf<super::opcode::Mask> for super::opcode::Ellipse {}
impl ContentOf<super::opcode::Mask> for super::opcode::Line {}
impl ContentOf<super::opcode::Mask> for super::opcode::Polyline {}
impl ContentOf<super::opcode::Mask> for super::opcode::Polygon {}
impl ContentOf<super::opcode::Mask> for super::opcode::LinearGradient {}
impl ContentOf<super::opcode::Mask> for super::opcode::RadialGradient {}
impl ApplyTo<super::opcode::Text> for super::opcode::TextLayout {}
impl ApplyTo<super::opcode::TextSpan> for super::opcode::TextLayout {}
impl ApplyTo<super::opcode::Text> for super::opcode::Font {}
impl ApplyTo<super::opcode::TextSpan> for super::opcode::Font {}
impl ApplyTo<super::opcode::Group> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Filter> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Mask> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Rect> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Circle> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Line> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Use> for super::opcode::EnableBackground {}
impl ApplyTo<super::opcode::Group> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Filter> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Mask> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Rect> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Circle> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Line> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Use> for super::opcode::WithTransform {}
impl ApplyTo<super::opcode::Group> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Filter> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Mask> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Rect> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Circle> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Line> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Use> for super::opcode::Fill {}
impl ApplyTo<super::opcode::Group> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Filter> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Mask> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Rect> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Circle> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Line> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Use> for super::opcode::Stroke {}
impl ApplyTo<super::opcode::Group> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Filter> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Mask> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Rect> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Circle> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Line> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Use> for super::opcode::WithFilter {}
impl ApplyTo<super::opcode::Group> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Filter> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Mask> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Rect> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Circle> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Line> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Use> for super::opcode::WithClipPath {}
impl ApplyTo<super::opcode::Group> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Filter> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Mask> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Rect> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Circle> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Line> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Use> for super::opcode::WithMask {}
impl ApplyTo<super::opcode::Group> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Filter> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Mask> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Rect> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Circle> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Ellipse> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Line> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Polyline> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Polygon> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::Use> for super::opcode::Opacity {}
impl ApplyTo<super::opcode::LinearGradient> for super::opcode::Id {}
impl ApplyTo<super::opcode::Group> for super::opcode::Id {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::Id {}
impl ApplyTo<super::opcode::Filter> for super::opcode::Id {}
impl ApplyTo<super::opcode::Mask> for super::opcode::Id {}
impl ApplyTo<super::opcode::Text> for super::opcode::Id {}
impl ApplyTo<super::opcode::TextSpan> for super::opcode::Id {}
impl ApplyTo<super::opcode::Canvas> for super::opcode::ViewBox {}
impl ApplyTo<super::opcode::Pattern> for super::opcode::ViewBox {}
impl<P0, P1> Graphics for (P0, P1)
where
    P0: Graphics,
    P1: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder)
    }
}
impl<E, P0, P1> ApplyTo<E> for (P0, P1)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
{
}
impl<E, P0, P1> ContentOf<E> for (P0, P1)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
{
}
impl<P0, P1> MapCollect<f32> for (P0, P1)
where
    Number: From<P0>,
    Number: From<P1>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![Number::from(self.0).0, Number::from(self.1).0]
    }
}
impl<P0, P1> MapCollect<super::opcode::Transform> for (P0, P1)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![self.0.into(), self.1.into()]
    }
}
impl<P0, P1> MapCollect<super::opcode::Angle> for (P0, P1)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![self.0.into(), self.1.into()]
    }
}
impl<P0, P1> MapCollect<super::opcode::Length> for (P0, P1)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![self.0.into(), self.1.into()]
    }
}
impl<P0, P1> MapCollect<super::opcode::PathEvent> for (P0, P1)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![self.0.into(), self.1.into()]
    }
}
impl<P0, P1> MapCollect<super::opcode::FontFamily> for (P0, P1)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![self.0.into(), self.1.into()]
    }
}
impl<P0, P1, P2> Graphics for (P0, P1, P2)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder)
    }
}
impl<E, P0, P1, P2> ApplyTo<E> for (P0, P1, P2)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
{
}
impl<E, P0, P1, P2> ContentOf<E> for (P0, P1, P2)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
{
}
impl<P0, P1, P2> MapCollect<f32> for (P0, P1, P2)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
        ]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::Transform> for (P0, P1, P2)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::Angle> for (P0, P1, P2)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::Length> for (P0, P1, P2)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::PathEvent> for (P0, P1, P2)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::FontFamily> for (P0, P1, P2)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2, P3> Graphics for (P0, P1, P2, P3)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder)
    }
}
impl<E, P0, P1, P2, P3> ApplyTo<E> for (P0, P1, P2, P3)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3> ContentOf<E> for (P0, P1, P2, P3)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
{
}
impl<P0, P1, P2, P3> MapCollect<f32> for (P0, P1, P2, P3)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
        ]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::Transform> for (P0, P1, P2, P3)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::Angle> for (P0, P1, P2, P3)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::Length> for (P0, P1, P2, P3)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::PathEvent> for (P0, P1, P2, P3)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::FontFamily> for (P0, P1, P2, P3)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3, P4> Graphics for (P0, P1, P2, P3, P4)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4> ApplyTo<E> for (P0, P1, P2, P3, P4)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4> ContentOf<E> for (P0, P1, P2, P3, P4)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4> MapCollect<f32> for (P0, P1, P2, P3, P4)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::Transform> for (P0, P1, P2, P3, P4)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::Angle> for (P0, P1, P2, P3, P4)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::Length> for (P0, P1, P2, P3, P4)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::PathEvent> for (P0, P1, P2, P3, P4)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::FontFamily> for (P0, P1, P2, P3, P4)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> Graphics for (P0, P1, P2, P3, P4, P5)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5> ApplyTo<E> for (P0, P1, P2, P3, P4, P5)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5> ContentOf<E> for (P0, P1, P2, P3, P4, P5)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<f32> for (P0, P1, P2, P3, P4, P5)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::Transform> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::Angle> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::Length> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::PathEvent> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::FontFamily> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> Graphics for (P0, P1, P2, P3, P4, P5, P6)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6> ApplyTo<E> for (P0, P1, P2, P3, P4, P5, P6)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6> ContentOf<E> for (P0, P1, P2, P3, P4, P5, P6)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<f32> for (P0, P1, P2, P3, P4, P5, P6)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::Angle> for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::Length> for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> Graphics for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7> ApplyTo<E> for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7> ContentOf<E> for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<f32> for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> Graphics for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8> ApplyTo<E> for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8> ContentOf<E> for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<f32> for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> Graphics for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> ApplyTo<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> ContentOf<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<f32>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> Graphics
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
    P10: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder);
        self.10.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> ApplyTo<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
    P10: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> ContentOf<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
    P10: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<f32>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
            Number::from(self.10).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
    super::opcode::Transform: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
    super::opcode::Angle: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
    super::opcode::Length: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
    super::opcode::PathEvent: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
    super::opcode::FontFamily: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> Graphics
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
    P10: Graphics,
    P11: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder);
        self.10.build(builder);
        self.11.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ApplyTo<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
    P10: ApplyTo<E>,
    P11: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ContentOf<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
    P10: ContentOf<E>,
    P11: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<f32>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
            Number::from(self.10).0,
            Number::from(self.11).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
    super::opcode::Transform: From<P10>,
    super::opcode::Transform: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
    super::opcode::Angle: From<P10>,
    super::opcode::Angle: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
    super::opcode::Length: From<P10>,
    super::opcode::Length: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
    super::opcode::PathEvent: From<P10>,
    super::opcode::PathEvent: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
    super::opcode::FontFamily: From<P10>,
    super::opcode::FontFamily: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> Graphics
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
    P10: Graphics,
    P11: Graphics,
    P12: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder);
        self.10.build(builder);
        self.11.build(builder);
        self.12.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> ApplyTo<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
    P10: ApplyTo<E>,
    P11: ApplyTo<E>,
    P12: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> ContentOf<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
    P10: ContentOf<E>,
    P11: ContentOf<E>,
    P12: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<f32>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
            Number::from(self.10).0,
            Number::from(self.11).0,
            Number::from(self.12).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
    super::opcode::Transform: From<P10>,
    super::opcode::Transform: From<P11>,
    super::opcode::Transform: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
    super::opcode::Angle: From<P10>,
    super::opcode::Angle: From<P11>,
    super::opcode::Angle: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
    super::opcode::Length: From<P10>,
    super::opcode::Length: From<P11>,
    super::opcode::Length: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
    super::opcode::PathEvent: From<P10>,
    super::opcode::PathEvent: From<P11>,
    super::opcode::PathEvent: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
    super::opcode::FontFamily: From<P10>,
    super::opcode::FontFamily: From<P11>,
    super::opcode::FontFamily: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> Graphics
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
    P10: Graphics,
    P11: Graphics,
    P12: Graphics,
    P13: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder);
        self.10.build(builder);
        self.11.build(builder);
        self.12.build(builder);
        self.13.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> ApplyTo<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
    P10: ApplyTo<E>,
    P11: ApplyTo<E>,
    P12: ApplyTo<E>,
    P13: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> ContentOf<E>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
    P10: ContentOf<E>,
    P11: ContentOf<E>,
    P12: ContentOf<E>,
    P13: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> MapCollect<f32>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
            Number::from(self.10).0,
            Number::from(self.11).0,
            Number::from(self.12).0,
            Number::from(self.13).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13>
    MapCollect<super::opcode::Transform>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
    super::opcode::Transform: From<P10>,
    super::opcode::Transform: From<P11>,
    super::opcode::Transform: From<P12>,
    super::opcode::Transform: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> MapCollect<super::opcode::Angle>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
    super::opcode::Angle: From<P10>,
    super::opcode::Angle: From<P11>,
    super::opcode::Angle: From<P12>,
    super::opcode::Angle: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> MapCollect<super::opcode::Length>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
    super::opcode::Length: From<P10>,
    super::opcode::Length: From<P11>,
    super::opcode::Length: From<P12>,
    super::opcode::Length: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13>
    MapCollect<super::opcode::PathEvent>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
    super::opcode::PathEvent: From<P10>,
    super::opcode::PathEvent: From<P11>,
    super::opcode::PathEvent: From<P12>,
    super::opcode::PathEvent: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13>
    MapCollect<super::opcode::FontFamily>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
    super::opcode::FontFamily: From<P10>,
    super::opcode::FontFamily: From<P11>,
    super::opcode::FontFamily: From<P12>,
    super::opcode::FontFamily: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14> Graphics
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    P0: Graphics,
    P1: Graphics,
    P2: Graphics,
    P3: Graphics,
    P4: Graphics,
    P5: Graphics,
    P6: Graphics,
    P7: Graphics,
    P8: Graphics,
    P9: Graphics,
    P10: Graphics,
    P11: Graphics,
    P12: Graphics,
    P13: Graphics,
    P14: Graphics,
{
    fn build(self, builder: &mut BuildContext) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
        self.7.build(builder);
        self.8.build(builder);
        self.9.build(builder);
        self.10.build(builder);
        self.11.build(builder);
        self.12.build(builder);
        self.13.build(builder);
        self.14.build(builder)
    }
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14> ApplyTo<E>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    P0: ApplyTo<E>,
    P1: ApplyTo<E>,
    P2: ApplyTo<E>,
    P3: ApplyTo<E>,
    P4: ApplyTo<E>,
    P5: ApplyTo<E>,
    P6: ApplyTo<E>,
    P7: ApplyTo<E>,
    P8: ApplyTo<E>,
    P9: ApplyTo<E>,
    P10: ApplyTo<E>,
    P11: ApplyTo<E>,
    P12: ApplyTo<E>,
    P13: ApplyTo<E>,
    P14: ApplyTo<E>,
{
}
impl<E, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14> ContentOf<E>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    P0: ContentOf<E>,
    P1: ContentOf<E>,
    P2: ContentOf<E>,
    P3: ContentOf<E>,
    P4: ContentOf<E>,
    P5: ContentOf<E>,
    P6: ContentOf<E>,
    P7: ContentOf<E>,
    P8: ContentOf<E>,
    P9: ContentOf<E>,
    P10: ContentOf<E>,
    P11: ContentOf<E>,
    P12: ContentOf<E>,
    P13: ContentOf<E>,
    P14: ContentOf<E>,
{
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14> MapCollect<f32>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![
            Number::from(self.0).0,
            Number::from(self.1).0,
            Number::from(self.2).0,
            Number::from(self.3).0,
            Number::from(self.4).0,
            Number::from(self.5).0,
            Number::from(self.6).0,
            Number::from(self.7).0,
            Number::from(self.8).0,
            Number::from(self.9).0,
            Number::from(self.10).0,
            Number::from(self.11).0,
            Number::from(self.12).0,
            Number::from(self.13).0,
            Number::from(self.14).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
    MapCollect<super::opcode::Transform>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    super::opcode::Transform: From<P0>,
    super::opcode::Transform: From<P1>,
    super::opcode::Transform: From<P2>,
    super::opcode::Transform: From<P3>,
    super::opcode::Transform: From<P4>,
    super::opcode::Transform: From<P5>,
    super::opcode::Transform: From<P6>,
    super::opcode::Transform: From<P7>,
    super::opcode::Transform: From<P8>,
    super::opcode::Transform: From<P9>,
    super::opcode::Transform: From<P10>,
    super::opcode::Transform: From<P11>,
    super::opcode::Transform: From<P12>,
    super::opcode::Transform: From<P13>,
    super::opcode::Transform: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
            self.14.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
    MapCollect<super::opcode::Angle>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    super::opcode::Angle: From<P0>,
    super::opcode::Angle: From<P1>,
    super::opcode::Angle: From<P2>,
    super::opcode::Angle: From<P3>,
    super::opcode::Angle: From<P4>,
    super::opcode::Angle: From<P5>,
    super::opcode::Angle: From<P6>,
    super::opcode::Angle: From<P7>,
    super::opcode::Angle: From<P8>,
    super::opcode::Angle: From<P9>,
    super::opcode::Angle: From<P10>,
    super::opcode::Angle: From<P11>,
    super::opcode::Angle: From<P12>,
    super::opcode::Angle: From<P13>,
    super::opcode::Angle: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
            self.14.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
    MapCollect<super::opcode::Length>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
    super::opcode::Length: From<P4>,
    super::opcode::Length: From<P5>,
    super::opcode::Length: From<P6>,
    super::opcode::Length: From<P7>,
    super::opcode::Length: From<P8>,
    super::opcode::Length: From<P9>,
    super::opcode::Length: From<P10>,
    super::opcode::Length: From<P11>,
    super::opcode::Length: From<P12>,
    super::opcode::Length: From<P13>,
    super::opcode::Length: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
            self.14.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
    MapCollect<super::opcode::PathEvent>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    super::opcode::PathEvent: From<P0>,
    super::opcode::PathEvent: From<P1>,
    super::opcode::PathEvent: From<P2>,
    super::opcode::PathEvent: From<P3>,
    super::opcode::PathEvent: From<P4>,
    super::opcode::PathEvent: From<P5>,
    super::opcode::PathEvent: From<P6>,
    super::opcode::PathEvent: From<P7>,
    super::opcode::PathEvent: From<P8>,
    super::opcode::PathEvent: From<P9>,
    super::opcode::PathEvent: From<P10>,
    super::opcode::PathEvent: From<P11>,
    super::opcode::PathEvent: From<P12>,
    super::opcode::PathEvent: From<P13>,
    super::opcode::PathEvent: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
            self.14.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
    MapCollect<super::opcode::FontFamily>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
    )
where
    super::opcode::FontFamily: From<P0>,
    super::opcode::FontFamily: From<P1>,
    super::opcode::FontFamily: From<P2>,
    super::opcode::FontFamily: From<P3>,
    super::opcode::FontFamily: From<P4>,
    super::opcode::FontFamily: From<P5>,
    super::opcode::FontFamily: From<P6>,
    super::opcode::FontFamily: From<P7>,
    super::opcode::FontFamily: From<P8>,
    super::opcode::FontFamily: From<P9>,
    super::opcode::FontFamily: From<P10>,
    super::opcode::FontFamily: From<P11>,
    super::opcode::FontFamily: From<P12>,
    super::opcode::FontFamily: From<P13>,
    super::opcode::FontFamily: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
            self.10.into(),
            self.11.into(),
            self.12.into(),
            self.13.into(),
            self.14.into(),
        ]
    }
}
impl<P> MapCollect<f32> for P
where
    Number: From<P>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![Number::from(self).0]
    }
}
impl<P> MapCollect<super::opcode::Transform> for P
where
    super::opcode::Transform: From<P>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![self.into()]
    }
}
impl<P> MapCollect<super::opcode::Angle> for P
where
    super::opcode::Angle: From<P>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![self.into()]
    }
}
impl<P> MapCollect<super::opcode::Length> for P
where
    super::opcode::Length: From<P>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![self.into()]
    }
}
impl<P> MapCollect<super::opcode::PathEvent> for P
where
    super::opcode::PathEvent: From<P>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![self.into()]
    }
}
impl<P> MapCollect<super::opcode::FontFamily> for P
where
    super::opcode::FontFamily: From<P>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![self.into()]
    }
}
impl<P0, P1> MapCollect<super::opcode::Point> for (P0, P1)
where
    Number: From<P0>,
    Number: From<P1>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![super::opcode::Point(
            Number::from(self.0).0,
            Number::from(self.1).0,
        )]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::Point> for (P0, P1, P2, P3)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::Point> for (P0, P1, P2, P3, P4, P5)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
    MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
    MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
    MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
        ]
    }
}
impl<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
    > MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
    Number: From<P20>,
    Number: From<P21>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
            super::opcode::Point(Number::from(self.20).0, Number::from(self.21).0),
        ]
    }
}
impl<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
    > MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
    Number: From<P20>,
    Number: From<P21>,
    Number: From<P22>,
    Number: From<P23>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
            super::opcode::Point(Number::from(self.20).0, Number::from(self.21).0),
            super::opcode::Point(Number::from(self.22).0, Number::from(self.23).0),
        ]
    }
}
impl<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
    > MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
    Number: From<P20>,
    Number: From<P21>,
    Number: From<P22>,
    Number: From<P23>,
    Number: From<P24>,
    Number: From<P25>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
            super::opcode::Point(Number::from(self.20).0, Number::from(self.21).0),
            super::opcode::Point(Number::from(self.22).0, Number::from(self.23).0),
            super::opcode::Point(Number::from(self.24).0, Number::from(self.25).0),
        ]
    }
}
impl<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
        P26,
        P27,
    > MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
        P26,
        P27,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
    Number: From<P20>,
    Number: From<P21>,
    Number: From<P22>,
    Number: From<P23>,
    Number: From<P24>,
    Number: From<P25>,
    Number: From<P26>,
    Number: From<P27>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
            super::opcode::Point(Number::from(self.20).0, Number::from(self.21).0),
            super::opcode::Point(Number::from(self.22).0, Number::from(self.23).0),
            super::opcode::Point(Number::from(self.24).0, Number::from(self.25).0),
            super::opcode::Point(Number::from(self.26).0, Number::from(self.27).0),
        ]
    }
}
impl<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
        P26,
        P27,
        P28,
        P29,
    > MapCollect<super::opcode::Point>
    for (
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
        P20,
        P21,
        P22,
        P23,
        P24,
        P25,
        P26,
        P27,
        P28,
        P29,
    )
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
    Number: From<P4>,
    Number: From<P5>,
    Number: From<P6>,
    Number: From<P7>,
    Number: From<P8>,
    Number: From<P9>,
    Number: From<P10>,
    Number: From<P11>,
    Number: From<P12>,
    Number: From<P13>,
    Number: From<P14>,
    Number: From<P15>,
    Number: From<P16>,
    Number: From<P17>,
    Number: From<P18>,
    Number: From<P19>,
    Number: From<P20>,
    Number: From<P21>,
    Number: From<P22>,
    Number: From<P23>,
    Number: From<P24>,
    Number: From<P25>,
    Number: From<P26>,
    Number: From<P27>,
    Number: From<P28>,
    Number: From<P29>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            super::opcode::Point(Number::from(self.0).0, Number::from(self.1).0),
            super::opcode::Point(Number::from(self.2).0, Number::from(self.3).0),
            super::opcode::Point(Number::from(self.4).0, Number::from(self.5).0),
            super::opcode::Point(Number::from(self.6).0, Number::from(self.7).0),
            super::opcode::Point(Number::from(self.8).0, Number::from(self.9).0),
            super::opcode::Point(Number::from(self.10).0, Number::from(self.11).0),
            super::opcode::Point(Number::from(self.12).0, Number::from(self.13).0),
            super::opcode::Point(Number::from(self.14).0, Number::from(self.15).0),
            super::opcode::Point(Number::from(self.16).0, Number::from(self.17).0),
            super::opcode::Point(Number::from(self.18).0, Number::from(self.19).0),
            super::opcode::Point(Number::from(self.20).0, Number::from(self.21).0),
            super::opcode::Point(Number::from(self.22).0, Number::from(self.23).0),
            super::opcode::Point(Number::from(self.24).0, Number::from(self.25).0),
            super::opcode::Point(Number::from(self.26).0, Number::from(self.27).0),
            super::opcode::Point(Number::from(self.28).0, Number::from(self.29).0),
        ]
    }
}
