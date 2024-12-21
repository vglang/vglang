mod tester;

use tester::*;
use vglang_sexpr::{
    apply, cubic_bezier, move_to,
    operand::{Canvas, Color, Fill, Font, FontWeight, Id, Path, Stroke, Text, TextPath, Use},
    Graphic, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_text() {
    svg("text path", text_path()).await;
}
fn text_path<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        Canvas::from((12.cm(), 3.6.cm())).viewbox((0, 0, 1000, 300)),
        (
            border(1000, 300),
            apply(
                Id::from("MyPath"),
                Path::from((
                    move_to(100, 200),
                    cubic_bezier((200, 100), (300, 0), (400, 100)),
                    cubic_bezier((500, 200), (600, 300), (700, 200)),
                    cubic_bezier((800, 100), (900, 100), (900, 100)),
                )),
            ),
            apply(
                (Fill::default(), Stroke::from(Color::red)),
                Use::from("MyPath"),
            ),
            apply(
                (
                    Font::from(42.5).family("Verdana").weight(FontWeight::W100),
                    Fill::from(Color::blue),
                    Stroke::from(Color::red).width(2),
                    Text::default(),
                    TextPath::from("MyPath"),
                ),
                " We go up, then we go down, then up again",
            ),
        ),
    )
}
