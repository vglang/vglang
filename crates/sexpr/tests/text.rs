mod sexpr;
use sexpr::svg;
use vglang_opcode::{
    attrs::{Fill, Stroke},
    data::Color,
    el::{Characters, Text, TextSpan},
};
use vglang_sexpr::*;

#[futures_test::test]
async fn test_text() {
    svg("test", Text::from((0, 0))).await;
    svg(
        "test",
        Text::from((0, 0))
            .apply(Fill::from(Color::red))
            .apply(Stroke::default())
            .child(Characters::from("hello world"))
            .child(TextSpan::default()),
    )
    .await;
}
