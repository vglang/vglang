use vglang::{
    opcode::*,
    sexpr::{rgb, Graphics, SfeInResult, SlengthCm, SlengthPercent, StransformTranslate},
};

/// Example from [`enable-background-01`](https://www.w3.org/TR/SVG11/images/filters/enable-background-01.svg)
pub fn enable_background_01() -> impl Graphics {
    Canvas::from((13.5.cm(), 2.7.cm()))
        .apply(ViewBox::from((0, 0, 1350, 270)))
        .children((
            Filter::from((0, 0, 1200, 400))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("ShiftBGAndBlur"))
                .children((
                    FeOffset::from((0, 125)).r#in(FeIn::BackgroundImage),
                    FeGaussianBlur::from(8),
                )),
            Filter::from((0, 0, 1200, 400))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("ShiftBGAndBlur_WithSourceGraphic"))
                .children((
                    FeOffset::default()
                        .r#in(FeIn::BackgroundImage)
                        .dx(0)
                        .dy(125),
                    FeGaussianBlur::from(8).result("blur"),
                    FeMerge::default().children((
                        FeMergeNode::from("blur".result()),
                        FeMergeNode::from(FeIn::SourceGraphic),
                    )),
                )),
            // The first picture is our reference graphic without filters.
            Group
                .apply(WithTransform::from((0, 0).translate()))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from(Color::Green)),
                        Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::Blue)),
                    )),
                    Rect::from((5, 5, 260, 260))
                        .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
                )),
            // The second adds an empty 'g' element which invokes ShiftBGAndBlur.
            Group
                .apply((
                    WithTransform::from((270, 0).translate()),
                    EnableBackground::from(Background::New(None)),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from(Color::Green)),
                        Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::Blue)),
                    )),
                    Group.apply(WithFilter::from("ShiftBGAndBlur")),
                    Rect::from((5, 5, 260, 260))
                        .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
                )),
            // The third invokes ShiftBGAndBlur on the inner group.
            Group
                .apply((
                    EnableBackground::from(Background::New(None)),
                    WithTransform::from((540, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::Red)),
                    Group
                        .apply((WithFilter::from("ShiftBGAndBlur"), Opacity::from(0.5)))
                        .children((
                            Circle::from((125, 75, 45)).apply(Fill::from(Color::Green)),
                            Polygon::from((160, 25, 160, 125, 240, 75))
                                .apply(Fill::from(Color::Blue)),
                        )),
                    Rect::from((5, 5, 260, 260))
                        .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
                )),
            // The fourth invokes ShiftBGAndBlur on the triangle.
            Group
                .apply((
                    EnableBackground::from(Background::New(None)),
                    WithTransform::from((810, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from(Color::Green)),
                        Polygon::from((160, 25, 160, 125, 240, 75))
                            .apply((Fill::from(Color::Blue), WithFilter::from("ShiftBGAndBlur"))),
                    )),
                    Rect::from((5, 5, 260, 260))
                        .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
                )),
            // The fifth invokes ShiftBGAndBlur_WithSourceGraphic on the triangle.
            Group
                .apply((
                    EnableBackground::from(Background::New(None)),
                    WithTransform::from((1080, 0).translate()),
                ))
                .children((
                    Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::Red)),
                    Group.apply(Opacity::from(0.5)).children((
                        Circle::from((125, 75, 45)).apply(Fill::from(Color::Green)),
                        Polygon::from((160, 25, 160, 125, 240, 75)).apply((
                            Fill::from(Color::Blue),
                            WithFilter::from("ShiftBGAndBlur_WithSourceGraphic"),
                        )),
                    )),
                    Rect::from((5, 5, 260, 260))
                        .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
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
            Rect::from((1, 1, 498, 498))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
            Group
                .apply(EnableBackground::from(Background::New(None)))
                .children((
                    Rect::from((100, 20, 300, 460)).apply(Fill::from("MyGradient")),
                    Group
                        .apply((
                            Font::default().family("Verdana").size(75),
                            Fill::from(Rgb::rgb(0x88, 0x88, 0x88)).opacity(0.6),
                        ))
                        .children((
                            Text::from((50, 90))
                                .apply(WithFilter::from("Normal"))
                                .children(Characters::from("Normal")),
                            Text::from((50, 180))
                                .apply(WithFilter::from("Multiply"))
                                .children(Characters::from("Multiply")),
                            Text::from((50, 270))
                                .apply(WithFilter::from("Screen"))
                                .children(Characters::from("Screen")),
                            Text::from((50, 360))
                                .apply(WithFilter::from("Darken"))
                                .children(Characters::from("Darken")),
                            Text::from((50, 450))
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
                .children(FeColorMatrix::from(FeColorMatrixValues::Matrix([
                    0.33, 0.33, 0.33, 0.0, 0.0, 0.33, 0.33, 0.33, 0.0, 0.0, 0.33, 0.33, 0.33, 0.0,
                    0.0, 0.33, 0.33, 0.33, 0.0, 0.0,
                ]))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Saturate40"))
                .children(FeColorMatrix::from(FeColorMatrixValues::Saturate(0.4))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("HueRotate90"))
                .children(FeColorMatrix::from(FeColorMatrixValues::HueRotate(90.0))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("LuminanceToAlpha"))
                .children((
                    FeColorMatrix::from(FeColorMatrixValues::LuminanceToAlpha).result("a"),
                    FeComposite::from("a")
                        .in2("a")
                        .operator(FeCompositeOperator::In),
                )),
            Rect::from((1, 1, 798, 498))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
            Group
                .apply((
                    Font::default()
                        .family("Verdana")
                        .size(75)
                        .weight(FontWeight::Bold),
                    Fill::from("MyGradient"),
                ))
                .children((
                    Rect::from((100, 0, 500, 20)),
                    Text::from((100, 90)).children(Characters::from("Unfiltered")),
                    Text::from((100, 190))
                        .apply(WithFilter::from("Matrix"))
                        .children(Characters::from("Matrix")),
                    Text::from((100, 290))
                        .apply(WithFilter::from("Saturate40"))
                        .children(Characters::from("Saturate40")),
                    Text::from((100, 390))
                        .apply(WithFilter::from("HueRotate90"))
                        .children(Characters::from("HueRotate90")),
                    Text::from((100, 490))
                        .apply(WithFilter::from("LuminanceToAlpha"))
                        .children(Characters::from("Luminance")),
                )),
        ))
}

/// Example from [`fe_component_transfer_01`](https://www.w3.org/TR/SVG11/images/filters/feComponentTransfer.svg)
pub fn fe_component_transfer_01() -> impl Graphics {
    Canvas::from((8.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 800, 400)))
        .children((
            LinearGradient::from((100, 0, 600, 0))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("MyGradient"))
                .children((
                    GradientStop::from(0).color(rgb!(#f00)),
                    GradientStop::from(0.33).color(rgb!(#0f0)),
                    GradientStop::from(0.67).color(rgb!(#00f)),
                    GradientStop::from(1).color(rgb!(#000000)),
                )),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Identity"))
                .children(FeComponentTransfer::default().children((
                    FeFuncR(FeFunc::Identity),
                    FeFuncG(FeFunc::Identity),
                    FeFuncB(FeFunc::Identity),
                    FeFuncA(FeFunc::Identity),
                ))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Table"))
                .children(FeComponentTransfer::default().children((
                    FeFuncR(FeFunc::Table(vec![0.0, 0.0, 1.0, 1.0])),
                    FeFuncG(FeFunc::Table(vec![1.0, 1.0, 0.0, 0.0])),
                    FeFuncB(FeFunc::Table(vec![0.0, 1.0, 1.0, 0.0])),
                ))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Linear"))
                .children(FeComponentTransfer::default().children((
                    FeFuncR(FeFunc::Linear {
                        slope: 0.5,
                        intercept: 0.25,
                    }),
                    FeFuncG(FeFunc::Linear {
                        slope: 0.5,
                        intercept: 0.0,
                    }),
                    FeFuncB(FeFunc::Linear {
                        slope: 0.5,
                        intercept: 0.5,
                    }),
                ))),
            Filter::from((0.percent(), 0.percent(), 100.percent(), 100.percent()))
                .units(Coords::ObjectBoundingBox)
                .apply(Id::from("Gamma"))
                .children(FeComponentTransfer::default().children((
                    FeFuncR(FeFunc::Gamma {
                        amplitude: 2.0,
                        exponent: 5.0,
                        offset: 0.0,
                    }),
                    FeFuncG(FeFunc::Gamma {
                        amplitude: 2.0,
                        exponent: 3.0,
                        offset: 0.0,
                    }),
                    FeFuncB(FeFunc::Gamma {
                        amplitude: 2.0,
                        exponent: 1.0,
                        offset: 0.0,
                    }),
                ))),
            Rect::from((1, 1, 798, 398))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
            Group
                .apply((
                    Font::default()
                        .family("Verdana")
                        .size(75)
                        .weight(FontWeight::Bold),
                    Fill::from("MyGradient"),
                ))
                .children((
                    Rect::from((100, 0, 600, 20)),
                    Text::from((100, 90)).children(Characters::from("Identity")),
                    Text::from((100, 190))
                        .apply(WithFilter::from("Table"))
                        .children(Characters::from("TableLookup")),
                    Text::from((100, 290))
                        .apply(WithFilter::from("Linear"))
                        .children(Characters::from("LinearFunc")),
                    Text::from((100, 390))
                        .apply(WithFilter::from("Gamma"))
                        .children(Characters::from("GammaFunc")),
                )),
        ))
}
