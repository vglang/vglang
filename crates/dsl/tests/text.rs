mod dsl;
use cotati_dsl::drawing::{apply, layer, rect, text};
use cotati_ir::{Fill, Layer, Measurement, RecognizedColor, Rect, Stroke, Text};
use dsl::svg;

#[futures_test::test]
async fn test_text() {
    svg(
        "basic",
        layer(
            Layer::attrs()
                .width(Measurement::cm(10.0))
                .height(Measurement::cm(3.0))
                .viewbox((0.0, 0.0, 1000.0, 300.0)),
            (
                // apply `fill`,`stroke` to `rect` element.
                apply(
                    (
                        Fill::default(),
                        Stroke::attrs().paint(RecognizedColor::blue),
                    ),
                    rect(Rect::attrs().x(1).y(1).width(998).height(298).rx(20)),
                ),
                text(
                    Text::attrs().x(450).y(150).rotate((-10, 20, 0, 0, 0, 10)),
                    "Hello VGL.",
                ),
            ),
        ),
    )
    .await;
}
