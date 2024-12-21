mod tester;
use tester::{border, svg};
use vglang_sexpr::{
    apply,
    operand::{
        Canvas, Color, Coords, Fill, GradientStop, Id, LinearGradient, RadialGradient, Rect, Stroke,
    },
    Graphic, Sgradient, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_gradient() {
    svg("linear", linear()).await;
    svg("radial", radial()).await;
}

fn linear<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((8.cm(), 4.cm())).viewbox((0, 0, 800, 400)),
            Fill::default(),
        ),
        (
            border(800, 400),
            apply(
                (Id::from("MyGradient"), LinearGradient::default()),
                (
                    GradientStop::from(0.05).color((0xff, 0x66, 0x00)),
                    GradientStop::from(0.95).color((0xff, 0xff, 0x66)),
                ),
            ),
            apply(
                (
                    Fill::from("MyGradient".gradient()),
                    Stroke::from(Color::black).width(5),
                ),
                Rect::from((100, 100, 600, 200)),
            ),
        ),
    )
}

fn radial<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((8.cm(), 4.cm())).viewbox((0, 0, 800, 400)),
            Fill::default(),
        ),
        (
            border(800, 400),
            apply(
                (
                    Id::from("MyGradient"),
                    RadialGradient::from(Coords::UserSpaceOnUse),
                ),
                (
                    GradientStop::from(0).color(Color::red),
                    GradientStop::from(0.5).color(Color::blue),
                    GradientStop::from(1).color(Color::red),
                ),
            ),
            apply(
                (
                    Fill::from("MyGradient".gradient()),
                    Stroke::from(Color::black).width(5),
                ),
                Rect::from((100, 100, 600, 200)),
            ),
        ),
    )
}
