mod sexpr;
use sexpr::svg;

use vglang_opcode::{
    attrs::{Fill, Stroke},
    data::Color,
    el::{Characters, Text, TextSpan},
};
use vglang_sexpr::{Graphics, Scontainer};
use vglang_surface::Builder;

#[futures_test::test]
async fn test_text() {
    svg("text_path", text_path()).await;
}

fn text_path<B>() -> impl Graphics<B>
where
    B: Builder,
{
    Text::from((0, 0))
        .apply(Fill::from(Color::red))
        .apply(Stroke::default())
        .child(Characters::from("hello world"))
        .child(TextSpan::default())
}
