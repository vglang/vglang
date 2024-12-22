mod tester;

use tester::{border, svg};
use vglang_sexpr::{
    apply,
    operand::{
        Canvas, Circle, ClipPath, ClipPathed, Color, Coords, Fill, Font, FontWeight, GradientStop,
        Id, LinearGradient, Mask, Masked, Opacity, Polygon, Rect, Stroke, Text, TextAnchor,
        TextLayout, Use,
    },
    Graphic, Sgradient, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_compositing() {
    svg("mask", mask()).await;
    svg("opacity", opacity()).await;
    svg("clipPath", clip_path()).await;
}

fn clip_path<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        Canvas::from((660, 220)),
        (
            border(660, 220),
            apply(
                (
                    Id::from("cut-text"),
                    ClipPath::default(),
                    Text::from((50, 133)).font(
                        Font::from(10.em())
                            .family("Helvetica")
                            .weight(FontWeight::Bold),
                    ),
                ),
                "Clipping Path",
            ),
            apply(
                ClipPathed::from("cut-text"),
                (
                    apply(
                        Fill::from((0xcc, 0x99, 0x99)),
                        Polygon::from((110, 10, 660, 110, 110, 210)),
                    ),
                    apply(
                        Fill::from((0x99, 0xcc, 0x66)),
                        Circle::from((110, 110, 100)),
                    ),
                ),
            ),
        ),
    )
}

fn mask<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((8.cm(), 3.cm())).viewbox((0, 0, 800, 300)),
            Fill::default(),
        ),
        (
            apply(
                (
                    Id::from("Gradient"),
                    LinearGradient::from((0, 0, 800, 0)).units(Coords::UserSpaceOnUse),
                ),
                (
                    GradientStop::from(0).color(Color::white).opacity(0),
                    GradientStop::from(1).color(Color::white).opacity(1),
                ),
            ),
            apply(
                (
                    Id::from("Mask"),
                    Mask::from((0, 0, 800, 300)).units(Coords::UserSpaceOnUse),
                    Fill::from("Gradient".gradient()),
                ),
                Rect::from((0, 0, 800, 300)),
            ),
            apply(
                (
                    Id::from("Text"),
                    Font::from("Verdana").size(100),
                    TextLayout::from(TextAnchor::Middle),
                    Text::from((400, 200)),
                ),
                "Masked text",
            ),
            apply(Fill::from((0xff, 0x80, 0x80)), Rect::from((0, 0, 800, 300))),
            apply(
                (Fill::from(Color::blue), Masked::from("Mask")),
                Use::from("Text"),
            ),
            apply(
                (Fill::default(), Stroke::from(Color::black).width(2)),
                Use::from("Text"),
            ),
        ),
    )
}

fn opacity<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((12.cm(), 3.5.cm())).viewbox((0, 0, 1200, 350)),
            Fill::default(),
        ),
        (
            border(1200, 350),
            apply(
                Fill::from((0x00, 0x00, 0xff)),
                Rect::from((100, 100, 1000, 150)),
            ),
            apply(
                (Fill::from(Color::red), Opacity::from(1)),
                Circle::from((200, 100, 50)),
            ),
            apply(
                (Fill::from(Color::red), Opacity::from(0.8)),
                Circle::from((400, 100, 50)),
            ),
            apply(
                (Fill::from(Color::red), Opacity::from(0.6)),
                Circle::from((600, 100, 50)),
            ),
            apply(
                (Fill::from(Color::red), Opacity::from(0.4)),
                Circle::from((800, 100, 50)),
            ),
            apply(
                (Fill::from(Color::red), Opacity::from(0.2)),
                Circle::from((1000, 100, 50)),
            ),
            apply(
                Opacity::from(1),
                (
                    apply(Fill::from(Color::red), Circle::from((182.5, 250, 50))),
                    apply(Fill::from(Color::green), Circle::from((217.5, 250, 50))),
                ),
            ),
            apply(
                Opacity::from(0.5),
                (
                    apply(Fill::from(Color::red), Circle::from((382.5, 250, 50))),
                    apply(Fill::from(Color::green), Circle::from((417.5, 250, 50))),
                ),
            ),
            apply(
                Opacity::from(1),
                (
                    apply(
                        (Fill::from(Color::red), Opacity::from(0.5)),
                        Circle::from((582.5, 250, 50)),
                    ),
                    apply(
                        (Fill::from(Color::green), Opacity::from(0.5)),
                        Circle::from((617.5, 250, 50)),
                    ),
                ),
            ),
            apply(
                Opacity::from(1),
                (
                    apply(
                        (Fill::from(Color::green), Opacity::from(0.5)),
                        Circle::from((817.5, 250, 50)),
                    ),
                    apply(
                        (Fill::from(Color::red), Opacity::from(0.5)),
                        Circle::from((782.5, 250, 50)),
                    ),
                ),
            ),
            apply(
                Opacity::from(0.5),
                (
                    apply(
                        (Fill::from(Color::red), Opacity::from(0.5)),
                        Circle::from((982.5, 250, 50)),
                    ),
                    apply(
                        (Fill::from(Color::green), Opacity::from(0.5)),
                        Circle::from((1017.5, 250, 50)),
                    ),
                ),
            ),
        ),
    )
}
