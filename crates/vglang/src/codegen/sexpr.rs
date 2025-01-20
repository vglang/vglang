impl super::opcode::Angle {
    pub fn deg<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Deg(Number::from(p0).0)
    }
    pub fn grad<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Grad(Number::from(p0).0)
    }
    pub fn rad<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Rad(Number::from(p0).0)
    }
}
impl super::opcode::Length {
    pub fn em<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Em(Number::from(p0).0)
    }
    pub fn ex<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Ex(Number::from(p0).0)
    }
    pub fn px<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Px(Number::from(p0).0)
    }
    pub fn inch<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Inch(Number::from(p0).0)
    }
    pub fn cm<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Cm(Number::from(p0).0)
    }
    pub fn mm<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Mm(Number::from(p0).0)
    }
    pub fn pt<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Pt(Number::from(p0).0)
    }
    pub fn pc<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Pc(Number::from(p0).0)
    }
    pub fn percent<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Percent(Number::from(p0).0)
    }
}
impl super::opcode::Color {}
impl<P0, P1, P2> From<(P0, P1, P2)> for super::opcode::Rgb
where
    u8: From<P0>,
    u8: From<P1>,
    u8: From<P2>,
{
    fn from(value: (P0, P1, P2)) -> Self {
        Self(value.0.into(), value.1.into(), value.2.into())
    }
}
impl super::opcode::Iri {
    pub fn local<P0>(p0: P0) -> Self
    where
        String: From<P0>,
    {
        Self::Local(p0.into())
    }
    pub fn path<P0>(p0: P0) -> Self
    where
        String: From<P0>,
    {
        Self::Path(p0.into())
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
impl super::opcode::Paint {
    pub fn color<P0>(p0: P0) -> Self
    where
        super::opcode::Rgb: From<P0>,
    {
        Self::Color(p0.into())
    }
    pub fn server<P0>(p0: P0) -> Self
    where
        super::opcode::FuncIri: From<P0>,
    {
        Self::Server(p0.into())
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
impl super::opcode::Coords {}
impl super::opcode::Transform {
    pub fn translate<P0, P1>(p0: P0, p1: P1) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
    {
        Self::Translate(Number::from(p0).0, Number::from(p1).0)
    }
    pub fn matrix<P0>(p0: P0) -> Self
    where
        [f32; 6usize]: From<P0>,
    {
        Self::Matrix(p0.into())
    }
    pub fn scale<P0, P1>(p0: P0, p1: P1) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
    {
        Self::Scale(Number::from(p0).0, Some(Number::from(p1).0))
    }
    pub fn rotate<P0, P1, P2>(angle: P0, cx: P1, cy: P2) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
    {
        Self::Rotate {
            angle: Number::from(angle).0,
            cx: Number::from(cx).0,
            cy: Number::from(cy).0,
        }
    }
    pub fn skew_x<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::SkewX(Number::from(p0).0)
    }
    pub fn skew_y<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::SkewY(Number::from(p0).0)
    }
}
impl super::opcode::Channel {}
impl super::opcode::ClipRule {}
impl super::opcode::PathEvent {
    pub fn move_to<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::MoveTo(p0.into())
    }
    pub fn move_to_relative<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::MoveToRelative(p0.into())
    }
    pub fn line_to<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::LineTo(p0.into())
    }
    pub fn line_to_relative<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::LineToRelative(p0.into())
    }
    pub fn polyline<P0>(p0: P0) -> Self
    where
        P0: MapCollect<super::opcode::Point>,
    {
        Self::Polyline(p0.map_collect())
    }
    pub fn polyline_relative<P0>(p0: P0) -> Self
    where
        P0: MapCollect<super::opcode::Point>,
    {
        Self::PolylineRelative(p0.map_collect())
    }
    pub fn cubic_bezier<P0, P1, P2>(ctrl1: P0, ctrl2: P1, to_point: P2) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
        super::opcode::Point: From<P2>,
    {
        Self::CubicBezier {
            ctrl1: ctrl1.into(),
            ctrl2: ctrl2.into(),
            to_point: to_point.into(),
        }
    }
    pub fn cubic_bezier_relative<P0, P1, P2>(ctrl1: P0, ctrl2: P1, to_point: P2) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
        super::opcode::Point: From<P2>,
    {
        Self::CubicBezierRelative {
            ctrl1: ctrl1.into(),
            ctrl2: ctrl2.into(),
            to_point: to_point.into(),
        }
    }
    pub fn cubic_bezier_smooth<P0, P1>(ctrl2: P0, to_point: P1) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
    {
        Self::CubicBezierSmooth {
            ctrl2: ctrl2.into(),
            to_point: to_point.into(),
        }
    }
    pub fn cubic_bezier_smooth_relative<P0, P1>(ctrl2: P0, to_point: P1) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
    {
        Self::CubicBezierSmoothRelative {
            ctrl2: ctrl2.into(),
            to_point: to_point.into(),
        }
    }
    pub fn quadratic_bezier<P0, P1>(ctrl: P0, to_point: P1) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
    {
        Self::QuadraticBezier {
            ctrl: ctrl.into(),
            to_point: to_point.into(),
        }
    }
    pub fn quadratic_bezier_relative<P0, P1>(ctrl: P0, to_point: P1) -> Self
    where
        super::opcode::Point: From<P0>,
        super::opcode::Point: From<P1>,
    {
        Self::QuadraticBezierRelative {
            ctrl: ctrl.into(),
            to_point: to_point.into(),
        }
    }
    pub fn quadratic_bezier_smooth<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::QuadraticBezierSmooth(p0.into())
    }
    pub fn quadratic_bezier_smooth_relative<P0>(p0: P0) -> Self
    where
        super::opcode::Point: From<P0>,
    {
        Self::QuadraticBezierSmoothRelative(p0.into())
    }
    pub fn arc<P0, P1, P2, P3, P4, P5>(
        rx: P0,
        ry: P1,
        x_rotation: P2,
        large_arc: P3,
        sweep: P4,
        to_point: P5,
    ) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
        bool: From<P3>,
        bool: From<P4>,
        super::opcode::Point: From<P5>,
    {
        Self::Arc {
            rx: Number::from(rx).0,
            ry: Number::from(ry).0,
            x_rotation: Number::from(x_rotation).0,
            large_arc: large_arc.into(),
            sweep: sweep.into(),
            to_point: to_point.into(),
        }
    }
    pub fn arc_relative<P0, P1, P2, P3, P4, P5>(
        rx: P0,
        ry: P1,
        x_rotation: P2,
        large_arc: P3,
        sweep: P4,
        to_point: P5,
    ) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
        bool: From<P3>,
        bool: From<P4>,
        super::opcode::Point: From<P5>,
    {
        Self::ArcRelative {
            rx: Number::from(rx).0,
            ry: Number::from(ry).0,
            x_rotation: Number::from(x_rotation).0,
            large_arc: large_arc.into(),
            sweep: sweep.into(),
            to_point: to_point.into(),
        }
    }
}
impl super::opcode::FillRule {}
impl super::opcode::StrokeLineCap {}
impl super::opcode::StrokeLineJoin {
    pub fn miter<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Miter(Number::from(p0).0)
    }
}
impl super::opcode::SpreadMethod {}
impl super::opcode::FontStyle {}
impl super::opcode::FontVariant {}
impl super::opcode::FontWeight {}
impl super::opcode::FontFamily {
    pub fn generic<P0>(p0: P0) -> Self
    where
        String: From<P0>,
    {
        Self::Generic(p0.into())
    }
}
impl super::opcode::FontStretch {}
impl super::opcode::Background {
    pub fn new<P0, P1, P2, P3>(x: P0, y: P1, width: P2, height: P3) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
        Number: From<P3>,
    {
        Self::New {
            x: Some(Number::from(x).0),
            y: Some(Number::from(y).0),
            width: Some(Number::from(width).0),
            height: Some(Number::from(height).0),
        }
    }
}
impl super::opcode::FeIn {
    pub fn result<P0>(p0: P0) -> Self
    where
        String: From<P0>,
    {
        Self::Result(p0.into())
    }
}
impl super::opcode::FeOut {
    pub fn named<P0>(p0: P0) -> Self
    where
        String: From<P0>,
    {
        Self::Named(p0.into())
    }
}
impl super::opcode::FeBlendMode {}
impl super::opcode::TextLengthAdjust {}
impl super::opcode::WritingMode {}
impl super::opcode::TextDirection {}
impl super::opcode::UnicodeBidi {}
impl super::opcode::TextAnchor {}
impl super::opcode::DominantBaseline {}
impl super::opcode::AlignmentBaseline {}
impl super::opcode::BaselineShift {
    pub fn value<P0>(p0: P0) -> Self
    where
        super::opcode::Length: From<P0>,
    {
        Self::Value(p0.into())
    }
}
impl super::opcode::TextDecoration {}
impl super::opcode::TextPathMethod {}
impl super::opcode::TextPathSpacing {}
impl super::opcode::LetterSpacing {
    pub fn length<P0>(p0: P0) -> Self
    where
        super::opcode::Length: From<P0>,
    {
        Self::Length(p0.into())
    }
}
impl super::opcode::WordSpacing {
    pub fn length<P0>(p0: P0) -> Self
    where
        super::opcode::Length: From<P0>,
    {
        Self::Length(p0.into())
    }
}
impl super::opcode::MeetOrSlice {}
impl super::opcode::PreserveAspectRatio {
    pub fn x_min_y_min<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMinYMin(p0.into())
    }
    pub fn x_mid_y_min<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMidYMin(p0.into())
    }
    pub fn x_max_y_min<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMaxYMin(p0.into())
    }
    pub fn x_min_y_mid<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMinYMid(p0.into())
    }
    pub fn x_mid_y_mid<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMidYMid(p0.into())
    }
    pub fn x_max_y_mid<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMaxYMid(p0.into())
    }
    pub fn x_min_y_max<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMinYMax(p0.into())
    }
    pub fn x_mid_y_max<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMidYMax(p0.into())
    }
    pub fn x_max_y_max<P0>(p0: P0) -> Self
    where
        super::opcode::MeetOrSlice: From<P0>,
    {
        Self::XMaxYMax(p0.into())
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
    pub fn new() -> Self where {
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
    pub fn from_write_mode<T>(value: T) -> Self
    where
        super::opcode::WritingMode: From<T>,
    {
        Self {
            write_mode: Some(value.into()),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_direction<T>(value: T) -> Self
    where
        super::opcode::TextDirection: From<T>,
    {
        Self {
            direction: Some(value.into()),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_unicode_bidi<T>(value: T) -> Self
    where
        super::opcode::UnicodeBidi: From<T>,
    {
        Self {
            unicode_bidi: Some(value.into()),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_anchor<T>(value: T) -> Self
    where
        super::opcode::TextAnchor: From<T>,
    {
        Self {
            anchor: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_dominant_baseline<T>(value: T) -> Self
    where
        super::opcode::DominantBaseline: From<T>,
    {
        Self {
            dominant_baseline: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_alignment_baseline<T>(value: T) -> Self
    where
        super::opcode::AlignmentBaseline: From<T>,
    {
        Self {
            alignment_baseline: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_baseline_shift<T>(value: T) -> Self
    where
        super::opcode::BaselineShift: From<T>,
    {
        Self {
            baseline_shift: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_decoration<T>(value: T) -> Self
    where
        super::opcode::TextDecoration: From<T>,
    {
        Self {
            decoration: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_letter_spacing<T>(value: T) -> Self
    where
        super::opcode::LetterSpacing: From<T>,
    {
        Self {
            letter_spacing: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::TextLayout {
    pub fn from_word_spacing<T>(value: T) -> Self
    where
        super::opcode::WordSpacing: From<T>,
    {
        Self {
            word_spacing: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::WithTransform
where
    P0: MapCollect<super::opcode::Transform>,
{
    fn from(value: P0) -> Self {
        Self(value.map_collect())
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
impl Default for super::opcode::Fill {
    fn default() -> Self {
        Self {
            paint: None,
            rule: None,
            opacity: None,
        }
    }
}
impl super::opcode::Fill {
    pub fn new() -> Self where {
        Self {
            paint: None,
            rule: None,
            opacity: None,
        }
    }
}
impl super::opcode::Fill {
    pub fn from_paint<T>(value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        Self {
            paint: Some(value.into()),
            ..Default::default()
        }
    }
}
impl super::opcode::Fill {
    pub fn from_rule<T>(value: T) -> Self
    where
        super::opcode::FillRule: From<T>,
    {
        Self {
            rule: Some(value.into()),
            ..Default::default()
        }
    }
}
impl super::opcode::Fill {
    pub fn from_opacity<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            opacity: Some(Number::from(value).0),
            ..Default::default()
        }
    }
}
impl super::opcode::Fill {
    pub fn paint<T>(mut self, value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        self.paint = Some(value.into());
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
impl Default for super::opcode::Stroke {
    fn default() -> Self {
        Self {
            paint: None,
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
    pub fn new() -> Self where {
        Self {
            paint: None,
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
    pub fn from_paint<T>(value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        Self {
            paint: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_linecap<T>(value: T) -> Self
    where
        super::opcode::StrokeLineCap: From<T>,
    {
        Self {
            linecap: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_linejoin<T>(value: T) -> Self
    where
        super::opcode::StrokeLineJoin: From<T>,
    {
        Self {
            linejoin: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_dasharray<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            dasharray: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_dashoffset<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            dashoffset: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn from_opacity<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            opacity: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Stroke {
    pub fn paint<T>(mut self, value: T) -> Self
    where
        super::opcode::Paint: From<T>,
    {
        self.paint = Some(super::opcode::variable::Variable::Constant(value.into()));
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
    pub fn new() -> Self where {
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
    pub fn from_family<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::FontFamily>,
    {
        Self {
            family: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Font {
    pub fn from_style<T>(value: T) -> Self
    where
        super::opcode::FontStyle: From<T>,
    {
        Self {
            style: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Font {
    pub fn from_variant<T>(value: T) -> Self
    where
        super::opcode::FontVariant: From<T>,
    {
        Self {
            variant: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Font {
    pub fn from_weight<T>(value: T) -> Self
    where
        super::opcode::FontWeight: From<T>,
    {
        Self {
            weight: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Font {
    pub fn from_size<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            size: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Font {
    pub fn from_stretch<T>(value: T) -> Self
    where
        super::opcode::FontStretch: From<T>,
    {
        Self {
            stretch: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::EnableBackground
where
    super::opcode::Background: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl<P0> From<P0> for super::opcode::WithFilter
where
    String: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
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
impl<P0> From<P0> for super::opcode::WithMask
where
    super::opcode::FuncIri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
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
impl<P0, P1, P2, P3> From<(P0, P1, P2, P3)> for super::opcode::ViewBox
where
    Number: From<P0>,
    Number: From<P1>,
    Number: From<P2>,
    Number: From<P3>,
{
    fn from(value: (P0, P1, P2, P3)) -> Self {
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
impl<P0, P1> From<(P0, P1)> for super::opcode::Canvas
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
{
    fn from(value: (P0, P1)) -> Self {
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
    pub fn new() -> Self where {
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
    pub fn from_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Mask {
    pub fn from_content_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            content_units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Mask {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Mask {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Mask {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Mask {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl Default for super::opcode::ClipPath {
    fn default() -> Self {
        Self(None)
    }
}
impl super::opcode::ClipPath {
    pub fn new() -> Self where {
        Self(None)
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
    pub fn new<P0, P1, P2, P3>(x: P0, y: P1, width: P2, height: P3) -> Self
    where
        super::opcode::Length: From<P0>,
        super::opcode::Length: From<P1>,
        super::opcode::Length: From<P2>,
        super::opcode::Length: From<P3>,
    {
        Self {
            units: None,
            primitive_units: None,
            x: Some(super::opcode::variable::Variable::Constant(x.into())),
            y: Some(super::opcode::variable::Variable::Constant(y.into())),
            width: Some(super::opcode::variable::Variable::Constant(width.into())),
            height: Some(super::opcode::variable::Variable::Constant(height.into())),
            res: None,
        }
    }
}
impl super::opcode::Filter {
    pub fn from_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_primitive_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            primitive_units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Filter {
    pub fn from_res<T>(value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        Self {
            res: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl Default for super::opcode::FeDistantLight {
    fn default() -> Self {
        Self {
            azimuth: None,
            elevation: None,
        }
    }
}
impl super::opcode::FeDistantLight {
    pub fn new() -> Self where {
        Self {
            azimuth: None,
            elevation: None,
        }
    }
}
impl super::opcode::FeDistantLight {
    pub fn from_azimuth<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            azimuth: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeDistantLight {
    pub fn from_elevation<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            elevation: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
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
    pub fn new() -> Self where {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }
}
impl super::opcode::FePointLight {
    pub fn from_x<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FePointLight {
    pub fn from_y<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FePointLight {
    pub fn from_z<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            z: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_x<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_y<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_z<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            z: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_point_at_x<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            point_at_x: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_point_at_y<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            point_at_y: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_point_at_z<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            point_at_z: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_specular_exponent<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            specular_exponent: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpotLight {
    pub fn from_limiting_cone_angle<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            limiting_cone_angle: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_mode<T>(value: T) -> Self
    where
        super::opcode::FeBlendMode: From<T>,
    {
        Self {
            mode: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_in2<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            in2: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeBlend {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl super::opcode::FeColorMatrixValues {
    pub fn matrix<P0>(p0: P0) -> Self
    where
        [f32; 20usize]: From<P0>,
    {
        Self::Matrix(p0.into())
    }
    pub fn saturate<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::Saturate(Number::from(p0).0)
    }
    pub fn hue_rotate<P0>(p0: P0) -> Self
    where
        Number: From<P0>,
    {
        Self::HueRotate(Number::from(p0).0)
    }
}
impl<P0, P1> From<(P0, P1)> for super::opcode::FeColorMatrix
where
    super::opcode::FeIn: From<P0>,
    super::opcode::FeColorMatrixValues: From<P1>,
{
    fn from(value: (P0, P1)) -> Self {
        Self {
            r#in: super::opcode::variable::Variable::Constant(value.0.into()),
            values: super::opcode::variable::Variable::Constant(value.1.into()),
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
        self.r#in = super::opcode::variable::Variable::Constant(value.into());
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
impl super::opcode::FeFunc {
    pub fn table<P0>(p0: P0) -> Self
    where
        P0: MapCollect<f32>,
    {
        Self::Table(p0.map_collect())
    }
    pub fn discrete<P0>(p0: P0) -> Self
    where
        P0: MapCollect<f32>,
    {
        Self::Discrete(p0.map_collect())
    }
    pub fn linear<P0, P1>(slope: P0, intercept: P1) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
    {
        Self::Linear {
            slope: Number::from(slope).0,
            intercept: Number::from(intercept).0,
        }
    }
    pub fn gamma<P0, P1, P2>(amplitude: P0, exponent: P1, offset: P2) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
    {
        Self::Gamma {
            amplitude: Number::from(amplitude).0,
            exponent: Number::from(exponent).0,
            offset: Number::from(offset).0,
        }
    }
}
impl super::opcode::FeCompositeOperator {
    pub fn arithmetic<P0, P1, P2, P3>(k1: P0, k2: P1, k3: P2, k4: P3) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
        Number: From<P2>,
        Number: From<P3>,
    {
        Self::Arithmetic {
            k1: Number::from(k1).0,
            k2: Number::from(k2).0,
            k3: Number::from(k3).0,
            k4: Number::from(k4).0,
        }
    }
}
impl super::opcode::FeConvolveMatrixEdgeMode {}
impl super::opcode::FeMorphologyOperator {}
impl super::opcode::FeStitchTiles {}
impl super::opcode::FeTurbulenceType {}
impl Default for super::opcode::FeComponentTransfer {
    fn default() -> Self {
        Self(None)
    }
}
impl super::opcode::FeComponentTransfer {
    pub fn new() -> Self where {
        Self(None)
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
impl<P0> From<P0> for super::opcode::FeFuncR
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
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
impl<P0> From<P0> for super::opcode::FeFuncB
where
    super::opcode::FeFunc: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl<P0> From<P0> for super::opcode::FeComposite
where
    super::opcode::FeIn: From<P0>,
{
    fn from(value: P0) -> Self {
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
impl<P0, P1, P2> From<(P0, P1, P2)> for super::opcode::FeConvolveMatrix
where
    P0: MapCollect<f32>,
    super::opcode::FeConvolveMatrixEdgeMode: From<P1>,
    bool: From<P2>,
{
    fn from(value: (P0, P1, P2)) -> Self {
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
impl<P0> From<P0> for super::opcode::FeDiffuseLighting
where
    super::opcode::FeIn: From<P0>,
{
    fn from(value: P0) -> Self {
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
impl<P0> From<P0> for super::opcode::FeDisplacementMap
where
    super::opcode::FeIn: From<P0>,
{
    fn from(value: P0) -> Self {
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
    pub fn new() -> Self where {
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
    pub fn from_color<T>(value: T) -> Self
    where
        super::opcode::Rgb: From<T>,
    {
        Self {
            color: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_opacity<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            opacity: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeFlood {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_std_deviation<T>(value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        Self {
            std_deviation: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMerge {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMerge {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMerge {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMerge {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::FeMergeNode
where
    super::opcode::FeIn: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(value.into()))
    }
}
impl<P0> From<P0> for super::opcode::FeImage
where
    super::opcode::FuncIri: From<P0>,
{
    fn from(value: P0) -> Self {
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
    pub fn new() -> Self where {
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
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_mode<T>(value: T) -> Self
    where
        super::opcode::FeMorphologyOperator: From<T>,
    {
        Self {
            mode: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_radius<T>(value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        Self {
            radius: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeMorphology {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new<P0, P1>(dx: P0, dy: P1) -> Self
    where
        Number: From<P0>,
        Number: From<P1>,
    {
        Self {
            r#in: None,
            dx: Some(super::opcode::variable::Variable::Constant(
                Number::from(dx).0,
            )),
            dy: Some(super::opcode::variable::Variable::Constant(
                Number::from(dy).0,
            )),
            x: None,
            y: None,
            width: None,
            height: None,
            result: None,
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_dx<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            dx: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_dy<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            dy: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeOffset {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_surface_scale<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            surface_scale: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_specular_constant<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            specular_constant: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_specular_exponent<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            specular_exponent: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_kernel_unit_len<T>(value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        Self {
            kernel_unit_len: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeSpecularLighting {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_in<T>(value: T) -> Self
    where
        super::opcode::FeIn: From<T>,
    {
        Self {
            r#in: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTile {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTile {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTile {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTile {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTile {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_base_frequency<T>(value: T) -> Self
    where
        super::opcode::NumberOptNumber: From<T>,
    {
        Self {
            base_frequency: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_num_octaves<T>(value: T) -> Self
    where
        i32: From<T>,
    {
        Self {
            num_octaves: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_seed<T>(value: T) -> Self
    where
        Number: From<T>,
    {
        Self {
            seed: Some(super::opcode::variable::Variable::Constant(
                Number::from(value).0,
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_stitch_tiles<T>(value: T) -> Self
    where
        super::opcode::FeStitchTiles: From<T>,
    {
        Self {
            stitch_tiles: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_type<T>(value: T) -> Self
    where
        super::opcode::FeTurbulenceType: From<T>,
    {
        Self {
            r#type: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::FeTurbulence {
    pub fn from_result<T>(value: T) -> Self
    where
        String: From<T>,
    {
        Self {
            result: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new<P0, P1, P2, P3>(x1: P0, y1: P1, x2: P2, y2: P3) -> Self
    where
        super::opcode::Length: From<P0>,
        super::opcode::Length: From<P1>,
        super::opcode::Length: From<P2>,
        super::opcode::Length: From<P3>,
    {
        Self {
            units: None,
            transform: None,
            x1: Some(super::opcode::variable::Variable::Constant(x1.into())),
            y1: Some(super::opcode::variable::Variable::Constant(y1.into())),
            x2: Some(super::opcode::variable::Variable::Constant(x2.into())),
            y2: Some(super::opcode::variable::Variable::Constant(y2.into())),
            spread: None,
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_transform<T>(value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        Self {
            transform: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_x1<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x1: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_y1<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y1: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_x2<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x2: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_y2<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y2: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::LinearGradient {
    pub fn from_spread<T>(value: T) -> Self
    where
        super::opcode::SpreadMethod: From<T>,
    {
        Self {
            spread: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new() -> Self where {
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
    pub fn from_unit<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            unit: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_transform<T>(value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        Self {
            transform: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_cx<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            cx: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_cy<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            cy: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_r<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            r: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_fx<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            fx: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_fy<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            fy: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::RadialGradient {
    pub fn from_spread<T>(value: T) -> Self
    where
        super::opcode::SpreadMethod: From<T>,
    {
        Self {
            spread: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::GradientStop
where
    Number: From<P0>,
{
    fn from(value: P0) -> Self {
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
impl Default for super::opcode::Group {
    fn default() -> Self {
        Self
    }
}
impl super::opcode::Group {
    pub fn new() -> Self where {
        Self
    }
}
impl<P0, P1> From<(P0, P1)> for super::opcode::Path
where
    P0: MapCollect<super::opcode::PathEvent>,
    super::opcode::Length: From<P1>,
{
    fn from(value: (P0, P1)) -> Self {
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
    pub fn new() -> Self where {
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
    pub fn from_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_content_units<T>(value: T) -> Self
    where
        super::opcode::Coords: From<T>,
    {
        Self {
            content_units: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_transform<T>(value: T) -> Self
    where
        super::opcode::Transform: From<T>,
    {
        Self {
            transform: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_x<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_y<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_width<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            width: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
        }
    }
}
impl super::opcode::Pattern {
    pub fn from_height<T>(value: T) -> Self
    where
        super::opcode::Length: From<T>,
    {
        Self {
            height: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::Use
where
    super::opcode::Iri: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(super::opcode::variable::Variable::Constant(value.into()))
    }
}
impl<P0, P1, P2, P3> From<(P0, P1, P2, P3)> for super::opcode::Rect
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
{
    fn from(value: (P0, P1, P2, P3)) -> Self {
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
impl<P0, P1, P2> From<(P0, P1, P2)> for super::opcode::Circle
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
{
    fn from(value: (P0, P1, P2)) -> Self {
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
impl<P0, P1> From<(P0, P1)> for super::opcode::Ellipse
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
{
    fn from(value: (P0, P1)) -> Self {
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
impl<P0, P1, P2, P3> From<(P0, P1, P2, P3)> for super::opcode::Line
where
    super::opcode::Length: From<P0>,
    super::opcode::Length: From<P1>,
    super::opcode::Length: From<P2>,
    super::opcode::Length: From<P3>,
{
    fn from(value: (P0, P1, P2, P3)) -> Self {
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
    pub fn new<P0, P1>(x: P0, y: P1) -> Self
    where
        P0: MapCollect<super::opcode::Length>,
        P1: MapCollect<super::opcode::Length>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(x.map_collect())),
            y: Some(super::opcode::variable::Variable::Constant(y.map_collect())),
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl super::opcode::Text {
    pub fn from_x<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_y<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_dx<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            dx: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_dy<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            dy: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_rotate<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Angle>,
    {
        Self {
            rotate: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_text_length<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            text_length: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::Text {
    pub fn from_length_adjust<T>(value: T) -> Self
    where
        super::opcode::TextLengthAdjust: From<T>,
    {
        Self {
            length_adjust: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
    pub fn new<P0, P1>(x: P0, y: P1) -> Self
    where
        P0: MapCollect<super::opcode::Length>,
        P1: MapCollect<super::opcode::Length>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(x.map_collect())),
            y: Some(super::opcode::variable::Variable::Constant(y.map_collect())),
            dx: None,
            dy: None,
            rotate: None,
            text_length: None,
            length_adjust: None,
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_x<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            x: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_y<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            y: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_dx<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            dx: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_dy<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            dy: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_rotate<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Angle>,
    {
        Self {
            rotate: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_text_length<T>(value: T) -> Self
    where
        T: MapCollect<super::opcode::Length>,
    {
        Self {
            text_length: Some(super::opcode::variable::Variable::Constant(
                value.map_collect(),
            )),
            ..Default::default()
        }
    }
}
impl super::opcode::TextSpan {
    pub fn from_length_adjust<T>(value: T) -> Self
    where
        super::opcode::TextLengthAdjust: From<T>,
    {
        Self {
            length_adjust: Some(super::opcode::variable::Variable::Constant(value.into())),
            ..Default::default()
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
impl<P0> From<P0> for super::opcode::Characters
where
    String: From<P0>,
{
    fn from(value: P0) -> Self {
        Self(value.into())
    }
}
impl<P0> From<P0> for super::opcode::TextPath
where
    super::opcode::Iri: From<P0>,
{
    fn from(value: P0) -> Self {
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
impl Graphics for super::opcode::FeDistantLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FePointLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeSpotLight {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeBlend {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeColorMatrix {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeFuncA {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeFuncR {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeFuncG {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeFuncB {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeComposite {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeConvolveMatrix {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeDisplacementMap {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeFlood {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeGaussianBlur {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeMergeNode {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeImage {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeMorphology {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeOffset {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeTile {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::FeTurbulence {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::GradientStop {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Path {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Use {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Rect {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Circle {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Ellipse {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Line {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Polyline {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Polygon {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::Characters {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Leaf::from(self));
    }
}
impl Graphics for super::opcode::TextLayout {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::WithTransform {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::Id {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::Fill {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::Stroke {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::Font {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::EnableBackground {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::WithFilter {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::WithClipPath {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::WithMask {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::Opacity {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
    }
}
impl Graphics for super::opcode::ViewBox {
    fn build(self, builder: &mut BuildContext) {
        builder.push(super::opcode::Attr::from(self));
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
impl super::opcode::Mask {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::Filter {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::FeDiffuseLighting {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::FeSpecularLighting {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::RadialGradient {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::Pattern {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::TextSpan {
    pub fn apply<A>(self, attrs: A) -> ApplyElement<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyElement { attrs, node: self }
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
impl super::opcode::FeDistantLight {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FePointLight {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeSpotLight {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeBlend {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeColorMatrix {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeFuncA {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeFuncR {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeFuncG {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeFuncB {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeComposite {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeConvolveMatrix {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeDisplacementMap {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeFlood {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeGaussianBlur {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeMergeNode {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeImage {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeMorphology {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeOffset {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeTile {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::FeTurbulence {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::GradientStop {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Path {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Use {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Rect {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Circle {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Ellipse {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Line {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Polyline {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Polygon {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
    }
}
impl super::opcode::Characters {
    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A, Self>
    where
        A: ApplyTo<Self>,
    {
        ApplyLeaf { attrs, node: self }
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
impl<P0> MapCollect<super::opcode::FontFamily> for P0
where
    super::opcode::FontFamily: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::FontFamily> {
        vec![self.into()]
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
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
        P15,
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
    super::opcode::FontFamily: From<P15>,
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
        P15,
        P16,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
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
            self.15.into(),
            self.16.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
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
        P15,
        P16,
        P17,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
        P15,
        P16,
        P17,
        P18,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
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
        P15,
        P16,
        P17,
        P18,
        P19,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    > MapCollect<super::opcode::FontFamily>
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
    super::opcode::FontFamily: From<P34>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
    super::opcode::FontFamily: From<P34>,
    super::opcode::FontFamily: From<P35>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
    super::opcode::FontFamily: From<P34>,
    super::opcode::FontFamily: From<P35>,
    super::opcode::FontFamily: From<P36>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
    super::opcode::FontFamily: From<P34>,
    super::opcode::FontFamily: From<P35>,
    super::opcode::FontFamily: From<P36>,
    super::opcode::FontFamily: From<P37>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<super::opcode::FontFamily>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    super::opcode::FontFamily: From<P15>,
    super::opcode::FontFamily: From<P16>,
    super::opcode::FontFamily: From<P17>,
    super::opcode::FontFamily: From<P18>,
    super::opcode::FontFamily: From<P19>,
    super::opcode::FontFamily: From<P20>,
    super::opcode::FontFamily: From<P21>,
    super::opcode::FontFamily: From<P22>,
    super::opcode::FontFamily: From<P23>,
    super::opcode::FontFamily: From<P24>,
    super::opcode::FontFamily: From<P25>,
    super::opcode::FontFamily: From<P26>,
    super::opcode::FontFamily: From<P27>,
    super::opcode::FontFamily: From<P28>,
    super::opcode::FontFamily: From<P29>,
    super::opcode::FontFamily: From<P30>,
    super::opcode::FontFamily: From<P31>,
    super::opcode::FontFamily: From<P32>,
    super::opcode::FontFamily: From<P33>,
    super::opcode::FontFamily: From<P34>,
    super::opcode::FontFamily: From<P35>,
    super::opcode::FontFamily: From<P36>,
    super::opcode::FontFamily: From<P37>,
    super::opcode::FontFamily: From<P38>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<P0> MapCollect<super::opcode::Point> for P0
where
    super::opcode::Point: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![self.into()]
    }
}
impl<P0, P1, P2> MapCollect<super::opcode::Point> for (P0, P1, P2)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}
impl<P0, P1, P2, P3> MapCollect<super::opcode::Point> for (P0, P1, P2, P3)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
impl<P0, P1, P2, P3, P4> MapCollect<super::opcode::Point> for (P0, P1, P2, P3, P4)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5> MapCollect<super::opcode::Point> for (P0, P1, P2, P3, P4, P5)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6> MapCollect<super::opcode::Point> for (P0, P1, P2, P3, P4, P5, P6)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> MapCollect<super::opcode::Point>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14>
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
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
        P30,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
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
        P30,
        P31,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
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
        P30,
        P31,
        P32,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
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
        P30,
        P31,
        P32,
        P33,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
        P30,
        P31,
        P32,
        P33,
        P34,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
    super::opcode::Point: From<P34>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
    super::opcode::Point: From<P34>,
    super::opcode::Point: From<P35>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
    super::opcode::Point: From<P34>,
    super::opcode::Point: From<P35>,
    super::opcode::Point: From<P36>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
    super::opcode::Point: From<P34>,
    super::opcode::Point: From<P35>,
    super::opcode::Point: From<P36>,
    super::opcode::Point: From<P37>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    )
where
    super::opcode::Point: From<P0>,
    super::opcode::Point: From<P1>,
    super::opcode::Point: From<P2>,
    super::opcode::Point: From<P3>,
    super::opcode::Point: From<P4>,
    super::opcode::Point: From<P5>,
    super::opcode::Point: From<P6>,
    super::opcode::Point: From<P7>,
    super::opcode::Point: From<P8>,
    super::opcode::Point: From<P9>,
    super::opcode::Point: From<P10>,
    super::opcode::Point: From<P11>,
    super::opcode::Point: From<P12>,
    super::opcode::Point: From<P13>,
    super::opcode::Point: From<P14>,
    super::opcode::Point: From<P15>,
    super::opcode::Point: From<P16>,
    super::opcode::Point: From<P17>,
    super::opcode::Point: From<P18>,
    super::opcode::Point: From<P19>,
    super::opcode::Point: From<P20>,
    super::opcode::Point: From<P21>,
    super::opcode::Point: From<P22>,
    super::opcode::Point: From<P23>,
    super::opcode::Point: From<P24>,
    super::opcode::Point: From<P25>,
    super::opcode::Point: From<P26>,
    super::opcode::Point: From<P27>,
    super::opcode::Point: From<P28>,
    super::opcode::Point: From<P29>,
    super::opcode::Point: From<P30>,
    super::opcode::Point: From<P31>,
    super::opcode::Point: From<P32>,
    super::opcode::Point: From<P33>,
    super::opcode::Point: From<P34>,
    super::opcode::Point: From<P35>,
    super::opcode::Point: From<P36>,
    super::opcode::Point: From<P37>,
    super::opcode::Point: From<P38>,
{
    fn map_collect(self) -> Vec<super::opcode::Point> {
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<P0> MapCollect<super::opcode::Transform> for P0
where
    super::opcode::Transform: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::Transform> {
        vec![self.into()]
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
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
        P15,
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
    super::opcode::Transform: From<P15>,
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
        P15,
        P16,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
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
            self.15.into(),
            self.16.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
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
        P15,
        P16,
        P17,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
        P15,
        P16,
        P17,
        P18,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
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
        P15,
        P16,
        P17,
        P18,
        P19,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    > MapCollect<super::opcode::Transform>
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
    > MapCollect<super::opcode::Transform>
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
        P30,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
    super::opcode::Transform: From<P34>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
    super::opcode::Transform: From<P34>,
    super::opcode::Transform: From<P35>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
    super::opcode::Transform: From<P34>,
    super::opcode::Transform: From<P35>,
    super::opcode::Transform: From<P36>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
    super::opcode::Transform: From<P34>,
    super::opcode::Transform: From<P35>,
    super::opcode::Transform: From<P36>,
    super::opcode::Transform: From<P37>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<super::opcode::Transform>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    super::opcode::Transform: From<P15>,
    super::opcode::Transform: From<P16>,
    super::opcode::Transform: From<P17>,
    super::opcode::Transform: From<P18>,
    super::opcode::Transform: From<P19>,
    super::opcode::Transform: From<P20>,
    super::opcode::Transform: From<P21>,
    super::opcode::Transform: From<P22>,
    super::opcode::Transform: From<P23>,
    super::opcode::Transform: From<P24>,
    super::opcode::Transform: From<P25>,
    super::opcode::Transform: From<P26>,
    super::opcode::Transform: From<P27>,
    super::opcode::Transform: From<P28>,
    super::opcode::Transform: From<P29>,
    super::opcode::Transform: From<P30>,
    super::opcode::Transform: From<P31>,
    super::opcode::Transform: From<P32>,
    super::opcode::Transform: From<P33>,
    super::opcode::Transform: From<P34>,
    super::opcode::Transform: From<P35>,
    super::opcode::Transform: From<P36>,
    super::opcode::Transform: From<P37>,
    super::opcode::Transform: From<P38>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<P0> MapCollect<super::opcode::PathEvent> for P0
where
    super::opcode::PathEvent: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::PathEvent> {
        vec![self.into()]
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
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
        P15,
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
    super::opcode::PathEvent: From<P15>,
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
        P15,
        P16,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
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
            self.15.into(),
            self.16.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
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
        P15,
        P16,
        P17,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
        P15,
        P16,
        P17,
        P18,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
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
        P15,
        P16,
        P17,
        P18,
        P19,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    > MapCollect<super::opcode::PathEvent>
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
    super::opcode::PathEvent: From<P34>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
    super::opcode::PathEvent: From<P34>,
    super::opcode::PathEvent: From<P35>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
    super::opcode::PathEvent: From<P34>,
    super::opcode::PathEvent: From<P35>,
    super::opcode::PathEvent: From<P36>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
    super::opcode::PathEvent: From<P34>,
    super::opcode::PathEvent: From<P35>,
    super::opcode::PathEvent: From<P36>,
    super::opcode::PathEvent: From<P37>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<super::opcode::PathEvent>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    super::opcode::PathEvent: From<P15>,
    super::opcode::PathEvent: From<P16>,
    super::opcode::PathEvent: From<P17>,
    super::opcode::PathEvent: From<P18>,
    super::opcode::PathEvent: From<P19>,
    super::opcode::PathEvent: From<P20>,
    super::opcode::PathEvent: From<P21>,
    super::opcode::PathEvent: From<P22>,
    super::opcode::PathEvent: From<P23>,
    super::opcode::PathEvent: From<P24>,
    super::opcode::PathEvent: From<P25>,
    super::opcode::PathEvent: From<P26>,
    super::opcode::PathEvent: From<P27>,
    super::opcode::PathEvent: From<P28>,
    super::opcode::PathEvent: From<P29>,
    super::opcode::PathEvent: From<P30>,
    super::opcode::PathEvent: From<P31>,
    super::opcode::PathEvent: From<P32>,
    super::opcode::PathEvent: From<P33>,
    super::opcode::PathEvent: From<P34>,
    super::opcode::PathEvent: From<P35>,
    super::opcode::PathEvent: From<P36>,
    super::opcode::PathEvent: From<P37>,
    super::opcode::PathEvent: From<P38>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<P0> MapCollect<f32> for P0
where
    Number: From<P0>,
{
    fn map_collect(self) -> Vec<f32> {
        vec![Number::from(self).0]
    }
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15> MapCollect<f32>
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
            Number::from(self.15).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16> MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17> MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
    MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
    MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
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
    > MapCollect<f32>
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
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
        P30,
    > MapCollect<f32>
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
        P30,
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
    Number: From<P30>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
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
        P30,
        P31,
    > MapCollect<f32>
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
        P30,
        P31,
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
    Number: From<P30>,
    Number: From<P31>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
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
        P30,
        P31,
        P32,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
    Number: From<P34>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
            Number::from(self.34).0,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
    Number: From<P34>,
    Number: From<P35>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
            Number::from(self.34).0,
            Number::from(self.35).0,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
    Number: From<P34>,
    Number: From<P35>,
    Number: From<P36>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
            Number::from(self.34).0,
            Number::from(self.35).0,
            Number::from(self.36).0,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
    Number: From<P34>,
    Number: From<P35>,
    Number: From<P36>,
    Number: From<P37>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
            Number::from(self.34).0,
            Number::from(self.35).0,
            Number::from(self.36).0,
            Number::from(self.37).0,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<f32>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    Number: From<P30>,
    Number: From<P31>,
    Number: From<P32>,
    Number: From<P33>,
    Number: From<P34>,
    Number: From<P35>,
    Number: From<P36>,
    Number: From<P37>,
    Number: From<P38>,
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
            Number::from(self.15).0,
            Number::from(self.16).0,
            Number::from(self.17).0,
            Number::from(self.18).0,
            Number::from(self.19).0,
            Number::from(self.20).0,
            Number::from(self.21).0,
            Number::from(self.22).0,
            Number::from(self.23).0,
            Number::from(self.24).0,
            Number::from(self.25).0,
            Number::from(self.26).0,
            Number::from(self.27).0,
            Number::from(self.28).0,
            Number::from(self.29).0,
            Number::from(self.30).0,
            Number::from(self.31).0,
            Number::from(self.32).0,
            Number::from(self.33).0,
            Number::from(self.34).0,
            Number::from(self.35).0,
            Number::from(self.36).0,
            Number::from(self.37).0,
            Number::from(self.38).0,
        ]
    }
}
impl<P0> MapCollect<super::opcode::Angle> for P0
where
    super::opcode::Angle: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::Angle> {
        vec![self.into()]
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
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
        P15,
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
    super::opcode::Angle: From<P15>,
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
        P15,
        P16,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
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
            self.15.into(),
            self.16.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
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
        P15,
        P16,
        P17,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
        P15,
        P16,
        P17,
        P18,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
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
        P15,
        P16,
        P17,
        P18,
        P19,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    > MapCollect<super::opcode::Angle>
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
    > MapCollect<super::opcode::Angle>
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
        P30,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
    super::opcode::Angle: From<P34>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
    super::opcode::Angle: From<P34>,
    super::opcode::Angle: From<P35>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
    super::opcode::Angle: From<P34>,
    super::opcode::Angle: From<P35>,
    super::opcode::Angle: From<P36>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
    super::opcode::Angle: From<P34>,
    super::opcode::Angle: From<P35>,
    super::opcode::Angle: From<P36>,
    super::opcode::Angle: From<P37>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<super::opcode::Angle>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    super::opcode::Angle: From<P15>,
    super::opcode::Angle: From<P16>,
    super::opcode::Angle: From<P17>,
    super::opcode::Angle: From<P18>,
    super::opcode::Angle: From<P19>,
    super::opcode::Angle: From<P20>,
    super::opcode::Angle: From<P21>,
    super::opcode::Angle: From<P22>,
    super::opcode::Angle: From<P23>,
    super::opcode::Angle: From<P24>,
    super::opcode::Angle: From<P25>,
    super::opcode::Angle: From<P26>,
    super::opcode::Angle: From<P27>,
    super::opcode::Angle: From<P28>,
    super::opcode::Angle: From<P29>,
    super::opcode::Angle: From<P30>,
    super::opcode::Angle: From<P31>,
    super::opcode::Angle: From<P32>,
    super::opcode::Angle: From<P33>,
    super::opcode::Angle: From<P34>,
    super::opcode::Angle: From<P35>,
    super::opcode::Angle: From<P36>,
    super::opcode::Angle: From<P37>,
    super::opcode::Angle: From<P38>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<P0> MapCollect<super::opcode::Length> for P0
where
    super::opcode::Length: From<P0>,
{
    fn map_collect(self) -> Vec<super::opcode::Length> {
        vec![self.into()]
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
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
        P15,
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
    super::opcode::Length: From<P15>,
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
            self.15.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
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
        P15,
        P16,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
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
            self.15.into(),
            self.16.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
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
        P15,
        P16,
        P17,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18>
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
        P15,
        P16,
        P17,
        P18,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
        ]
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
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
        P15,
        P16,
        P17,
        P18,
        P19,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
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
    > MapCollect<super::opcode::Length>
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
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
        P30,
    > MapCollect<super::opcode::Length>
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
        P30,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
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
        P30,
        P31,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
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
        P30,
        P31,
        P32,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
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
        P30,
        P31,
        P32,
        P33,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
    super::opcode::Length: From<P34>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
    super::opcode::Length: From<P34>,
    super::opcode::Length: From<P35>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
    super::opcode::Length: From<P34>,
    super::opcode::Length: From<P35>,
    super::opcode::Length: From<P36>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
    super::opcode::Length: From<P34>,
    super::opcode::Length: From<P35>,
    super::opcode::Length: From<P36>,
    super::opcode::Length: From<P37>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > MapCollect<super::opcode::Length>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    super::opcode::Length: From<P15>,
    super::opcode::Length: From<P16>,
    super::opcode::Length: From<P17>,
    super::opcode::Length: From<P18>,
    super::opcode::Length: From<P19>,
    super::opcode::Length: From<P20>,
    super::opcode::Length: From<P21>,
    super::opcode::Length: From<P22>,
    super::opcode::Length: From<P23>,
    super::opcode::Length: From<P24>,
    super::opcode::Length: From<P25>,
    super::opcode::Length: From<P26>,
    super::opcode::Length: From<P27>,
    super::opcode::Length: From<P28>,
    super::opcode::Length: From<P29>,
    super::opcode::Length: From<P30>,
    super::opcode::Length: From<P31>,
    super::opcode::Length: From<P32>,
    super::opcode::Length: From<P33>,
    super::opcode::Length: From<P34>,
    super::opcode::Length: From<P35>,
    super::opcode::Length: From<P36>,
    super::opcode::Length: From<P37>,
    super::opcode::Length: From<P38>,
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
            self.15.into(),
            self.16.into(),
            self.17.into(),
            self.18.into(),
            self.19.into(),
            self.20.into(),
            self.21.into(),
            self.22.into(),
            self.23.into(),
            self.24.into(),
            self.25.into(),
            self.26.into(),
            self.27.into(),
            self.28.into(),
            self.29.into(),
            self.30.into(),
            self.31.into(),
            self.32.into(),
            self.33.into(),
            self.34.into(),
            self.35.into(),
            self.36.into(),
            self.37.into(),
            self.38.into(),
        ]
    }
}
impl<Target, P0, P1> ApplyTo<Target> for (P0, P1)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2> ApplyTo<Target> for (P0, P1, P2)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3> ApplyTo<Target> for (P0, P1, P2, P3)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4> ApplyTo<Target> for (P0, P1, P2, P3, P4)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5> ApplyTo<Target> for (P0, P1, P2, P3, P4, P5)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6> ApplyTo<Target> for (P0, P1, P2, P3, P4, P5, P6)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7> ApplyTo<Target> for (P0, P1, P2, P3, P4, P5, P6, P7)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13> ApplyTo<Target>
    for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13)
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14> ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15> ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16>
    ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
{
}
impl<Target, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
    ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
{
}
impl<
        Target,
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
    > ApplyTo<Target>
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
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
    > ApplyTo<Target>
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
        P30,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
    > ApplyTo<Target>
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
        P30,
        P31,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
        P34,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
    P34: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
    P34: ApplyTo<Target>,
    P35: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
    P34: ApplyTo<Target>,
    P35: ApplyTo<Target>,
    P36: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
    P34: ApplyTo<Target>,
    P35: ApplyTo<Target>,
    P36: ApplyTo<Target>,
    P37: ApplyTo<Target>,
{
}
impl<
        Target,
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > ApplyTo<Target>
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    )
where
    P0: ApplyTo<Target>,
    P1: ApplyTo<Target>,
    P2: ApplyTo<Target>,
    P3: ApplyTo<Target>,
    P4: ApplyTo<Target>,
    P5: ApplyTo<Target>,
    P6: ApplyTo<Target>,
    P7: ApplyTo<Target>,
    P8: ApplyTo<Target>,
    P9: ApplyTo<Target>,
    P10: ApplyTo<Target>,
    P11: ApplyTo<Target>,
    P12: ApplyTo<Target>,
    P13: ApplyTo<Target>,
    P14: ApplyTo<Target>,
    P15: ApplyTo<Target>,
    P16: ApplyTo<Target>,
    P17: ApplyTo<Target>,
    P18: ApplyTo<Target>,
    P19: ApplyTo<Target>,
    P20: ApplyTo<Target>,
    P21: ApplyTo<Target>,
    P22: ApplyTo<Target>,
    P23: ApplyTo<Target>,
    P24: ApplyTo<Target>,
    P25: ApplyTo<Target>,
    P26: ApplyTo<Target>,
    P27: ApplyTo<Target>,
    P28: ApplyTo<Target>,
    P29: ApplyTo<Target>,
    P30: ApplyTo<Target>,
    P31: ApplyTo<Target>,
    P32: ApplyTo<Target>,
    P33: ApplyTo<Target>,
    P34: ApplyTo<Target>,
    P35: ApplyTo<Target>,
    P36: ApplyTo<Target>,
    P37: ApplyTo<Target>,
    P38: ApplyTo<Target>,
{
}
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
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15> Graphics
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
    P15: Graphics,
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
        self.14.build(builder);
        self.15.build(builder)
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16> Graphics
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
    P15: Graphics,
    P16: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder)
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17> Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder)
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18> Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder)
    }
}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19>
    Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder)
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
    > Graphics
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder)
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
        P30,
    > Graphics
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
        P30,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder)
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
        P30,
        P31,
    > Graphics
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
        P30,
        P31,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder)
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
        P30,
        P31,
        P32,
    > Graphics
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
        P30,
        P31,
        P32,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder)
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
        P30,
        P31,
        P32,
        P33,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder)
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
        P30,
        P31,
        P32,
        P33,
        P34,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
        P34,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
    P34: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder);
        self.34.build(builder)
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
    P34: Graphics,
    P35: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder);
        self.34.build(builder);
        self.35.build(builder)
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
    P34: Graphics,
    P35: Graphics,
    P36: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder);
        self.34.build(builder);
        self.35.build(builder);
        self.36.build(builder)
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
    P34: Graphics,
    P35: Graphics,
    P36: Graphics,
    P37: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder);
        self.34.build(builder);
        self.35.build(builder);
        self.36.build(builder);
        self.37.build(builder)
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
    > Graphics
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
        P30,
        P31,
        P32,
        P33,
        P34,
        P35,
        P36,
        P37,
        P38,
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
    P15: Graphics,
    P16: Graphics,
    P17: Graphics,
    P18: Graphics,
    P19: Graphics,
    P20: Graphics,
    P21: Graphics,
    P22: Graphics,
    P23: Graphics,
    P24: Graphics,
    P25: Graphics,
    P26: Graphics,
    P27: Graphics,
    P28: Graphics,
    P29: Graphics,
    P30: Graphics,
    P31: Graphics,
    P32: Graphics,
    P33: Graphics,
    P34: Graphics,
    P35: Graphics,
    P36: Graphics,
    P37: Graphics,
    P38: Graphics,
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
        self.14.build(builder);
        self.15.build(builder);
        self.16.build(builder);
        self.17.build(builder);
        self.18.build(builder);
        self.19.build(builder);
        self.20.build(builder);
        self.21.build(builder);
        self.22.build(builder);
        self.23.build(builder);
        self.24.build(builder);
        self.25.build(builder);
        self.26.build(builder);
        self.27.build(builder);
        self.28.build(builder);
        self.29.build(builder);
        self.30.build(builder);
        self.31.build(builder);
        self.32.build(builder);
        self.33.build(builder);
        self.34.build(builder);
        self.35.build(builder);
        self.36.build(builder);
        self.37.build(builder);
        self.38.build(builder)
    }
}
