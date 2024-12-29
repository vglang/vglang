use vglang::{
    opcode::{
        attrs::{Fill, Font, Stroke, TextLayout, ViewBox},
        data::{Color, Rgb, TextAnchor},
        el::{Canvas, Characters, Circle, Group, Path, Polyline, Rect, Text},
    },
    sexpr::{
        arc_relative, close, cubic_bezier, cubic_bezier_smooth, line_to, line_to_relative, move_to,
        quadratic_bezier, quadratic_bezier_smooth, Graphics, Slength,
    },
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

    let connect = || {
        (
            Fill::default(),
            Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(1),
        )
    };

    let sample_path = || (Fill::default(), Stroke::from(Color::red).width(5));

    let end_point = || {
        (
            Fill::default(),
            Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
        )
    };

    let ctl_point = || (Fill::from(Rgb::rgb(0x88, 0x88, 0x88)), Stroke::default());

    let auto_ctl_point = || (Fill::default(), Stroke::from(Color::blue).width(4));

    let label = || Font::from_family("Verdana").size(22);

    Canvas::from((5.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 500, 400)))
        .children((
            Rect::from((1, 1, 498, 398)).apply(border()),
            Polyline::from((100, 200, 100, 100)).apply(connect()),
            Polyline::from((250, 100, 250, 200)).apply(connect()),
            Polyline::from((250, 200, 250, 300)).apply(connect()),
            Polyline::from((400, 300, 400, 200)).apply(connect()),
            Path::from((
                move_to(100, 200),
                cubic_bezier((100, 100), (250, 100), (250, 200)),
                cubic_bezier_smooth((400, 300), (400, 200)),
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
            Text::from((325, 300))
                .apply((label(), TextLayout::from(TextAnchor::Middle)))
                .children(Characters::from("S400,300 400,200")),
        ))
}

/// Example [`cubic01`](https://www.w3.org/TR/SVG11/images/paths/cubic01.svg)
pub fn quad_01() -> impl Graphics {
    Canvas::from((12.cm(), 6.cm()))
        .apply(ViewBox::from((0, 0, 1200, 600)))
        .children((
            Rect::from((1, 1, 1198, 598))
                .apply((Fill::default(), Stroke::from(Color::blue).width(1))),
            Path::from((
                move_to(200, 300),
                quadratic_bezier((400, 50), (600, 300)),
                quadratic_bezier_smooth((100, 300)),
            ))
            .apply((Fill::default(), Stroke::from(Color::red).width(5))),
            Group.apply(Fill::from(Color::black)).children((
                Circle::from((200, 300, 10)),
                Circle::from((600, 300, 10)),
                Circle::from((1000, 300, 10)),
            )),
            Group
                .apply(Fill::from(Rgb::rgb(0x88, 0x88, 0x88)))
                .children((Circle::from((400, 50, 10)), Circle::from((800, 550, 10)))),
            Path::from((
                move_to(200, 300),
                line_to(400, 50),
                line_to(600, 300),
                line_to(800, 550),
                line_to(1000, 300),
            ))
            .apply((
                Fill::default(),
                Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
            )),
        ))
}

/// Example [`arcs_01`](https://www.w3.org/TR/SVG11/images/paths/arcs01.svg)
pub fn arcs_01() -> impl Graphics {
    Canvas::from((12.cm(), 5.25.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children((
            Rect::from((1, 1, 1198, 598))
                .apply((Fill::default(), Stroke::from(Color::blue).width(1))),
            Path::from((
                move_to(300, 200),
                line_to_relative(-150, 0),
                arc_relative(150, 150, 0, true, false, (150, -150)),
                close(),
            ))
            .apply((Fill::from(Color::red), Stroke::from(Color::blue).width(5))),
            Path::from((
                move_to(275, 175),
                line_to_relative(0, -150),
                arc_relative(150, 150, 0, false, false, (150, -150)),
                close(),
            ))
            .apply((
                Fill::from(Color::yellow),
                Stroke::from(Color::blue).width(5),
            )),
            Path::from((
                move_to(600, 350),
                line_to_relative(50, -25),
                arc_relative(25, 25, -30, false, true, (50, -25)),
                line_to_relative(50, -50),
                arc_relative(25, 50, -30, false, true, (50, -25)),
                line_to_relative(50, -50),
                arc_relative(25, 75, -30, false, true, (50, -25)),
                line_to_relative(50, -50),
                arc_relative(25, 100, -30, false, true, (50, -25)),
                line_to_relative(50, -50),
                close(),
            ))
            .apply((Fill::default(), Stroke::from(Color::red).width(5))),
        ))
}
