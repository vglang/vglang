mod svg;
use svg::svg;
use vglang_sexpr::{apply, canvas, operand::*, with, Graphic, Slength};
use vglang_svg::Builder;

fn border<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        (
            Fill::from(Color::yellow),
            Stroke::from(Color::yellowgreen).width(1),
        ),
        Rect::from((0, 0, 100.percent(), 100.percent())).rx(20),
    )
}

#[futures_test::test]
async fn test_viewbox() {
    svg(
        "stretch to fit",
        canvas(
            Canvas::from((300, 600)).viewbox((0, 0, 1500, 1000)),
            (
                border(),
                apply(
                    (
                        Fill::from(Color::black),
                        TextLayout::from(TextAnchor::Middle),
                        Font::from("Verdana").size(200),
                    ),
                    with(
                        Text::default().x(50.percent()).y(60.percent()),
                        "Stretch to fit",
                    ),
                ),
            ),
        ),
    )
    .await;

    svg(
        "PreserveAspectRatio",
        canvas(
            Canvas::from((300, 600)).viewbox((
                0,
                0,
                1500,
                1000,
                PreserveAspectRatio::xMidYMid(MeetOrSlice::Meet),
            )),
            (
                border(),
                apply(
                    (
                        Fill::from(Color::black),
                        TextLayout::from(TextAnchor::Middle),
                        Font::from(FontFamily::from("Verdana")).size(200),
                    ),
                    with(Text::from((50.percent(), 60.percent())), "Stretch to fit"),
                ),
            ),
        ),
    )
    .await;

    svg(
        "tspan",
        canvas(
            Canvas::from((800, 600)),
            apply(
                (Font::from(1.em()), TextLayout::from(TextAnchor::Middle)),
                (
                    border(),
                    with(
                        Text::from((50.percent(), 50.percent())),
                        (
                            "You are",
                            with(
                                TextSpan::default()
                                    .font(Font::from(FontWeight::Bolder))
                                    .fill(Color::red),
                                " not",
                            ),
                            " a banana",
                        ),
                    ),
                ),
            ),
        ),
    )
    .await;
}
