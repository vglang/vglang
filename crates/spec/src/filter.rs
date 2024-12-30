use vglang::{
    opcode::{
        attrs::{Fill, Id, Opacity, Stroke, WithTransform},
        data::{Color, Coords, FeIn},
        el::{
            Canvas, Circle, FeGaussianBlur, FeMerge, FeMergeNode, FeOffset, Filter, Group, Polygon,
            Rect,
        },
    },
    sexpr::{Graphics, Slength, Stranslate},
};

/// Example from [`enable-background-01`](https://www.w3.org/TR/SVG11/images/filters/enable-background-01.svg)
pub fn enable_background_01() -> impl Graphics {
    Canvas::from((13.5.cm(), 2.7.cm())).children((
        Filter::from((0, 0, 1200, 400))
            .units(Coords::UserSpaceOnUse)
            .apply(Id::from("ShiftBGAndBlur"))
            .children((
                FeOffset::from(FeIn::BackgroundImage).dx(0).dy(125),
                FeGaussianBlur::default().std_deviation(8),
            )),
        Filter::from((0, 0, 1200, 400))
            .units(Coords::UserSpaceOnUse)
            .apply(Id::from("ShiftBGAndBlur_WithSourceGraphic"))
            .children((
                FeOffset::from(FeIn::BackgroundImage).dx(0).dy(125),
                FeGaussianBlur::from("blur").std_deviation(8),
                FeMerge::default().children((
                    FeMergeNode::from("blur"),
                    FeMergeNode::from(FeIn::SourceGraphic),
                )),
            )),
        // The first picture is our reference graphic without filters.
        Group
            .apply(WithTransform::from((0, 0).translate()))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::blue)),
                )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
        // The second adds an empty 'g' element which invokes ShiftBGAndBlur.
        Group
            .apply(WithTransform::from((270, 0).translate()))
            .children((
                Rect::from((25, 25, 100, 100)).apply(Fill::from(Color::red)),
                Group.apply(Opacity::from(0.5)).children((
                    Circle::from((125, 75, 45)).apply(Fill::from(Color::green)),
                    Polygon::from((160, 25, 160, 125, 240, 75)).apply(Fill::from(Color::blue)),
                )),
                Rect::from((5, 5, 260, 260)).apply((Fill::default(), Stroke::from(Color::blue))),
            )),
    ))
}
