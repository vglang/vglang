use vglang::{
    opcode::{
        attrs::{Fill, Stroke, Transformed, ViewBox},
        data::Color,
        el::{Canvas, Ellipse, Group, Line, Rect},
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
                .apply(Transformed::from(((700, 210).translate(), (-30).rotate())))
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
                .apply(Transformed::from((300, 200).translate()))
                .children(Ellipse::from((250, 100))),
            Group
                .apply(Transformed::from(((900, 200).translate(), (-30).rotate())))
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
