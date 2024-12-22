mod tester;
use tester::{border, svg};
use vglang_sexpr::{
    apply, close, line_to, move_to,
    operand::{
        Canvas, Color, Coords, Ellipse, Fill, GradientStop, Id, LinearGradient, Path, Pattern,
        RadialGradient, Rect, Stroke,
    },
    Graphic, Sgradient, Slength, Spattern,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_gradient() {
    svg("linear", linear()).await;
    svg("radial", radial()).await;
}

#[futures_test::test]
async fn test_pattern() {
    svg("pattern", pattern()).await;
}

fn pattern<G>() -> impl Graphic<G>
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
                    Id::from("TrianglePattern"),
                    Pattern::from((0, 0, 100, 100))
                        .viewbox((0, 0, 10, 10))
                        .units(Coords::UserSpaceOnUse),
                    Fill::from(Color::red),
                    Stroke::from(Color::blue),
                ),
                Path::from((move_to(0, 0), line_to(7, 0), line_to(3.5, 7), close())),
            ),
            apply(
                (
                    Fill::from("TrianglePattern".pattern()),
                    Stroke::from(Color::black).width(5),
                ),
                Ellipse::from((400, 200, 350, 150)),
            ),
        ),
    )
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
