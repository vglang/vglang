mod svg;
use svg::svg;
use vglang_sexpr::{apply, operand::*, with_content, Graphic, Slength};
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
        with_content(
            Canvas::from((300, 600)).viewbox((0, 0, 1500, 1000)),
            (
                border(),
                apply(
                    (
                        Fill::from(Color::black),
                        TextLayout::from(TextAnchor::Middle),
                        Font::from("Verdana").size(200),
                    ),
                    with_content(
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
        with_content(
            Canvas::from((300, 600)).viewbox((
                0,
                0,
                1500,
                1000,
                PreserveAspectRatio::xMinYMin(MeetOrSlice::Meet),
            )),
            (
                border(),
                apply(
                    (
                        Fill::from(Color::black),
                        TextLayout::from(TextAnchor::Middle),
                        Font::from(FontFamily::from("Verdana")).size(200),
                    ),
                    with_content(Text::from((50.percent(), 60.percent())), "Stretch to fit"),
                ),
            ),
        ),
    )
    .await;

    svg(
        "tspan",
        with_content(
            Canvas::from((800, 600)),
            apply(
                (
                    Font::from(3.em()),
                    TextLayout::from(TextAnchor::Middle)
                        .dominant_baseline(DominantBaseline::Hanging),
                ),
                (
                    border(),
                    with_content(
                        Text::from((50.percent(), 50.percent())),
                        (
                            "You are",
                            with_content(
                                TextSpan::default()
                                    .font(Font::from(FontWeight::Bolder).size(2.em()))
                                    .fill(Color::red),
                                " NOT(1)",
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
