use vglang::{
    opcode::{
        attrs::{Fill, Stroke, ViewBox},
        data::Color,
        el::{Canvas, Path, Rect},
    },
    sexpr::{close, line_to, move_to, Graphics, Slength},
};

/// Example [`triangle01`](https://www.w3.org/TR/SVG11/images/paths/triangle01.svg)
pub fn triangle_01() -> impl Graphics {
    Canvas::from((4.em(), 4.em()))
        .apply(ViewBox::from((0, 0, 400, 400)))
        .children((
            Rect::from((1, 1, 398, 398)),
            Path::from((
                move_to(100, 100),
                line_to(300, 100),
                line_to(200, 300),
                close(),
            )),
        ))
}

/// Example [`cubic01`](https://www.w3.org/TR/SVG11/images/paths/cubic01.svg)
pub fn cubic_01() -> impl Graphics {
    let border = || (Fill::default(), Stroke::from(Color::blue).width(1));

    let connect = || (Fill::default(), Stroke::from(Color::blue).width(1));

    Canvas::from((5.cm(), 4.cm())).apply(ViewBox::from((0, 0, 500, 400)))
}
