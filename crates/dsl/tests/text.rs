mod dsl;
use cotati_dsl::drawing::{apply, layer, with};
use cotati_ir::{
    Color, Fill, FontFamily, FontSize, FontStyle, Layer, Measurement, Rect, Stroke, Text,
};
use dsl::svg;

#[futures_test::test]
async fn test_text() {
    svg(
        "basic",
        layer(
            Layer::default()
                .width(Measurement::cm(10.0))
                .height(Measurement::cm(3.0))
                .viewbox((0.0, 0.0, 1000.0, 300.0)),
            (
                // apply `fill`,`stroke` to `rect` element.
                apply(
                    (Fill::default(), Stroke::default().paint(Color::blue)),
                    Rect::default().x(1).y(1).width(998).height(298).rx(20),
                ),
                apply(
                    (FontFamily::Monospace, FontSize::from(50), FontStyle::Italic),
                    // append child element to `Text`.
                    with(
                        Text::default().x(450).y(150).rotate((-10, 20, 0, 0, 0, 10)),
                        "Hello VGL.",
                    ),
                ),
            ),
        ),
    )
    .await;
}
