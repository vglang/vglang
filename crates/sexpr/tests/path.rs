mod tester;
use tester::{border, svg};
use vglang_sexpr::{
    apply, close, cubic_bezier, line_to, move_to,
    operand::{
        Canvas, Circle, Color, Fill, Font, Id, Path, Polyline, Rect, Rgb, Stroke, Text, Use,
    },
    quadratic_bezier, Graphic, Slength,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_path() {
    svg("triangle", triangle()).await;
    svg("cubic", cubic()).await;
    svg("quadratic", quadratic()).await;
    svg("arc", arc()).await;
}

fn arc<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        Canvas::from((12.cm(), 5.25.cm())).viewbox((0, 0, 1200, 400)),
        (
            border(1200, 400),
            apply(
                (Fill::from(Color::red), Stroke::from(Color::blue).width(5)),
                Path::from((
                    move_to(300, 200),
                    line_to(150, 200),
                    vglang_sexpr::arc(150, 150, 0, true, false, (300, 50)),
                    close(),
                )),
            ),
            apply(
                (
                    Fill::from(Color::yellow),
                    Stroke::from(Color::blue).width(5),
                ),
                Path::from((
                    move_to(275, 175),
                    line_to(275, 25),
                    vglang_sexpr::arc(150, 150, 0, false, false, (125, 175)),
                    close(),
                )),
            ),
            apply(
                (Fill::default(), Stroke::from(Color::red).width(5)),
                Path::from((
                    move_to(600, 350),
                    line_to(650, 325),
                    vglang_sexpr::arc(25, 25, -30, false, true, (700, 300)),
                    line_to(750, 275),
                    vglang_sexpr::arc(25, 50, -30, false, true, (800, 250)),
                    line_to(850, 225),
                    vglang_sexpr::arc(25, 75, -30, false, true, (900, 200)),
                    line_to(950, 175),
                    vglang_sexpr::arc(25, 100, -30, false, true, (1000, 150)),
                    line_to(1050, 125),
                )),
            ),
        ),
    )
}

fn quadratic<G>() -> impl Graphic<G>
where
    G: Builder,
{
    apply(
        Canvas::from((12.cm(), 6.cm())).viewbox((0, 0, 1200, 600)),
        (
            border(1200, 600),
            apply(
                (Fill::default(), Stroke::from(Color::red).width(5)),
                Path::from((
                    move_to(200, 300),
                    quadratic_bezier((400, 50), (600, 300)),
                    quadratic_bezier((800, 550), (1000, 300)),
                )),
            ),
            apply(
                Fill::from(Color::black),
                (
                    Circle::from((200, 300, 10)),
                    Circle::from((600, 300, 10)),
                    Circle::from((1000, 300, 10)),
                ),
            ),
            apply(
                Fill::from((0x88, 0x88, 0x88)),
                (Circle::from((400, 50, 10)), Circle::from((800, 550, 10))),
            ),
            apply(
                (Fill::default(), Stroke::from((0x88, 0x88, 0x88)).width(2)),
                Path::from((
                    move_to(200, 300),
                    line_to(400, 50),
                    line_to(600, 300),
                    line_to(800, 550),
                    line_to(1000, 300),
                )),
            ),
        ),
    )
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
                Path::from((
                    move_to(100, 100),
                    line_to(300, 100),
                    line_to(200, 300),
                    close(),
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
            (
                Fill::default(),
                Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
            ),
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
            (Fill::from(Rgb::rgb(0x88, 0x88, 0x88)), Stroke::default()),
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
            (
                Fill::default(),
                Stroke::from(Rgb::rgb(0x88, 0x88, 0x88)).width(2),
            ),
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
            sample_path_style(Path::from((
                move_to(100, 200),
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
