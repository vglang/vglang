mod dsl;
use cotati_dsl::drawing::{text, Graphic};
use cotati_ir::Text;
use dsl::dsl_test;

#[futures_test::test]
async fn test_text() {
    dsl_test("basic", |g| {
        text(Text::default(), "hello world").draw(g);
    })
    .await;
}
