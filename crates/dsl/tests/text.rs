mod dsl;
use cotati_dsl::drawing::{layer, text, Graphic};
use cotati_ir::{Layer, Text};
use dsl::dsl_test;

#[futures_test::test]
async fn test_text() {
    dsl_test("basic", |g| {
        layer(
            Layer::attrs().width(300).height(200),
            text(
                Text::attrs()
                    .x(100)
                    .y((103, 107, 111, 115, 120, 120, 120, 115, 110, 105, 110))
                    .rotate((-10, 20, 0, 0, 0, 10)),
                "Hello VGL.",
            ),
        )
        .draw(g);
    })
    .await;
}
