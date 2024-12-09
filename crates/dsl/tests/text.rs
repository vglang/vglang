mod dsl;
use cotati_dsl::drawing::{apply, layer, rect, text, Graphic};
use cotati_ir::{Fill, Layer, Measurement, RecognizedColor, Rect, Stroke, Text};
use dsl::dsl_test;

#[futures_test::test]
async fn test_text() {
    dsl_test("basic", |g| {
        layer(
            Layer::attrs()
                .width(Measurement::cm(10.0))
                .height(Measurement::cm(3.0))
                .viewbox((0.0, 0.0, 1000.0, 300.0)),
            (
                // apply `fill`,`stroke` attributes to `rect` element.
                apply(
                    (
                        Fill::default(),
                        Stroke::attrs().paint(RecognizedColor::blue),
                    ),
                    rect(Rect::attrs().x(1).y(1).width(998).height(298).rx(20).ry(50)),
                ),
                text(
                    Text::attrs().x(250).y(150).rotate((-10, 20, 0, 0, 0, 10)),
                    "Hello VGL.",
                ),
            ),
        )
        .draw(g);
    })
    .await;
}
