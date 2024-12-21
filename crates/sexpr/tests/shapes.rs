mod tester;
use tester::{border, svg};
use vglang_sexpr::{
    apply,
    operand::{Canvas, Color, Fill, Polygon, Polyline, Stroke},
    Graphic, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_shapes() {
    svg("polyline", polyline()).await;
    svg("polygon", polygon()).await;
}

fn polyline<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((12.cm(), 4.cm())).viewbox((0, 0, 1200, 400)),
            Fill::default(),
            Stroke::from(Color::blue).width(10),
        ),
        (
            border(1200, 400),
            Polyline::from((
                50, 375, 150, 375, 150, 325, 250, 325, 250, 375, 350, 375, 350, 250, 450, 250, 450,
                375, 550, 375, 550, 175, 650, 175, 650, 375, 750, 375, 750, 100, 850, 100, 850,
                375, 950, 375, 950, 25, 1050, 25, 1050, 375, 1150, 375,
            )),
        ),
    )
}

fn polygon<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Canvas::from((12.cm(), 4.cm())).viewbox((0, 0, 1200, 400)),
            Fill::default(),
            Stroke::from(Color::blue).width(10),
        ),
        (
            border(1200, 400),
            apply(
                (Fill::from(Color::red), Stroke::from(Color::blue).width(10)),
                Polygon::from((
                    350, 75, 379, 161, 469, 161, 397, 215, 423, 301, 350, 250, 277, 301, 303, 215,
                    231, 161, 321, 161,
                )),
            ),
            apply(
                (Fill::from(Color::lime), Stroke::from(Color::blue).width(10)),
                Polygon::from((
                    850, 75, 958, 137.5, 958, 262.5, 850, 325, 742, 262.6, 742, 137.5,
                )),
            ),
        ),
    )
}
