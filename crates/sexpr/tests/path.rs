mod tester;
use tester::{border, svg};
use vglang_sexpr::{
    apply, close_path, cubic_bezier, line_to, move_to,
    operand::{
        Canvas, Circle, Color, Fill, Font, Id, Path, Polyline, Rect, Rgb, Stroke, Text, Use,
    },
    FromPathEventBuilder, Graphic, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_path() {
    svg("triangle", triangle()).await;
    svg("cubic", cubic()).await;
}

fn triangle<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        Canvas::from((4.cm(), 4.cm())).viewbox((0, 0, 400, 400)),
        (
            border(398, 398),
            apply(
                (Fill::from(Color::red), Stroke::from(Color::blue).width(3)),
                Path::from_events((
                    move_to((100, 100)),
                    line_to((300, 100)),
                    line_to((200, 300)),
                    close_path(),
                )),
            ),
        ),
    )
}

fn cubic<G>() -> impl Graphic<G>
where
    G: Builder,
{
    fn border_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::default(), Stroke::from(Color::blue).width(1)),
            content,
        )
    }

    fn connect_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::default(), Stroke::from(Rgb::rgb(88, 88, 88)).width(2)),
            content,
        )
    }

    fn sample_path_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::default(), Stroke::from(Color::red).width(5)),
            content,
        )
    }

    fn ctrl_point_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::from(Rgb::rgb(88, 88, 88)), Stroke::default()),
            content,
        )
    }

    fn auto_ctrl_point_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::default(), Stroke::from(Color::blue).width(4)),
            content,
        )
    }

    fn end_point_style<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(
            (Fill::default(), Stroke::from(Rgb::rgb(88, 88, 88)).width(2)),
            content,
        )
    }

    fn label<G, C>(content: C) -> impl Graphic<G>
    where
        G: Builder,
        C: Graphic<G>,
    {
        apply(Font::from(22).family("Verdana"), content)
    }

    apply(
        Canvas::from((5.cm(), 4.cm())).viewbox((0, 0, 500, 400)),
        (
            border_style(Rect::from((1, 1, 498, 398))),
            sample_path_style(Path::from_events((
                move_to((100, 200)),
                cubic_bezier((100, 100), (250, 100), (250, 200)),
                cubic_bezier((250, 300), (400, 300), (400, 200)),
            ))),
            connect_style((
                Polyline::from((100, 200, 100, 100)),
                Polyline::from((250, 100, 250, 200)),
                Polyline::from((250, 200, 250, 300)),
                Polyline::from((400, 300, 400, 200)),
            )),
            end_point_style((
                Circle::from((100, 200, 10)),
                Circle::from((250, 200, 10)),
                Circle::from((400, 200, 10)),
            )),
            ctrl_point_style((
                Circle::from((100, 100, 10)),
                Circle::from((250, 100, 10)),
                Circle::from((400, 300, 10)),
            )),
            auto_ctrl_point_style(Circle::from((250, 300, 9))),
            label(apply(
                Text::from((25, 70)),
                "M100,200 C100,100 250,100 250,200",
            )),
            apply(
                Id::from("content"),
                label(apply(Text::from((220, 350)), "S400,300 400,200")),
            ),
            apply(Fill::from(Color::chocolate), Use::from("content")),
        ),
    )
}
