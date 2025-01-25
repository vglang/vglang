use vglang::{opcode::*, sexpr::*};

/// Example [`triangle01`](https://www.w3.org/TR/SVG11/images/paths/triangle01.svg)
pub fn triangle_01() -> impl Graphics {
    Canvas::from((4.em(), 4.em()))
        .apply(ViewBox::from((0, 0, 400, 400)))
        .children((
            Rect::from((1, 1, 398, 398))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue))),
            Path::from((
                (100, 100).move_to(),
                (300, 100).line_to(),
                (200, 300).line_to(),
                PathEvent::Close,
            ))
            .apply((Fill::from(Color::Red), Stroke::from(Color::Blue).width(3))),
        ))
}

/// Example [`cubic01`](https://www.w3.org/TR/SVG11/images/paths/cubic01.svg)
pub fn cubic_01() -> impl Graphics {
    let border = || (Fill::from(Paint::None), Stroke::from(Color::Blue).width(1));

    let connect = || {
        (
            Fill::from(Paint::None),
            Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(1),
        )
    };

    let sample_path = || (Fill::from(Paint::None), Stroke::from(Color::Red).width(5));

    let end_point = || {
        (
            Fill::from(Paint::None),
            Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
        )
    };

    let ctl_point = || {
        (
            Fill::from(Rgb::rgb(0x88, 0x88, 0x88)),
            Stroke::from(Paint::None),
        )
    };

    let auto_ctl_point = || (Fill::from(Paint::None), Stroke::from(Color::Blue).width(4));

    let label = || Font::default().family("Verdana").size(22);

    Canvas::from((5.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 500, 400)))
        .children((
            Rect::from((1, 1, 498, 398)).apply(border()),
            Polyline::from((100, 200, 100, 100)).apply(connect()),
            Polyline::from((250, 100, 250, 200)).apply(connect()),
            Polyline::from((250, 200, 250, 300)).apply(connect()),
            Polyline::from((400, 300, 400, 200)).apply(connect()),
            Path::from((
                (100, 200).move_to(),
                ((100, 100), (250, 100), (250, 200)).cubic_bezier(),
                ((400, 300), (400, 200)).cubic_bezier_smooth(),
            ))
            .apply(sample_path()),
            Circle::from((100, 200, 10)).apply(end_point()),
            Circle::from((250, 200, 10)).apply(end_point()),
            Circle::from((400, 200, 10)).apply(end_point()),
            Circle::from((100, 100, 10)).apply(ctl_point()),
            Circle::from((250, 100, 10)).apply(ctl_point()),
            Circle::from((400, 300, 10)).apply(ctl_point()),
            Circle::from((250, 300, 9)).apply(auto_ctl_point()),
            Text::from((25, 70))
                .apply(label())
                .children(Characters::from("M100,200 C100,100 250,100 250,200")),
            Text::from((325, 350))
                .apply((label(), TextLayout::default().anchor(TextAnchor::Middle)))
                .children(Characters::from("S400,300 400,200")),
        ))
}

/// Example [`cubic01`](https://www.w3.org/TR/SVG11/images/paths/cubic01.svg)
pub fn quad_01() -> impl Graphics {
    Canvas::from((12.cm(), 6.cm()))
        .apply(ViewBox::from((0, 0, 1200, 600)))
        .children((
            Rect::from((1, 1, 1198, 598))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue).width(1))),
            Path::from((
                (200, 300).move_to(),
                ((400, 50), (600, 300)).quadratic_bezier(),
                ((1000, 300)).quadratic_bezier_smooth(),
            ))
            .apply((Fill::from(Paint::None), Stroke::from(Color::Red).width(5))),
            Group.apply(Fill::from(Color::Black)).children((
                Circle::from((200, 300, 10)),
                Circle::from((600, 300, 10)),
                Circle::from((1000, 300, 10)),
            )),
            Group
                .apply(Fill::from(Rgb::rgb(0x88, 0x88, 0x88)))
                .children((Circle::from((400, 50, 10)), Circle::from((800, 550, 10)))),
            Path::from((
                (200, 300).move_to(),
                (400, 50).line_to(),
                (600, 300).line_to(),
                (800, 550).line_to(),
                (1000, 300).line_to(),
            ))
            .apply((
                Fill::from(Paint::None),
                Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
            )),
        ))
}

/// Example [`arcs_01`](https://www.w3.org/TR/SVG11/images/paths/arcs01.svg)
pub fn arcs_01() -> impl Graphics {
    Canvas::from((12.cm(), 5.25.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 398))
                .apply((Fill::from(Paint::None), Stroke::from(Color::Blue).width(1))),
            Path::from((
                (300, 200).move_to(),
                (-150, 0).line_to_relative(),
                (150, 150, 0, true, false, (150, -150)).arc_relative(),
                PathEvent::Close,
            ))
            .apply((Fill::from(Color::Red), Stroke::from(Color::Blue).width(5))),
            Path::from((
                (275, 175).move_to(),
                (0, -150).line_to_relative(),
                (150, 150, 0, false, false, (-150, 150)).arc_relative(),
                PathEvent::Close,
            ))
            .apply((
                Fill::from(Color::Yellow),
                Stroke::from(Color::Blue).width(5),
            )),
            Path::from((
                (600, 350).move_to(),
                (50, -25).line_to_relative(),
                (25, 25, -30, false, true, (50, -25)).arc_relative(),
                (50, -25).line_to_relative(),
                (25, 50, -30, false, true, (50, -25)).arc_relative(),
                (50, -25).line_to_relative(),
                (25, 75, -30, false, true, (50, -25)).arc_relative(),
                (50, -25).line_to_relative(),
                (25, 100, -30, false, true, (50, -25)).arc_relative(),
                (50, -25).line_to_relative(),
            ))
            .apply((Fill::from(Paint::None), Stroke::from(Color::Red).width(5))),
        ))
}
