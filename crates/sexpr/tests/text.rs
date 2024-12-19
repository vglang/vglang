mod svg;
use svg::svg;
use vglang_sexpr::{apply, operand::*, Graphic, Slength};
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
        apply(
            (
                Canvas::from((300, 600)).viewbox((0, 0, 1500, 1000)),
                Fill::from(Color::black),
                TextLayout::from(TextAnchor::Middle),
                Font::from("Verdana").size(200),
            ),
            (
                border(),
                apply(
                    Text::default().x(50.percent()).y(60.percent()),
                    "Stretch to fit",
                ),
            ),
        ),
    )
    .await;

    svg(
        "PreserveAspectRatio",
        apply(
            (
                Canvas::from((300, 600)).viewbox((
                    0,
                    0,
                    1500,
                    1000,
                    PreserveAspectRatio::xMinYMin(MeetOrSlice::Meet),
                )),
                Fill::from(Color::black),
                TextLayout::from(TextAnchor::Middle),
                Font::from(FontFamily::from("Verdana")).size(200),
            ),
            (
                border(),
                apply(Text::from((50.percent(), 60.percent())), "Stretch to fit"),
            ),
        ),
    )
    .await;

    svg(
        "tspan",
        apply(
            (
                Canvas::from((200, 400)).viewbox((
                    0,
                    0,
                    1500,
                    1000,
                    PreserveAspectRatio::xMidYMid(MeetOrSlice::Meet),
                )),
                Font::from(3.em()),
                TextLayout::from(TextAnchor::Middle).dominant_baseline(DominantBaseline::Hanging),
            ),
            (
                border(),
                apply(
                    Text::from((50.percent(), 50.percent())),
                    (
                        "You are",
                        apply(
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
    )
    .await;
}
