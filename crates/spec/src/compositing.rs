use vglang::{
    opcode::*,
    sexpr::{Graphics, Slength},
};

/// Example from [`mask01`](https://www.w3.org/TR/SVG11/images/masking/mask01.svg)
pub fn mask_01() -> impl Graphics {
    Canvas::from((8.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 800, 300)))
        .children((
            LinearGradient::from((0, 0, 800, 0))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("Gradient"))
                .children((
                    GradientStop::from(0).color(Color::white).opacity(0),
                    GradientStop::from(1).color(Color::white).opacity(1),
                )),
            Mask::from((0, 0, 800, 300))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("Mask"))
                .children(Rect::from((0, 0, 800, 300)).apply(Fill::from("Gradient"))),
            Text::from((400, 200)).apply((
                Id::from("Text"),
                Font::from_family("Verdana").size(100),
                TextLayout::from(TextAnchor::Middle),
            )),
            Rect::from((0, 0, 800, 300)).apply(Fill::from(Rgb::rgb(0xff, 0x80, 0x80))),
            Use::from("Text").apply((Fill::from(Color::blue), WithMask::from("Maks"))),
            Use::from("Text").apply((Fill::default(), Stroke::from(Color::black).width(2))),
        ))
}

/// Example from [`opacity_01`](https://www.w3.org/TR/SVG11/images/masking/opacity01.svg)
pub fn opacity_01() -> impl Graphics {
    Canvas::from((12.cm(), 3.5.cm()))
        .apply(ViewBox::from((0, 0, 1200, 350)))
        .children((
            Rect::from((1, 1, 1198, 348)).apply((Fill::default(), Stroke::from(Color::blue))),
            Rect::from((100, 100, 1000, 150)).apply(Fill::from(Rgb::rgb(0x00, 0x00, 0xff))),
            Circle::from((200, 100, 50)).apply((Fill::from(Color::red), Opacity::from(1))),
            Circle::from((400, 100, 50)).apply((Fill::from(Color::red), Opacity::from(0.8))),
            Circle::from((600, 100, 50)).apply((Fill::from(Color::red), Opacity::from(0.6))),
            Circle::from((800, 100, 50)).apply((Fill::from(Color::red), Opacity::from(0.4))),
            Circle::from((1000, 100, 50)).apply((Fill::from(Color::red), Opacity::from(0.2))),
            Group.apply(Opacity::from(1)).children((
                Circle::from((182.5, 250, 50)).apply((Fill::from(Color::red), Opacity::from(1))),
                Circle::from((217.5, 250, 50)).apply((Fill::from(Color::green), Opacity::from(1))),
            )),
            Group.apply(Opacity::from(0.5)).children((
                Circle::from((382.5, 250, 50)).apply((Fill::from(Color::red), Opacity::from(1))),
                Circle::from((417.5, 250, 50)).apply((Fill::from(Color::green), Opacity::from(1))),
            )),
            Group.apply(Opacity::from(1)).children((
                Circle::from((582.5, 250, 50)).apply((Fill::from(Color::red), Opacity::from(0.5))),
                Circle::from((617.5, 250, 50))
                    .apply((Fill::from(Color::green), Opacity::from(0.5))),
            )),
            Group.apply(Opacity::from(1)).children((
                Circle::from((817.5, 250, 50))
                    .apply((Fill::from(Color::green), Opacity::from(0.5))),
                Circle::from((782.5, 250, 50)).apply((Fill::from(Color::red), Opacity::from(0.5))),
            )),
            Group.apply(Opacity::from(1)).children((
                Circle::from((982.5, 250, 50)).apply((Fill::from(Color::red), Opacity::from(0.5))),
                Circle::from((1017.5, 250, 50))
                    .apply((Fill::from(Color::green), Opacity::from(0.5))),
            )),
        ))
}
