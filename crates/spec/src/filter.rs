use vglang::{
    opcode::{
        attrs::{
            EnableBackground, Fill, Font, Id, Opacity, Stroke, ViewBox, WithFilter, WithTransform,
        },
        data::{
            Background, Color, Coords, FeBlendMode, FeColorMatrixValues, FeCompositeOperator, FeIn,
            FontWeight, Rgb,
        },
        el::{
            Canvas, Characters, Circle, FeBlend, FeColorMatrix, FeComposite, FeGaussianBlur,
            FeMerge, FeMergeNode, FeOffset, Filter, GradientStop, Group, LinearGradient, Polygon,
            Rect, Text,
        },
    },
    sexpr::{Graphics, Slength, Stranslate},
};

/// Example from [`enable-background-01`](https://www.w3.org/TR/SVG11/images/filters/enable-background-01.svg)
pub fn enable_background_01() -> impl Graphics {
    Canvas::from((13.5.cm(), 2.7.cm())).children((
        Filter::from((0, 0, 1200, 400))
            .units(Coords::UserSpaceOnUse)
            .apply(Id::from("ShiftBGAndBlur"))
            .children((
                FeOffset::from(FeIn::BackgroundImage).dx(0).dy(125),
                FeGaussianBlur::default().std_deviation(8),
            )),
        Filter::from((0, 0, 1200, 400))
            .units(Coords::UserSpaceOnUse)
            .apply(Id::from("ShiftBGAndBlur_WithSourceGraphic"))
            .children((
                FeOffset::from(FeIn::BackgroundImage).dx(0).dy(125),
                FeGaussianBlur::from("blur").std_deviation(8),
                FeMerge::default().children((
                    FeMergeNode::from("blur"),
                    FeMergeNode::from(FeIn::SourceGraphic),
                )),
            )),
        // The first picture is our reference graphic without filters.
        Group
            .apply(WithTransform::from((0, 0).translate()))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::blue)),
                )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
        // The second adds an empty 'g' element which invokes ShiftBGAndBlur.
        Group
            .apply((
                WithTransform::from((270, 0).translate()),
                EnableBackground::from(Background::new()),
            ))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::blue)),
                )),
                Group.apply(WithFilter::from("ShiftBGAndBlur")),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
        // The third invokes ShiftBGAndBlur on the inner group.
        Group
            .apply((
                EnableBackground::from(Background::new()),
                WithTransform::from((270, 0).translate()),
            ))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group
                    .apply((WithFilter::from("ShiftBGAndBlur"), Opacity::from(0.5)))
                    .children((
                        Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                        Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::blue)),
                    )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
        // The fourth invokes ShiftBGAndBlur on the triangle.
        Group
            .apply((
                EnableBackground::from(Background::new()),
                WithTransform::from((270, 0).translate()),
            ))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75))
                        .apply((Fill::from(Color::blue), WithFilter::from("ShiftBGAndBlur"))),
                )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
        // The fifth invokes ShiftBGAndBlur_WithSourceGraphic on the triangle.
        Group
            .apply((
                EnableBackground::from(Background::new()),
                WithTransform::from((270, 0).translate()),
            ))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75)).apply((
                        Fill::from(Color::blue),
                        WithFilter::from("ShiftBGAndBlur_WithSourceGraphic"),
                    )),
                )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
    ))
}

/// Example from [`feblend_01`](https://www.w3.org/TR/SVG11/images/filters/feBlend.svg)
pub fn feblend_01() -> impl Graphics {
    Canvas::from((5.cm(), 5.cm()))
        .apply(ViewBox::from((0, 0, 500, 500)))
        .children((
            LinearGradient::from((100, 0, 300, 0))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("MyGradient"))
                .children((
                    GradientStop::from(0).color(Rgb::rgb(0x00, 0x00, 0x00)),
                    GradientStop::from(0.33).color(Rgb::rgb(0xff, 0xff, 0xff)),
                    GradientStop::from(0.67).color(Rgb::rgb(0xff, 0x00, 0x00)),
                    GradientStop::from(1).color(Rgb::rgb(0x80, 0x80, 0x80)),
                )),
            Filter::default().apply(Id::from("Normal")).children(
                FeBlend::default()
                    .mode(FeBlendMode::Normal)
                    .in2(FeIn::BackgroundImage),
            ),
            Filter::default().apply(Id::from("Multiply")).children(
                FeBlend::default()
                    .mode(FeBlendMode::Multiply)
                    .in2(FeIn::BackgroundImage),
            ),
            Filter::default().apply(Id::from("Screen")).children(
                FeBlend::default()
                    .mode(FeBlendMode::Screen)
                    .in2(FeIn::BackgroundImage),
            ),
            Filter::default().apply(Id::from("Darken")).children(
                FeBlend::default()
                    .mode(FeBlendMode::Darken)
                    .in2(FeIn::BackgroundImage),
            ),
            Filter::default().apply(Id::from("Lighten")).children(
                FeBlend::default()
                    .mode(FeBlendMode::Lighten)
                    .in2(FeIn::BackgroundImage),
            ),
            Rect::from((1, 1, 498, 498)).apply((Fill::default(), Stroke::from(Color::blue))),
            Group.apply(EnableBackground::new()).children((
                Rect::from((100, 20, 300, 460)).apply(Fill::from("MyGradient")),
                Group
                    .apply((
                        Font::from_family("Verdana").size(75),
                        Fill::from(Rgb::rgb(0x88, 0x88, 0x88)).opacity(0.6),
                    ))
                    .children((
                        Text::from((50, 90))
                            .apply(WithFilter::from("Normal"))
                            .children(Characters::from("Normal")),
                        Text::from((50, 90))
                            .apply(WithFilter::from("Multiply"))
                            .children(Characters::from("Multiply")),
                        Text::from((50, 90))
                            .apply(WithFilter::from("Screen"))
                            .children(Characters::from("Screen")),
                        Text::from((50, 90))
                            .apply(WithFilter::from("Darken"))
                            .children(Characters::from("Darken")),
                        Text::from((50, 90))
                            .apply(WithFilter::from("Lighten"))
                            .children(Characters::from("Lighten")),
                    )),
            )),
        ))
}

/// Example from [`fecolormatrix_01`](https://www.w3.org/TR/SVG11/images/filters/feColorMatrix.svg)
pub fn fecolormatrix_01() -> impl Graphics {
    Canvas::from((8.cm(), 5.cm()))
        .apply(ViewBox::from((0, 0, 800, 500)))
        .children((
            LinearGradient::from((100, 0, 300, 0))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("MyGradient"))
                .children((
                    GradientStop::from(0).color(Rgb::rgb(0xff, 0x00, 0xff)),
                    GradientStop::from(0.33).color(Rgb::rgb(0x88, 0xff, 0x88)),
                    GradientStop::from(0.67).color(Rgb::rgb(0x20, 0x20, 0xff)),
                    GradientStop::from(1).color(Rgb::rgb(0xd0, 0x00, 0x00)),
                )),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Matrix"))
                .children(
                    FeColorMatrix::default().values(FeColorMatrixValues::Matrix([
                        0.33, 0.33, 0.33, 0.0, 0.0, 0.33, 0.33, 0.33, 0.0, 0.0, 0.33, 0.33, 0.33,
                        0.0, 0.0, 0.33, 0.33, 0.33, 0.0, 0.0,
                    ])),
                ),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Saturate40"))
                .children(FeColorMatrix::default().values(FeColorMatrixValues::Saturate(0.4))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("HueRotate90"))
                .children(FeColorMatrix::default().values(FeColorMatrixValues::HueRotate(90.0))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("LuminanceToAlpha"))
                .children((
                    FeColorMatrix::from("a").values(FeColorMatrixValues::LuminanceToAlpha),
                    FeComposite::default()
                        .r#in(FeIn::SourceGraphic)
                        .in2("a")
                        .operator(FeCompositeOperator::In),
                )),
            Rect::from((1, 1, 798, 498)).apply((Fill::default(), Stroke::from(Color::blue))),
            Group
                .apply((
                    Font::from_family("Verdana")
                        .size(75)
                        .weight(FontWeight::Bold),
                    Fill::from("MyGradient"),
                ))
                .children((
                    Rect::from((100, 0, 500, 20)),
                    Text::from((100, 90)).children(Characters::from("Unfiltered")),
                    Text::from((100, 90))
                        .apply(WithFilter::from("Matrix"))
                        .children(Characters::from("Matrix")),
                    Text::from((100, 90))
                        .apply(WithFilter::from("Saturate40"))
                        .children(Characters::from("Saturate40")),
                    Text::from((100, 90))
                        .apply(WithFilter::from("HueRotate90"))
                        .children(Characters::from("HueRotate90")),
                    Text::from((100, 90))
                        .apply(WithFilter::from("LuminanceToAlpha"))
                        .children(Characters::from("Luminance")),
                )),
        ))
}
