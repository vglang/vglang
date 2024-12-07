mod dsl;
use cotati_dsl::drawing::{layer, text, Graphic};
use cotati_ir::{Layer, Text};
use dsl::dsl_test;

#[futures_test::test]
async fn test_text() {
    dsl_test("basic", |g| {
        layer(
            Layer::default().width(300).height(200),
            text(
                Text::default().y(vec![100, 105, 110, 115, 120]),
                "hello world",
            ),
        )
        .draw(g);
    })
    .await;
}
