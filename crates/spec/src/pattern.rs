use vglang::{
    opcode::{
        attrs::{Fill, Id, Stroke, ViewBox},
        data::{Color, Coords},
        el::{Canvas, Ellipse, Path, Pattern, Rect},
    },
    sexpr::{line_to, move_to, Graphics, Slength},
};

/// Example from [`pattern01`](https://www.w3.org/TR/SVG11/images/pservers/pattern01.svg)
pub fn pattern_01() -> impl Graphics {
    Canvas::from((8.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 800, 400)))
        .children((
            Pattern::from((0, 0, 100, 100))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("TrianglePattern"))
                .children(Path::from((move_to(0, 0), line_to(7, 0)))),
            Rect::from((1, 1, 798, 398)).apply((Stroke::from(Color::blue), Fill::default())),
            Ellipse::from((400, 200, 350, 150)).apply((
                Stroke::from(Color::black).width(5),
                Fill::from("TrianglePattern"),
            )),
        ))
}
