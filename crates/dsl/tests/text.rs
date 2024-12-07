mod dsl;
use cotati_dsl::drawing::{layer, text, Graphic};
use cotati_ir::{Layer, Text};
use dsl::dsl_test;

#[futures_test::test]
async fn test_text() {
    dsl_test("basic", |g| {
        layer(
            Layer::default().width(300.0).height(200.0),
            text(
                Text::default().y(vec![
                    100.0.into(),
                    105.0.into(),
                    110.0.into(),
                    115.0.into(),
                    120.0.into(),
                ]),
                "hello world",
            ),
        )
        .draw(g);
    })
    .await;
}
