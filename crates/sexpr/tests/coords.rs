use svg::svg;
use vglang_sexpr::{apply, operand::Canvas, Graphic};
use vglang_svg::Builder;

mod svg;

#[futures_test::test]
async fn test_coords() {
    svg("aspect", aspect()).await;
}

fn aspect<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(Canvas::from((450, 300)), "")
}
