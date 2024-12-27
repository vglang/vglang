//! test cases for text element.

use vglang::{
    opcode::{
        attrs::{Fill, Font, Stroke, ViewBox},
        data::{Color, FontFamily, FontWeight, Rgb},
        el::{Canvas, Characters, Group, Rect, Text, TextSpan},
    },
    sexpr::{Graphics, Slength},
};

/// Example from [`text01`](https://www.w3.org/TR/SVG11/images/text/text01.svg)
pub fn text_0_1() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Text::from((250, 150))
                .apply((Font::from("Verdana").size(55), Fill::from(Color::blue)))
                .children(Characters::from("Hello, out there")),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}

/// Example from [`tspan01`](https://www.w3.org/TR/SVG11/images/text/tspan01.svg)
pub fn tspan_0_1() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Group.apply(Font::from("Verdana").size(55)).children(
                Text::from((250, 150))
                    .apply(Fill::from(Color::blue))
                    .children((
                        Characters::from("You are"),
                        TextSpan::default()
                            .apply((Font::from(FontWeight::Bold), Fill::from(Color::red)))
                            .children(Characters::from("not")),
                        Characters::from("a banana."),
                    )),
            ),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}

/// Example from [`tspan02`](https://www.w3.org/TR/SVG11/images/text/tspan02.svg)
pub fn tspan_0_2() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Group.apply(Font::from("Verdana").size(55)).children(
                Text::from((250, 150))
                    .apply(Fill::from(Color::blue))
                    .children((
                        Characters::from("But you"),
                        TextSpan::default()
                            .dx(2.em())
                            .dy(-50)
                            .apply((Font::from(FontWeight::Bold), Fill::from(Color::red)))
                            .children(Characters::from("are")),
                        TextSpan::default()
                            .dy(100)
                            .children(Characters::from("a peach!")),
                    )),
            ),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}

/// Example from [`tspan03`](https://www.w3.org/TR/SVG11/images/text/tspan03.svg)
pub fn tspan_0_3() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Group.apply(Font::from("Verdana").size(45)).children(
                Text::default()
                    .apply(Fill::from(Rgb::rgb(255, 164, 0)))
                    .children(
                        TextSpan::default()
                            .x((375, 425, 475, 525, 575))
                            .y(200)
                            .children(Characters::from("fuzzy")),
                    ),
            ),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}

/// Example from [`tspan04`](https://www.w3.org/TR/SVG11/images/text/tspan04.svg)
pub fn tspan_0_4() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Group
                .apply((Font::from("Verdana").size(55), Fill::from(Color::blue)))
                .children(
                    Text::from((250, 150))
                        .rotate((-30, 0, 30))
                        .apply(Fill::from(Rgb::rgb(255, 164, 0)))
                        .children(Characters::from("Hello, out there")),
                ),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}

/// Example from [`tspan05`](https://www.w3.org/TR/SVG11/images/text/tspan05.svg)
pub fn tspan_0_5() -> impl Graphics {
    Canvas::from((1.percent(), 1.percent()))
        .apply(ViewBox::from((0, 0, 500, 120)))
        .children((
            Text::from((40, 40))
                .rotate((5, 15, 25, 35, 45, 55))
                .apply((
                    Font::from(("Arial".to_string(), FontFamily::SansSerif)).size(32),
                    Fill::from(Color::red),
                ))
                .children((
                    Characters::from("Not"),
                    TextSpan::default()
                        .rotate((70, 60, 50, 40, 30, 20, 10))
                        .apply(Fill::from(Color::yellow))
                        .children((
                            Characters::from("all characters"),
                            TextSpan::default()
                                .rotate((70, 60, 50, 40, 30, 20, 10))
                                .apply(Fill::from(Color::yellow)),
                            Characters::from("in"),
                            TextSpan::default().children(Characters::from("the")),
                            TextSpan::from((40, 90)).children(Characters::from("text")),
                            Characters::from("have a"),
                        )),
                    TextSpan::default()
                        .rotate(-10)
                        .apply(Fill::from(Color::blue))
                        .children(Characters::from("specified")),
                    Characters::from("rotation"),
                )),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}
