use vglang::{
    opcode::{
        Canvas, Characters, Color, Coords, Fill, Font, FontFamily, GradientStop, Id, Iri,
        LinearGradient, Mask, Paint, Rect, Rgb, Stroke, Text, TextAnchor, TextLayout, Use, ViewBox,
        WithMask,
    },
    sexpr::{Graphics, Slength},
};

/// Example from [`mask01`](https://www.w3.org/TR/SVG11/images/masking/mask01.svg)
pub fn mask_01() -> impl Graphics {
    Canvas::from((8.cm(), 3.cm()))
        .apply(ViewBox::from((0, 0, 800, 300)))
        .children((
            LinearGradient::new(0, 0, 800, 0)
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("Gradient"))
                .children((
                    GradientStop::from(0).color(Color::White).opacity(0),
                    GradientStop::from(1).color(Color::White).opacity(1),
                )),
            Mask::from((0, 0, 800, 300))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::from("Mask"))
                .children(Rect::from((0, 0, 800, 300)).apply(Fill::from_paint("Gradient"))),
            Text::new(400, 200)
                .apply((
                    Id::from("Text"),
                    Font::from_family(FontFamily::Generic("Verdana".into())).size(100),
                    TextLayout::from_anchor(TextAnchor::Middle),
                ))
                .children(Characters::from("Masked text")),
            Rect::from((0, 0, 800, 300)).apply(Fill::from_paint(Rgb::rgb(0xff, 0x80, 0x80))),
            Use::from(Iri::local("Text"))
                .apply((Fill::from_paint(Color::Blue), WithMask::from("Mask"))),
            Use::from(Iri::local("Text")).apply((
                Fill::from_paint(Paint::None),
                Stroke::from_paint(Paint::color(Color::Black)).width(2),
            )),
        ))
}
