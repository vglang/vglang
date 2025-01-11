//! test cases for text element.

use vglang::{
    opcode::{
        attrs::{Fill, Font, Id, Stroke, TextLayout, ViewBox},
        data::{Color, FontFamily, FontWeight, Rgb, TextDecoration, TextDirection, UnicodeBidi},
        el::{Canvas, Characters, Group, Path, Rect, Text, TextPath, TextSpan, Use},
    },
    sexpr::{cubic_bezier, move_to, Graphics, Slength},
};

/// Example from [`text01`](https://www.w3.org/TR/SVG11/images/text/text01.svg)
pub fn text_0_1() -> impl Graphics {
    Canvas::from((10.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Text::from((250, 150))
                .apply((
                    Font::from_family("Verdana").size(55),
                    Fill::from(Color::blue),
                ))
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
            Group.apply(Font::from_family("Verdana").size(55)).children(
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
            Group.apply(Font::from_family("Verdana").size(55)).children(
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
            Group.apply(Font::from_family("Verdana").size(45)).children(
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
                .apply((
                    Font::from_family("Verdana").size(55),
                    Fill::from(Color::blue),
                ))
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
    Canvas::from((100.percent(), 100.percent()))
        .apply(ViewBox::from((0, 0, 500, 120)))
        .children((
            Text::from((40, 40))
                .rotate((5, 15, 25, 35, 45, 55))
                .apply((
                    Font::from_family(("Arial".to_string(), FontFamily::SansSerif)).size(32),
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

/// Example from [`rtl_text`](https://www.w3.org/TR/SVG11/images/text/rtl-complex.svg)
pub fn rtl_text() -> impl Graphics {
    Canvas::from((100.percent(), 100.percent()))
        .apply(ViewBox::from((0, 0, 400, 400)))
        .children(
            Text::from((40, 40)).apply(Font::from(20)).children((
                Characters::from("כתובת MAC:&#x200F;"),
                TextSpan::default()
                    .apply(TextLayout::from(TextDirection::Ltr).unicode_bidi(UnicodeBidi::Embed))
                    .children(Characters::from("00-24-AF-2A-55-FC")),
            )),
        )
}

/// Example from [`text_decoration_01`](https://www.w3.org/TR/SVG11/images/text/textdecoration01.svg)
pub fn text_decoration_01() -> impl Graphics {
    Canvas::from((12.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 1200, 400)))
        .children(
            Group
                .apply((
                    Font::from(60),
                    Fill::from(Color::blue),
                    Stroke::from(Color::red).width(1),
                ))
                .children((
                    Text::from((100, 75)).children(Characters::from("Normal text")),
                    Text::from((100, 165))
                        .apply(TextLayout::from(TextDecoration::LineThrough))
                        .children(Characters::from("Text with line-through")),
                    Text::from((100, 255))
                        .apply(TextLayout::from(TextDecoration::Underline))
                        .children(Characters::from("Underlined text")),
                    Text::from((100, 345))
                        .apply(TextLayout::from(TextDecoration::Underline))
                        .children((
                            TextSpan::default().children(Characters::from("One ")),
                            TextSpan::default()
                                .apply((Fill::from(Color::yellow), Stroke::from(Color::purple)))
                                .children(Characters::from("word ")),
                            TextSpan::default()
                                .apply((Fill::from(Color::yellow), Stroke::from(Color::black)))
                                .children(Characters::from("has ")),
                            TextSpan::default()
                                .apply((
                                    Fill::from(Color::yellow),
                                    Stroke::from(Color::darkgreen),
                                    TextLayout::from(TextDecoration::Underline),
                                ))
                                .children(Characters::from("different ")),
                            TextSpan::default()
                                .apply((Fill::from(Color::yellow), Stroke::from(Color::blue)))
                                .children(Characters::from("underlining")),
                        )),
                )),
        )
}

/// Example from [`toap_01`](https://www.w3.org/TR/SVG11/images/text/toap01.svg)
pub fn toap_01() -> impl Graphics {
    Canvas::from((12.cm(), 3.6.cm()))
        .apply(ViewBox::from((0, 0, 1000, 300)))
        .children((
            Path::from((
                move_to(100, 200),
                cubic_bezier((200, 100), (300, 0), (400, 100)),
                cubic_bezier((500, 200), (600, 300), (700, 200)),
                cubic_bezier((800, 100), (900, 100), (900, 100)),
            ))
            .apply(Id::from("MyPath")),
            Use::from("MyPath").apply((Fill::default(), Stroke::from(Color::red))),
            Text::default()
                .apply((
                    Font::from_family("Verdana").size(42.5),
                    Fill::from(Color::blue),
                ))
                .children(
                    TextPath::from("MyPath")
                        .start_offset(10.percent())
                        .children(Characters::from("We go up, then we go down, then up again")),
                ),
            Rect::from((1, 1, 998, 298))
                .apply((Fill::default(), Stroke::from(Color::blue).width(2))),
        ))
}
