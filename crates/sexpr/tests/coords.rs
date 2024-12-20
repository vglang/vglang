mod tester;

use tester::{border, svg};
use vglang_sexpr::{
    apply,
    operand::{
        Canvas, Circle, Color, Fill, Font, MeetOrSlice, Path, PathEvent, PreserveAspectRatio, Rect,
        Stroke, Text, Transform,
    },
    Graphic, Stranslate,
};
use vglang_svg::Builder;

#[futures_test::test]
async fn test_coords() {
    svg("aspect", aspect()).await;
}

/// See [`https://www.w3.org/TR/SVG11/coords.html#TransformAttribute`]
fn aspect<G>() -> impl Graphic<G>
where
    G: Builder,
{
    // defines smile logo.
    let smile = || {
        (
            apply(
                (Fill::from(Color::black), Stroke::from(Color::red)),
                Rect::from((0.5, 0.5, 29, 39)),
            ),
            apply(
                (0, 5).translate(),
                (
                    apply(Fill::from(Color::yellow), Circle::from((15, 15, 10))),
                    apply(Fill::from(Color::black), Circle::from((12, 12, 1.5))),
                    apply(Fill::from(Color::black), Circle::from((17, 12, 1.5))),
                    apply(
                        Stroke::from(Color::black).width(2),
                        Path::from((
                            PathEvent::MoveTo((10, 19).into()),
                            PathEvent::MoveTo((10, 19).into()),
                            PathEvent::Arc {
                                rx: 8.0,
                                ry: 8.0,
                                x_rotation: 0.0,
                                sweep: false,
                                large_arc: false,
                                to: (20, 19).into(),
                            },
                        )),
                    ),
                ),
            ),
        )
    };

    let viewport_1 = || {
        apply(
            (Fill::default(), Stroke::from(Color::blue)),
            Rect::from((0.5, 0.5, 49, 29)),
        )
    };

    let viewport_2 = || {
        apply(
            (Fill::default(), Stroke::from(Color::blue)),
            Rect::from((0.5, 0.5, 29, 59)),
        )
    };

    let group_item_1 = |title: &str, transform: Transform, aspect: PreserveAspectRatio| {
        apply(
            transform,
            (
                apply(Text::default().y(-10), title.to_string()),
                viewport_1(),
                apply(
                    Canvas::from((50, 30)).viewbox((0, 0, 30, 40, aspect)),
                    smile(),
                ),
            ),
        )
    };
    let group_item_2 = |title: &str, transform: Transform, aspect: PreserveAspectRatio| {
        apply(
            transform,
            (
                apply(Text::default().y(-10), title.to_string()),
                viewport_2(),
                apply(
                    Canvas::from((30, 60)).viewbox((0, 0, 30, 40, aspect)),
                    smile(),
                ),
            ),
        )
    };

    let meet_group_1 = || {
        (
            apply(
                Text::default().y(-30),
                "--------------- meet ---------------",
            ),
            group_item_1(
                "xMin*",
                (0, 0).translate(),
                PreserveAspectRatio::xMinYMin(MeetOrSlice::Meet),
            ),
            group_item_1(
                "xMid*",
                (70, 0).translate(),
                PreserveAspectRatio::xMidYMid(MeetOrSlice::Meet),
            ),
            group_item_1(
                "xMax*",
                (0, 70).translate(),
                PreserveAspectRatio::xMaxYMax(MeetOrSlice::Meet),
            ),
        )
    };

    let meet_group_2 = || {
        (
            apply(
                Text::default().y(-30),
                "--------------- meet ---------------",
            ),
            group_item_2(
                "*yMin",
                (0, 0).translate(),
                PreserveAspectRatio::xMinYMin(MeetOrSlice::Meet),
            ),
            group_item_2(
                "*yMid",
                (50, 0).translate(),
                PreserveAspectRatio::xMidYMid(MeetOrSlice::Meet),
            ),
            group_item_2(
                "*yMax",
                (100, 0).translate(),
                PreserveAspectRatio::xMaxYMax(MeetOrSlice::Meet),
            ),
        )
    };

    let slice_group_1 = || {
        (
            apply(
                Text::default().y(-30),
                "--------------- slice ---------------",
            ),
            group_item_2(
                "xMin*",
                (0, 0).translate(),
                PreserveAspectRatio::xMinYMin(MeetOrSlice::Slice),
            ),
            group_item_2(
                "xMid*",
                (50, 0).translate(),
                PreserveAspectRatio::xMidYMid(MeetOrSlice::Slice),
            ),
            group_item_2(
                "xMax*",
                (100, 0).translate(),
                PreserveAspectRatio::xMaxYMax(MeetOrSlice::Slice),
            ),
        )
    };

    let slice_group_2 = || {
        (
            apply(
                Text::default().y(-30),
                "--------------- slice ---------------",
            ),
            group_item_1(
                "*YMin",
                (0, 0).translate(),
                PreserveAspectRatio::xMinYMin(MeetOrSlice::Slice),
            ),
            group_item_1(
                "*yMid",
                (70, 0).translate(),
                PreserveAspectRatio::xMidYMid(MeetOrSlice::Slice),
            ),
            group_item_1(
                "*yMax",
                (140, 0).translate(),
                PreserveAspectRatio::xMaxYMax(MeetOrSlice::Slice),
            ),
        )
    };

    // create main canvas.
    apply(
        (
            Canvas::from((900, 600)).viewbox((0, 0, 450, 300)),
            Font::from(9),
        ),
        (
            border(448, 298),
            apply(Text::default().x(10).y(30), "SVG to fit"),
            apply((20, 40).translate(), smile()),
            apply((10, 120).translate(), viewport_1()),
            apply((20, 190).translate(), viewport_2()),
            apply((100, 60).translate(), meet_group_1()),
            apply((250, 220).translate(), slice_group_2()),
            apply((250, 60).translate(), meet_group_2()),
            apply((100, 220).translate(), slice_group_1()),
        ),
    )
}
