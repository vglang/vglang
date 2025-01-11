use vglang::{
    opcode::{
        attrs::{Fill, Stroke, ViewBox, WithTransform},
        data::Color,
        el::{Canvas, Ellipse, Group, Line, Polygon, Polyline, Rect},
    },
    sexpr::{Graphics, Slength, Srotate, Stranslate},
};

/// See [`rect_01`](https://www.w3.org/TR/SVG11/images/shapes/rect01.svg)
pub fn rect_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Rect::from((400, 100, 400, 200)).apply((
                Fill::from(Color::yellow),
                Stroke::from(Color::navy).width(10),
            )),
        ))
}

/// See [`rect_02`](https://www.w3.org/TR/SVG11/images/shapes/rect02.svg)
pub fn rect_02() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Rect::from((100, 400, 400, 200)).rx(50).apply((
                Fill::from(Color::green),
                Stroke::from(Color::navy).width(10),
            )),
            Group
                .apply(WithTransform::from((
                    (700, 210).translate(),
                    (-30).rotate(),
                )))
                .children(
                    Rect::from((0, 0, 400, 200))
                        .rx(50)
                        .apply((Fill::default(), Stroke::from(Color::purple).width(30))),
                ),
        ))
}

/// See [`ellipse_01`](https://www.w3.org/TR/SVG11/images/shapes/ellipse01.svg)
pub fn ellipse_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Group
                .apply(WithTransform::from((300, 200).translate()))
                .children(Ellipse::from((250, 100))),
            Group
                .apply(WithTransform::from((
                    (900, 200).translate(),
                    (-30).rotate(),
                )))
                .children(
                    Ellipse::from((250, 100))
                        .apply((Fill::default(), Stroke::from(Color::purple).width(30))),
                ),
        ))
}

/// See [`line_01`](https://www.w3.org/TR/SVG11/images/shapes/line01.svg)
pub fn line_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Group.apply(Stroke::from(Color::green)).children((
                Line::from((100, 300, 300, 100)).apply(Stroke::default().width(5)),
                Line::from((300, 300, 500, 100)).apply(Stroke::default().width(10)),
                Line::from((500, 300, 700, 100)).apply(Stroke::default().width(15)),
                Line::from((700, 300, 900, 100)).apply(Stroke::default().width(20)),
                Line::from((900, 300, 1100, 100)).apply(Stroke::default().width(25)),
            )),
        ))
}

/// See [`polyline_01`](https://www.w3.org/TR/SVG11/images/shapes/polyline01.svg)
pub fn polyline_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Polyline::from((
                150, 375, 150, 325, 250, 325, 250, 375, 350, 375, 350, 250, 450, 250, 450, 375,
                550, 375, 550, 175, 650, 175, 650, 375, 750, 375, 750, 100, 850, 100, 850, 375,
                950, 375, 950, 25, 1050, 25, 1050, 375, 1150, 375,
            ))
            .apply((Fill::default(), Stroke::from(Color::blue).width(10))),
        ))
}

/// See [`polygon_01`](https://www.w3.org/TR/SVG11/images/shapes/polygon01.svg)
pub fn polygon_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398)),
            Polygon::from((
                350, 75, 379, 161, 469, 161, 397, 215, 423, 301, 350, 250, 277, 301, 303, 215, 231,
                161, 321, 161,
            ))
            .apply((Fill::from(Color::red), Stroke::from(Color::blue).width(10))),
            Polygon::from((
                850, 75, 958, 137.5, 958, 262.5, 850, 325, 742, 262.6, 742, 137.5,
            ))
            .apply((Fill::from(Color::lime), Stroke::from(Color::blue).width(10))),
        ))
}
