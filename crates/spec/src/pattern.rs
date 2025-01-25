use vglang::{opcode::*, sexpr::*};

/// Example from [`pattern01`](https://www.w3.org/TR/SVG11/images/pservers/pattern01.svg)
pub fn pattern_01() -> impl Graphics {
    Canvas::from((8.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 800, 400)))
        .children((
            Pattern::from((0, 0, 100, 100))
                .units(Coords::UserSpaceOnUse)
                .apply((Id::from("TrianglePattern"), ViewBox::from((0, 0, 10, 10))))
                .children(
                    Path::from((
                        (0, 0).move_to(),
                        (7, 0).line_to(),
                        (3.5, 7).line_to(),
                        PathEvent::Close,
                    ))
                    .apply((Fill::from(Color::Red), Stroke::from(Color::Blue))),
                ),
            Rect::from((1, 1, 798, 398))
                .apply((Stroke::from(Color::Blue), Fill::from(Paint::None))),
            Ellipse::from((350, 150)).cx(400).cy(200).apply((
                Stroke::from(Color::Black).width(5),
                Fill::from("TrianglePattern"),
            )),
        ))
}
