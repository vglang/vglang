mod dsl;
use dsl::svg;
use vglang_dsl::{
    attrs::*,
    drawing::{apply, layer, with},
};

#[futures_test::test]
async fn test_viewbox() {
    svg(
        "stretch to fit",
        layer(
            Layer::from((300, 600)).viewbox(ViewBox::from((0, 0, 1500, 1000))),
            apply(
                (
                    Fill::from(Color::yellow),
                    Stroke::from(Color::blue).width(1),
                ),
                (
                    Rect::from((0, 0, 100.percentage(), 100.percentage())).rx(20),
                    apply(
                        (
                            Fill::from(Color::black),
                            TextLayout::from(TextAnchor::Middle),
                            Font::from(FontFamily::from("Verdana")).size(200),
                        ),
                        with(
                            Text::default().x(50.percentage()).y(60.percentage()),
                            "Stretch to fit",
                        ),
                    ),
                ),
            ),
        ),
    )
    .await;
}
