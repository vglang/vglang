use vglang::{
    opcode::{
        attrs::{Fill, Id, Stroke, ViewBox},
        data::{Color, Coords, Rgb},
        el::{Canvas, GradientStop, LinearGradient, RadialGradient, Rect},
    },
    sexpr::{Graphics, Slength},
};

/// Example from [`lingrad_01`](https://www.w3.org/TR/SVG11/images/pservers/lingrad01.svg)
pub fn lingrad_01() -> impl Graphics {
    Canvas::from((8.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 800, 400)))
        .children((
            LinearGradient::default()
                .apply(Id::from("MyGradient"))
                .children((
                    GradientStop::from(0.05).color(Rgb::rgb(0xff, 0x66, 0x00)),
                    GradientStop::from(0.95).color(Rgb::rgb(0xff, 0xff, 0x66)),
                )),
            Rect::from((1, 1, 798, 398)).apply(Stroke::from(Color::blue)),
            Rect::from((100, 100, 600, 200))
                .apply((Stroke::from(Color::black), Fill::from("MyGradient"))),
        ))
}

/// Example from [`radgrad_01`](https://www.w3.org/TR/SVG11/images/pservers/radgrad01.svg)
pub fn radgrad_01() -> impl Graphics {
    Canvas::from((8.cm(), 4.cm()))
        .apply(ViewBox::from((0, 0, 800, 400)))
        .children((
            RadialGradient::from((400, 200, 300, 400, 200))
                .unit(Coords::UserSpaceOnUse)
                .apply(Id::from("MyGradient"))
                .children((
                    GradientStop::from(0.00).color(Color::red),
                    GradientStop::from(0.5).color(Color::blue),
                    GradientStop::from(1.0).color(Color::red),
                )),
            Rect::from((1, 1, 798, 398)).apply(Stroke::from(Color::blue)),
            Rect::from((100, 100, 600, 200))
                .apply((Stroke::from(Color::black), Fill::from("MyGradient"))),
        ))
}
