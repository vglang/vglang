mod dsl;
use dsl::svg;
use vglang_dsl::{
    attrs::*,
    dsl::{apply, layer, with, Graphic, MeasurementDsl},
    generator::Generator,
};

fn border<G>() -> impl Graphic<G>
where
    G: Generator,
{
    apply(
        (
            Fill::from(Color::yellow),
            Stroke::from(Color::yellowgreen).width(1),
        ),
        Rect::from((0, 0, 100.percentage(), 100.percentage())).rx(20),
    )
}

#[futures_test::test]
async fn test_viewbox() {
    svg(
        "stretch to fit",
        layer(
            Layer::from((300, 600)).viewbox((0, 0, 1500, 1000)),
            (
                border(),
                apply(
                    (
                        Fill::from(Color::black),
                        TextLayout::from(TextAnchor::Middle),
                        Font::from("Verdana").size(200),
                    ),
                    with(
                        Text::default().x(50.percentage()).y(60.percentage()),
                        "Stretch to fit",
                    ),
                ),
            ),
        ),
    )
    .await;

    svg(
        "PreserveAspectRatio",
        layer(
            Layer::from((300, 600)).viewbox((
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
                    with(
                        Text::from((50.percentage(), 60.percentage())),
                        "Stretch to fit",
                    ),
                ),
            ),
        ),
    )
    .await;

    svg(
        "tspan",
        layer(
            Layer::from((800, 600)),
            apply(
                (Font::from(1.em()), TextLayout::from(TextAnchor::Middle)),
                (
                    border(),
                    with(
                        Text::from((50.percentage(), 50.percentage())),
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
