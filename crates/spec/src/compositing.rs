use vglang::{
    opcode::{
        Canvas, Color, Coords, Fill, Font, FontFamily, FuncIri, GradientStop, Id, Iri,
        LinearGradient, Mask, Paint, Rect, Rgb, Stroke, Text, Use, ViewBox, WithMask,
    },
    sexpr::{Graphics, Slength},
};

/// Example from [`mask01`](https://www.w3.org/TR/SVG11/images/masking/mask01.svg)
pub fn mask_01() -> impl Graphics {
    Canvas::new(8.cm(), 3.cm())
        .apply(ViewBox::new(0, 0, 800, 300))
        .children((
            LinearGradient::new(0, 0, 800, 0)
                .units(Coords::UserSpaceOnUse)
                .apply(Id::new("Gradient"))
                .children((
                    GradientStop::new(0).color(Color::White).opacity(0),
                    GradientStop::new(1).color(Color::White).opacity(1),
                )),
            Mask::from((0, 0, 800, 300))
                .units(Coords::UserSpaceOnUse)
                .apply(Id::new("Mask"))
                .children(
                    Rect::new(0, 0, 800, 300)
                        .apply(Fill::new(Paint::server(FuncIri::path("Gradient")))),
                ),
            Text::new(400, 200).apply((
                Id::new("Text"),
                Font::from_family(FontFamily::Generic("Verdana".into())).size(100),
            )),
            Rect::new(0, 0, 800, 300).apply(Fill::new(Paint::Color(Rgb::rgb(0xff, 0x80, 0x80)))),
            Use::new(Iri::new("Text")).apply((
                Fill::new(Paint::color(Color::Blue)),
                WithMask::new(FuncIri::path("Gradient")),
            )),
            Use::new(Iri::new("Text")).apply((
                Fill::default(),
                Stroke::from_paint(Paint::color(Color::Black)).width(2),
            )),
        ))
}
