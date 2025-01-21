use vglang::{
    opcode::{
        Background, Canvas, Circle, Color, Coords, EnableBackground, FeGaussianBlur, FeIn, FeMerge,
        FeMergeNode, FeOffset, Fill, Filter, Group, Id, Opacity, Paint, Polygon, Rect, Stroke,
        ViewBox, WithFilter, WithTransform,
    },
    sexpr::{Graphics, Slength, Stranslate},
};

/// Example from [`enable-background-01`](https://www.w3.org/TR/SVG11/images/filters/enable-background-01.svg)
pub fn enable_background_01() -> impl Graphics {
    Canvas::from((13.5.cm(), 2.7.cm()))
        .apply(ViewBox::from((0, 0, 1350, 270)))
        .children((
            Filter::new(0, 0, 1200, 400)
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("ShiftBGAndBlur"))
                .children((
                    FeOffset::from_in(FeIn::BackgroundImage).dx(0).dy(125),
                    FeGaussianBlur::from_std_deviation(8),
                )),
            Filter::new(0, 0, 1200, 400)
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("ShiftBGAndBlur_WithSourceGraphic"))
                .children((
                    FeOffset::from_in(FeIn::BackgroundImage).dx(0).dy(125),
                    FeGaussianBlur::from_result("blur").std_deviation(8),
                    FeMerge::default().children((
                        FeMergeNode::from(FeIn::result("blur")),
                        FeMergeNode::from(FeIn::SourceGraphic),
                    )),
                )),
            // The first picture is our reference graphic without filters.
            Group
                .apply(WithTransform::from((0, 0).translate()))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from_paint(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from_paint(Color::Green)),
                        Polygon::from(((160, 25), (160, 125), (240, 75)))
                            .apply(Fill::from_paint(Color::Blue)),
                    )),
                    Rect::from((5, 5, 260, 260)).apply((
                        Fill::from_paint(Paint::None),
                        Stroke::from_paint(Color::Blue),
                    )),
                )),
            // The second adds an empty 'g' element which invokes ShiftBGAndBlur.
            Group
                .apply((
                    WithTransform::from((270, 0).translate()),
                    EnableBackground::from(Background::default()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from_paint(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from_paint(Color::Green)),
                        Polygon::from(((160, 25), (160, 125), (240, 75)))
                            .apply(Fill::from_paint(Color::Blue)),
                    )),
                    Group.apply(WithFilter::from("ShiftBGAndBlur")),
                    Rect::from((5, 5, 260, 260)).apply((
                        Fill::from_paint(Paint::None),
                        Stroke::from_paint(Color::Blue),
                    )),
                )),
            // The third invokes ShiftBGAndBlur on the inner group.
            Group
                .apply((
                    EnableBackground::from(Background::default()),
                    WithTransform::from((270, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from_paint(Color::Red)),
                    Group
                        .apply((WithFilter::from("ShiftBGAndBlur"), Opacity::from(0.5)))
                        .children((
                            Circle::from((125, 75, 45)).apply(Fill::from_paint(Color::Green)),
                            Polygon::from(((160, 25), (160, 125), (240, 75)))
                                .apply(Fill::from_paint(Color::Blue)),
                        )),
                    Rect::from((5, 5, 260, 260)).apply((
                        Fill::from_paint(Paint::None),
                        Stroke::from_paint(Color::Blue),
                    )),
                )),
            // The fourth invokes ShiftBGAndBlur on the triangle.
            Group
                .apply((
                    EnableBackground::from(Background::default()),
                    WithTransform::from((270, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from_paint(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from_paint(Color::Green)),
                        Polygon::from(((160, 25), (160, 125), (240, 75))).apply((
                            Fill::from_paint(Color::Blue),
                            WithFilter::from("ShiftBGAndBlur"),
                        )),
                    )),
                    Rect::from((5, 5, 260, 260)).apply((
                        Fill::from_paint(Paint::None),
                        Stroke::from_paint(Color::Blue),
                    )),
                )),
            // The fifth invokes ShiftBGAndBlur_WithSourceGraphic on the triangle.
            Group
                .apply((
                    EnableBackground::from(Background::default()),
                    WithTransform::from((270, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from_paint(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from_paint(Color::Green)),
                        Polygon::from(((160, 25), (160, 125), (240, 75))).apply((
                            Fill::from_paint(Color::Blue),
                            WithFilter::from("ShiftBGAndBlur_WithSourceGraphic"),
                        )),
                    )),
                    Rect::from((5, 5, 260, 260)).apply((
                        Fill::from_paint(Paint::None),
                        Stroke::from_paint(Color::Blue),
                    )),
                )),
        ))
}
