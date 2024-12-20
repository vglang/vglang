use crate::Graphic;
use num::ToPrimitive;
use vglang_opcode::{
    operand::{Path, PathEvent, Point},
    MapCollect,
};
use vglang_targets::Builder;

impl<G> Graphic<G> for Path
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}

/// A path builder element must implement this trait.
pub trait PathEventBuilder {
    /// create new path events.
    fn build(self, events: &mut Vec<PathEvent>);
}

impl<F> PathEventBuilder for F
where
    F: FnOnce(&mut Vec<PathEvent>),
{
    fn build(self, events: &mut Vec<PathEvent>) {
        self(events)
    }
}

macro_rules! tuple_path_builder {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+> PathEventBuilder for ($header, $($tail),+)
        where
            $header: PathEventBuilder,
            $($tail: PathEventBuilder),+,
        {
            #[allow(non_snake_case)]
            fn build(self, events: &mut Vec<PathEvent>) {
                let ($header, $($tail),+) = self;
                $header.build(events);
                $($tail.build(events);)+
            }
        }

        tuple_path_builder!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_path_builder!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
    A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39
);

/// Create [`PathEvent::MoveTo`].
pub fn move_to<P>(point: P) -> impl PathEventBuilder
where
    Point: From<P>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::MoveTo(point.into()));
    }
}

/// Create [`PathEvent::LineTo`].
pub fn line_to<P>(point: P) -> impl PathEventBuilder
where
    Point: From<P>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::LineTo(point.into()));
    }
}

/// Create [`PathEvent::CubicBezier`].
pub fn cubic_bezier<C1, C2, T>(ctrl1: C1, ctrl2: C2, to: T) -> impl PathEventBuilder
where
    Point: From<C1> + From<C2> + From<T>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::CubicBezier {
            ctrl1: ctrl1.into(),
            ctrl2: ctrl2.into(),
            to: to.into(),
        });
    }
}

/// Create [`PathEvent::QuadraticBezier`].
pub fn quadratic_bezier<C, T>(ctrl: C, to: T) -> impl PathEventBuilder
where
    Point: From<C> + From<T>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::QuadraticBezier {
            ctrl: ctrl.into(),
            to: to.into(),
        });
    }
}

/// Create [`PathEvent::Arc`].
pub fn arc<RX, RY, R, T>(
    rx: RX,
    ry: RY,
    x_rotation: R,
    large_arc: bool,
    sweep: bool,
    to: T,
) -> impl PathEventBuilder
where
    RX: ToPrimitive,
    RY: ToPrimitive,
    R: ToPrimitive,
    Point: From<T>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::Arc {
            rx: rx.to_f32().unwrap(),
            ry: ry.to_f32().unwrap(),
            x_rotation: x_rotation.to_f32().unwrap(),
            large_arc,
            sweep,
            to: to.into(),
        });
    }
}

/// Create [`PathEvent::Close`].
pub fn close_path() -> impl PathEventBuilder {
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::Close);
    }
}

/// Create [`PathEvent::Polyline`].
pub fn polyline<P>(data: P) -> impl PathEventBuilder
where
    P: MapCollect<Point>,
{
    move |events: &mut Vec<PathEvent>| {
        events.push(PathEvent::Polyline(data.map_collect()));
    }
}

/// Create a new [`Path`]
pub fn path<P>(events: P) -> Path
where
    P: PathEventBuilder,
{
    let mut data = Vec::new();

    events.build(&mut data);

    Path {
        data,
        ..Default::default()
    }
}
