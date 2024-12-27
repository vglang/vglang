//! test cases for text element.

use vglang::{
    opcode::{
        attrs::{Fill, Font, Stroke, ViewBox},
        data::{Color, FontWeight},
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

/// Example from [`tspan01`](https://www.w3.org/TR/SVG11/images/text/tspan02.svg)
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
